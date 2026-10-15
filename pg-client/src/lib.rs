#![doc = include_str!("../README.md")]
#![deny(clippy::as_conversions)]

pub mod authentication;
pub mod catalog;
pub mod config;
pub mod connection;
pub mod name;
pub mod parameter;
pub mod pg_dump;
pub mod protocol;
pub mod session;
pub mod value;

pub use connection::Connection;
pub use name::{Database, QualifiedTable, Role, User};
pub use pg_dump::{PgSchemaDump, RestrictKey};

#[cfg(feature = "tools")]
pub mod tools;

pub mod libpq;
