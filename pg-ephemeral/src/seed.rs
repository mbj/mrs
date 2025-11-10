#[derive(Clone, Debug, Hash, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(try_from = "String")]
pub struct SeedName(String);

impl SeedName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for SeedName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Seed name cannot be empty")]
pub struct SeedNameError;

#[derive(Debug, PartialEq, Eq, thiserror::Error)]
#[error("Duplicate seed name: {0}")]
pub struct DuplicateSeedName(pub SeedName);

impl std::str::FromStr for SeedName {
    type Err = SeedNameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty() {
            Err(SeedNameError)
        } else {
            Ok(Self(value.to_string()))
        }
    }
}

impl TryFrom<String> for SeedName {
    type Error = SeedNameError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(SeedNameError)
        } else {
            Ok(Self(value))
        }
    }
}

impl TryFrom<&str> for SeedName {
    type Error = SeedNameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value.parse()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Seed {
    ApplyPendingMigrations,
    ApplyPendingMigrationsNoSchemaDump,
    SqlFile(std::path::PathBuf),
    SqlFileGitRevision {
        git_revision: &'static str,
        path: std::path::PathBuf,
    },
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_seed_name_rejects_empty_string() {
        assert_eq!("".parse::<SeedName>(), Err(SeedNameError));
        assert_eq!(SeedName::try_from(""), Err(SeedNameError));
        assert_eq!(SeedName::try_from(String::new()), Err(SeedNameError));
    }

    #[test]
    fn test_seed_name_accepts_non_empty_string() {
        assert_eq!(
            "valid-name".parse::<SeedName>(),
            Ok(SeedName("valid-name".to_string()))
        );
        assert_eq!(
            SeedName::try_from("valid-name"),
            Ok(SeedName("valid-name".to_string()))
        );
        assert_eq!(
            SeedName::try_from("valid-name".to_string()),
            Ok(SeedName("valid-name".to_string()))
        );
    }

    #[test]
    fn test_seed_name_display() {
        let name: SeedName = "test-seed".parse().unwrap();
        assert_eq!(name.to_string(), "test-seed");
        assert_eq!(name.as_str(), "test-seed");
    }
}
