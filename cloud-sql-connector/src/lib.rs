#![doc = include_str!("../README.md")]

mod cache;
mod peer_filter;
mod tls;

pub mod login_token;
pub mod principal;
pub mod service_account;
pub mod user;

pub use peer_filter::PeerFilter;

use core::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, SystemTime};

use google_cloud_auth::credentials::Credentials;
use google_cloud_sql_v1::client::SqlConnectService;
use google_cloud_sql_v1::model::{ConnectSettings, IpMapping, SqlIpAddressType, SslCert};
use rsa::RsaPrivateKey;
use rsa::pkcs8::EncodePrivateKey as _;
use rsa::pkcs8::EncodePublicKey as _;
use rustls::pki_types::{PrivateKeyDer, ServerName};
use tokio::io::copy_bidirectional;
use tokio::net::{TcpListener, TcpStream, UnixListener};
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;

use crate::cache::{Fetch, RefreshCache};

/// Cloud SQL proxy port (used by the Cloud SQL server-side proxy).
const CLOUD_SQL_PORT: u16 = 3307;

/// Refresh cached connect info this long before the ephemeral certificate
/// expires, so a dial never starts a handshake with a certificate about to
/// lapse.
const CERT_REFRESH_MARGIN: Duration = Duration::from_secs(4 * 60);

/// RSA key size in bits for ephemeral certificate requests.
const RSA_KEY_BITS: usize = 2048;

/// Cloud SQL connector errors.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Cloud SQL API client builder error.
    #[error(transparent)]
    ClientBuilder(#[from] google_cloud_gax::client_builder::Error),
    /// Cloud SQL API request error.
    #[error(transparent)]
    CloudSqlApi(#[from] google_cloud_sql_v1::Error),
    /// Ephemeral certificate PEM is empty.
    #[error("ephemeral certificate PEM is empty")]
    EphemeralCertEmpty,
    /// Ephemeral certificate expiration time is out of representable range.
    #[error("ephemeral certificate expiration_time is out of range")]
    EphemeralCertExpiration,
    /// Ephemeral certificate missing from API response.
    #[error("ephemeral certificate missing from generateEphemeralCert response")]
    EphemeralCertMissing,
    /// Ephemeral certificate response carries no expiration time.
    #[error("ephemeral certificate missing expiration_time in generateEphemeralCert response")]
    EphemeralCertNoExpiration,
    /// Failed to parse IP address from API response.
    #[error("invalid IP address from Cloud SQL API: {address}")]
    InvalidIpAddress {
        /// The invalid address string.
        address: String,
        /// The underlying parse error.
        #[source]
        source: core::net::AddrParseError,
    },
    /// IO error.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// No certificates found in PEM data.
    #[error("no certificates found in PEM data")]
    NoCertificatesInPem,
    /// No primary IP address found for the instance.
    #[error("no primary IP address found for Cloud SQL instance")]
    NoPrimaryIp,
    /// PKCS#8 encoding error.
    #[error("failed to encode RSA key: {0}")]
    Pkcs8(#[from] rsa::pkcs8::Error),
    /// RSA key generation error.
    #[error("failed to generate RSA key: {0}")]
    RsaKeyGeneration(#[from] rsa::Error),
    /// Server CA certificate PEM is empty.
    #[error("server CA certificate PEM is empty")]
    ServerCaCertEmpty,
    /// Server CA certificate missing from API response.
    #[error("server CA certificate missing from connectSettings response")]
    ServerCaCertMissing,
    /// SPKI encoding error.
    #[error("failed to encode RSA public key: {0}")]
    Spki(#[from] rsa::pkcs8::spki::Error),
    /// TLS configuration error.
    #[error("TLS configuration error: {0}")]
    TlsConfig(#[from] rustls::Error),
}

/// Per-instance connect info: everything needed to open a connection except the
/// socket itself — the instance's primary IP and a pre-built TLS configuration
/// carrying the server CA, the ephemeral client certificate, and the private
/// key. Held behind a [`RefreshCache`] and shared across concurrent dials; its
/// refresh deadline (the certificate's expiry) is tracked by the cache.
#[derive(Debug)]
struct ConnectInfo {
    /// Primary IP address of the Cloud SQL instance.
    primary_ip: IpAddr,
    /// Pre-built TLS client configuration for the instance.
    tls_config: Arc<rustls::ClientConfig>,
}

impl ConnectInfo {
    /// Open a fresh TLS connection to the instance.
    ///
    /// Fails only with an I/O error — from the TCP connect or the TLS handshake.
    /// Those are exactly the failures a refreshed `ConnectInfo` (new IP, CA, or
    /// certificate) can address, which is why [`Dialer::dial`] refreshes and
    /// retries on them and nothing else.
    async fn open_stream(&self) -> std::io::Result<TlsStream<TcpStream>> {
        let tcp_stream = TcpStream::connect((self.primary_ip, CLOUD_SQL_PORT)).await?;

        // The server name is not used for hostname verification — the custom
        // CloudSqlCertVerifier validates the CA chain only. A value is required
        // by rustls for the TLS handshake SNI extension.
        let server_name = ServerName::IpAddress(self.primary_ip.into());

        let connector = TlsConnector::from(Arc::clone(&self.tls_config));

        connector.connect(server_name, tcp_stream).await
    }
}

/// Fetches per-instance [`ConnectInfo`] from the Cloud SQL Admin API.
///
/// Owns everything a fetch needs — the API client, the project/instance names,
/// and the RSA key — so the [`RefreshCache`] can own it without borrowing back
/// into the [`Dialer`].
#[derive(Debug)]
struct ConnectInfoFetcher {
    /// Cloud SQL Admin API client.
    client: SqlConnectService,
    /// Cloud SQL instance name.
    instance: String,
    /// GCP project ID.
    project: String,
    /// RSA private key for ephemeral certificate requests.
    rsa_private_key: RsaPrivateKey,
}

impl ConnectInfoFetcher {
    /// Retrieve connect settings for the Cloud SQL instance.
    async fn connect_settings(&self) -> Result<ConnectSettings, Error> {
        Ok(self
            .client
            .get_connect_settings()
            .set_project(&self.project)
            .set_instance(&self.instance)
            .send()
            .await?)
    }

    /// Request an ephemeral client certificate for the Cloud SQL instance.
    async fn ephemeral_cert(&self) -> Result<SslCert, Error> {
        let response = self
            .client
            .generate_ephemeral_cert()
            .set_project(&self.project)
            .set_instance(&self.instance)
            .set_public_key(&self.public_key_pem()?)
            .send()
            .await?;

        response.ephemeral_cert.ok_or(Error::EphemeralCertMissing)
    }

    /// Encode the RSA private key as PKCS#8 DER for TLS.
    fn private_key_der(&self) -> Result<PrivateKeyDer<'static>, Error> {
        let der = self.rsa_private_key.to_pkcs8_der()?;
        Ok(PrivateKeyDer::Pkcs8(
            rustls::pki_types::PrivatePkcs8KeyDer::from(der.as_bytes().to_vec()),
        ))
    }

    /// Encode the RSA public key as PEM for the Cloud SQL Admin API.
    fn public_key_pem(&self) -> Result<String, Error> {
        Ok(self
            .rsa_private_key
            .to_public_key()
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)?)
    }
}

impl Fetch for ConnectInfoFetcher {
    type Value = ConnectInfo;
    type Error = Error;

    /// Fetch connect settings and a fresh ephemeral certificate, building the
    /// per-instance [`ConnectInfo`] and the time it should be refreshed (the
    /// certificate's expiry minus [`CERT_REFRESH_MARGIN`]).
    async fn fetch(&self) -> Result<(ConnectInfo, SystemTime), Error> {
        log::info!(
            "Fetching Cloud SQL connect info for {}/{}",
            self.project,
            self.instance
        );

        let (settings, cert) = tokio::try_join!(self.connect_settings(), self.ephemeral_cert())?;

        let primary_ip = extract_primary_ip(&settings.ip_addresses)?;
        let server_ca = tls::extract_server_ca_cert(&settings)?;
        if cert.cert.is_empty() {
            return Err(Error::EphemeralCertEmpty);
        }

        let expiration = cert
            .expiration_time
            .ok_or(Error::EphemeralCertNoExpiration)?;
        let expires_at =
            SystemTime::try_from(expiration).map_err(|_| Error::EphemeralCertExpiration)?;
        let refresh_after = expires_at
            .checked_sub(CERT_REFRESH_MARGIN)
            .unwrap_or(expires_at);

        let client_cert = tls::parse_pem_cert(&cert.cert)?;
        let private_key_der = self.private_key_der()?;

        let tls_config = Arc::new(tls::build_config(server_ca, client_cert, private_key_der)?);

        Ok((
            ConnectInfo {
                primary_ip,
                tls_config,
            },
            refresh_after,
        ))
    }
}

/// Cloud SQL Auth Proxy dialer.
///
/// Manages ephemeral certificates and establishes TLS connections
/// to a Cloud SQL instance through the Cloud SQL Admin API.
#[derive(Debug)]
pub struct Dialer {
    /// Per-instance connect info, fetched lazily and refreshed when the
    /// ephemeral certificate nears expiry or a dial fails over. Shared across
    /// all concurrent dials.
    connect_info: RefreshCache<ConnectInfoFetcher>,
}

impl Dialer {
    /// Dial the Cloud SQL instance, returning an authenticated TLS stream.
    ///
    /// Reuses cached connect settings and the ephemeral certificate across
    /// calls, refetching from the Cloud SQL Admin API only when the cache is
    /// empty or the certificate is nearing expiry. Each call opens a fresh
    /// TLS 1.3 connection with mutual authentication to the instance.
    ///
    /// If opening the connection fails — which can signal a failover or CA
    /// rotation that invalidated the cached info — the info is refreshed and the
    /// dial is retried once.
    pub async fn dial(&self) -> Result<TlsStream<TcpStream>, Error> {
        let info = self.connect_info.get(SystemTime::now()).await?;

        // Only a connection-establishment failure (`open_stream`'s `io::Error`)
        // triggers a refresh + retry — that's the failure a fresh `ConnectInfo`
        // can address. Fetch errors from `get`/`refresh` propagate as-is.
        match info.open_stream().await {
            Ok(stream) => Ok(stream),
            Err(error) => {
                log::warn!(
                    "Cloud SQL connection failed, refreshing connect info and retrying once: {error}"
                );
                let info = self.connect_info.refresh(SystemTime::now(), &info).await?;
                Ok(info.open_stream().await?)
            }
        }
    }

    /// Create a new Cloud SQL Auth Proxy dialer for a specific instance.
    ///
    /// Builds the API client — resolving Application Default Credentials — and
    /// generates an RSA 2048-bit keypair. Use [`new_with_credentials`] to reuse
    /// credentials the caller has already built.
    ///
    /// [`new_with_credentials`]: Self::new_with_credentials
    pub async fn new(
        project: impl Into<String>,
        instance: impl Into<String>,
    ) -> Result<Self, Error> {
        let client = SqlConnectService::builder().build().await?;
        Self::with_client(project, instance, client)
    }

    /// Create a new Cloud SQL Auth Proxy dialer authenticating with
    /// caller-supplied credentials.
    ///
    /// Reuses `credentials` instead of resolving Application Default Credentials
    /// internally, sparing the duplicate credential discovery a caller who
    /// already holds them would otherwise trigger. The credentials refresh their
    /// own access tokens, so one instance can back the dialer for its lifetime.
    pub async fn new_with_credentials(
        project: impl Into<String>,
        instance: impl Into<String>,
        credentials: Credentials,
    ) -> Result<Self, Error> {
        let client = SqlConnectService::builder()
            .with_credentials(credentials)
            .build()
            .await?;
        Self::with_client(project, instance, client)
    }

    /// Finish constructing a dialer around an already-built API client.
    fn with_client(
        project: impl Into<String>,
        instance: impl Into<String>,
        client: SqlConnectService,
    ) -> Result<Self, Error> {
        let rsa_private_key = RsaPrivateKey::new(&mut rsa::rand_core::OsRng, RSA_KEY_BITS)?;

        let fetcher = ConnectInfoFetcher {
            client,
            instance: instance.into(),
            project: project.into(),
            rsa_private_key,
        };

        Ok(Self {
            connect_info: RefreshCache::new(fetcher),
        })
    }
}

/// Unix socket proxy server for a Cloud SQL instance.
///
/// Binds a Unix socket on construction, guaranteeing the socket is ready
/// to accept connections once the struct is obtained.
#[derive(Debug)]
pub struct UnixSocketServer {
    dialer: Arc<Dialer>,
    listener: UnixListener,
}

impl UnixSocketServer {
    /// Bind a Unix socket proxy for a Cloud SQL instance.
    ///
    /// The socket is bound immediately — if this returns `Ok`, the server
    /// is ready to accept connections.
    pub fn new(dialer: Arc<Dialer>, socket_path: &Path) -> Result<Self, Error> {
        let listener = UnixListener::bind(socket_path)?;

        log::info!("Cloud SQL proxy listening on {}", socket_path.display());

        Ok(Self { dialer, listener })
    }

    /// Accept connections and proxy traffic to the Cloud SQL instance.
    ///
    /// Runs until the listener encounters an accept error.
    pub async fn serve(&self) -> Result<(), Error> {
        loop {
            let (mut local_stream, _addr) = self.listener.accept().await?;

            let dialer = Arc::clone(&self.dialer);

            tokio::spawn(async move {
                match dialer.dial().await {
                    Ok(mut tls_stream) => {
                        if let Err(error) =
                            copy_bidirectional(&mut local_stream, &mut tls_stream).await
                        {
                            log::warn!("Cloud SQL proxy connection ended: {error}");
                        }
                    }
                    Err(error) => {
                        log::warn!("Cloud SQL proxy dial failed: {error}");
                    }
                }
            });
        }
    }
}

/// TCP proxy server for a Cloud SQL instance.
///
/// Binds a TCP listener on construction, guaranteeing the socket is ready
/// to accept connections once the struct is obtained.
#[derive(Debug)]
pub struct TcpServer {
    dialer: Arc<Dialer>,
    listener: TcpListener,
    local_addr: SocketAddr,
    peer_filter: PeerFilter,
}

impl TcpServer {
    /// Bind a TCP proxy for a Cloud SQL instance.
    ///
    /// The socket is bound immediately — if this returns `Ok`, the server
    /// is ready to accept connections.
    pub async fn new(
        dialer: Arc<Dialer>,
        address: SocketAddr,
        peer_filter: PeerFilter,
    ) -> Result<Self, Error> {
        let listener = TcpListener::bind(address).await?;
        let local_addr = listener.local_addr()?;

        log::info!("Cloud SQL proxy listening on {local_addr}");

        Ok(Self {
            dialer,
            listener,
            local_addr,
            peer_filter,
        })
    }

    /// Bind a TCP proxy on `localhost` with an OS-assigned port.
    ///
    /// Accepts all incoming connections since the listener is bound to
    /// loopback and only reachable from the local machine. Use [`TcpServer::new`]
    /// if you need to filter on loopback.
    ///
    /// Use [`TcpServer::local_addr`] to discover the assigned port.
    pub async fn new_localhost_v4(dialer: Arc<Dialer>) -> Result<Self, Error> {
        Self::new(
            dialer,
            SocketAddr::from((Ipv4Addr::LOCALHOST, 0)),
            PeerFilter::All,
        )
        .await
    }

    /// Bind a TCP proxy on IPv6 `localhost` with an OS-assigned port.
    ///
    /// Accepts all incoming connections since the listener is bound to
    /// loopback and only reachable from the local machine. Use [`TcpServer::new`]
    /// if you need to filter on loopback.
    ///
    /// Use [`TcpServer::local_addr`] to discover the assigned port.
    pub async fn new_localhost_v6(dialer: Arc<Dialer>) -> Result<Self, Error> {
        Self::new(
            dialer,
            SocketAddr::from((Ipv6Addr::LOCALHOST, 0)),
            PeerFilter::All,
        )
        .await
    }

    /// Return the local address this server is bound to.
    #[must_use]
    pub fn local_addr(&self) -> SocketAddr {
        self.local_addr
    }

    /// Accept connections and proxy traffic to the Cloud SQL instance.
    ///
    /// Runs until the listener encounters an accept error.
    pub async fn serve(&self) -> Result<(), Error> {
        loop {
            let (mut local_stream, peer_addr) = self.listener.accept().await?;

            if !self.peer_filter.is_allowed(peer_addr) {
                log::warn!("Cloud SQL proxy rejected connection from {peer_addr}");
                continue;
            }

            let dialer = Arc::clone(&self.dialer);

            tokio::spawn(async move {
                match dialer.dial().await {
                    Ok(mut tls_stream) => {
                        if let Err(error) =
                            copy_bidirectional(&mut local_stream, &mut tls_stream).await
                        {
                            log::warn!("Cloud SQL proxy connection ended: {error}");
                        }
                    }
                    Err(error) => {
                        log::warn!("Cloud SQL proxy dial failed: {error}");
                    }
                }
            });
        }
    }
}

/// Extract the primary IP address from instance IP mappings.
fn extract_primary_ip(ip_addresses: &[IpMapping]) -> Result<IpAddr, Error> {
    for mapping in ip_addresses {
        if mapping.r#type == SqlIpAddressType::Primary {
            return mapping.ip_address.parse::<IpAddr>().map_err(|source| {
                Error::InvalidIpAddress {
                    address: mapping.ip_address.clone(),
                    source,
                }
            });
        }
    }

    Err(Error::NoPrimaryIp)
}
