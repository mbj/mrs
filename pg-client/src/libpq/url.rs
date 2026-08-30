use super::{Config, Endpoint, Session, SslMode};
use std::fmt;

mod parser;

pub(crate) fn encode_into(target: &mut String, value: &str) {
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~') {
            target.push(char::from(byte));
        } else {
            target.push_str(&format!("%{byte:02X}"));
        }
    }
}

pub(crate) fn append_query_pair(query: &mut String, key: &str, value: &str) {
    if !query.is_empty() {
        query.push('&');
    }
    encode_into(query, key);
    query.push('=');
    encode_into(query, value);
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid URL syntax at byte offset {offset}")]
    InvalidUrl { offset: usize },
    #[error("Invalid URL scheme: expected 'postgres' or 'postgresql', got '{0}'")]
    InvalidScheme(String),
    #[error("Missing host in URL")]
    MissingHost,
    #[error("Multiple hosts are not supported")]
    MultipleHosts,
    #[error("Missing required parameter '{0}' in URL")]
    MissingParameter(&'static str),
    #[error("Parameter '{0}' specified in both URL and query string")]
    ConflictingParameter(&'static str),
    #[error("Unknown query parameter: '{0}'")]
    InvalidQueryParameter(String),
    #[error("Invalid query parameter encoding: {0}")]
    InvalidQueryParameterEncoding(DecodeError),
    #[error("{0}")]
    Field(#[from] FieldError),
    #[error("Unsupported parameter for socket path connection: '{0}'")]
    UnsupportedSocketPathParameter(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DecodeError {
    #[error("invalid percent-encoded token")]
    InvalidPercentEncoding,
    #[error("forbidden value %00 in percent-encoded value")]
    Nul,
    #[error("unexpected spaces, use percent-encoded spaces (%20) instead")]
    UnexpectedSpaces,
    #[error("invalid utf-8 encoding: {0}")]
    InvalidUtf8(std::str::Utf8Error),
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
    Port,
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
            Field::Port => f.write_str("port"),
            Field::SslMode => f.write_str("sslmode"),
            Field::SslRootCert => f.write_str("sslrootcert"),
            Field::ApplicationName => f.write_str("application_name"),
            Field::ChannelBinding => f.write_str("channel_binding"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldErrorCause {
    InvalidEncoding(DecodeError),
    InvalidIdentifier(crate::name::ParseError),
    InvalidValue(String),
}

impl fmt::Display for FieldErrorCause {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldErrorCause::InvalidEncoding(error) => write!(f, "{error}"),
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
/// use pg_client::libpq::SslMode;
///
/// let config = pg_client::libpq::url::parse(
///     "postgres://user@localhost:5432/mydb",
/// ).unwrap();
///
/// assert_eq!(config.session.user.as_str(), "user");
/// assert_eq!(config.session.database.as_str(), "mydb");
/// assert_eq!(config.ssl_mode, SslMode::VerifyFull);
/// ```
pub fn parse(url: &str) -> Result<Config, ParseError> {
    let parser::Url {
        authority,
        database,
        query,
    } = parser::parse(url)?;

    let host = exclusive("host", authority.host, query.host)?.ok_or(ParseError::MissingHost)?;
    let port = exclusive("port", authority.port, query.port)?;

    let endpoint = match host {
        parser::Target::Network(host) => Endpoint::Network {
            host,
            channel_binding: query.channel_binding.flatten(),
            host_addr: query.host_addr.flatten(),
            port,
        },
        parser::Target::Socket(path) => {
            if query.channel_binding.flatten().is_some() {
                return Err(ParseError::UnsupportedSocketPathParameter(
                    "channel_binding",
                ));
            }
            if query.host_addr.flatten().is_some() {
                return Err(ParseError::UnsupportedSocketPathParameter("hostaddr"));
            }

            Endpoint::SocketPath { path, port }
        }
    };

    let user = exclusive("user", authority.user, query.user)?
        .ok_or(ParseError::MissingParameter("user"))?;
    let password = exclusive("password", authority.password, query.password)?;
    let database = exclusive("dbname", database, query.database)?
        .ok_or(ParseError::MissingParameter("dbname"))?;

    Ok(Config {
        endpoint,
        session: Session {
            application_name: query.application_name.flatten(),
            database,
            password,
            user,
        },
        ssl_mode: query.ssl_mode.flatten().unwrap_or(SslMode::VerifyFull),
        ssl_root_cert: query.ssl_root_cert.flatten(),
    })
}

fn exclusive<T>(
    name: &'static str,
    url_value: Option<T>,
    query_value: Option<Option<T>>,
) -> Result<Option<T>, ParseError> {
    match (url_value, query_value) {
        (Some(_), Some(_)) => Err(ParseError::ConflictingParameter(name)),
        (url_value, query_value) => Ok(url_value.or(query_value.flatten())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{ChannelBinding, Port};
    use crate::libpq::SslRootCert;

    fn network(host: &str, port: Option<u16>, host_addr: Option<&str>) -> Endpoint {
        Endpoint::Network {
            host: host.parse().unwrap(),
            channel_binding: None,
            port: port.map(Port::new),
            host_addr: host_addr.map(|address| address.parse().unwrap()),
        }
    }

    fn socket(path: &str) -> Endpoint {
        Endpoint::SocketPath {
            path: path.into(),
            port: None,
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
            endpoint,
            session: Session {
                user: user.parse().unwrap(),
                password: password.map(|value| value.parse().unwrap()),
                database: database.parse().unwrap(),
                application_name: application_name.map(|value| value.parse().unwrap()),
            },
            ssl_mode,
            ssl_root_cert,
        }
    }

    fn field_error(origin: FieldSource, field: Field, cause: FieldErrorCause) -> ParseError {
        ParseError::Field(FieldError {
            origin,
            field,
            cause,
        })
    }

    fn invalid_utf8() -> DecodeError {
        DecodeError::InvalidUtf8(String::from_utf8(vec![0xFF]).unwrap_err().utf8_error())
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
                    socket("/var/run/postgresql"),
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
                    socket("/socket"),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "abstract_socket",
                "postgres://?host=%40abstract&user=postgres&dbname=mydb",
                Ok(success(
                    "postgres",
                    None,
                    "mydb",
                    socket("@abstract"),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "unencoded_at_sign_starts_userinfo",
                "postgres://?host=@abstract&user=postgres&dbname=mydb",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("Not a socket address or FQDN".to_string()),
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
                    socket("/cloudsql/project:region:instance"),
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
                    socket("/cloudsql/project:region:instance"),
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
                    socket("/cloudsql/project:region:instance"),
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
                    socket("/cloudsql/project:region:instance"),
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
                    FieldErrorCause::InvalidValue("invalid IPv6 address syntax".to_string()),
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
                "hash_is_part_of_database",
                "postgres://user@localhost/mydb#section",
                Ok(success(
                    "user",
                    None,
                    "mydb#section",
                    network("localhost", None, None),
                    SslMode::VerifyFull,
                    None,
                    None,
                )),
            ),
            (
                "space_in_host",
                "postgres://user@local host/mydb",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidEncoding(DecodeError::UnexpectedSpaces),
                )),
            ),
            (
                "invalid_percent_encoding",
                "postgres://user@localhost/my%zzdb",
                Err(field_error(
                    FieldSource::Path,
                    Field::Database,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            (
                "invalid_ipv6_host",
                "postgres://user@[::g]/mydb",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("invalid IPv6 address syntax".to_string()),
                )),
            ),
            (
                "invalid_utf8_user",
                "postgres://%FF@localhost/mydb",
                Err(field_error(
                    FieldSource::Authority,
                    Field::User,
                    FieldErrorCause::InvalidEncoding(invalid_utf8()),
                )),
            ),
            (
                "port_overflow",
                "postgres://user@localhost:65536/mydb",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Port,
                    FieldErrorCause::InvalidValue("invalid postgresql port string".to_string()),
                )),
            ),
            (
                "query_parameter_without_separator",
                "postgres://user@localhost/mydb?sslmode",
                Err(ParseError::InvalidUrl { offset: 38 }),
            ),
            (
                "invalid_utf8_query_key",
                "postgres://user@localhost/mydb?%FF=1",
                Err(ParseError::InvalidQueryParameterEncoding(invalid_utf8())),
            ),
            (
                "invalid_utf8_query_value",
                "postgres://user@localhost/mydb?application_name=%FF",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::ApplicationName,
                    FieldErrorCause::InvalidEncoding(invalid_utf8()),
                )),
            ),
            (
                "empty_query_user",
                "postgres://?host=/socket&user=&dbname=mydb",
                Err(ParseError::MissingParameter("user")),
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
                let roundtrip_url = config.to_url();
                let roundtrip_config = parse(&roundtrip_url).unwrap_or_else(|error| {
                    panic!("{name}: roundtrip parse failed: {error}, url: {roundtrip_url}")
                });
                assert_eq!(roundtrip_config, config, "{name}: roundtrip");
            }
        }
    }
}
