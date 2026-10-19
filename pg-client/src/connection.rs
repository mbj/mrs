//! # No late TLS upgrade
//!
//! The `SSLRequest` handshake (ask in plaintext, read a bare `S` or `N` byte,
//! then upgrade) is deliberately left out, because every hazard it carries is
//! a hazard direct TLS does not have. Its preamble is plaintext and the
//! protocol gives the server no way to force encryption, so a server with SSL
//! off and an attacker answering `N` are indistinguishable. After `S` the
//! server sends exactly one byte and waits, and any further readable byte is a
//! man-in-the-middle stuffing the buffer to be read back later as though it had
//! arrived encrypted ([CVE-2021-23222]). The reply may instead be an
//! `ErrorResponse` that must never be shown, the server being unauthenticated
//! at that point ([CVE-2024-10977]). And it spends a round trip to reach a worse place.
//!
//! The cost is that a server older than PostgreSQL 17 cannot be reached over
//! TCP at all.
//!
//! [CVE-2021-23222]: https://www.postgresql.org/support/security/CVE-2021-23222/
//! [CVE-2024-10977]: https://www.postgresql.org/support/security/CVE-2024-10977/

use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf};

use crate::protocol::{
    self, Frame, ReceiveBuffer, ReceiveBufferSize, ReceiveError, SendBuffer, SendError,
};

#[derive(Clone, Debug)]
pub struct Config {
    pub endpoint: Endpoint,
    pub receive_buffer: ReceiveBufferSize,
}

#[derive(Clone, Debug)]
pub enum Endpoint {
    Tcp {
        host: crate::config::Host,
        /// PostgreSQL's `hostaddr`.
        address: Option<crate::config::HostAddr>,
        port: crate::config::Port,
        tls: Option<Tls>,
    },
    /// PostgreSQL does not encrypt Unix-socket connections.
    Unix { path: std::path::PathBuf },
}

#[derive(Clone, Debug)]
pub struct Tls {
    client: std::sync::Arc<rustls::ClientConfig>,
}

impl Tls {
    #[must_use]
    pub fn new(roots: rustls::RootCertStore) -> Self {
        Self::from_client_config(
            rustls::ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth(),
        )
    }

    /// For a configuration of your own, for example one presenting a client
    /// certificate. Its ALPN protocols are replaced: direct TLS requires
    /// exactly `postgresql`.
    #[must_use]
    pub fn from_client_config(mut client: rustls::ClientConfig) -> Self {
        client.alpn_protocols = vec![ALPN_PROTOCOL.to_vec()];

        Self {
            client: std::sync::Arc::new(client),
        }
    }

    #[must_use]
    pub const fn client(&self) -> &std::sync::Arc<rustls::ClientConfig> {
        &self.client
    }

    pub fn from_certificates(
        certificates: impl IntoIterator<
            Item = Result<
                rustls::pki_types::CertificateDer<'static>,
                rustls::pki_types::pem::Error,
            >,
        >,
    ) -> Result<Self, RootCertificateError> {
        let mut roots = rustls::RootCertStore::empty();

        for certificate in certificates {
            roots
                .add(certificate.map_err(RootCertificateError::Read)?)
                .map_err(RootCertificateError::TrustAnchor)?;
        }

        if roots.is_empty() {
            return Err(RootCertificateError::Empty);
        }

        Ok(Self::new(roots))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RootCertificateError {
    #[error("not readable as PEM")]
    Read(#[source] rustls::pki_types::pem::Error),
    #[error("not a valid trust anchor")]
    TrustAnchor(#[source] rustls::Error),
    #[error("contains no certificate")]
    Empty,
}

pub trait Transport: AsyncRead + AsyncWrite + Send {}

impl<T: AsyncRead + AsyncWrite + Send> Transport for T {}

impl<T: AsyncRead + Unpin + Send> protocol::Reader for T {
    fn read<'a>(
        &'a mut self,
        buffer: &'a mut Vec<u8>,
    ) -> impl Future<Output = std::io::Result<usize>> + Send + 'a {
        AsyncReadExt::read_buf(self, buffer)
    }
}

impl<T: AsyncWrite + Unpin + Send> protocol::Writer for T {
    fn write<'a>(
        &'a mut self,
        bytes: &'a [u8],
    ) -> impl Future<Output = std::io::Result<usize>> + Send + 'a {
        AsyncWriteExt::write(self, bytes)
    }

    fn flush(&mut self) -> impl Future<Output = std::io::Result<()>> + Send {
        AsyncWriteExt::flush(self)
    }
}

pub enum Stream {
    Tcp(tokio::net::TcpStream),
    Unix(tokio::net::UnixStream),
    Tls(Box<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>),
    Boxed(Pin<Box<dyn Transport>>),
}

impl Stream {
    #[must_use]
    pub fn peer_certificate(&self) -> Option<&rustls::pki_types::CertificateDer<'static>> {
        match self {
            Self::Tls(stream) => stream
                .get_ref()
                .1
                .peer_certificates()
                .and_then(<[_]>::first),
            Self::Tcp(_) | Self::Unix(_) | Self::Boxed(_) => None,
        }
    }

    #[must_use]
    pub fn boxed<T: Transport + 'static>(stream: T) -> Self {
        Self::Boxed(Box::pin(stream))
    }

    pub async fn tls(
        socket: tokio::net::TcpStream,
        config: std::sync::Arc<tokio_rustls::rustls::ClientConfig>,
        server_name: tokio_rustls::rustls::pki_types::ServerName<'static>,
    ) -> Result<Self, TlsError> {
        let stream = tokio_rustls::TlsConnector::from(config)
            .connect(server_name, socket)
            .await
            .map_err(TlsError::Handshake)?;

        let selected = stream.get_ref().1.alpn_protocol();

        if selected == Some(ALPN_PROTOCOL) {
            Ok(Self::Tls(Box::new(stream)))
        } else {
            Err(TlsError::Alpn {
                selected: selected.map(|value| String::from_utf8_lossy(value).into_owned()),
            })
        }
    }
}

pub use tokio_rustls::rustls;

/// The ALPN protocol id direct TLS requires, as registered with IANA.
pub const ALPN_PROTOCOL: &[u8] = b"postgresql";

#[derive(Debug, thiserror::Error)]
pub enum TlsError {
    #[error("TLS handshake failed")]
    Handshake(#[source] std::io::Error),
    /// ALPN is what keeps the connection from being handed to a protocol that
    /// is not this one.
    #[error("server selected ALPN protocol {selected:?}, not \"postgresql\"")]
    Alpn { selected: Option<String> },
}

impl core::fmt::Debug for Stream {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Tcp(stream) => formatter.debug_tuple("Tcp").field(stream).finish(),
            Self::Unix(stream) => formatter.debug_tuple("Unix").field(stream).finish(),
            Self::Tls(stream) => {
                let (socket, session) = stream.get_ref();

                formatter
                    .debug_struct("Tls")
                    .field("socket", socket)
                    .field("handshaking", &session.is_handshaking())
                    .field("handshake_kind", &session.handshake_kind())
                    .field("version", &session.protocol_version())
                    .field("cipher_suite", &session.negotiated_cipher_suite())
                    .field(
                        "key_exchange",
                        &session
                            .negotiated_key_exchange_group()
                            .map(|group| group.name()),
                    )
                    .field(
                        "alpn",
                        &session.alpn_protocol().map(String::from_utf8_lossy),
                    )
                    .field(
                        "peer_certificates",
                        &session.peer_certificates().map(<[_]>::len),
                    )
                    .field("wants_read", &session.wants_read())
                    .field("wants_write", &session.wants_write())
                    .finish()
            }
            Self::Boxed(_) => formatter.write_str("Boxed"),
        }
    }
}

impl From<tokio::net::TcpStream> for Stream {
    fn from(stream: tokio::net::TcpStream) -> Self {
        Self::Tcp(stream)
    }
}

impl From<tokio::net::UnixStream> for Stream {
    fn from(stream: tokio::net::UnixStream) -> Self {
        Self::Unix(stream)
    }
}

impl AsyncRead for Stream {
    fn poll_read(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Tcp(stream) => Pin::new(stream).poll_read(context, buffer),
            Self::Unix(stream) => Pin::new(stream).poll_read(context, buffer),
            Self::Tls(stream) => Pin::new(stream.as_mut()).poll_read(context, buffer),
            Self::Boxed(stream) => stream.as_mut().poll_read(context, buffer),
        }
    }
}

impl AsyncWrite for Stream {
    fn poll_write(
        self: Pin<&mut Self>,
        context: &mut Context<'_>,
        buffer: &[u8],
    ) -> Poll<std::io::Result<usize>> {
        match self.get_mut() {
            Self::Tcp(stream) => Pin::new(stream).poll_write(context, buffer),
            Self::Unix(stream) => Pin::new(stream).poll_write(context, buffer),
            Self::Tls(stream) => Pin::new(stream.as_mut()).poll_write(context, buffer),
            Self::Boxed(stream) => stream.as_mut().poll_write(context, buffer),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Tcp(stream) => Pin::new(stream).poll_flush(context),
            Self::Unix(stream) => Pin::new(stream).poll_flush(context),
            Self::Tls(stream) => Pin::new(stream.as_mut()).poll_flush(context),
            Self::Boxed(stream) => stream.as_mut().poll_flush(context),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<std::io::Result<()>> {
        match self.get_mut() {
            Self::Tcp(stream) => Pin::new(stream).poll_shutdown(context),
            Self::Unix(stream) => Pin::new(stream).poll_shutdown(context),
            Self::Tls(stream) => Pin::new(stream.as_mut()).poll_shutdown(context),
            Self::Boxed(stream) => stream.as_mut().poll_shutdown(context),
        }
    }
}

#[derive(Debug)]
pub struct Connection {
    stream: Stream,
    receive_buffer: ReceiveBuffer,
    send_buffer: SendBuffer,
}

impl Connection {
    #[must_use]
    pub fn new(stream: Stream, receive_buffer_size: ReceiveBufferSize) -> Self {
        Self {
            stream,
            receive_buffer: ReceiveBuffer::new(receive_buffer_size),
            send_buffer: SendBuffer::new(),
        }
    }

    pub fn queue_send(
        &mut self,
        message: impl protocol::frontend::Message,
    ) -> Result<(), protocol::frontend::MessageError> {
        message.write_to(&mut self.send_buffer)
    }

    pub async fn flush_send(&mut self) -> Result<(), SendError> {
        self.send_buffer.flush(&mut self.stream).await
    }

    pub async fn receive(&mut self) -> Result<Frame, ReceiveError> {
        self.receive_buffer.read_frame(&mut self.stream).await
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::protocol;
    use pretty_assertions::assert_eq;

    fn frame(tag: u8, body: &[u8]) -> Vec<u8> {
        let mut bytes = Vec::new();

        protocol::encode(tag, body, &mut bytes);

        bytes
    }

    #[tokio::test]
    async fn receives_an_oversized_frame_then_resumes_batched_reads() {
        let oversized = vec![0x2a; ReceiveBufferSize::DEFAULT.oversized() * 2];

        let mut bytes = frame(b'1', &[]);

        bytes.extend(frame(b'D', &oversized));
        bytes.extend(frame(b'Z', b"I"));

        let mut connection = Connection::new(
            Stream::boxed(std::io::Cursor::new(bytes)),
            ReceiveBufferSize::DEFAULT,
        );

        let first = connection.receive().await.unwrap();
        let second = connection.receive().await.unwrap();
        let third = connection.receive().await.unwrap();

        assert_eq!(first.tag(), b'1');
        assert!(first.body().as_slice().is_empty());

        assert_eq!(second.tag(), b'D');
        assert_eq!(second.body().as_slice(), oversized.as_slice());

        assert_eq!(third.tag(), b'Z');
        assert_eq!(third.body().as_slice(), b"I");
    }

    #[test]
    fn offers_only_postgresql_over_alpn() {
        let mut client = rustls::ClientConfig::builder()
            .with_root_certificates(rustls::RootCertStore::empty())
            .with_no_client_auth();
        client.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];

        let tls = Tls::from_client_config(client);

        assert_eq!(tls.client().alpn_protocols, vec![ALPN_PROTOCOL.to_vec()]);

        let tls = Tls::new(rustls::RootCertStore::empty());

        assert_eq!(tls.client().alpn_protocols, vec![ALPN_PROTOCOL.to_vec()]);
    }
}
