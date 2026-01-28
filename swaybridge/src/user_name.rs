const MAX_LENGTH: usize = 32;

#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(try_from = "String")]
pub struct UserName(String);

impl UserName {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for UserName {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl std::str::FromStr for UserName {
    type Err = UserNameError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(UserNameError::Empty);
        }
        if string.len() > MAX_LENGTH {
            return Err(UserNameError::TooLong);
        }
        Ok(Self(string.to_string()))
    }
}

impl TryFrom<String> for UserName {
    type Error = UserNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserNameError {
    #[error("User name cannot be empty")]
    Empty,
    #[error("User name cannot exceed {MAX_LENGTH} characters")]
    TooLong,
}
