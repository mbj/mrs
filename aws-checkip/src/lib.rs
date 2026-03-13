use core::net::IpAddr;
use std::sync::LazyLock;

const URL: &str = "https://checkip.amazonaws.com";

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("decode error: {0}")]
    Decode(#[from] typed_reqwest::decoder::DecodeError),
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
}

pub static DECODER: LazyLock<typed_reqwest::decoder::Response<ipnet::IpNet>> =
    LazyLock::new(|| {
        (|| -> Result<_, typed_reqwest::decoder::ResponseBuilderError> {
            Ok(typed_reqwest::decoder::Response::build()
                .status_code(http::StatusCode::OK, |content_types| {
                    content_types.any(decode_body);
                    Ok(())
                })?
                .finish())
        })()
        .unwrap_or_else(|error| panic!("Failed to build decoder: {error}"))
    });

fn decode_body(body: &[u8]) -> Result<ipnet::IpNet, typed_reqwest::decoder::DecodeError> {
    let text = core::str::from_utf8(body).map_err(|error| typed_reqwest::decoder::DecodeError {
        reason: typed_reqwest::decoder::ErrorReason::BodyDecodeError,
        source: Some(Box::new(error)),
    })?;

    let ip: IpAddr = text
        .trim()
        .parse()
        .map_err(|error| typed_reqwest::decoder::DecodeError {
            reason: typed_reqwest::decoder::ErrorReason::BodyDecodeError,
            source: Some(Box::new(error)),
        })?;

    Ok(ipnet::IpNet::from(ip))
}

/// Fetch the public IP address as seen by AWS.
pub async fn read_net(client: &reqwest::Client) -> Result<ipnet::IpNet, Error> {
    let response = client.get(URL).send().await?;

    Ok(DECODER.decode(response).await?)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    fn text_response(status: http::StatusCode, body: &'static str) -> reqwest::Response {
        http::Response::builder()
            .status(status)
            .body(body)
            .unwrap()
            .into()
    }

    #[tokio::test]
    async fn decode_empty_body() {
        let response = text_response(http::StatusCode::OK, "");
        let result = DECODER.decode(response).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn decode_invalid_body() {
        let response = text_response(http::StatusCode::OK, "not an ip");
        let result = DECODER.decode(response).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn decode_ipv4() {
        let response = text_response(http::StatusCode::OK, "127.0.0.1\n");
        let result = DECODER.decode(response).await.unwrap();

        assert_eq!(result, "127.0.0.1/32".parse::<ipnet::IpNet>().unwrap());
    }

    #[tokio::test]
    async fn decode_ipv6() {
        let response = text_response(http::StatusCode::OK, "::1\n");
        let result = DECODER.decode(response).await.unwrap();

        assert_eq!(result, "::1/128".parse::<ipnet::IpNet>().unwrap());
    }

    #[tokio::test]
    async fn decode_unexpected_status() {
        let response = text_response(http::StatusCode::INTERNAL_SERVER_ERROR, "error");
        let result = DECODER.decode(response).await;

        assert_eq!(
            result.unwrap_err().reason,
            typed_reqwest::decoder::ErrorReason::UnexpectedStatusCode {
                status_code: http::StatusCode::INTERNAL_SERVER_ERROR,
            }
        );
    }
}
