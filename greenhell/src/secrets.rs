#[derive(Clone, Debug, Eq, PartialEq, strum::Display, strum::EnumIter, strum::EnumString)]
pub enum Secret {
    GitHubApp,
}

impl stack_deploy::secrets::SecretType for Secret {
    fn to_arn_output_key(&self) -> stack_deploy::types::OutputKey {
        match self {
            Self::GitHubApp => "GitHubAppSecretArn".into(),
        }
    }

    fn to_env_variable_name(&self) -> &str {
        match self {
            Self::GitHubApp => "GITHUB_APP_SECRET_ARN",
        }
    }

    fn validate(&self, input: &str) -> Result<(), String> {
        match self {
            Self::GitHubApp => {
                serde_json::from_str::<GitHubApp>(input).map_err(|error| error.to_string())?;
                Ok(())
            }
        }
    }
}

use std::num::NonZeroU64;

#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct GitHubApp {
    pub app_id: NonZeroU64,
    pub private_key: PrivateKey,
    pub webhook_secret: WebhookSecret,
}

#[derive(Debug, Eq, PartialEq)]
pub struct PrivateKey(String);

impl PrivateKey {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for PrivateKey {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        rcgen::KeyPair::from_pem(&value).map_err(|error| {
            serde::de::Error::custom(format!("invalid private key PEM: {error}"))
        })?;
        Ok(Self(value))
    }
}

impl serde::Serialize for PrivateKey {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct WebhookSecret(String);

impl WebhookSecret {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl<'de> serde::Deserialize<'de> for WebhookSecret {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value.is_empty() {
            return Err(serde::de::Error::custom("webhook_secret must be non-empty"));
        }
        Ok(Self(value))
    }
}

impl serde::Serialize for WebhookSecret {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
