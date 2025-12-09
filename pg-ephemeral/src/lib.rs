pub mod certificate;
pub mod cli;
pub mod config;
mod container;
pub mod definition;
pub mod image;
pub mod seed;

pub use config::{Config, Instance};
pub use container::Container;
pub use definition::Definition;
pub use image::Image;
pub use seed::Command;
pub use seed::DuplicateSeedName;
pub use seed::LoadError;
pub use seed::LoadedSeed;
pub use seed::Seed;
pub use seed::SeedName;
pub use seed::SeedNameError;

pub(crate) const VERSION_STR: &str = env!("CARGO_PKG_VERSION");
pub(crate) const LOCALHOST_IP: std::net::IpAddr =
    std::net::IpAddr::V4(std::net::Ipv4Addr::LOCALHOST);
pub(crate) const UNSPECIFIED_IP: std::net::IpAddr =
    std::net::IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);
pub(crate) const LOCALHOST_HOST_ADDR_IP: pg_client::HostAddr = pg_client::HostAddr(LOCALHOST_IP);

pub fn version() -> &'static semver::Version {
    static VERSION: std::sync::LazyLock<semver::Version> =
        std::sync::LazyLock::new(|| semver::Version::parse(VERSION_STR).unwrap());
    &VERSION
}

pub(crate) fn convert_schema(value: &[u8]) -> String {
    std::str::from_utf8(value)
        .expect("schema contains invalid utf8")
        .to_string()
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
pub struct InstanceName(pub String);

impl std::default::Default for InstanceName {
    fn default() -> Self {
        Self("main".to_string())
    }
}

impl std::str::FromStr for InstanceName {
    type Err = std::convert::Infallible;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self(value.into()))
    }
}

impl std::fmt::Display for InstanceName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(formatter, "{}", self.0)
    }
}

pub type InstanceMap = std::collections::BTreeMap<InstanceName, config::Instance>;
