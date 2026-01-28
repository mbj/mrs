use std::collections::BTreeMap;

use crate::{OutputName, SinkName, UserName};

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Source {
    pub sinks: BTreeMap<SinkName, Sink>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sink {
    #[serde(deserialize_with = "crate::config::deserialize_host")]
    pub host: url::Host<String>,
    pub user: UserName,
    /// Mapping from sink output name to positioning config.
    /// Resolution and port are discovered from sink at runtime.
    #[serde(default)]
    pub outputs: BTreeMap<OutputName, Output>,
}

/// Positioning for a virtual output on the source.
/// The key in the parent map is the sink's output name (e.g., DP-1).
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    pub position: Position,
    /// Source output to position relative to (e.g., eDP-1).
    pub relative_to: OutputName,
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Position {
    Left,
    Right,
    Above,
    Below,
}

impl Source {
    #[must_use]
    pub fn default_path() -> Option<std::path::PathBuf> {
        crate::config::config_dir().map(|dir| dir.join("source.toml"))
    }

    pub fn load_default() -> Result<Self, crate::Error> {
        let path = Self::default_path().ok_or(crate::Error::ConfigDirNotFound)?;
        Ok(Self::load(&path)?)
    }

    pub fn load(path: &std::path::Path) -> Result<Self, crate::ConfigError> {
        let content = std::fs::read_to_string(path)?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    fn parse(input: &str) -> Result<super::Source, toml::de::Error> {
        toml::from_str(input)
    }

    #[test]
    fn test_valid_domain_host() {
        let config = parse(indoc! {r#"
            [sinks.workstation]
            host = "workstation.local"
            user = "alice"
        "#})
        .unwrap();

        assert_eq!(config.sinks.len(), 1);
        let key: crate::SinkName = "workstation".parse().unwrap();
        let sink = &config.sinks[&key];
        assert_eq!(sink.host.to_string(), "workstation.local");
        assert_eq!(sink.user.as_str(), "alice");
    }

    #[test]
    fn test_valid_ipv4_host() {
        let config = parse(indoc! {r#"
            [sinks.workstation]
            host = "192.168.1.50"
            user = "alice"
        "#})
        .unwrap();

        let key: crate::SinkName = "workstation".parse().unwrap();
        let sink = &config.sinks[&key];
        assert_eq!(sink.host.to_string(), "192.168.1.50");
    }

    #[test]
    fn test_valid_ipv6_host() {
        let config = parse(indoc! {r#"
            [sinks.workstation]
            host = "[::1]"
            user = "alice"
        "#})
        .unwrap();

        let key: crate::SinkName = "workstation".parse().unwrap();
        let sink = &config.sinks[&key];
        assert_eq!(sink.host.to_string(), "[::1]");
    }

    #[test]
    fn test_multiple_sinks() {
        let config = parse(indoc! {r#"
            [sinks.left]
            host = "192.168.1.50"
            user = "alice"

            [sinks.right]
            host = "192.168.1.51"
            user = "bob"
        "#})
        .unwrap();

        assert_eq!(config.sinks.len(), 2);
        assert!(config.sinks.contains_key(&"left".parse().unwrap()));
        assert!(config.sinks.contains_key(&"right".parse().unwrap()));
    }

    #[test]
    fn test_empty_sinks() {
        let config = parse("[sinks]\n").unwrap();
        assert!(config.sinks.is_empty());
    }

    #[test]
    fn test_unknown_field_config() {
        let error = parse(indoc! {r#"
            unknown = true
            [sinks]
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("unknown"), "{error}");
    }

    #[test]
    fn test_unknown_field_sink() {
        let error = parse(indoc! {r#"
            [sinks.workstation]
            host = "192.168.1.50"
            user = "alice"
            unknown = true
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("unknown"), "{error}");
    }

    #[test]
    fn test_empty_user_name() {
        let error = parse(indoc! {r#"
            [sinks.workstation]
            host = "192.168.1.50"
            user = ""
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("empty"), "{error}");
    }

    #[test]
    fn test_user_name_too_long() {
        let long_user = "a".repeat(33);
        let input = format!(
            "[sinks.workstation]\nhost = \"192.168.1.50\"\nuser = \"{long_user}\"\n"
        );

        let error = parse(&input).unwrap_err();
        assert!(error.to_string().contains("32"), "{error}");
    }

    #[test]
    fn test_missing_required_field() {
        let error = parse(indoc! {r#"
            [sinks.workstation]
            host = "192.168.1.50"
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("user"), "{error}");
    }

    #[test]
    fn test_sink_with_outputs() {
        let config = parse(indoc! {r#"
            [sinks.workstation]
            host = "192.168.1.50"
            user = "alice"

            [sinks.workstation.outputs.DP-1]
            position = "right"
            relative_to = "eDP-1"

            [sinks.workstation.outputs.DP-2]
            position = "below"
            relative_to = "DP-1"
        "#})
        .unwrap();

        let key: crate::SinkName = "workstation".parse().unwrap();
        let sink = &config.sinks[&key];
        assert_eq!(sink.outputs.len(), 2);

        let output1 = &sink.outputs[&"DP-1".parse().unwrap()];
        assert!(matches!(output1.position, super::Position::Right));
        assert_eq!(output1.relative_to.to_string(), "eDP-1");

        let output2 = &sink.outputs[&"DP-2".parse().unwrap()];
        assert!(matches!(output2.position, super::Position::Below));
        assert_eq!(output2.relative_to.to_string(), "DP-1");
    }

    #[test]
    fn test_output_invalid_position() {
        let error = parse(indoc! {r#"
            [sinks.workstation]
            host = "192.168.1.50"
            user = "alice"

            [sinks.workstation.outputs.DP-1]
            position = "diagonal"
            relative_to = "eDP-1"
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("position"), "{error}");
    }

}
