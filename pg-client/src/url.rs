use crate::{Config, Database, Endpoint, Host, Password, Port, SslMode, SslRootCert, User};
use fluent_uri::pct_enc::EStr;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] ::fluent_uri::ParseError),
    #[error("Invalid URL scheme: expected 'postgres' or 'postgresql', got '{0}'")]
    InvalidScheme(String),
    #[error("Invalid URL fragment: '{0}'")]
    InvalidFragment(String),
    #[error("Missing host in URL")]
    MissingHost,
    #[error("Missing required parameter '{0}' in URL")]
    MissingParameter(&'static str),
    #[error("Parameter '{0}' specified in both URL and query string")]
    ConflictingParameter(&'static str),
    #[error("Unknown query parameter: '{0}'")]
    InvalidQueryParameter(String),
    #[error("Invalid query parameter encoding: {0}")]
    InvalidQueryParameterEncoding(std::str::Utf8Error),
    #[error("{0}")]
    Field(#[from] FieldError),
    #[error("Unsupported parameter for socket path connection: '{0}'")]
    UnsupportedSocketPathParameter(&'static str),
    #[error("Invalid port: {0}")]
    InvalidPort(#[from] std::num::ParseIntError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldSource {
    Authority,
    Path,
    QueryParam,
}

impl fmt::Display for FieldSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldSource::Authority => f.write_str("authority"),
            FieldSource::Path => f.write_str("path"),
            FieldSource::QueryParam => f.write_str("query"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Field {
    User,
    Password,
    Database,
    Host,
    HostAddr,
    SslMode,
    SslRootCert,
    ApplicationName,
    ChannelBinding,
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Field::User => f.write_str("user"),
            Field::Password => f.write_str("password"),
            Field::Database => f.write_str("dbname"),
            Field::Host => f.write_str("host"),
            Field::HostAddr => f.write_str("hostaddr"),
            Field::SslMode => f.write_str("sslmode"),
            Field::SslRootCert => f.write_str("sslrootcert"),
            Field::ApplicationName => f.write_str("application_name"),
            Field::ChannelBinding => f.write_str("channel_binding"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldErrorCause {
    InvalidUtf8(std::str::Utf8Error),
    InvalidIdentifier(crate::identifier::ParseError),
    InvalidValue(String),
}

impl fmt::Display for FieldErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldErrorCause::InvalidUtf8(error) => {
                write!(f, "invalid utf-8 encoding: {error}")
            }
            FieldErrorCause::InvalidIdentifier(error) => {
                write!(f, "invalid value: {error}")
            }
            FieldErrorCause::InvalidValue(error) if error.is_empty() => {
                f.write_str("invalid value")
            }
            FieldErrorCause::InvalidValue(error) => write!(f, "invalid value: {error}"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[error("Invalid {field} in {origin}: {cause}")]
pub struct FieldError {
    pub origin: FieldSource,
    pub field: Field,
    pub cause: FieldErrorCause,
}

/// Parse a PostgreSQL connection URL into a Config.
///
/// Supports both `postgres://` and `postgresql://` schemes.
///
/// When the URL does not specify `sslmode`, it defaults to `verify-full`
/// to ensure secure connections by default.
///
/// # URL Format
///
/// Network connections:
/// ```text
/// postgres://username:password@host:port/database?params
/// ```
///
/// Socket connections (via query params when host starts with `/` or `@`):
/// ```text
/// postgres://?host=/path/to/socket&user=username&dbname=database
/// ```
///
/// Cloud SQL connections (user/password/database in URL, socket path in query):
/// ```text
/// postgres://username:password@/database?host=/cloudsql/project:region:instance
/// ```
///
/// # Query Parameters
///
/// - `sslmode`: SSL mode (allow, disable, prefer, require, verify-ca, verify-full)
/// - `sslrootcert`: Path to SSL root certificate or "system"
/// - `application_name`: Application name
/// - `hostaddr`: IP address for the host
/// - `channel_binding`: Channel binding (disable, prefer, require)
/// - `host`: Socket path (when URL has no host component)
/// - `user`: User (when URL has no username component)
/// - `dbname`: Database name (when URL has no path component)
/// - `password`: Password (when URL has no password component)
///
/// # Errors
///
/// Returns an error if the same parameter is specified both in the URL
/// components and as a query parameter (e.g., password in both places).
///
/// # Example
///
/// ```
/// use pg_client::{Config, SslMode};
///
/// let config = pg_client::url::parse(
///     "postgres://user@localhost:5432/mydb",
/// ).unwrap();
///
/// assert_eq!(config.user.as_str(), "user");
/// assert_eq!(config.database.as_str(), "mydb");
/// assert_eq!(config.ssl_mode, SslMode::VerifyFull);
/// ```
pub fn parse(url: &str) -> Result<Config, ParseError> {
    let uri = ::fluent_uri::Uri::parse(url)?;

    // Validate scheme
    let scheme = uri.scheme().as_str();
    if scheme != "postgres" && scheme != "postgresql" {
        return Err(ParseError::InvalidScheme(scheme.to_string()));
    }

    if let Some(fragment) = uri.fragment() {
        return Err(ParseError::InvalidFragment(fragment.as_str().to_string()));
    }

    // Parse query string into decoded key-value map
    let query_map = parse_query(uri.query())?;
    let mut query_params = QueryParams::new(&query_map);

    // Extract userinfo (user and password from URL authority)
    let authority = uri.authority();
    let (url_user, url_password) = extract_userinfo(authority.as_ref())?;

    // Extract database from URL path
    let url_database = decode_path_database(uri.path())?;

    // Resolve endpoint from authority host or query host parameter
    let query_host = query_params.take("host");

    let endpoint = match authority.as_ref() {
        Some(authority) if !authority.host().is_empty() => {
            if query_host.is_some() {
                return Err(ParseError::ConflictingParameter("host"));
            }

            let host = match authority.host_parsed() {
                fluent_uri::component::Host::RegName(name) => {
                    let decoded = decode_to_string(name).map_err(|error| FieldError {
                        origin: FieldSource::Authority,
                        field: Field::Host,
                        cause: FieldErrorCause::InvalidUtf8(error),
                    })?;
                    decoded.parse::<Host>().map_err(|error: &str| FieldError {
                        origin: FieldSource::Authority,
                        field: Field::Host,
                        cause: FieldErrorCause::InvalidValue(error.to_string()),
                    })?
                }
                fluent_uri::component::Host::Ipv4(addr) => Host::IpAddr(addr.into()),
                fluent_uri::component::Host::Ipv6(addr) => Host::IpAddr(addr.into()),
                _ => {
                    let host = authority.host();
                    let message = if host.starts_with("[v") || host.starts_with("[V") {
                        "unsupported host type: ipvfuture"
                    } else {
                        "unsupported host type"
                    };
                    return Err(FieldError {
                        origin: FieldSource::Authority,
                        field: Field::Host,
                        cause: FieldErrorCause::InvalidValue(message.to_string()),
                    }
                    .into());
                }
            };

            let host_addr = match query_params.take("hostaddr") {
                Some(addr_str) => Some(addr_str.parse().map_err(|error: &str| FieldError {
                    origin: FieldSource::QueryParam,
                    field: Field::HostAddr,
                    cause: FieldErrorCause::InvalidValue(error.to_string()),
                })?),
                None => None,
            };

            let channel_binding = match query_params.take("channel_binding") {
                Some(binding_str) => Some(binding_str.parse().map_err(|_| FieldError {
                    origin: FieldSource::QueryParam,
                    field: Field::ChannelBinding,
                    cause: FieldErrorCause::InvalidValue(binding_str.to_string()),
                })?),
                None => None,
            };

            let port = authority.port_to_u16()?.map(Port::new);

            Endpoint::Network {
                host,
                channel_binding,
                host_addr,
                port,
            }
        }
        _ => {
            let host = query_host.ok_or(ParseError::MissingHost)?;

            if !host.starts_with('/') && !host.starts_with('@') {
                return Err(FieldError {
                    origin: FieldSource::QueryParam,
                    field: Field::Host,
                    cause: FieldErrorCause::InvalidValue(
                        "query host must be a socket path (start with / or @)".to_string(),
                    ),
                }
                .into());
            }

            for name in ["channel_binding", "hostaddr"] {
                if query_params.take(name).is_some() {
                    return Err(ParseError::UnsupportedSocketPathParameter(name));
                }
            }

            Endpoint::SocketPath(host.into())
        }
    };

    let user_value = access_field(
        "user",
        url_user.map(|value| FieldValue::new(FieldSource::Authority, value)),
        &mut query_params,
    )?
    .ok_or(ParseError::MissingParameter("user"))?;
    if user_value.value.is_empty() {
        return Err(ParseError::MissingParameter("user"));
    }
    let user: User = user_value.value.parse().map_err(|error| FieldError {
        origin: user_value.origin,
        field: Field::User,
        cause: FieldErrorCause::InvalidIdentifier(error),
    })?;

    let password: Option<Password> = match access_field(
        "password",
        url_password.map(|value| FieldValue::new(FieldSource::Authority, value)),
        &mut query_params,
    )? {
        Some(value) => Some(value.value.parse().map_err(|error: String| FieldError {
            origin: value.origin,
            field: Field::Password,
            cause: FieldErrorCause::InvalidValue(error.to_string()),
        })?),
        None => None,
    };

    let database_value = access_field(
        "dbname",
        url_database.map(|value| FieldValue::new(FieldSource::Path, value)),
        &mut query_params,
    )?
    .ok_or(ParseError::MissingParameter("dbname"))?;
    let database: Database = database_value.value.parse().map_err(|error| FieldError {
        origin: database_value.origin,
        field: Field::Database,
        cause: FieldErrorCause::InvalidIdentifier(error),
    })?;

    // Parse sslmode, defaulting to verify-full for secure connections
    let ssl_mode = match query_params.take("sslmode") {
        Some(mode_str) => mode_str.parse().map_err(|_| FieldError {
            origin: FieldSource::QueryParam,
            field: Field::SslMode,
            cause: FieldErrorCause::InvalidValue(mode_str.to_string()),
        })?,
        None => SslMode::VerifyFull,
    };

    // Parse sslrootcert
    let ssl_root_cert = query_params.take("sslrootcert").map(|cert_str| {
        if cert_str == "system" {
            SslRootCert::System
        } else {
            SslRootCert::File(cert_str.to_string().into())
        }
    });

    // Parse application_name
    let application_name = match query_params.take("application_name") {
        Some(name_str) => Some(name_str.parse().map_err(|error: String| FieldError {
            origin: FieldSource::QueryParam,
            field: Field::ApplicationName,
            cause: FieldErrorCause::InvalidValue(error),
        })?),
        None => None,
    };

    if let Some(unknown) = query_params.unknown_param() {
        return Err(ParseError::InvalidQueryParameter(unknown.to_string()));
    }

    Ok(Config {
        application_name,
        database,
        endpoint,
        password,
        ssl_mode,
        ssl_root_cert,
        user,
    })
}

fn extract_userinfo(
    authority: Option<&fluent_uri::component::Authority<'_>>,
) -> Result<(Option<String>, Option<String>), ParseError> {
    let userinfo = match authority.and_then(|authority| authority.userinfo()) {
        Some(info) => info,
        None => return Ok((None, None)),
    };

    match userinfo.split_once(':') {
        Some((user_enc, pass_enc)) => {
            let user = decode_to_string(user_enc).map_err(|error| FieldError {
                origin: FieldSource::Authority,
                field: Field::User,
                cause: FieldErrorCause::InvalidUtf8(error),
            })?;
            let password = decode_to_string(pass_enc).map_err(|error| FieldError {
                origin: FieldSource::Authority,
                field: Field::Password,
                cause: FieldErrorCause::InvalidUtf8(error),
            })?;
            let user = non_empty(user);
            let password = non_empty(password);
            Ok((user, password))
        }
        None => {
            let user = decode_to_string(userinfo).map_err(|error| FieldError {
                origin: FieldSource::Authority,
                field: Field::User,
                cause: FieldErrorCause::InvalidUtf8(error),
            })?;
            Ok((non_empty(user), None))
        }
    }
}

fn decode_to_string<E: fluent_uri::pct_enc::Encoder>(
    estr: &EStr<E>,
) -> Result<String, std::str::Utf8Error> {
    let bytes = estr.decode().to_bytes();
    String::from_utf8(bytes.into_owned()).map_err(|error| error.utf8_error())
}

fn non_empty(value: String) -> Option<String> {
    if value.is_empty() { None } else { Some(value) }
}

fn decode_path_database(
    path: &EStr<fluent_uri::pct_enc::encoder::Path>,
) -> Result<Option<String>, ParseError> {
    let decoded = decode_to_string(path).map_err(|error| FieldError {
        origin: FieldSource::Path,
        field: Field::Database,
        cause: FieldErrorCause::InvalidUtf8(error),
    })?;

    let stripped = decoded.strip_prefix('/').unwrap_or(&decoded);

    Ok(non_empty(stripped.to_string()))
}

fn parse_query(
    query: Option<&EStr<fluent_uri::pct_enc::encoder::Query>>,
) -> Result<BTreeMap<String, String>, ParseError> {
    let query = match query {
        Some(query) => query,
        None => return Ok(BTreeMap::new()),
    };

    query
        .split('&')
        .map(|pair| {
            let (key, value) = pair.split_once('=').unwrap_or((pair, EStr::EMPTY));
            let key = decode_to_string(key).map_err(ParseError::InvalidQueryParameterEncoding)?;
            let field = query_field(&key);
            let value = decode_to_string(value).map_err(|error| match field {
                Some(field) => FieldError {
                    origin: FieldSource::QueryParam,
                    field,
                    cause: FieldErrorCause::InvalidUtf8(error),
                }
                .into(),
                None => ParseError::InvalidQueryParameterEncoding(error),
            })?;
            Ok((key, value))
        })
        .collect()
}

fn access_field(
    name: &'static str,
    url_value: Option<FieldValue>,
    query_params: &mut QueryParams<'_>,
) -> Result<Option<FieldValue>, ParseError> {
    let query_value = query_params
        .take(name)
        .map(|value| FieldValue::new(FieldSource::QueryParam, value.to_string()));
    match (url_value, query_value) {
        (Some(_), Some(_)) => Err(ParseError::ConflictingParameter(name)),
        (Some(value), None) => Ok(Some(value)),
        (None, Some(value)) => Ok(Some(value)),
        (None, None) => Ok(None),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FieldValue {
    origin: FieldSource,
    value: String,
}

impl FieldValue {
    fn new(origin: FieldSource, value: String) -> Self {
        Self { origin, value }
    }
}

fn query_field(name: &str) -> Option<Field> {
    match name {
        "user" => Some(Field::User),
        "password" => Some(Field::Password),
        "dbname" => Some(Field::Database),
        "host" => Some(Field::Host),
        "hostaddr" => Some(Field::HostAddr),
        "sslmode" => Some(Field::SslMode),
        "sslrootcert" => Some(Field::SslRootCert),
        "application_name" => Some(Field::ApplicationName),
        "channel_binding" => Some(Field::ChannelBinding),
        _ => None,
    }
}

struct QueryParams<'a> {
    params: &'a BTreeMap<String, String>,
    remaining: BTreeSet<&'a str>,
}

impl<'a> QueryParams<'a> {
    fn new(params: &'a BTreeMap<String, String>) -> Self {
        let remaining = params.keys().map(|key| key.as_str()).collect();
        Self { params, remaining }
    }

    fn take(&mut self, name: &str) -> Option<&'a str> {
        let value = self.params.get(name).map(|value| value.as_str());
        if value.is_some() {
            self.remaining.remove(name);
        }
        value
    }

    fn unknown_param(&self) -> Option<&&'a str> {
        self.remaining.iter().next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ChannelBinding;
    use crate::SslMode;

    fn network(host: &str, port: Option<u16>, host_addr: Option<&str>) -> Endpoint {
        Endpoint::Network {
            host: host.parse().unwrap(),
            channel_binding: None,
            port: port.map(Port::new),
            host_addr: host_addr.map(|address| address.parse().unwrap()),
        }
    }

    fn success(
        user: &str,
        password: Option<&str>,
        database: &str,
        endpoint: Endpoint,
        ssl_mode: SslMode,
        ssl_root_cert: Option<SslRootCert>,
        application_name: Option<&str>,
    ) -> Config {
        Config {
            user: user.parse().unwrap(),
            password: password.map(|value| value.parse().unwrap()),
            database: database.parse().unwrap(),
            endpoint,
            ssl_mode,
            ssl_root_cert,
            application_name: application_name.map(|value| value.parse().unwrap()),
        }
    }

    fn field_error(origin: FieldSource, field: Field, cause: FieldErrorCause) -> ParseError {
        ParseError::Field(FieldError {
            origin,
            field,
            cause,
        })
    }

    #[test]
    fn test_parse() {
        type Expected = Result<Config, ParseError>;

        let cases: Vec<(&str, &str, Expected)> = vec![
            // Success cases
            (
                "basic_network",
                "postgres://user@localhost:5432/mydb",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", Some(5432), None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "with_password",
                "postgres://user:secret@localhost/mydb",
                Ok(success(
                    "user",
                    Some("secret"),
                    "mydb",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "percent_encoded_password",
                "postgres://user:p%40ss%2Fword@localhost/mydb",
                Ok(success(
                    "user",
                    Some("p@ss/word"),
                    "mydb",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "with_sslmode_disable",
                "postgres://user@localhost/mydb?sslmode=disable",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", None, None),
                    SslMode::Disable,
                    None,
                    None,
                )),
            ),
            (
                "with_sslmode_require",
                "postgres://user@localhost/mydb?sslmode=require",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", None, None),
                    SslMode::Require,
                    None,
                    None,
                )),
            ),
            (
                "with_channel_binding",
                "postgres://user@localhost/mydb?channel_binding=require",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    Endpoint::Network {
                        host: "localhost".parse().unwrap(),
                        channel_binding: Some(ChannelBinding::Require),
                        port: None,
                        host_addr: None,
                    },
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "with_application_name",
                "postgres://user@localhost/mydb?application_name=myapp",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    None,
                    Some("myapp"),
                )),
            ),
            (
                "with_hostaddr",
                "postgres://user@example.com/mydb?hostaddr=192.168.1.1",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("example.com", None, Some("192.168.1.1")),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "with_sslrootcert_file",
                "postgres://user@localhost/mydb?sslrootcert=/path/to/cert.pem",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    Some(SslRootCert::File("/path/to/cert.pem".into())),
                    None,
                )),
            ),
            (
                "with_sslrootcert_system",
                "postgres://user@localhost/mydb?sslrootcert=system",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    Some(SslRootCert::System),
                    None,
                )),
            ),
            (
                "socket_path",
                "postgres://?host=/var/run/postgresql&user=postgres&dbname=mydb",
                Ok(success(
                    "postgres",
                    None,
                    "mydb",
                    Endpoint::SocketPath("/var/run/postgresql".into()),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "socket_with_password",
                "postgres://?host=/socket&user=user&password=pass&dbname=mydb",
                Ok(success(
                    "user",
                    Some("pass"),
                    "mydb",
                    Endpoint::SocketPath("/socket".into()),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "abstract_socket",
                "postgres://?host=@abstract&user=postgres&dbname=mydb",
                Ok(success(
                    "postgres",
                    None,
                    "mydb",
                    Endpoint::SocketPath("@abstract".into()),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "postgresql_scheme",
                "postgresql://user@localhost/mydb",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "ipv6_host",
                "postgres://user@[::1]:5432/mydb",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("::1", Some(5432), None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "ipv4_host",
                "postgres://user@192.168.1.1:5432/mydb",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("192.168.1.1", Some(5432), None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "no_port",
                "postgres://user@localhost/mydb",
                Ok(success(
                    "user",
                    None,
                    "mydb",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            // Cloud SQL success cases
            (
                "cloud_sql_socket",
                "postgres://user:secret@/main?host=/cloudsql/project:region:instance",
                Ok(success(
                    "user",
                    Some("secret"),
                    "main",
                    Endpoint::SocketPath("/cloudsql/project:region:instance".into()),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "cloud_sql_socket_no_password",
                "postgres://user@/main?host=/cloudsql/project:region:instance",
                Ok(success(
                    "user",
                    None,
                    "main",
                    Endpoint::SocketPath("/cloudsql/project:region:instance".into()),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "cloud_sql_socket_sslmode_disable",
                "postgres://user:secret@/main?host=/cloudsql/project:region:instance&sslmode=disable",
                Ok(success(
                    "user",
                    Some("secret"),
                    "main",
                    Endpoint::SocketPath("/cloudsql/project:region:instance".into()),
                    SslMode::Disable,
                    None,
                    None,
                )),
            ),
            (
                "cloud_sql_socket_query_params",
                "postgres://?host=/cloudsql/project:region:instance&user=user&password=secret&dbname=main",
                Ok(success(
                    "user",
                    Some("secret"),
                    "main",
                    Endpoint::SocketPath("/cloudsql/project:region:instance".into()),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            // Error cases
            (
                "invalid_scheme",
                "mysql://user@localhost/mydb",
                Err(ParseError::InvalidScheme("mysql".to_string())),
            ),
            (
                "missing_username",
                "postgres://localhost/mydb",
                Err(ParseError::MissingParameter("user")),
            ),
            (
                "missing_database",
                "postgres://user@localhost",
                Err(ParseError::MissingParameter("dbname")),
            ),
            (
                "missing_host",
                "postgres://?user=user&dbname=mydb",
                Err(ParseError::MissingHost),
            ),
            (
                "conflicting_host",
                "postgres://user@localhost/mydb?host=/socket",
                Err(ParseError::ConflictingParameter("host")),
            ),
            (
                "conflicting_user",
                "postgres://user@localhost/mydb?user=other",
                Err(ParseError::ConflictingParameter("user")),
            ),
            (
                "conflicting_password",
                "postgres://user:secret@localhost/mydb?password=other",
                Err(ParseError::ConflictingParameter("password")),
            ),
            (
                "conflicting_dbname",
                "postgres://user@localhost/mydb?dbname=other",
                Err(ParseError::ConflictingParameter("dbname")),
            ),
            (
                "invalid_sslmode",
                "postgres://user@localhost/mydb?sslmode=invalid",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::SslMode,
                    FieldErrorCause::InvalidValue("invalid".to_string()),
                )),
            ),
            (
                "invalid_channel_binding",
                "postgres://user@localhost/mydb?channel_binding=invalid",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::ChannelBinding,
                    FieldErrorCause::InvalidValue("invalid".to_string()),
                )),
            ),
            (
                "invalid_hostaddr",
                "postgres://user@localhost/mydb?hostaddr=not-an-ip",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::HostAddr,
                    FieldErrorCause::InvalidValue("invalid IP address".to_string()),
                )),
            ),
            (
                "unsupported_ipvfuture_host",
                "postgres://user@[v1.fe80]/mydb",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("unsupported host type: ipvfuture".to_string()),
                )),
            ),
            (
                "unknown_parameter",
                "postgres://user@localhost/mydb?unknown_parameter=1",
                Err(ParseError::InvalidQueryParameter(
                    "unknown_parameter".to_string(),
                )),
            ),
            (
                "fragment",
                "postgres://user@localhost/mydb#section",
                Err(ParseError::InvalidFragment("section".to_string())),
            ),
            (
                "socket_missing_user",
                "postgres://?host=/socket&dbname=mydb",
                Err(ParseError::MissingParameter("user")),
            ),
            (
                "socket_missing_dbname",
                "postgres://?host=/socket&user=user",
                Err(ParseError::MissingParameter("dbname")),
            ),
            (
                "socket_with_channel_binding",
                "postgres://?host=/socket&user=user&dbname=mydb&channel_binding=require",
                Err(ParseError::UnsupportedSocketPathParameter(
                    "channel_binding",
                )),
            ),
            (
                "socket_with_hostaddr",
                "postgres://?host=/socket&user=user&dbname=mydb&hostaddr=127.0.0.1",
                Err(ParseError::UnsupportedSocketPathParameter("hostaddr")),
            ),
            // Cloud SQL error cases
            (
                "cloud_sql_conflicting_user",
                "postgres://user@/main?host=/cloudsql/project:region:instance&user=other",
                Err(ParseError::ConflictingParameter("user")),
            ),
            (
                "cloud_sql_conflicting_password",
                "postgres://user:secret@/main?host=/cloudsql/project:region:instance&password=other",
                Err(ParseError::ConflictingParameter("password")),
            ),
            (
                "cloud_sql_conflicting_dbname",
                "postgres://user@/main?host=/cloudsql/project:region:instance&dbname=other",
                Err(ParseError::ConflictingParameter("dbname")),
            ),
        ];

        for (name, url_str, expected) in cases {
            let actual = parse(url_str);

            assert_eq!(actual, expected, "{name}: {url_str}");

            if let Ok(config) = actual {
                let roundtrip_url = config.to_url_string();
                let roundtrip_config = parse(&roundtrip_url).unwrap_or_else(|error| {
                    panic!("{name}: roundtrip parse failed: {error}, url: {roundtrip_url}")
                });
                assert_eq!(roundtrip_config, config, "{name}: roundtrip");
            }
        }
    }
}
