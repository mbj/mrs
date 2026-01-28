const CONFIG_DIR: &str = "swaybridge";

pub(crate) fn config_dir() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|dir| dir.join(CONFIG_DIR))
}

pub(crate) fn deserialize_host<'de, D>(deserializer: D) -> Result<url::Host<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let string: String = serde::Deserialize::deserialize(deserializer)?;
    url::Host::parse(&string).map_err(serde::de::Error::custom)
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse config: {0}")]
    Parse(#[from] toml::de::Error),
}
