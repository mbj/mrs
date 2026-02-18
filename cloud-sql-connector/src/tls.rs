//! TLS configuration for Cloud SQL connections.
//!
//! Provides a custom certificate verifier and configuration builder for
//! mutual TLS connections to Cloud SQL instances. The per-instance CA
//! certificate authenticates the server identity, so hostname verification
//! is skipped.

use std::io::BufReader;
use std::sync::Arc;

use google_cloud_sql_v1::model::ConnectSettings;
use rustls::RootCertStore;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime};

use crate::Error;

/// Custom TLS certificate verifier for Cloud SQL connections.
///
/// Validates the server certificate chain against the instance-specific CA
/// certificate from the Cloud SQL Admin API. Hostname verification is skipped
/// because the per-instance CA already authenticates the server identity —
/// no other server can present a certificate signed by this CA.
///
/// This mirrors the Go Cloud SQL connector which sets `InsecureSkipVerify: true`
/// and provides a custom `VerifyPeerCertificate` callback that only checks the
/// CA chain.
#[derive(Debug)]
struct CloudSqlCertVerifier {
    /// Crypto provider for signature verification.
    provider: Arc<rustls::crypto::CryptoProvider>,
    /// Root certificate store containing the instance-specific CA.
    roots: Arc<RootCertStore>,
}

impl ServerCertVerifier for CloudSqlCertVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let cert = rustls::server::ParsedCertificate::try_from(end_entity)?;

        rustls::client::verify_server_cert_signed_by_trust_anchor(
            &cert,
            &self.roots,
            intermediates,
            now,
            self.provider.signature_verification_algorithms.all,
        )?;

        Ok(ServerCertVerified::assertion())
    }

    /// Cloud SQL exclusively uses TLS 1.3 — reject TLS 1.2 signatures.
    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Err(rustls::Error::PeerIncompatible(
            rustls::PeerIncompatible::ServerTlsVersionIsDisabledByOurConfig,
        ))
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(
            message,
            cert,
            dss,
            &self.provider.signature_verification_algorithms,
        )
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        self.provider
            .signature_verification_algorithms
            .supported_schemes()
    }
}

/// Build a `rustls` TLS client configuration for Cloud SQL connections.
///
/// Uses a custom certificate verifier that validates the CA chain against the
/// instance-specific server CA certificate but skips hostname verification.
pub(crate) fn build_config(
    server_ca: CertificateDer<'static>,
    client_cert: CertificateDer<'static>,
    private_key: PrivateKeyDer<'static>,
) -> Result<rustls::ClientConfig, Error> {
    let mut root_store = RootCertStore::empty();
    root_store.add(server_ca)?;

    let provider = Arc::new(rustls::crypto::aws_lc_rs::default_provider());

    let verifier = CloudSqlCertVerifier {
        provider: Arc::clone(&provider),
        roots: Arc::new(root_store),
    };

    let config = rustls::ClientConfig::builder_with_provider(provider)
        .with_safe_default_protocol_versions()?
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(verifier))
        .with_client_auth_cert(vec![client_cert], private_key)?;

    Ok(config)
}

/// Extract and parse the server CA certificate from connect settings.
pub(crate) fn extract_server_ca_cert(
    settings: &ConnectSettings,
) -> Result<CertificateDer<'static>, Error> {
    let ssl_cert = settings
        .server_ca_cert
        .as_ref()
        .ok_or(Error::ServerCaCertMissing)?;

    if ssl_cert.cert.is_empty() {
        return Err(Error::ServerCaCertEmpty);
    }

    parse_pem_cert(&ssl_cert.cert)
}

/// Parse a single certificate from PEM data.
pub(crate) fn parse_pem_cert(pem: &str) -> Result<CertificateDer<'static>, Error> {
    let mut reader = BufReader::new(pem.as_bytes());

    rustls_pemfile::certs(&mut reader)
        .next()
        .ok_or(Error::NoCertificatesInPem)?
        .map_err(Error::from)
}
