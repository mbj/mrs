#![doc = include_str!("../README.md")]

mod tls;

use core::net::IpAddr;
use std::path::Path;
use std::sync::Arc;

use google_cloud_sql_v1::client::SqlConnectService;
use google_cloud_sql_v1::model::{ConnectSettings, IpMapping, SqlIpAddressType, SslCert};
use rsa::RsaPrivateKey;
use rsa::pkcs8::EncodePrivateKey as _;
use rsa::pkcs8::EncodePublicKey as _;
use rustls::pki_types::{PrivateKeyDer, ServerName};
use tokio::io::copy_bidirectional;
use tokio::net::{TcpStream, UnixListener};
use tokio_rustls::TlsConnector;
use tokio_rustls::client::TlsStream;

/// Cloud SQL proxy port (used by the Cloud SQL server-side proxy).
const CLOUD_SQL_PORT: u16 = 3307;

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
    /// Ephemeral certificate missing from API response.
    #[error("ephemeral certificate missing from generateEphemeralCert response")]
    EphemeralCertMissing,
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

/// Cloud SQL Auth Proxy dialer.
///
/// Manages ephemeral certificates and establishes TLS connections
/// to a Cloud SQL instance through the Cloud SQL Admin API.
#[derive(Debug)]
pub struct Dialer {
    /// Cloud SQL Admin API client.
    client: SqlConnectService,
    /// Cloud SQL instance name.
    instance: String,
    /// GCP project ID.
    project: String,
    /// RSA private key for ephemeral certificate requests.
    rsa_private_key: RsaPrivateKey,
}

impl Dialer {
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

    /// Dial the Cloud SQL instance, returning an authenticated TLS stream.
    ///
    /// Fetches connect settings and an ephemeral certificate from the Cloud SQL
    /// Admin API, then establishes a TLS 1.3 connection with mutual
    /// authentication to the instance.
    pub async fn dial(&self) -> Result<TlsStream<TcpStream>, Error> {
        let (settings, cert) = tokio::try_join!(self.connect_settings(), self.ephemeral_cert(),)?;

        let primary_ip = extract_primary_ip(&settings.ip_addresses)?;
        let server_ca = tls::extract_server_ca_cert(&settings)?;
        if cert.cert.is_empty() {
            return Err(Error::EphemeralCertEmpty);
        }

        let client_cert = tls::parse_pem_cert(&cert.cert)?;
        let private_key_der = self.private_key_der()?;

        let tls_config = tls::build_config(server_ca, client_cert, private_key_der)?;
        let connector = TlsConnector::from(Arc::new(tls_config));

        let tcp_stream = TcpStream::connect((primary_ip, CLOUD_SQL_PORT)).await?;

        // The server name is not used for hostname verification — the custom
        // CloudSqlCertVerifier validates the CA chain only. A value is required
        // by rustls for the TLS handshake SNI extension.
        let server_name = ServerName::IpAddress(primary_ip.into());

        Ok(connector.connect(server_name, tcp_stream).await?)
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

    /// Create a new Cloud SQL Auth Proxy dialer for a specific instance.
    ///
    /// Builds the API client and generates an RSA 2048-bit keypair.
    pub async fn new(
        project: impl Into<String>,
        instance: impl Into<String>,
    ) -> Result<Self, Error> {
        let client = SqlConnectService::builder().build().await?;

        let rsa_private_key = RsaPrivateKey::new(&mut rsa::rand_core::OsRng, RSA_KEY_BITS)?;

        Ok(Self {
            client,
            instance: instance.into(),
            project: project.into(),
            rsa_private_key,
        })
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
