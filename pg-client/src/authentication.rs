//! Cleartext and MD5 prove the client to the server and tell the client
//! nothing in return, which is the property SCRAM's server signature exists to
//! supply; GSSAPI would mean linking a Kerberos library.
//!
//! See: <https://datatracker.ietf.org/doc/html/rfc7677> for SCRAM-SHA-256 and
//! <https://datatracker.ietf.org/doc/html/rfc5802> for SCRAM itself.

use std::borrow::Cow;

use crate::name::User;

use base64::Engine as _;
use hmac::{KeyInit as _, Mac as _};
use sha2::Digest as _;

/// A server offers the `-PLUS` variant exactly when the connection is
/// encrypted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mechanism {
    ScramSha256,
    ScramSha256Plus,
}

impl Mechanism {
    #[must_use]
    pub fn name(self) -> &'static crate::protocol::String<'static> {
        static SCRAM_SHA_256: crate::protocol::String<'static> =
            crate::protocol::String::from_static_or_panic("SCRAM-SHA-256");
        static SCRAM_SHA_256_PLUS: crate::protocol::String<'static> =
            crate::protocol::String::from_static_or_panic("SCRAM-SHA-256-PLUS");

        match self {
            Self::ScramSha256 => &SCRAM_SHA_256,
            Self::ScramSha256Plus => &SCRAM_SHA_256_PLUS,
        }
    }

    #[must_use]
    pub fn from_name(name: &str) -> Option<Self> {
        [Self::ScramSha256, Self::ScramSha256Plus]
            .into_iter()
            .find(|mechanism| mechanism.name().as_str() == name)
    }
}

/// What the client declares about channel binding in its `gs2-cbind-flag`.
///
/// [`Self::Unoffered`] is what makes a stripped mechanism list detectable: a
/// man in the middle can edit that list but not the client's claim, which the
/// server checks against what it advertised.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ClientBinding {
    Unavailable,
    Unoffered,
    Bound(ChannelBindingData),
}

impl ClientBinding {
    #[must_use]
    pub const fn mechanism(&self) -> Mechanism {
        match self {
            Self::Bound(_) => Mechanism::ScramSha256Plus,
            Self::Unavailable | Self::Unoffered => Mechanism::ScramSha256,
        }
    }

    /// The empty field between the commas is the GS2 authorization identity.
    fn header(&self) -> String {
        let flag = match self {
            Self::Unavailable => "n",
            Self::Unoffered => "y",
            Self::Bound(data) => data.flag(),
        };

        format!("{flag},,")
    }
}

/// RFC 5802 requires the nonce be unguessable but sets no length; this matches
/// what libpq uses.
const NONCE_BYTES: usize = 18;

type HmacSha256 = hmac::Hmac<sha2::Sha256>;

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ScramError {
    #[error("server message is not UTF-8")]
    NotUtf8,
    #[error("server message is missing its {attribute:?} attribute")]
    MissingAttribute { attribute: char },
    #[error("server {attribute:?} attribute is malformed: {value:?}")]
    MalformedAttribute { attribute: char, value: String },
    /// A server that does not echo the client nonce as a prefix is not the one
    /// that received our first message.
    #[error("server nonce does not extend the one that was sent")]
    NonceMismatch,
    #[error("server rejected the exchange: {0}")]
    Rejected(String),
    #[error("server signature does not verify")]
    SignatureMismatch,
}

/// SCRAM derives its keys from `Hi(Normalize(password), salt, i)`, and RFC 7677
/// fixes `Normalize` as SASLprep. This is therefore part of the key derivation
/// and not a validation step: the server built its stored verifier over its own
/// normalized form, so preparing the password differently derives a different
/// `SaltedPassword` and the exchange fails.
///
/// PostgreSQL keeps the raw password when SASLprep does not succeed, in both
/// `pg_be_scram_build_secret` and `scram_verify_plain_password`:
///
/// ```c
/// rc = pg_saslprep(password, &prep_password);
/// if (rc == SASLPREP_SUCCESS)
///     password = (const char *) prep_password;
/// ```
///
/// Its reason is that the encoding in use during authentication is undefined,
/// so a password need not be valid UTF-8 at all. Whatever the reason, the
/// verifier on the server was then built over the raw bytes, and refusing here
/// would make such a password unusable rather than safer.
///
/// That path is well travelled. SASLprep prohibits unassigned code points, and
/// the table it consults is frozen at Unicode 3.2, published in 2002, so every
/// character assigned since is unassigned as far as it is concerned — emoji
/// included. A password holding one normalizes to nothing and is hashed raw.
///
/// Two properties of the normalization itself are worth knowing, neither of
/// them ours to choose:
///
/// - It is NFKC, which folds compatibility variants together, so `Ⅸ` becomes
///   `IX`, `ª` becomes `a`, and fullwidth forms become ASCII. Distinct
///   characters collapse into one password. RFC 8265 replaced SASLprep for
///   precisely this reason, mandating NFC to get "fewer false accepts", but the
///   server derives with NFKC and matching it is the only way in.
/// - The folding does not cross scripts. Cyrillic `а` and Latin `a` remain
///   different passwords.
fn prepare(password: &str) -> Cow<'_, str> {
    stringprep::saslprep(password).unwrap_or(Cow::Borrowed(password))
}

/// `pg_hba.conf` matches on the user name, and the stored verifier is per user.
#[derive(Clone, Debug)]
pub struct Config {
    pub user: User<'static>,
    pub methods: Methods,
}

/// The server chooses the method, from `pg_hba.conf`; a client cannot ask for
/// one. These are the requests this client answers, and any other ends the
/// startup with [`crate::session::StartError::MethodNotConfigured`].
#[derive(Clone, Debug)]
pub struct Methods {
    /// Accept a server that sends `AuthenticationOk` without requesting any
    /// credential. On the wire the `pg_hba.conf` methods `trust`, `peer`,
    /// `ident` and `cert` all look like this, so accepting one accepts them all.
    pub accept_unchallenged: bool,
    pub scram: Option<method::Scram>,
    pub cleartext_password: Option<method::CleartextPassword>,
}

pub mod method {
    #[derive(Clone, Debug)]
    pub struct Scram {
        pub password: super::Password,
        pub channel_binding: crate::config::ChannelBinding,
    }

    /// What `pg_hba.conf` methods `password`, `ldap`, `pam`, `radius` and
    /// `bsd` request, and how IAM tokens for RDS and Cloud SQL are sent.
    ///
    /// The password crosses the connection as is, so it is only sent over TLS
    /// or a Unix socket, never over unencrypted TCP.
    #[derive(Clone, Debug)]
    pub struct CleartextPassword {
        pub password: super::Password,
    }
}

#[derive(Clone)]
pub enum Password {
    Static(crate::config::Password),
    Dynamic(std::sync::Arc<dyn PasswordSource>),
}

impl core::fmt::Debug for Password {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Static(_) => formatter.write_str("Static(..)"),
            Self::Dynamic(_) => formatter.write_str("Dynamic(..)"),
        }
    }
}

pub type PasswordError = Box<dyn std::error::Error + Send + Sync>;

pub trait PasswordSource: Send + Sync {
    fn password(
        &self,
    ) -> core::pin::Pin<
        Box<dyn Future<Output = Result<crate::config::Password, PasswordError>> + Send + '_>,
    >;
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum ChannelBindingError {
    #[error("server certificate could not be parsed")]
    Certificate,
    /// RFC 5929 leaves the binding undefined where the signature algorithm
    /// names no single hash, as Ed25519 does not.
    #[error("signature algorithm {algorithm} names no hash to bind with")]
    UnsupportedSignature { algorithm: String },
}

// RFC 8017, appendix A.2.4
const MD5_WITH_RSA: &[u64] = &[1, 2, 840, 113_549, 1, 1, 4];
const SHA1_WITH_RSA: &[u64] = &[1, 2, 840, 113_549, 1, 1, 5];
const SHA256_WITH_RSA: &[u64] = &[1, 2, 840, 113_549, 1, 1, 11];
const SHA384_WITH_RSA: &[u64] = &[1, 2, 840, 113_549, 1, 1, 12];
const SHA512_WITH_RSA: &[u64] = &[1, 2, 840, 113_549, 1, 1, 13];

// RFC 5758, section 3.2
const ECDSA_WITH_SHA224: &[u64] = &[1, 2, 840, 10_045, 4, 3, 1];
const ECDSA_WITH_SHA256: &[u64] = &[1, 2, 840, 10_045, 4, 3, 2];
const ECDSA_WITH_SHA384: &[u64] = &[1, 2, 840, 10_045, 4, 3, 3];
const ECDSA_WITH_SHA512: &[u64] = &[1, 2, 840, 10_045, 4, 3, 4];

/// The `cbind-data` a SCRAM proof commits to.
///
/// Neither side sends it: each derives it from the certificate it saw, so they
/// agree only if they saw the same one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChannelBindingData(Vec<u8>);

impl ChannelBindingData {
    /// RFC 5929 uses the hash the certificate's own signature algorithm names,
    /// except that MD5 and SHA-1 are promoted to SHA-256.
    pub fn from_certificate(
        certificate: &tokio_rustls::rustls::pki_types::CertificateDer<'_>,
    ) -> Result<Self, ChannelBindingError> {
        // RFC 5929 hashes the certificate "octet for octet, in the server's
        // Certificate message".
        let certificate: &[u8] = certificate;

        let algorithm = yasna::parse_der(certificate, |reader| {
            reader.read_sequence(|certificate| {
                certificate.next().read_der()?; // tbsCertificate

                let algorithm = certificate.next().read_sequence(|identifier| {
                    let algorithm = identifier.next().read_oid()?;

                    identifier.read_optional(|parameters| parameters.read_der())?;

                    Ok(algorithm)
                })?;

                certificate.next().read_der()?; // signatureValue

                Ok(algorithm)
            })
        })
        .map_err(|_| ChannelBindingError::Certificate)?;

        let hash = match algorithm.components().as_slice() {
            MD5_WITH_RSA | SHA1_WITH_RSA | SHA256_WITH_RSA | ECDSA_WITH_SHA256 => {
                sha2::Sha256::digest(certificate).to_vec()
            }
            SHA384_WITH_RSA | ECDSA_WITH_SHA384 => sha2::Sha384::digest(certificate).to_vec(),
            SHA512_WITH_RSA | ECDSA_WITH_SHA512 => sha2::Sha512::digest(certificate).to_vec(),
            ECDSA_WITH_SHA224 => sha2::Sha224::digest(certificate).to_vec(),
            _ => {
                return Err(ChannelBindingError::UnsupportedSignature {
                    algorithm: algorithm.to_string(),
                });
            }
        };

        Ok(Self(hash))
    }

    const fn flag(&self) -> &'static str {
        "p=tls-server-end-point"
    }

    fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

fn hmac(key: &[u8], message: &[u8]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key length");

    mac.update(message);

    mac.finalize().into_bytes().into()
}

fn sha256(message: &[u8]) -> [u8; 32] {
    sha2::Sha256::digest(message).into()
}

fn attribute(message: &str, key: char) -> Result<&str, ScramError> {
    message
        .split(',')
        .find_map(|part| part.strip_prefix(key)?.strip_prefix('='))
        .ok_or(ScramError::MissingAttribute { attribute: key })
}

#[derive(Clone, Debug)]
pub struct Scram {
    password: String,
    client_first_bare: String,
    nonce: String,
    binding: ClientBinding,
}

impl Scram {
    /// The user name is left empty: PostgreSQL takes it from the startup packet
    /// and ignores this field.
    #[must_use]
    pub fn new(password: &str, nonce: String, binding: ClientBinding) -> (Self, Vec<u8>) {
        let password = prepare(password).into_owned();

        let client_first_bare = format!("n=,r={nonce}");
        let payload = format!("{}{client_first_bare}", binding.header());

        (
            Self {
                password,
                client_first_bare,
                nonce,
                binding,
            },
            payload.into_bytes(),
        )
    }

    #[must_use]
    pub fn nonce() -> String {
        let mut bytes = [0u8; NONCE_BYTES];

        rand::fill(&mut bytes);

        base64::engine::general_purpose::STANDARD.encode(bytes)
    }

    pub fn respond(self, server_first: &[u8]) -> Result<(Verifier, Vec<u8>), ScramError> {
        let server_first = core::str::from_utf8(server_first).map_err(|_| ScramError::NotUtf8)?;

        let combined_nonce = attribute(server_first, 'r')?;

        if !combined_nonce.starts_with(&self.nonce) {
            return Err(ScramError::NonceMismatch);
        }

        let salt = base64::engine::general_purpose::STANDARD
            .decode(attribute(server_first, 's')?)
            .map_err(|_| ScramError::MalformedAttribute {
                attribute: 's',
                value: attribute(server_first, 's').unwrap_or_default().to_owned(),
            })?;

        let iterations: u32 =
            attribute(server_first, 'i')?
                .parse()
                .map_err(|_| ScramError::MalformedAttribute {
                    attribute: 'i',
                    value: attribute(server_first, 'i').unwrap_or_default().to_owned(),
                })?;

        let mut salted = [0u8; 32];

        pbkdf2::pbkdf2_hmac::<sha2::Sha256>(
            self.password.as_bytes(),
            &salt,
            iterations,
            &mut salted,
        );

        let client_key = hmac(&salted, b"Client Key");
        let stored_key = sha256(&client_key);
        let server_key = hmac(&salted, b"Server Key");

        let mut channel = self.binding.header().into_bytes();

        if let ClientBinding::Bound(data) = &self.binding {
            channel.extend_from_slice(data.as_slice());
        }

        let without_proof = format!(
            "c={},r={combined_nonce}",
            base64::engine::general_purpose::STANDARD.encode(channel),
        );
        let auth_message = format!("{},{server_first},{without_proof}", self.client_first_bare);

        let client_signature = hmac(&stored_key, auth_message.as_bytes());

        let mut proof = client_key;

        for (byte, signature) in proof.iter_mut().zip(client_signature) {
            *byte ^= signature;
        }

        let payload = format!(
            "{without_proof},p={}",
            base64::engine::general_purpose::STANDARD.encode(proof),
        );

        Ok((
            Verifier {
                server_signature: hmac(&server_key, auth_message.as_bytes()),
            },
            payload.into_bytes(),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Verifier {
    server_signature: [u8; 32],
}

impl Verifier {
    /// This is not a formality: it is what proves the server holds the
    /// password, rather than merely having accepted ours.
    pub fn verify(self, server_final: &[u8]) -> Result<(), ScramError> {
        let server_final = core::str::from_utf8(server_final).map_err(|_| ScramError::NotUtf8)?;

        if let Ok(error) = attribute(server_final, 'e') {
            return Err(ScramError::Rejected(error.to_owned()));
        }

        let signature = base64::engine::general_purpose::STANDARD
            .decode(attribute(server_final, 'v')?)
            .map_err(|_| ScramError::MalformedAttribute {
                attribute: 'v',
                value: attribute(server_final, 'v').unwrap_or_default().to_owned(),
            })?;

        if signature == self.server_signature {
            Ok(())
        } else {
            Err(ScramError::SignatureMismatch)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    /// The worked example from RFC 7677 section 3.
    #[test]
    fn matches_the_rfc_7677_exchange() {
        let nonce = "rOprNGfwEbeRWgbNEkqO".to_owned();

        let (mut scram, _) = Scram::new("pencil", nonce.clone(), ClientBinding::Unavailable);

        // The RFC names the user in the client first message.
        scram.client_first_bare = format!("n=user,r={nonce}");

        let server_first = concat!(
            "r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,",
            "s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096",
        );

        let (verifier, payload) = scram.respond(server_first.as_bytes()).unwrap();

        assert_eq!(
            core::str::from_utf8(&payload).unwrap(),
            concat!(
                "c=biws,r=rOprNGfwEbeRWgbNEkqO%hvYDpWUa2RaTCAfuxFIlj)hNlF$k0,",
                "p=dHzbZapWIk4jUhN+Ute9ytag9zjfMHgsqmmiz7AndVQ=",
            ),
        );

        assert_eq!(
            verifier.verify(b"v=6rriTRBi23WpRR/wtup+mMhUZUn/dB5nLTJRsjl95G4="),
            Ok(()),
        );
    }

    #[test]
    fn normalizes_a_password_saslprep_accepts() {
        let (scram, _) = Scram::new(
            "pa\u{FB01}sh",
            "clientnonce".to_owned(),
            ClientBinding::Unavailable,
        );

        assert_eq!(scram.password, "pafish");
    }

    #[test]
    fn keeps_a_password_saslprep_rejects() {
        let (scram, _) = Scram::new(
            "pass\u{1F512}word",
            "clientnonce".to_owned(),
            ClientBinding::Unavailable,
        );

        assert_eq!(scram.password, "pass\u{1F512}word");
    }

    fn certificate(
        algorithm: &[u64],
        null_parameters: bool,
    ) -> tokio_rustls::rustls::pki_types::CertificateDer<'static> {
        tokio_rustls::rustls::pki_types::CertificateDer::from(yasna::construct_der(|writer| {
            writer.write_sequence(|certificate| {
                certificate.next().write_sequence(|_| ()); // tbsCertificate
                certificate.next().write_sequence(|identifier| {
                    identifier
                        .next()
                        .write_oid(&yasna::models::ObjectIdentifier::from_slice(algorithm));

                    if null_parameters {
                        identifier.next().write_null();
                    }
                });
                certificate.next().write_bitvec_bytes(&[0xab; 64], 512); // signatureValue
            });
        }))
    }

    #[test]
    fn derives_binding_data_from_the_signature_hash() {
        type Digest = fn(&[u8]) -> Vec<u8>;

        let sha256: Digest = |bytes| sha2::Sha256::digest(bytes).to_vec();
        let sha384: Digest = |bytes| sha2::Sha384::digest(bytes).to_vec();
        let sha512: Digest = |bytes| sha2::Sha512::digest(bytes).to_vec();
        let sha224: Digest = |bytes| sha2::Sha224::digest(bytes).to_vec();

        let cases: [(&[u64], bool, Digest); 9] = [
            (MD5_WITH_RSA, true, sha256),
            (SHA1_WITH_RSA, true, sha256),
            (SHA256_WITH_RSA, true, sha256),
            (SHA384_WITH_RSA, true, sha384),
            (SHA512_WITH_RSA, true, sha512),
            (ECDSA_WITH_SHA224, false, sha224),
            (ECDSA_WITH_SHA256, false, sha256),
            (ECDSA_WITH_SHA384, false, sha384),
            (ECDSA_WITH_SHA512, false, sha512),
        ];

        for (algorithm, null_parameters, hash) in cases {
            let certificate = certificate(algorithm, null_parameters);

            assert_eq!(
                ChannelBindingData::from_certificate(&certificate),
                Ok(ChannelBindingData(hash(&certificate))),
                "{algorithm:?}",
            );
        }
    }

    #[test]
    fn refuses_a_certificate_whose_signature_names_no_hash() {
        assert_eq!(
            ChannelBindingData::from_certificate(&certificate(&[1, 3, 101, 112], false)),
            Err(ChannelBindingError::UnsupportedSignature {
                algorithm: "1.3.101.112".to_owned(),
            }),
        );
    }

    #[test]
    fn refuses_something_that_is_not_a_certificate() {
        assert_eq!(
            ChannelBindingData::from_certificate(
                &tokio_rustls::rustls::pki_types::CertificateDer::from(
                    b"not a certificate".to_vec()
                )
            ),
            Err(ChannelBindingError::Certificate),
        );
    }

    #[test]
    fn refuses_every_truncation_of_a_certificate() {
        let certificate = certificate(SHA256_WITH_RSA, true);

        for length in 0..certificate.len() {
            assert_eq!(
                ChannelBindingData::from_certificate(
                    &tokio_rustls::rustls::pki_types::CertificateDer::from(
                        certificate[..length].to_vec()
                    )
                ),
                Err(ChannelBindingError::Certificate),
                "{length} bytes",
            );
        }
    }

    #[test]
    fn refuses_a_certificate_followed_by_trailing_bytes() {
        let mut bytes = certificate(ECDSA_WITH_SHA256, false).to_vec();
        bytes.push(0);

        assert_eq!(
            ChannelBindingData::from_certificate(
                &tokio_rustls::rustls::pki_types::CertificateDer::from(bytes)
            ),
            Err(ChannelBindingError::Certificate),
        );
    }

    #[test]
    fn rejects_a_server_that_does_not_echo_the_nonce() {
        let (scram, _) = Scram::new(
            "pencil",
            "clientnonce".to_owned(),
            ClientBinding::Unavailable,
        );

        assert_eq!(
            scram
                .respond(b"r=somethingelse,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096")
                .err(),
            Some(ScramError::NonceMismatch),
        );
    }

    #[test]
    fn rejects_a_server_signature_that_does_not_verify() {
        let (scram, _) = Scram::new(
            "pencil",
            "clientnonce".to_owned(),
            ClientBinding::Unavailable,
        );

        let (verifier, _) = scram
            .respond(b"r=clientnonceserver,s=W22ZaJ0SNY7soEsUEjb6gQ==,i=4096")
            .unwrap();

        assert_eq!(
            verifier.verify(b"v=6rriTRBi23WpRR/wtup+mMhUZUn/dB5nLTJRsjl95G4="),
            Err(ScramError::SignatureMismatch),
        );
    }
}
