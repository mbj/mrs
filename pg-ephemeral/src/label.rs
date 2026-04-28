//! pg-ephemeral metadata labels for created containers.
//!
//! Three namespaces:
//!
//! - `pg-ephemeral.superuser.*` — superuser connection identity.
//! - `pg-ephemeral.ssl.*` — server SSL configuration (shared by any client).
//! - `pg-ephemeral.*` — pg-ephemeral's own metadata.

use ociman::label;

pub const IMAGE_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.image");
pub const INSTANCE_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.instance");
pub const SEEDS_KEY: label::Key = label::Key::from_static_or_panic("pg-ephemeral.seeds");
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
