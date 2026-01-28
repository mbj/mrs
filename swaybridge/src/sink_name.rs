const MAX_LENGTH: usize = 128;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct SinkName(String);

impl SinkName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SinkName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::str::FromStr for SinkName {
    type Err = SinkNameError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(SinkNameError::Empty);
        }
        if string.len() > MAX_LENGTH {
            return Err(SinkNameError::TooLong);
        }
        Ok(Self(string.to_string()))
    }
}

impl TryFrom<String> for SinkName {
    type Error = SinkNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum SinkNameError {
    #[error("Sink name cannot be empty")]
    Empty,
    #[error("Sink name cannot exceed {MAX_LENGTH} characters")]
    TooLong,
}
