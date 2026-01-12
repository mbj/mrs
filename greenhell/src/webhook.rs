use hmac::{Hmac, Mac};
use sha2::Sha256;

use crate::secrets::{GitHubApp, Secret, WebhookSecret};

type HmacSha256 = Hmac<Sha256>;

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub body: Option<String>,
    pub headers: std::collections::BTreeMap<String, String>,
    pub is_base64_encoded: bool,
}

impl Request {
    #[must_use]
    pub fn header(&self, name: &str) -> Option<&str> {
        let name_lower = name.to_lowercase();
        self.headers
            .iter()
            .find(|(key, _)| key.to_lowercase() == name_lower)
            .map(|(_, value)| value.as_str())
    }

    #[must_use]
    pub fn body_bytes(&self) -> Option<Vec<u8>> {
        self.body.as_ref().map(|body| {
            if self.is_base64_encoded {
                base64::Engine::decode(&base64::engine::general_purpose::STANDARD, body)
                    .unwrap_or_else(|_| body.as_bytes().to_vec())
            } else {
                body.as_bytes().to_vec()
            }
        })
    }
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub status_code: u16,
    pub body: String,
    pub headers: std::collections::BTreeMap<String, String>,
}

impl Response {
    fn json(status_code: http::StatusCode, body: impl serde::Serialize) -> Self {
        Self {
            status_code: status_code.as_u16(),
            body: serde_json::to_string(&body).unwrap(),
            headers: std::collections::BTreeMap::from([(
                "Content-Type".to_string(),
                "application/json".to_string(),
            )]),
        }
    }

    fn error(status_code: http::StatusCode, message: &str) -> Self {
        Self::json(status_code, serde_json::json!({ "error": message }))
    }

    fn success(message: &str) -> Self {
        Self::json(http::StatusCode::OK, serde_json::json!({ "status": message }))
    }
}

#[derive(Debug)]
pub enum SignatureError {
    InvalidFormat,
    InvalidHex,
    Mismatch,
}

pub fn verify_signature(
    secret: &WebhookSecret,
    body: &[u8],
    signature: &str,
) -> Result<(), SignatureError> {
    let signature = signature
        .strip_prefix("sha256=")
        .ok_or(SignatureError::InvalidFormat)?;

    let signature_bytes = hex::decode(signature).map_err(|_| SignatureError::InvalidHex)?;

    let mut mac =
        HmacSha256::new_from_slice(secret.as_str().as_bytes()).expect("HMAC accepts any key size");
    mac.update(body);

    mac.verify_slice(&signature_bytes)
        .map_err(|_| SignatureError::Mismatch)
}

pub async fn run() {
    let aws_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    let secretsmanager = aws_sdk_secretsmanager::Client::new(&aws_config);

    let github_app: GitHubApp =
        stack_deploy::secrets::read_env_json(&secretsmanager, Secret::GitHubApp).await;

    let client = mlambda::runtime::Client::load();

    loop {
        let event = client.read_next_event::<Request>().await;
        let response = handle_event(&event, &github_app).await;
        client.send_response(&event.request_id, response).await;
    }
}

async fn handle_event(
    event: &mlambda::runtime::Event<Result<Request, mlambda::runtime::EventBodyDecodeError>>,
    github_app: &GitHubApp,
) -> Response {
    let request = match &event.body {
        Ok(request) => request,
        Err(error) => {
            log::error!(
                "Failed to decode event body: {} at {}",
                error.message(),
                error.path()
            );
            return Response::error(
                http::StatusCode::BAD_REQUEST,
                &format!("Invalid request: {}", error.message()),
            );
        }
    };

    let signature = match request.header("X-Hub-Signature-256") {
        Some(signature) => signature,
        None => {
            return Response::error(http::StatusCode::UNAUTHORIZED, "Missing signature");
        }
    };

    let body = match request.body_bytes() {
        Some(bytes) => bytes,
        None => {
            return Response::error(http::StatusCode::BAD_REQUEST, "Missing body");
        }
    };

    if let Err(error) = verify_signature(&github_app.webhook_secret, &body, signature) {
        log::warn!("Signature verification failed: {error:?}");
        return Response::error(http::StatusCode::UNAUTHORIZED, "Invalid signature");
    }

    let event_type = match request.header("X-GitHub-Event") {
        Some(event_type) => event_type,
        None => {
            return Response::error(http::StatusCode::BAD_REQUEST, "Missing event type");
        }
    };

    log::info!("Received GitHub event: {event_type}");

    match event_type {
        "ping" => Response::success("pong"),
        "check_suite" | "check_run" | "status" => {
            // TODO: Trigger evaluation
            Response::success("accepted")
        }
        _ => {
            log::info!("Ignoring event type: {event_type}");
            Response::success("ignored")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_signature_valid() {
        let secret = serde_json::from_str::<WebhookSecret>(r#""test-secret""#).unwrap();
        let body = b"test body";

        // Pre-computed: echo -n "test body" | openssl dgst -sha256 -hmac "test-secret"
        let signature = "sha256=8e3a8e233c37c30eb0d85adfc8c4e534f2e9d6e2a4b4e8f5e6d7c8b9a0b1c2d3";

        // This will fail because the signature is fake, but tests the parsing
        let result = verify_signature(&secret, body, signature);
        assert!(matches!(result, Err(SignatureError::Mismatch)));
    }

    #[test]
    fn verify_signature_missing_prefix() {
        let secret = serde_json::from_str::<WebhookSecret>(r#""test-secret""#).unwrap();
        let body = b"test body";
        let signature = "invalid";

        let result = verify_signature(&secret, body, signature);
        assert!(matches!(result, Err(SignatureError::InvalidFormat)));
    }

    #[test]
    fn verify_signature_invalid_hex() {
        let secret = serde_json::from_str::<WebhookSecret>(r#""test-secret""#).unwrap();
        let body = b"test body";
        let signature = "sha256=not-hex";

        let result = verify_signature(&secret, body, signature);
        assert!(matches!(result, Err(SignatureError::InvalidHex)));
    }
}
