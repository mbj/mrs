pub struct Bundle {
    pub ca_cert_pem: String,
    pub server_cert_pem: String,
    pub server_key_pem: String,
}

/// Failure modes of [`write_ca_pem_to_temp`].
#[derive(Debug, thiserror::Error)]
pub enum WriteCaPemError {
    /// The system clock reports a time before the Unix epoch, so the
    /// unique temp-file suffix cannot be derived.
    #[error("system clock is before the Unix epoch")]
    SystemClock(#[source] std::time::SystemTimeError),
    /// The CA certificate PEM could not be written to the temp file.
    #[error("failed to write CA certificate PEM to temp file")]
    Write(#[source] std::io::Error),
}

/// Persist a CA-certificate PEM to a uniquely-named file under
/// `std::env::temp_dir()` and return its path.
///
/// Leak-on-purpose: the caller becomes the file's owner and nothing
/// removes it today. Centralised here so the `pg_ephemeral_ca_*.crt`
/// naming scheme has a single source of truth — both `Container`
/// startup and `label::Metadata::prepare_config` route through this.
pub(crate) fn write_ca_pem_to_temp(pem: &[u8]) -> Result<std::path::PathBuf, WriteCaPemError> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(WriteCaPemError::SystemClock)?
        .as_nanos();
    let path = std::env::temp_dir().join(format!("pg_ephemeral_ca_{timestamp}.crt"));
    std::fs::write(&path, pem).map_err(WriteCaPemError::Write)?;
    Ok(path)
}

impl Bundle {
    pub fn generate(hostname: &str) -> Result<Self, rcgen::Error> {
        let ca_key = rcgen::KeyPair::generate()?;
        let mut ca_params = rcgen::CertificateParams::new(vec![])?;
        ca_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, "pg-ephemeral CA");
        ca_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
        ca_params.key_usages = vec![
            rcgen::KeyUsagePurpose::KeyCertSign,
            rcgen::KeyUsagePurpose::CrlSign,
        ];

        let ca_cert = ca_params.self_signed(&ca_key)?;
        let ca_cert_pem = ca_cert.pem();

        let server_key = rcgen::KeyPair::generate()?;
        let mut server_params = rcgen::CertificateParams::new(vec![hostname.to_string()])?;
        server_params
            .distinguished_name
            .push(rcgen::DnType::CommonName, "pg-ephemeral server");

        server_params.key_usages = vec![
            rcgen::KeyUsagePurpose::DigitalSignature,
            rcgen::KeyUsagePurpose::KeyEncipherment,
        ];

        server_params.extended_key_usages = vec![rcgen::ExtendedKeyUsagePurpose::ServerAuth];

        let ca_issuer = rcgen::Issuer::from_params(&ca_params, &ca_key);
        let server_cert = server_params.signed_by(&server_key, &ca_issuer)?;
        let server_cert_pem = server_cert.pem();
        let server_key_pem = server_key.serialize_pem();

        Ok(Bundle {
            ca_cert_pem,
            server_cert_pem,
            server_key_pem,
        })
    }

    pub fn build(
        ca_cert_pem: String,
        server_cert_pem: String,
        server_key_pem: String,
        expected_hostname: &str,
    ) -> Result<Self, ValidationError> {
        use rustls_pki_types::pem::PemObject;

        let ca_cert = rustls_pki_types::CertificateDer::from_pem_slice(ca_cert_pem.as_bytes())
            .map_err(|e| {
                ValidationError::ParseError(format!("Failed to parse CA certificate PEM: {e}"))
            })?;
        let trust_anchor = webpki::anchor_from_trusted_cert(&ca_cert).map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse CA certificate: {e}"))
        })?;
        let ca = webpki::EndEntityCert::try_from(&ca_cert).map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse CA certificate: {e}"))
        })?;

        if ca.subject() != common_name_der("pg-ephemeral CA").as_slice() {
            return Err(ValidationError::ValidationError(
                "CA subject should be CN=pg-ephemeral CA".to_string(),
            ));
        }

        let ca_basic_constraints = extension_value(&ca_cert, BASIC_CONSTRAINTS_OID)
            .map_err(|e| ValidationError::ParseError(format!("Failed to read CA extensions: {e}")))?
            .ok_or_else(|| {
                ValidationError::ValidationError(
                    "CA basic constraints should be present".to_string(),
                )
            })?;

        let ca_flag = yasna::parse_der(&ca_basic_constraints, |reader| {
            reader.read_sequence(|basic_constraints| {
                let ca_flag = basic_constraints.read_default(false, |reader| reader.read_bool())?;
                basic_constraints.read_optional(|reader| reader.read_u64())?;
                Ok(ca_flag)
            })
        })
        .map_err(|e| {
            ValidationError::ValidationError(format!("CA should have basic constraints: {e}"))
        })?;

        if !ca_flag {
            return Err(ValidationError::ValidationError(
                "CA certificate should have CA=true".to_string(),
            ));
        }

        let server_cert_der = rustls_pki_types::CertificateDer::from_pem_slice(
            server_cert_pem.as_bytes(),
        )
        .map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse server certificate PEM: {e}"))
        })?;
        let server_cert = webpki::EndEntityCert::try_from(&server_cert_der).map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse server certificate: {e}"))
        })?;

        if server_cert.subject() != common_name_der("pg-ephemeral server").as_slice() {
            return Err(ValidationError::ValidationError(
                "Server subject should be CN=pg-ephemeral server".to_string(),
            ));
        }

        server_cert
            .verify_for_usage(
                webpki::ALL_VERIFICATION_ALGS,
                &[trust_anchor],
                &[],
                rustls_pki_types::UnixTime::now(),
                webpki::KeyUsage::server_auth(),
                None,
                None,
            )
            .map_err(|e| {
                ValidationError::ValidationError(format!(
                    "Server certificate should be issued by CA: {e}"
                ))
            })?;

        let subject_alternative_name = extension_value(&server_cert_der, SUBJECT_ALT_NAME_OID)
            .map_err(|e| {
                ValidationError::ParseError(format!(
                    "Failed to read server certificate extensions: {e}"
                ))
            })?
            .ok_or_else(|| {
                ValidationError::ValidationError(
                    "Server certificate should have subject alternative name extension".to_string(),
                )
            })?;

        let dns_names = yasna::parse_der(&subject_alternative_name, |reader| {
            let mut dns_names = Vec::new();

            reader.read_sequence_of(|general_name| {
                if general_name.lookahead_tag()? == DNS_NAME_TAG {
                    dns_names.push(
                        general_name.read_tagged_implicit(DNS_NAME_TAG, |reader| {
                            reader.read_ia5_string()
                        })?,
                    );
                } else {
                    general_name.read_der()?;
                }

                Ok(())
            })?;

            Ok(dns_names)
        })
        .map_err(|e| {
            ValidationError::ValidationError(format!(
                "Failed to read subject alternative name: {e}"
            ))
        })?;

        if !dns_names
            .iter()
            .any(|dns_name| dns_name == expected_hostname)
        {
            return Err(ValidationError::ValidationError(format!(
                "Server certificate DNS names {dns_names:?} should contain {expected_hostname}"
            )));
        }

        let server_key = rcgen::KeyPair::from_pem(&server_key_pem)
            .map_err(|e| ValidationError::ParseError(format!("Failed to parse server key: {e}")))?;

        let server_key_public = rcgen::PublicKeyData::subject_public_key_info(&server_key);

        if server_key_public != server_cert.subject_public_key_info().as_ref() {
            return Err(ValidationError::ValidationError(
                "Server certificate public key should match server private key".to_string(),
            ));
        }

        Ok(Bundle {
            ca_cert_pem,
            server_cert_pem,
            server_key_pem,
        })
    }
}

// id-at-commonName, RFC 5280 appendix A.1
const COMMON_NAME_OID: &[u64] = &[2, 5, 4, 3];

// id-ce-basicConstraints, RFC 5280 section 4.2.1.9
const BASIC_CONSTRAINTS_OID: &[u64] = &[2, 5, 29, 19];

// id-ce-subjectAltName, RFC 5280 section 4.2.1.6
const SUBJECT_ALT_NAME_OID: &[u64] = &[2, 5, 29, 17];

// TBSCertificate `version [0] EXPLICIT`, RFC 5280 section 4.1
const VERSION_TAG: yasna::Tag = yasna::Tag {
    tag_class: yasna::TagClass::ContextSpecific,
    tag_number: 0,
};

// TBSCertificate `extensions [3] EXPLICIT`, RFC 5280 section 4.1
const EXTENSIONS_TAG: yasna::Tag = yasna::Tag {
    tag_class: yasna::TagClass::ContextSpecific,
    tag_number: 3,
};

// GeneralName `dNSName [2] IA5String` (implicitly tagged), RFC 5280 section 4.2.1.6
const DNS_NAME_TAG: yasna::Tag = yasna::Tag {
    tag_class: yasna::TagClass::ContextSpecific,
    tag_number: 2,
};

fn common_name_der(common_name: &str) -> Vec<u8> {
    yasna::construct_der(|writer| {
        writer.write_set(|writer| {
            writer.next().write_sequence(|writer| {
                writer
                    .next()
                    .write_oid(&yasna::models::ObjectIdentifier::from_slice(
                        COMMON_NAME_OID,
                    ));
                writer.next().write_utf8_string(common_name);
            });
        });
    })
}

fn extension_value(cert: &[u8], oid: &[u64]) -> yasna::ASN1Result<Option<Vec<u8>>> {
    let oid = yasna::models::ObjectIdentifier::from_slice(oid);

    yasna::parse_der(cert, |reader| {
        reader.read_sequence(|certificate| {
            let value = certificate.next().read_sequence(|tbs_certificate| {
                tbs_certificate.read_optional(|reader| {
                    reader.read_tagged(VERSION_TAG, |reader| reader.read_u8())
                })?;

                tbs_certificate.next().read_der()?; // serialNumber
                tbs_certificate.next().read_der()?; // signature
                tbs_certificate.next().read_der()?; // issuer
                tbs_certificate.next().read_der()?; // validity
                tbs_certificate.next().read_der()?; // subject
                tbs_certificate.next().read_der()?; // subjectPublicKeyInfo

                let mut value = None;

                tbs_certificate.read_optional(|reader| {
                    reader.read_tagged(EXTENSIONS_TAG, |reader| {
                        reader.read_sequence_of(|reader| {
                            reader.read_sequence(|extension| {
                                let extension_oid = extension.next().read_oid()?;
                                extension.read_default(false, |reader| reader.read_bool())?;
                                let extension_value = extension.next().read_bytes()?;

                                if extension_oid == oid {
                                    if value.is_some() {
                                        return Err(yasna::ASN1Error::new(
                                            yasna::ASN1ErrorKind::Invalid,
                                        ));
                                    }
                                    value = Some(extension_value);
                                }

                                Ok(())
                            })
                        })
                    })
                })?;

                Ok(value)
            })?;

            certificate.next().read_der()?; // signatureAlgorithm
            certificate.next().read_der()?; // signatureValue

            Ok(value)
        })
    })
}

#[derive(Debug)]
pub enum ValidationError {
    ParseError(String),
    ValidationError(String),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            ValidationError::ValidationError(msg) => write!(f, "Validation error: {msg}"),
        }
    }
}

impl std::error::Error for ValidationError {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_generate_bundle() {
        let hostname = "test.example.com";
        let generated = Bundle::generate(hostname).unwrap();

        Bundle::build(
            generated.ca_cert_pem,
            generated.server_cert_pem,
            generated.server_key_pem,
            hostname,
        )
        .unwrap();
    }
}
