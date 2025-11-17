mod container;

pub mod certificate;
pub mod cli;
pub mod config;
pub mod definition;
pub mod image;
pub mod seed;

pub use config::Config;
pub use container::Container;
pub use definition::{BackendSelection, Definition};
pub use image::Image;
pub use seed::Command;
pub use seed::DuplicateSeedName;
pub use seed::Seed;
pub use seed::SeedName;
pub use seed::SeedNameError;

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

pub type InstanceMap = std::collections::BTreeMap<InstanceName, Definition>;
