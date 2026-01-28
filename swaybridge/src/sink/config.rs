use std::collections::BTreeMap;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sink {
    /// Sway output names offered for streaming.
    pub output: BTreeMap<crate::OutputName, Output>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Output {
    /// UDP port for SRT listener.
    pub port: std::num::NonZeroU16,
}

impl Sink {
    #[must_use]
    pub fn default_path() -> Option<std::path::PathBuf> {
        crate::config::config_dir().map(|dir| dir.join("sink.toml"))
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

    fn parse(input: &str) -> Result<super::Sink, toml::de::Error> {
        toml::from_str(input)
    }

    #[test]
    fn test_single_output() {
        let config = parse(indoc! {r#"
            [output.HDMI-A-1]
            port = 9000
        "#})
        .unwrap();

        assert_eq!(config.output.len(), 1);

        let key: crate::OutputName = "HDMI-A-1".parse().unwrap();
        assert_eq!(config.output[&key].port.get(), 9000);
    }

    #[test]
    fn test_multiple_output() {
        let config = parse(indoc! {r#"
            [output.HDMI-A-1]
            port = 9000

            [output.DP-2]
            port = 9001
        "#})
        .unwrap();

        assert_eq!(config.output.len(), 2);

        let key: crate::OutputName = "HDMI-A-1".parse().unwrap();
        assert_eq!(config.output[&key].port.get(), 9000);

        let key: crate::OutputName = "DP-2".parse().unwrap();
        assert_eq!(config.output[&key].port.get(), 9001);
    }

    #[test]
    fn test_empty_output() {
        let config = parse("[output]\n").unwrap();
        assert!(config.output.is_empty());
    }

    #[test]
    fn test_missing_port() {
        let error = parse(indoc! {r#"
            [output.HDMI-A-1]
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("port"), "{error}");
    }

    #[test]
    fn test_zero_port() {
        let error = parse(indoc! {r#"
            [output.HDMI-A-1]
            port = 0
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("port"), "{error}");
    }

    #[test]
    fn test_unknown_output_field() {
        let error = parse(indoc! {r#"
            [output.HDMI-A-1]
            port = 9000
            unknown = true
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("unknown"), "{error}");
    }

    #[test]
    fn test_unknown_top_level_field() {
        let error = parse(indoc! {r#"
            [output]
            unknown = true
        "#})
        .unwrap_err();

        assert!(error.to_string().contains("unknown"), "{error}");
    }

    #[test]
    fn test_missing_output() {
        let error = parse("").unwrap_err();
        assert!(error.to_string().contains("output"), "{error}");
    }
}
