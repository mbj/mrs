//! pg-ephemeral metadata labels for created containers.
//!
//! Three namespaces:
//!
//! - `pg-ephemeral.superuser.*` — superuser connection identity.
//! - `pg-ephemeral.ssl.*` — server SSL configuration (shared by any client).
//! - `pg-ephemeral.*` — pg-ephemeral's own metadata.

use ociman::label;

use crate::config::SeedConfig;
use crate::seed::{SeedHash, SeedName};

/// One seed as recorded on a container: its on-disk config plus the
/// content-addressed cache hash (or `None` for uncacheable seeds).
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct SeedEntry {
    pub name: SeedName,
    #[serde(flatten)]
    pub config: SeedConfig,
    pub hash: Option<SeedHash>,
}

pub const IMAGE_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.image");
pub const INSTANCE_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.instance");
pub const SEEDS_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.seeds");
pub const SESSION_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.session");
pub const SSL_CA_CERT_PEM_KEY: label::Key =
    label::Key::from_static_or_panic("pg-ephemeral.ssl.ca-cert-pem");
pub const SSL_HOSTNAME_KEY: label::Key =
    label::Key::from_static_or_panic("pg-ephemeral.ssl.hostname");
pub const SUPERUSER_APPLICATION_KEY: label::Key =
    label::Key::from_static_or_panic("pg-ephemeral.superuser.application");
pub const SUPERUSER_DATABASE_KEY: label::Key =
    label::Key::from_static_or_panic("pg-ephemeral.superuser.database");
pub const SUPERUSER_PASSWORD_KEY: label::Key =
    label::Key::from_static_or_panic("pg-ephemeral.superuser.password");
pub const SUPERUSER_USER_KEY: label::Key =
    label::Key::from_static_or_panic("pg-ephemeral.superuser.user");
pub const VERSION_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.version");

/// Errors produced by [`apply`] when one of the metadata values cannot be
/// stored as a label.
#[derive(Debug, thiserror::Error)]
pub enum ApplyError {
    #[error("label {key} value exceeds limits")]
    OversizedValue {
        key: label::Key,
        #[source]
        source: label::Error,
    },
    #[error("failed to serialize seeds as JSON")]
    SeedsJson(#[source] serde_json::Error),
}

/// Decoded pg-ephemeral metadata read back from an image or container's
/// labels. Container labels propagate verbatim from the image they were
/// launched from, so the same shape covers both scopes.
#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
    pub version: semver::Version,
    pub instance: crate::InstanceName,
    pub image: ociman::image::Reference,
    pub superuser: SuperuserMetadata,
    pub seeds: Vec<SeedEntry>,
    pub ssl: Option<SslMetadata>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SuperuserMetadata {
    pub user: pg_client::User,
    pub database: pg_client::Database,
    pub password: pg_client::config::Password,
    pub application: Option<pg_client::config::ApplicationName>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SslMetadata {
    pub hostname: pg_client::config::HostName,
    pub ca_cert_pem: String,
}

/// Errors produced by [`Metadata::prepare_config`] when materializing a
/// [`pg_client::Config`] from decoded metadata.
#[derive(Debug, thiserror::Error)]
pub enum PrepareConfigError {
    #[error("failed to materialize CA certificate")]
    WriteCaCert(#[from] crate::certificate::WriteCaPemError),
}

impl Metadata {
    /// Materialize a [`pg_client::Config`] for the given runtime endpoint.
    ///
    /// If [`Metadata::ssl`] is set, the CA certificate PEM is written to a
    /// uniquely-named file in `std::env::temp_dir()` via
    /// [`crate::certificate::write_ca_pem_to_temp`]. The file is not
    /// removed automatically.
    pub fn prepare_config(
        self,
        host: pg_client::config::Host,
        host_addr: Option<pg_client::config::HostAddr>,
        port: pg_client::config::Port,
    ) -> Result<pg_client::Config, PrepareConfigError> {
        let (ssl_mode, ssl_root_cert) = match self.ssl {
            Some(ssl) => {
                let path = crate::certificate::write_ca_pem_to_temp(ssl.ca_cert_pem.as_bytes())?;
                (
                    pg_client::config::SslMode::VerifyFull,
                    Some(pg_client::config::SslRootCert::File(path)),
                )
            }
            None => (pg_client::config::SslMode::Disable, None),
        };

        Ok(pg_client::Config {
            endpoint: pg_client::config::Endpoint::Network {
                host,
                channel_binding: None,
                host_addr,
                port: Some(port),
            },
            session: pg_client::config::Session {
                application_name: self.superuser.application,
                database: self.superuser.database,
                password: Some(self.superuser.password),
                user: self.superuser.user,
            },
            ssl_mode,
            ssl_root_cert,
            sqlx: Default::default(),
        })
    }
}

/// Errors produced by [`read_image`] / [`read_container`] when stored
/// label values cannot be decoded back into [`Metadata`].
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error("required label {0} is missing")]
    Missing(label::Key),
    #[error("label {key} value could not be parsed: {message}")]
    ValueParse { key: label::Key, message: String },
    #[error("label {key} JSON could not be decoded")]
    Json {
        key: label::Key,
        #[source]
        source: serde_json::Error,
    },
    #[error(
        "ssl labels are inconsistent: {present} is set but {missing} is not — both must be \
         present together"
    )]
    SslLabelsInconsistent {
        present: label::Key,
        missing: label::Key,
    },
}

/// Decode the pg-ephemeral metadata from an image's label set.
pub fn read_image(labels: &ociman::label::ImageLabels) -> Result<Metadata, ReadError> {
    read(labels)
}

/// Decode the pg-ephemeral metadata from a container's label set.
pub fn read_container(labels: &ociman::label::ContainerLabels) -> Result<Metadata, ReadError> {
    read(labels)
}

fn read<S: ociman::label::Scope>(
    labels: &ociman::label::ReadLabels<S>,
) -> Result<Metadata, ReadError> {
    let version = parse_required(labels, &VERSION_KEY)?;
    let instance = parse_required(labels, &INSTANCE_KEY)?;
    let image = parse_required_string_err(labels, &IMAGE_KEY)?;

    let superuser = SuperuserMetadata {
        user: parse_required(labels, &SUPERUSER_USER_KEY)?,
        database: parse_required(labels, &SUPERUSER_DATABASE_KEY)?,
        password: parse_required(labels, &SUPERUSER_PASSWORD_KEY)?,
        application: parse_optional(labels, &SUPERUSER_APPLICATION_KEY)?,
    };

    let seeds_json = required(labels, &SEEDS_KEY)?;
    let seeds: Vec<SeedEntry> =
        serde_json::from_str(seeds_json).map_err(|source| ReadError::Json {
            key: SEEDS_KEY.clone(),
            source,
        })?;

    let ssl_hostname: Option<pg_client::config::HostName> =
        parse_optional(labels, &SSL_HOSTNAME_KEY)?;
    let ssl_ca_cert_pem = optional(labels, &SSL_CA_CERT_PEM_KEY).map(str::to_owned);

    let ssl = match (ssl_hostname, ssl_ca_cert_pem) {
        (Some(hostname), Some(ca_cert_pem)) => Some(SslMetadata {
            hostname,
            ca_cert_pem,
        }),
        (None, None) => None,
        (Some(_), None) => {
            return Err(ReadError::SslLabelsInconsistent {
                present: SSL_HOSTNAME_KEY.clone(),
                missing: SSL_CA_CERT_PEM_KEY.clone(),
            });
        }
        (None, Some(_)) => {
            return Err(ReadError::SslLabelsInconsistent {
                present: SSL_CA_CERT_PEM_KEY.clone(),
                missing: SSL_HOSTNAME_KEY.clone(),
            });
        }
    };

    Ok(Metadata {
        version,
        instance,
        image,
        superuser,
        seeds,
        ssl,
    })
}

fn optional<'a, S: ociman::label::Scope>(
    labels: &'a ociman::label::ReadLabels<S>,
    key: &label::Key,
) -> Option<&'a str> {
    labels.get(key).map(ociman::label::ReadValue::as_str)
}

fn required<'a, S: ociman::label::Scope>(
    labels: &'a ociman::label::ReadLabels<S>,
    key: &label::Key,
) -> Result<&'a str, ReadError> {
    optional(labels, key).ok_or_else(|| ReadError::Missing(key.clone()))
}

fn parse_required<T, S: ociman::label::Scope>(
    labels: &ociman::label::ReadLabels<S>,
    key: &label::Key,
) -> Result<T, ReadError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let raw = required(labels, key)?;
    raw.parse().map_err(|error: T::Err| ReadError::ValueParse {
        key: key.clone(),
        message: error.to_string(),
    })
}

fn parse_optional<T, S: ociman::label::Scope>(
    labels: &ociman::label::ReadLabels<S>,
    key: &label::Key,
) -> Result<Option<T>, ReadError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match optional(labels, key) {
        Some(raw) => raw
            .parse()
            .map(Some)
            .map_err(|error: T::Err| ReadError::ValueParse {
                key: key.clone(),
                message: error.to_string(),
            }),
        None => Ok(None),
    }
}

/// Specialised variant for types whose `FromStr::Err` is `String` (e.g.
/// [`ociman::image::Reference`]).
fn parse_required_string_err<T, S: ociman::label::Scope>(
    labels: &ociman::label::ReadLabels<S>,
    key: &label::Key,
) -> Result<T, ReadError>
where
    T: std::str::FromStr<Err = String>,
{
    let raw = required(labels, key)?;
    raw.parse()
        .map_err(|message: String| ReadError::ValueParse {
            key: key.clone(),
            message,
        })
}

/// Apply pg-ephemeral's metadata labels onto an [`ociman::Definition`].
pub(crate) fn apply(
    ociman_definition: ociman::Definition,
    definition: &crate::Definition,
    password: &pg_client::config::Password,
    ssl_bundle: Option<&crate::certificate::Bundle>,
    seeds: &[SeedEntry],
) -> Result<ociman::Definition, ApplyError> {
    let image_reference = ociman::image::Reference::from(&definition.image).to_string();
    let seeds_json = serde_json::to_string(seeds).map_err(ApplyError::SeedsJson)?;

    let mut pairs: Vec<(label::Key, label::Value)> = vec![
        (
            VERSION_KEY.clone(),
            to_value(&VERSION_KEY, crate::VERSION_STR)?,
        ),
        (
            INSTANCE_KEY.clone(),
            to_value(&INSTANCE_KEY, definition.instance_name.as_str())?,
        ),
        (IMAGE_KEY.clone(), to_value(&IMAGE_KEY, &image_reference)?),
        (
            SUPERUSER_USER_KEY.clone(),
            to_value(&SUPERUSER_USER_KEY, definition.superuser.as_ref())?,
        ),
        (
            SUPERUSER_DATABASE_KEY.clone(),
            to_value(&SUPERUSER_DATABASE_KEY, definition.database.as_ref())?,
        ),
        (
            SUPERUSER_PASSWORD_KEY.clone(),
            to_value(&SUPERUSER_PASSWORD_KEY, password.as_ref())?,
        ),
        (SEEDS_KEY.clone(), to_value(&SEEDS_KEY, &seeds_json)?),
    ];

    if let Some(application_name) = &definition.application_name {
        pairs.push((
            SUPERUSER_APPLICATION_KEY.clone(),
            to_value(&SUPERUSER_APPLICATION_KEY, application_name.as_ref())?,
        ));
    }

    if let Some(crate::definition::SslConfig::Generated { hostname }) = &definition.ssl_config {
        pairs.push((
            SSL_HOSTNAME_KEY.clone(),
            to_value(&SSL_HOSTNAME_KEY, hostname.as_str())?,
        ));
    }

    if let Some(bundle) = ssl_bundle {
        pairs.push((
            SSL_CA_CERT_PEM_KEY.clone(),
            to_value(&SSL_CA_CERT_PEM_KEY, &bundle.ca_cert_pem)?,
        ));
    }

    if let Some(session_name) = &definition.session_name {
        pairs.push((
            SESSION_KEY.clone(),
            to_value(&SESSION_KEY, session_name.as_str())?,
        ));
    }

    Ok(ociman_definition.labels(pairs.iter().map(|(key, value)| (key, value))))
}

fn to_value(key: &label::Key, raw: &str) -> Result<label::Value, ApplyError> {
    label::Value::try_from(raw.to_string()).map_err(|source| ApplyError::OversizedValue {
        key: key.clone(),
        source,
    })
}

/// Build the [`SeedEntry`] list by pairing each loaded seed's cache hash with
/// the original [`SeedConfig`] from the definition.
pub(crate) fn build_seed_entries(
    definition: &crate::Definition,
    loaded_seeds: &crate::seed::LoadedSeeds<'_>,
) -> Vec<SeedEntry> {
    let mut entries = Vec::with_capacity(definition.seeds.len());
    for loaded_seed in loaded_seeds.iter_seeds() {
        let name = loaded_seed.name().clone();
        let seed = match definition.seeds.get(loaded_seed.name()) {
            Some(seed) => seed,
            None => unreachable!(
                "loaded seed {name} must exist in definition.seeds; \
                 load_seeds populates from this map",
            ),
        };
        entries.push(SeedEntry {
            name,
            config: seed.into(),
            hash: loaded_seed.cache_status().hash().cloned(),
        });
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed::SeedCacheConfig;

    #[test]
    fn seed_entry_json_round_trip_compliant_hash() {
        let entry = SeedEntry {
            name: "schema".parse().unwrap(),
            config: SeedConfig::SqlFile {
                path: "schema.sql".into(),
                git_revision: None,
            },
            hash: Some(
                "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
                    .parse()
                    .unwrap(),
            ),
        };

        let json = serde_json::to_string(&entry).unwrap();
        assert_eq!(
            json,
            r#"{"name":"schema","type":"sql-file","path":"schema.sql","git_revision":null,"hash":"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"}"#
        );

        let parsed: SeedEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, entry);
    }

    #[test]
    fn seed_entry_json_round_trip_uncacheable() {
        let entry = SeedEntry {
            name: "dynamic".parse().unwrap(),
            config: SeedConfig::Command {
                command: "psql".to_string(),
                arguments: vec!["-c".to_string(), "SELECT 1".to_string()],
                cache: SeedCacheConfig::None,
            },
            hash: None,
        };

        let json = serde_json::to_string(&entry).unwrap();
        let parsed: SeedEntry = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, entry);

        // Spot-check: hash field present and null.
        let value: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(value["hash"], serde_json::Value::Null);
    }

    #[test]
    fn seed_list_json_round_trip() {
        let entries = vec![
            SeedEntry {
                name: "a".parse().unwrap(),
                config: SeedConfig::SqlStatement {
                    statement: "CREATE TABLE t (id INT)".to_string(),
                },
                hash: Some(
                    "1111111111111111111111111111111111111111111111111111111111111111"
                        .parse()
                        .unwrap(),
                ),
            },
            SeedEntry {
                name: "b".parse().unwrap(),
                config: SeedConfig::ContainerScript {
                    script: "apt-get install -y foo".to_string(),
                },
                hash: Some(
                    "2222222222222222222222222222222222222222222222222222222222222222"
                        .parse()
                        .unwrap(),
                ),
            },
        ];

        let json = serde_json::to_string(&entries).unwrap();
        let parsed: Vec<SeedEntry> = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, entries);
    }
}
