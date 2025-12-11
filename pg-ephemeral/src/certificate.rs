pub struct Bundle {
    pub ca_cert_pem: String,
    pub server_cert_pem: String,
    pub server_key_pem: String,
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

        let server_cert = server_params.signed_by(&server_key, &ca_cert, &ca_key)?;
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
        use x509_parser::prelude::*;
        use x509_parser::verify::verify_signature;

        let (_, pem) = parse_x509_pem(ca_cert_pem.as_bytes()).map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse CA certificate PEM: {e}"))
        })?;
        let ca_cert = pem.parse_x509().map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse CA certificate: {e}"))
        })?;

        let ca_basic_constraints = ca_cert
            .basic_constraints()
            .map_err(|e| {
                ValidationError::ValidationError(format!("CA should have basic constraints: {e}"))
            })?
            .ok_or_else(|| {
                ValidationError::ValidationError(
                    "CA basic constraints should be present".to_string(),
                )
            })?;

        if !ca_basic_constraints.value.ca {
            return Err(ValidationError::ValidationError(
                "CA certificate should have CA=true".to_string(),
            ));
        }

        if ca_cert.subject().to_string() != "CN=pg-ephemeral CA" {
            return Err(ValidationError::ValidationError(format!(
                "CA subject should be CN=pg-ephemeral CA, got: {}",
                ca_cert.subject()
            )));
        }

        let (_, pem) = parse_x509_pem(server_cert_pem.as_bytes()).map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse server certificate PEM: {e}"))
        })?;
        let server_cert = pem.parse_x509().map_err(|e| {
            ValidationError::ParseError(format!("Failed to parse server certificate: {e}"))
        })?;

        if server_cert.subject().to_string() != "CN=pg-ephemeral server" {
            return Err(ValidationError::ValidationError(format!(
                "Server subject should be CN=pg-ephemeral server, got: {}",
                server_cert.subject()
            )));
        }

        // Validate that the DNS name in SAN matches the expected hostname
        let san_ext = server_cert
            .subject_alternative_name()
            .map_err(|e| {
                ValidationError::ValidationError(format!(
                    "Failed to read subject alternative name: {e}"
                ))
            })?
            .ok_or_else(|| {
                ValidationError::ValidationError(
                    "Server certificate should have subject alternative name extension".to_string(),
                )
            })?;

        let dns_names: Vec<&str> = san_ext
            .value
            .general_names
            .iter()
            .filter_map(|name| {
                if let x509_parser::extensions::GeneralName::DNSName(dns) = name {
                    Some(*dns)
                } else {
                    None
                }
            })
            .collect();

        if !dns_names.contains(&expected_hostname) {
            return Err(ValidationError::ValidationError(format!(
                "Server certificate DNS names {dns_names:?} should contain {expected_hostname}"
            )));
        }

        if server_cert.issuer().to_string() != ca_cert.subject().to_string() {
            return Err(ValidationError::ValidationError(
                "Server certificate issuer should match CA subject".to_string(),
            ));
        }

        verify_signature(
            ca_cert.public_key(),
            &server_cert.signature_algorithm,
            &server_cert.signature_value,
            server_cert.tbs_certificate.as_ref(),
        )
        .map_err(|e| {
            ValidationError::ValidationError(format!(
                "Server certificate should be signed by CA: {e}"
            ))
        })?;

        let server_key = rcgen::KeyPair::from_pem(&server_key_pem)
            .map_err(|e| ValidationError::ParseError(format!("Failed to parse server key: {e}")))?;

        let server_key_public = server_key.public_key_der();
        let cert_public_key = server_cert.public_key().raw;

        if server_key_public != cert_public_key {
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
