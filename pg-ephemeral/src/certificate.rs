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

#[derive(Debug, thiserror::Error)]
pub enum GenerateError {
    #[error("host name {0:?} is neither an IP address nor ASCII")]
    Hostname(String),
    #[error("failed to generate key pair")]
    KeyGeneration(#[source] aws_lc_rs::error::Unspecified),
    #[error("failed to encode key")]
    KeyEncoding(#[source] aws_lc_rs::error::Unspecified),
    #[error("failed to sign certificate")]
    Signing(#[source] aws_lc_rs::error::Unspecified),
    #[error("failed to encode PEM")]
    PemEncoding(#[source] pem_rfc7468::Error),
}

impl Bundle {
    pub fn generate(hostname: &str) -> Result<Self, GenerateError> {
        use rand::RngExt;

        let subject_alternative_name = SubjectAlternativeName::parse(hostname)?;

        let ca_key = generate_key()?;
        let server_key = generate_key()?;

        let mut rng = rand::rng();

        let ca_tbs_certificate = ca_tbs_certificate(rng.random_range(1..=u64::MAX), &ca_key)?;

        let server_tbs_certificate = server_tbs_certificate(
            rng.random_range(1..=u64::MAX),
            &subject_alternative_name,
            &server_key,
            &ca_key,
        )?;

        Ok(Bundle {
            ca_cert_pem: certificate_pem(&ca_tbs_certificate, &ca_key)?,
            server_cert_pem: certificate_pem(&server_tbs_certificate, &ca_key)?,
            server_key_pem: key_pem(&server_key)?,
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

        if ca.subject() != common_name_der(CA_COMMON_NAME).as_slice() {
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

        if server_cert.subject() != common_name_der(SERVER_COMMON_NAME).as_slice() {
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

        let server_key =
            rustls_pki_types::PrivatePkcs8KeyDer::from_pem_slice(server_key_pem.as_bytes())
                .map_err(|e| {
                    ValidationError::ParseError(format!("Failed to parse server key PEM: {e}"))
                })?;

        let server_key_public = private_key_subject_public_key_info(server_key.secret_pkcs8_der())
            .ok_or_else(|| ValidationError::ParseError("Failed to parse server key".to_string()))?;

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

enum SubjectAlternativeName<'a> {
    DnsName(&'a str),
    IpAddress(std::net::IpAddr),
}

impl<'a> SubjectAlternativeName<'a> {
    fn parse(hostname: &'a str) -> Result<Self, GenerateError> {
        if let Ok(ip_address) = hostname.parse() {
            Ok(Self::IpAddress(ip_address))
        } else if hostname.is_ascii() {
            Ok(Self::DnsName(hostname))
        } else {
            Err(GenerateError::Hostname(hostname.to_string()))
        }
    }

    fn write(&self, writer: yasna::DERWriter) {
        match self {
            Self::DnsName(dns_name) => {
                writer.write_tagged_implicit(DNS_NAME_TAG, |writer| {
                    writer.write_ia5_string(dns_name);
                });
            }
            Self::IpAddress(std::net::IpAddr::V4(ip_address)) => {
                writer.write_tagged_implicit(IP_ADDRESS_TAG, |writer| {
                    writer.write_bytes(&ip_address.octets());
                });
            }
            Self::IpAddress(std::net::IpAddr::V6(ip_address)) => {
                writer.write_tagged_implicit(IP_ADDRESS_TAG, |writer| {
                    writer.write_bytes(&ip_address.octets());
                });
            }
        }
    }
}

fn generate_key() -> Result<aws_lc_rs::signature::EcdsaKeyPair, GenerateError> {
    aws_lc_rs::signature::EcdsaKeyPair::generate(
        &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING,
    )
    .map_err(GenerateError::KeyGeneration)
}

fn key_pem(key: &aws_lc_rs::signature::EcdsaKeyPair) -> Result<String, GenerateError> {
    let pkcs8 = key.to_pkcs8v1().map_err(GenerateError::KeyEncoding)?;

    pem(PRIVATE_KEY_PEM_LABEL, pkcs8.as_ref())
}

fn certificate_pem(
    tbs_certificate: &[u8],
    issuer_key: &aws_lc_rs::signature::EcdsaKeyPair,
) -> Result<String, GenerateError> {
    let signature = issuer_key
        .sign(&aws_lc_rs::rand::SystemRandom::new(), tbs_certificate)
        .map_err(GenerateError::Signing)?;

    let certificate = yasna::construct_der(|writer| {
        writer.write_sequence(|certificate| {
            certificate.next().write_der(tbs_certificate);
            write_signature_algorithm(certificate.next());
            certificate
                .next()
                .write_bitvec_bytes(signature.as_ref(), signature.as_ref().len() * 8);
        });
    });

    pem(CERTIFICATE_PEM_LABEL, &certificate)
}

fn pem(label: &str, der: &[u8]) -> Result<String, GenerateError> {
    pem_rfc7468::encode_string(label, pem_rfc7468::LineEnding::LF, der)
        .map_err(GenerateError::PemEncoding)
}

fn ca_tbs_certificate(
    serial_number: u64,
    key: &aws_lc_rs::signature::EcdsaKeyPair,
) -> Result<Vec<u8>, GenerateError> {
    let subject_public_key_info = subject_public_key_info(key)?;
    let subject_key_identifier = key_identifier(key);

    Ok(tbs_certificate(
        serial_number,
        CA_COMMON_NAME,
        CA_COMMON_NAME,
        &subject_public_key_info,
        |extensions| {
            write_extension(extensions.next(), BASIC_CONSTRAINTS_OID, true, |writer| {
                writer.write_sequence(|basic_constraints| {
                    basic_constraints.next().write_bool(true); // cA
                });
            });
            write_extension(extensions.next(), KEY_USAGE_OID, true, |writer| {
                // keyCertSign (5), cRLSign (6)
                writer.write_bitvec_bytes(&[0b0000_0110], 7);
            });
            write_extension(
                extensions.next(),
                SUBJECT_KEY_IDENTIFIER_OID,
                false,
                |writer| writer.write_bytes(&subject_key_identifier),
            );
        },
    ))
}

fn server_tbs_certificate(
    serial_number: u64,
    subject_alternative_name: &SubjectAlternativeName,
    key: &aws_lc_rs::signature::EcdsaKeyPair,
    issuer_key: &aws_lc_rs::signature::EcdsaKeyPair,
) -> Result<Vec<u8>, GenerateError> {
    let subject_public_key_info = subject_public_key_info(key)?;
    let subject_key_identifier = key_identifier(key);
    let authority_key_identifier = key_identifier(issuer_key);

    Ok(tbs_certificate(
        serial_number,
        CA_COMMON_NAME,
        SERVER_COMMON_NAME,
        &subject_public_key_info,
        |extensions| {
            write_extension(extensions.next(), KEY_USAGE_OID, true, |writer| {
                // digitalSignature (0), keyEncipherment (2)
                writer.write_bitvec_bytes(&[0b1010_0000], 3);
            });
            write_extension(extensions.next(), EXTENDED_KEY_USAGE_OID, false, |writer| {
                writer.write_sequence(|key_purposes| {
                    key_purposes
                        .next()
                        .write_oid(&yasna::models::ObjectIdentifier::from_slice(
                            SERVER_AUTH_OID,
                        ));
                });
            });
            write_extension(extensions.next(), SUBJECT_ALT_NAME_OID, false, |writer| {
                writer.write_sequence(|general_names| {
                    subject_alternative_name.write(general_names.next());
                });
            });
            write_extension(
                extensions.next(),
                SUBJECT_KEY_IDENTIFIER_OID,
                false,
                |writer| writer.write_bytes(&subject_key_identifier),
            );
            write_extension(
                extensions.next(),
                AUTHORITY_KEY_IDENTIFIER_OID,
                false,
                |writer| {
                    writer.write_sequence(|authority_key_identifier_value| {
                        authority_key_identifier_value
                            .next()
                            .write_tagged_implicit(KEY_IDENTIFIER_TAG, |writer| {
                                writer.write_bytes(&authority_key_identifier)
                            });
                    });
                },
            );
        },
    ))
}

fn tbs_certificate(
    serial_number: u64,
    issuer: &str,
    subject: &str,
    subject_public_key_info: &[u8],
    write_extensions: impl FnOnce(&mut yasna::DERWriterSeq),
) -> Vec<u8> {
    yasna::construct_der(|writer| {
        writer.write_sequence(|tbs_certificate| {
            tbs_certificate
                .next()
                .write_tagged(VERSION_TAG, |writer| writer.write_u8(VERSION_V3));
            tbs_certificate.next().write_u64(serial_number);
            write_signature_algorithm(tbs_certificate.next());
            write_name(tbs_certificate.next(), issuer);
            tbs_certificate.next().write_sequence(|validity| {
                validity
                    .next()
                    .write_tagged_implicit(yasna::tags::TAG_UTCTIME, |writer| {
                        writer.write_bytes(NOT_BEFORE);
                    });
                validity
                    .next()
                    .write_tagged_implicit(yasna::tags::TAG_GENERALIZEDTIME, |writer| {
                        writer.write_bytes(NOT_AFTER);
                    });
            });
            write_name(tbs_certificate.next(), subject);
            tbs_certificate.next().write_der(subject_public_key_info);
            tbs_certificate
                .next()
                .write_tagged(EXTENSIONS_TAG, |writer| {
                    writer.write_sequence(write_extensions);
                });
        });
    })
}

fn write_signature_algorithm(writer: yasna::DERWriter) {
    writer.write_sequence(|algorithm_identifier| {
        algorithm_identifier
            .next()
            .write_oid(&yasna::models::ObjectIdentifier::from_slice(
                ECDSA_WITH_SHA256_OID,
            ));
    });
}

fn write_name(writer: yasna::DERWriter, common_name: &str) {
    writer.write_sequence(|name| {
        name.next().write_der(&common_name_der(common_name));
    });
}

fn write_extension(
    writer: yasna::DERWriter,
    oid: &[u64],
    critical: bool,
    write_value: impl FnOnce(yasna::DERWriter),
) {
    writer.write_sequence(|extension| {
        extension
            .next()
            .write_oid(&yasna::models::ObjectIdentifier::from_slice(oid));
        if critical {
            extension.next().write_bool(true);
        }
        extension
            .next()
            .write_bytes(&yasna::construct_der(write_value));
    });
}

fn subject_public_key_info(
    key: &aws_lc_rs::signature::EcdsaKeyPair,
) -> Result<Vec<u8>, GenerateError> {
    use aws_lc_rs::encoding::AsDer;
    use aws_lc_rs::signature::KeyPair;

    key.public_key()
        .as_der()
        .map(|der| der.as_ref().to_vec())
        .map_err(GenerateError::KeyEncoding)
}

// RFC 7093 section 2 method 1: leftmost 160 bits of the SHA-256 hash of the
// subjectPublicKey BIT STRING value
fn key_identifier(key: &aws_lc_rs::signature::EcdsaKeyPair) -> Vec<u8> {
    use aws_lc_rs::signature::KeyPair;
    use sha2::Digest;

    sha2::Sha256::digest(key.public_key().as_ref())
        .iter()
        .take(20)
        .copied()
        .collect()
}

fn private_key_subject_public_key_info(pkcs8: &[u8]) -> Option<Vec<u8>> {
    use aws_lc_rs::encoding::AsDer;
    use aws_lc_rs::signature::KeyPair;

    let ecdsa_key = [
        &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING,
        &aws_lc_rs::signature::ECDSA_P384_SHA384_ASN1_SIGNING,
        &aws_lc_rs::signature::ECDSA_P521_SHA512_ASN1_SIGNING,
    ]
    .into_iter()
    .find_map(|algorithm| aws_lc_rs::signature::EcdsaKeyPair::from_pkcs8(algorithm, pkcs8).ok());

    let public_key = if let Some(key) = ecdsa_key {
        key.public_key().as_der()
    } else if let Ok(key) = aws_lc_rs::signature::Ed25519KeyPair::from_pkcs8(pkcs8) {
        key.public_key().as_der()
    } else {
        aws_lc_rs::signature::RsaKeyPair::from_pkcs8(pkcs8)
            .ok()?
            .public_key()
            .as_der()
    };

    public_key.ok().map(|der| der.as_ref().to_vec())
}

const CA_COMMON_NAME: &str = "pg-ephemeral CA";

const SERVER_COMMON_NAME: &str = "pg-ephemeral server";

const CERTIFICATE_PEM_LABEL: &str = "CERTIFICATE";

const PRIVATE_KEY_PEM_LABEL: &str = "PRIVATE KEY";

// `Version ::= INTEGER { v1(0), v2(1), v3(2) }`, RFC 5280 section 4.1
const VERSION_V3: u8 = 2;

const NOT_BEFORE: &[u8] = b"750101000000Z";

// No well-defined expiration date, RFC 5280 section 4.1.2.5
const NOT_AFTER: &[u8] = b"99991231235959Z";

// ecdsa-with-SHA256, RFC 5758 section 3.2
const ECDSA_WITH_SHA256_OID: &[u64] = &[1, 2, 840, 10045, 4, 3, 2];

// id-ce-subjectKeyIdentifier, RFC 5280 section 4.2.1.2
const SUBJECT_KEY_IDENTIFIER_OID: &[u64] = &[2, 5, 29, 14];

// id-ce-keyUsage, RFC 5280 section 4.2.1.3
const KEY_USAGE_OID: &[u64] = &[2, 5, 29, 15];

// id-ce-authorityKeyIdentifier, RFC 5280 section 4.2.1.1
const AUTHORITY_KEY_IDENTIFIER_OID: &[u64] = &[2, 5, 29, 35];

// id-ce-extKeyUsage, RFC 5280 section 4.2.1.12
const EXTENDED_KEY_USAGE_OID: &[u64] = &[2, 5, 29, 37];

// id-kp-serverAuth, RFC 5280 section 4.2.1.12
const SERVER_AUTH_OID: &[u64] = &[1, 3, 6, 1, 5, 5, 7, 3, 1];

// AuthorityKeyIdentifier `keyIdentifier [0]` (implicitly tagged), RFC 5280 section 4.2.1.1
const KEY_IDENTIFIER_TAG: yasna::Tag = yasna::Tag {
    tag_class: yasna::TagClass::ContextSpecific,
    tag_number: 0,
};

// GeneralName `iPAddress [7] OCTET STRING` (implicitly tagged), RFC 5280 section 4.2.1.6
const IP_ADDRESS_TAG: yasna::Tag = yasna::Tag {
    tag_class: yasna::TagClass::ContextSpecific,
    tag_number: 7,
};

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

    #[test]
    fn test_encoding_golden() {
        const CA_KEY_PEM: &str = indoc::indoc! {"
            -----BEGIN PRIVATE KEY-----
            MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgSR7TZOtUy2wJw1qx
            Zr83jGmkasU/cNeJgcSGnwusN5qhRANCAATFx4mOL+VgTDggO/Y36cSQ2BS/52bC
            XNpVC9XeCCup8te6Pq+9qe6CBIXn+hrqgLLYKpyyXxEwVJnYnfdbDcJI
            -----END PRIVATE KEY-----
        "};

        const SERVER_KEY_PEM: &str = indoc::indoc! {"
            -----BEGIN PRIVATE KEY-----
            MIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQgjDk/gH9W86FU2Q25
            dbbssISvywVIW89bOWeB/xMjCaOhRANCAAQlteBKeUUNs/vTXPmnwJjiWaUTYap6
            KRocofJRFB5m8KbFtz956LSpIHlHwtgGsNooNoKLYZ6fQNgrvwF+VQJ2
            -----END PRIVATE KEY-----
        "};

        const CA_TBS_CERTIFICATE: &str = indoc::indoc! {"
            3082010da003020102020101300a06082a8648ce3d040302301a311830160603
            5504030c0f70672d657068656d6572616c2043413020170d3735303130313030
            303030305a180f39393939313233313233353935395a301a3118301606035504
            030c0f70672d657068656d6572616c2043413059301306072a8648ce3d020106
            082a8648ce3d03010703420004c5c7898e2fe5604c38203bf637e9c490d814bf
            e766c25cda550bd5de082ba9f2d7ba3eafbda9ee820485e7fa1aea80b2d82a9c
            b25f11305499d89df75b0dc248a3423040300f0603551d130101ff0405300301
            01ff300e0603551d0f0101ff040403020106301d0603551d0e0416041410e1b1
            fe1e99e3eafd4d02cd4e8adc316f08ce16
        "};

        const SERVER_TBS_CERTIFICATE: &str = indoc::indoc! {"
            30820155a003020102020102300a06082a8648ce3d040302301a311830160603
            5504030c0f70672d657068656d6572616c2043413020170d3735303130313030
            303030305a180f39393939313233313233353935395a301e311c301a06035504
            030c1370672d657068656d6572616c207365727665723059301306072a8648ce
            3d020106082a8648ce3d0301070342000425b5e04a79450db3fbd35cf9a7c098
            e259a51361aa7a291a1ca1f251141e66f0a6c5b73f79e8b4a9207947c2d806b0
            da2836828b619e9f40d82bbf017e550276a38185308182300e0603551d0f0101
            ff0404030205a030130603551d25040c300a06082b06010505070301301b0603
            551d11041430128210746573742e6578616d706c652e636f6d301d0603551d0e
            041604142884950d049b2ca3d910c32942a49373dc305819301f0603551d2304
            183016801410e1b1fe1e99e3eafd4d02cd4e8adc316f08ce16
        "};

        fn parse_key(pem: &str) -> aws_lc_rs::signature::EcdsaKeyPair {
            use rustls_pki_types::pem::PemObject;

            aws_lc_rs::signature::EcdsaKeyPair::from_pkcs8(
                &aws_lc_rs::signature::ECDSA_P256_SHA256_ASN1_SIGNING,
                rustls_pki_types::PrivatePkcs8KeyDer::from_pem_slice(pem.as_bytes())
                    .unwrap()
                    .secret_pkcs8_der(),
            )
            .unwrap()
        }

        let ca_key = parse_key(CA_KEY_PEM);
        let server_key = parse_key(SERVER_KEY_PEM);

        assert_eq!(key_pem(&ca_key).unwrap(), CA_KEY_PEM);
        assert_eq!(key_pem(&server_key).unwrap(), SERVER_KEY_PEM);

        assert_eq!(
            hex::encode(ca_tbs_certificate(1, &ca_key).unwrap()),
            CA_TBS_CERTIFICATE.replace('\n', "")
        );

        assert_eq!(
            hex::encode(
                server_tbs_certificate(
                    2,
                    &SubjectAlternativeName::DnsName("test.example.com"),
                    &server_key,
                    &ca_key,
                )
                .unwrap()
            ),
            SERVER_TBS_CERTIFICATE.replace('\n', "")
        );
    }
}
