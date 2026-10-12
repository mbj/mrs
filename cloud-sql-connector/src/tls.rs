//! TLS configuration for Cloud SQL connections.
//!
//! Provides a custom certificate verifier and configuration builder for
//! mutual TLS connections to Cloud SQL instances. The per-instance CA
//! certificate authenticates the server identity, so hostname verification
//! is skipped.

use std::sync::Arc;
use std::time::{Duration, SystemTime};

use google_cloud_sql_v1::model::ConnectSettings;
use rustls::RootCertStore;
use rustls::client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier};
use rustls::pki_types::pem::{self, PemObject};
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

/// Extract the `notAfter` expiry from a DER-encoded certificate.
///
/// Used for the ephemeral client certificate: its own validity, not the
/// unreliable `expiration_time` field of the `generateEphemeralCert` response,
/// determines when the cached connect info must be refreshed.
///
/// Only the certificate's v3 layout is enforced; every field other than
/// `notAfter` is skipped without checking its contents. The certificate is
/// our own client credential, received over the authenticated Admin API, and
/// no trust decision depends on it here: the Cloud SQL server verifies it
/// during the TLS handshake. A misread expiry only shifts the refresh time,
/// and a failed dial triggers a refresh regardless.
pub(crate) fn cert_not_after(cert: &CertificateDer<'_>) -> Result<SystemTime, Error> {
    let not_after = yasna::parse_der(cert, |reader| {
        reader.read_sequence(|certificate| {
            let not_after = certificate.next().read_sequence(|tbs_certificate| {
                let version = tbs_certificate
                    .next()
                    .read_tagged(VERSION_TAG, |reader| reader.read_u8())?;

                if version != VERSION_V3 {
                    return Err(yasna::ASN1Error::new(yasna::ASN1ErrorKind::Invalid));
                }

                tbs_certificate.next().read_der()?; // serialNumber
                tbs_certificate.next().read_der()?; // signature
                tbs_certificate.next().read_der()?; // issuer

                let not_after = tbs_certificate.next().read_sequence(|validity| {
                    validity.next().read_der()?; // notBefore

                    let not_after = validity.next();

                    if not_after.lookahead_tag()? == yasna::tags::TAG_UTCTIME {
                        Ok(*not_after.read_utctime()?.datetime())
                    } else {
                        Ok(*not_after.read_generalized_time()?.datetime())
                    }
                })?;

                tbs_certificate.next().read_der()?; // subject
                tbs_certificate.next().read_der()?; // subjectPublicKeyInfo

                tbs_certificate.read_optional(|reader| {
                    reader.read_tagged(EXTENSIONS_TAG, |reader| reader.read_der())
                })?;

                Ok(not_after)
            })?;

            certificate.next().read_der()?; // signatureAlgorithm
            certificate.next().read_der()?; // signatureValue

            Ok(not_after)
        })
    })
    .map_err(|_| Error::EphemeralCertParse)?;

    let secs =
        u64::try_from(not_after.unix_timestamp()).map_err(|_| Error::EphemeralCertExpiration)?;

    Ok(SystemTime::UNIX_EPOCH + Duration::from_secs(secs))
}

// TBSCertificate `version [0] EXPLICIT`, RFC 5280 section 4.1
const VERSION_TAG: yasna::Tag = yasna::Tag {
    tag_class: yasna::TagClass::ContextSpecific,
    tag_number: 0,
};

// `Version ::= INTEGER { v1(0), v2(1), v3(2) }`, RFC 5280 section 4.1
const VERSION_V3: u8 = 2;

// TBSCertificate `extensions [3] EXPLICIT`, RFC 5280 section 4.1
const EXTENSIONS_TAG: yasna::Tag = yasna::Tag {
    tag_class: yasna::TagClass::ContextSpecific,
    tag_number: 3,
};

/// Parse a single certificate from PEM data.
pub(crate) fn parse_pem_cert(input: &str) -> Result<CertificateDer<'static>, Error> {
    CertificateDer::from_pem_slice(input.as_bytes()).map_err(|source| match source {
        pem::Error::NoItemsFound => Error::NoCertificatesInPem,
        source => Error::CertificatePem { source },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `cert_not_after` reads the expiry from the certificate's own `notAfter`
    /// rather than any API-supplied field.
    #[test]
    fn cert_not_after_reads_validity() {
        // A fixed expiry inside X.509's representable range; whole seconds
        // because X.509 time has one-second granularity.
        const EXPIRY_UNIX: u64 = 1_900_000_000; // 2030-03-17T18:46:40Z

        let key = rcgen::KeyPair::generate().expect("generate key");
        let mut params =
            rcgen::CertificateParams::new(vec!["test".to_string()]).expect("cert params");
        params.not_after =
            time::OffsetDateTime::from_unix_timestamp(i64::try_from(EXPIRY_UNIX).unwrap())
                .expect("notAfter");
        let cert = params.self_signed(&key).expect("self-signed cert");

        let der = parse_pem_cert(&cert.pem()).expect("parse PEM");
        let expires_at = cert_not_after(&der).expect("read notAfter");

        assert_eq!(
            expires_at,
            SystemTime::UNIX_EPOCH + Duration::from_secs(EXPIRY_UNIX)
        );
    }
}
