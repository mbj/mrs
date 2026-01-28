const MAX_LENGTH: usize = 128;

/// Native Sway output name (e.g. "HDMI-A-1", "DP-2").
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct OutputName(String);

impl OutputName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for OutputName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::str::FromStr for OutputName {
    type Err = OutputNameError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(OutputNameError::Empty);
        }
        if string.len() > MAX_LENGTH {
            return Err(OutputNameError::TooLong);
        }
        Ok(Self(string.to_string()))
    }
}

impl TryFrom<String> for OutputName {
    type Error = OutputNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OutputNameError {
    #[error("Output name cannot be empty")]
    Empty,
    #[error("Output name cannot exceed {MAX_LENGTH} characters")]
    TooLong,
}
