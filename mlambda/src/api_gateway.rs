/// API gateway v2 types
///
/// Some of these types are still just regular primitives
/// but should be wrapped in newtypes over time.
use serde::ser::SerializeMap;

/// API GW compatible header map
///
/// HTTP headers can encode values that are abitrary bytes.
/// But as API GW communicates over JSON and JSON cannot communicate
/// abitrary bytes in strings.
#[derive(Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HeaderMap(std::collections::BTreeMap<HeaderName, HeaderValue>);

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize)]
pub struct HeaderName(String);
#[derive(Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct HeaderValue(String);
#[derive(Debug, PartialEq, serde::Deserialize)]
pub struct RequestId(String);

type PathParameters = std::collections::BTreeMap<String, String>;
type QueryStringParameters = std::collections::BTreeMap<String, String>;
type StageVariables = std::collections::BTreeMap<String, String>;

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ProxyRequest {
    body: Option<String>,
    #[serde(default)]
    cookies: Vec<String>,
    headers: HeaderMap,
    is_base64_encoded: bool,
    #[serde(default)]
    path_parameters: PathParameters,
    #[serde(default)]
    query_string_parameters: QueryStringParameters,
    raw_path: String,
    raw_query_string: String,
    request_context: RequestContext,
    route_key: String,
    #[serde(default)]
    stage_variables: StageVariables,
    version: String,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RequestContext {
    account_id: String,
    api_id: String,
    authentication: Option<serde_json::Value>,
    authorizer: Option<serde_json::Value>,
    domain_name: String,
    domain_prefix: String,
    http: RequestContextHttp,
    request_id: RequestId,
    route_key: String,
    stage: String,
    time: String,
    time_epoch: u64,
}

#[derive(Debug, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RequestContextHttp {
    method: String,
    path: String,
    protocol: String,
    source_ip: String,
    user_agent: String,
}

#[derive(Debug, PartialEq)]
pub struct ProxyResponse<T> {
    body: T,
    headers: HeaderMap,
    is_base64_encoded: bool,
    status_code: reqwest::StatusCode,
}

impl<T: serde::Serialize> ProxyResponse<T> {
    pub fn from_json_body(status_code: http::StatusCode, body: T) -> ProxyResponse<T> {
        ProxyResponse {
            body,
            headers: HeaderMap(std::collections::BTreeMap::from([(
                HeaderName("Content-Type".to_string()),
                HeaderValue("application/json".to_string()),
            )])),
            status_code,
            is_base64_encoded: false,
        }
    }
}

impl<T: serde::Serialize> serde::Serialize for ProxyResponse<T> {
    /// Serialize api gateway proxy response
    ///
    /// ```
    /// use mlambda::api_gateway::*;
    /// let response = ProxyResponse::from_json_body(reqwest::StatusCode::OK, serde_json::Value::Null);
    ///
    /// assert_eq!(
    ///   serde_json::json!(
    ///     {
    ///       "body": "null",
    ///       "headers": {
    ///         "Content-Type": "application/json"
    ///       },
    ///       "isBase64Encoded": false,
    ///       "statusCode": 200
    ///     }
    ///   ),
    ///   serde_json::to_value(response).unwrap()
    /// )
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut object = serializer.serialize_map(Some(4))?;
        object.serialize_entry("body", &serde_json::to_string(&self.body).unwrap())?;
        object.serialize_entry("headers", &self.headers)?;
        object.serialize_entry("isBase64Encoded", &self.is_base64_encoded)?;
        object.serialize_entry("statusCode", &self.status_code.as_u16())?;
        object.end()
    }
}
