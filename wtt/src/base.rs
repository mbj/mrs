#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Base(String);

impl Base {
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for Base {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

impl AsRef<std::ffi::OsStr> for Base {
    fn as_ref(&self) -> &std::ffi::OsStr {
        self.0.as_ref()
    }
}

impl std::str::FromStr for Base {
    type Err = BaseError;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        if string.is_empty() {
            return Err(BaseError::Empty);
        }
        Ok(Self(string.to_string()))
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BaseError {
    #[error("Base reference cannot be empty")]
    Empty,
}
