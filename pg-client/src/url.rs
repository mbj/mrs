use crate::{Config, Database, Endpoint, Host, Password, Port, SslMode, SslRootCert, Username};
use percent_encoding::percent_decode_str;
use std::borrow::Cow;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ParseError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] ::url::ParseError),
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
    #[error("Invalid username: {0}")]
    InvalidUsername(String),
    #[error("Invalid password: {0}")]
    InvalidPassword(String),
    #[error("Invalid database: {0}")]
    InvalidDatabase(#[from] crate::identifier::ParseError),
    #[error("Invalid database encoding: {0}")]
    InvalidDatabaseEncoding(std::str::Utf8Error),
    #[error("Invalid host: {0}")]
    InvalidHost(String),
    #[error("Invalid hostaddr: {0}")]
    InvalidHostAddr(String),
    #[error("Invalid sslmode: {0}")]
    InvalidSslMode(String),
    #[error("Invalid application_name: {0}")]
    InvalidApplicationName(String),
    #[error("Invalid channel binding: {0}")]
    InvalidChannelBinding(String),
    #[error("Unsupported parameter for this connection type: '{0}'")]
    UnsupportedParameter(&'static str),
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
/// # Query Parameters
///
/// - `sslmode`: SSL mode (allow, disable, prefer, require, verify-ca, verify-full)
/// - `sslrootcert`: Path to SSL root certificate or "system"
/// - `application_name`: Application name
/// - `hostaddr`: IP address for the host
/// - `channel_binding`: Channel binding (disable, prefer, require)
/// - `host`: Socket path (when URL has no host component)
/// - `user`: Username (when URL has no username component)
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
///     &"postgres://user@localhost:5432/mydb".parse().unwrap(),
/// ).unwrap();
///
/// assert_eq!(config.username.as_str(), "user");
/// assert_eq!(config.database.as_str(), "mydb");
/// assert_eq!(config.ssl_mode, SslMode::VerifyFull);
/// ```
pub fn parse(url: &::url::Url) -> Result<Config, ParseError> {
    // Validate scheme
    let scheme = url.scheme();
    if scheme != "postgres" && scheme != "postgresql" {
        return Err(ParseError::InvalidScheme(scheme.to_string()));
    }

    if let Some(fragment) = url.fragment() {
        return Err(ParseError::InvalidFragment(fragment.to_string()));
    }

    let query_pairs: BTreeMap<_, _> = url.query_pairs().collect();
    let mut query_params = QueryParams::new(&query_pairs);

    // Resolve host - check for conflicts between URL host and query param host
    let url_host = url.host();
    let query_host = query_params.take("host");

    let (endpoint, username, password, database) = match (url_host, query_host) {
        (Some(_), Some(_)) => return Err(ParseError::ConflictingParameter("host")),
        (Some(url_host), None) => parse_network_connection(url_host, url, &mut query_params)?,
        (None, Some(host)) => {
            if host.starts_with('/') || host.starts_with('@') {
                parse_socket_connection(host, &mut query_params)?
            } else {
                return Err(ParseError::InvalidHost(
                    "query host must be a socket path (start with / or @)".to_string(),
                ));
            }
        }
        (None, None) => return Err(ParseError::MissingHost),
    };

    // Parse sslmode, defaulting to verify-full for secure connections
    let ssl_mode = match query_params.take("sslmode") {
        Some(mode_str) => mode_str
            .parse()
            .map_err(|_| ParseError::InvalidSslMode(mode_str.to_string()))?,
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
        Some(name_str) => Some(
            name_str
                .parse()
                .map_err(|error: String| ParseError::InvalidApplicationName(error))?,
        ),
        None => None,
    };

    if let Some(unknown) = query_params.unknown_param() {
        return Err(ParseError::InvalidQueryParameter((*unknown).to_string()));
    }

    Ok(Config {
        application_name,
        database,
        endpoint,
        password,
        ssl_mode,
        ssl_root_cert,
        username,
    })
}

fn parse_socket_connection<'a>(
    socket_path: &str,
    query_params: &mut QueryParams<'a>,
) -> Result<(Endpoint, Username, Option<Password>, Database), ParseError> {
    for name in ["channel_binding", "hostaddr"] {
        if query_params.take(name).is_some() {
            return Err(ParseError::UnsupportedParameter(name));
        }
    }

    let username: Username = query_params
        .take("user")
        .ok_or(ParseError::MissingParameter("user"))?
        .parse()
        .map_err(ParseError::InvalidUsername)?;

    let password: Option<Password> = query_params
        .take("password")
        .map(|value| value.parse().map_err(ParseError::InvalidPassword))
        .transpose()?;

    let database: Database = query_params
        .take("dbname")
        .ok_or(ParseError::MissingParameter("dbname"))?
        .parse()?;

    Ok((
        Endpoint::SocketPath(socket_path.into()),
        username,
        password,
        database,
    ))
}

fn access_field<'a>(
    name: &'static str,
    url_value: Option<&'a str>,
    query_params: &mut QueryParams<'a>,
) -> Result<Option<&'a str>, ParseError> {
    let query_value = query_params.take(name);
    match (url_value, query_value) {
        (Some(_), Some(_)) => Err(ParseError::ConflictingParameter(name)),
        (Some(value), None) => Ok(Some(value)),
        (None, Some(value)) => Ok(Some(value)),
        (None, None) => Ok(None),
    }
}

struct QueryParams<'a> {
    params: &'a BTreeMap<Cow<'a, str>, Cow<'a, str>>,
    remaining: BTreeSet<&'a str>,
}

impl<'a> QueryParams<'a> {
    fn new(params: &'a BTreeMap<Cow<'a, str>, Cow<'a, str>>) -> Self {
        let remaining = params.keys().map(|key| key.as_ref()).collect();
        Self { params, remaining }
    }

    fn take(&mut self, name: &'static str) -> Option<&'a str> {
        let value = self.params.get(name).map(|value| value.as_ref());
        if value.is_some() {
            self.remaining.remove(name);
        }
        value
    }

    fn unknown_param(&self) -> Option<&&'a str> {
        self.remaining.iter().next()
    }
}

fn parse_network_connection<'a>(
    url_host: ::url::Host<&str>,
    url: &'a ::url::Url,
    query_params: &mut QueryParams<'a>,
) -> Result<(Endpoint, Username, Option<Password>, Database), ParseError> {
    let host = match url_host {
        ::url::Host::Domain(domain) => domain
            .parse::<Host>()
            .map_err(|error: &str| ParseError::InvalidHost(error.to_string()))?,
        ::url::Host::Ipv4(ipv4) => Host::IpAddr(ipv4.into()),
        ::url::Host::Ipv6(ipv6) => Host::IpAddr(ipv6.into()),
    };

    let host_addr = match query_params.take("hostaddr") {
        Some(addr_str) => Some(
            addr_str
                .parse()
                .map_err(|error: &str| ParseError::InvalidHostAddr(error.to_string()))?,
        ),
        None => None,
    };

    let channel_binding = match query_params.take("channel_binding") {
        Some(binding_str) => Some(
            binding_str
                .parse()
                .map_err(|_| ParseError::InvalidChannelBinding(binding_str.to_string()))?,
        ),
        None => None,
    };

    let port = url.port().map(Port::new);

    let username_encoded = access_field("user", Some(url.username()), query_params)?
        .ok_or(ParseError::MissingParameter("user"))?;
    if username_encoded.is_empty() {
        return Err(ParseError::MissingParameter("user"));
    }
    let username_decoded = percent_decode_str(username_encoded)
        .decode_utf8()
        .map_err(|err| ParseError::InvalidUsername(err.to_string()))?;
    let username: Username = username_decoded
        .parse()
        .map_err(ParseError::InvalidUsername)?;

    let password = match access_field("password", url.password(), query_params)? {
        Some(password_encoded) => {
            let password_decoded = percent_decode_str(password_encoded)
                .decode_utf8()
                .map_err(|err| ParseError::InvalidPassword(err.to_string()))?;
            Some(
                password_decoded
                    .parse()
                    .map_err(ParseError::InvalidPassword)?,
            )
        }
        None => None,
    };

    let path = url.path();
    let database_raw = match path.strip_prefix('/').unwrap_or(path) {
        "" => None,
        value => Some(value),
    };
    let database_encoded = access_field("dbname", database_raw, query_params)?
        .ok_or(ParseError::MissingParameter("dbname"))?;
    let database_decoded = percent_decode_str(database_encoded)
        .decode_utf8()
        .map_err(ParseError::InvalidDatabaseEncoding)?;
    let database: Database = database_decoded.parse()?;

    Ok((
        Endpoint::Network {
            host,
            channel_binding,
            host_addr,
            port,
        },
        username,
        password,
        database,
    ))
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
        username: &str,
        password: Option<&str>,
        database: &str,
        endpoint: Endpoint,
        ssl_mode: SslMode,
        ssl_root_cert: Option<SslRootCert>,
        application_name: Option<&str>,
    ) -> Config {
        Config {
            username: username.parse().unwrap(),
            password: password.map(|value| value.parse().unwrap()),
            database: database.parse().unwrap(),
            endpoint,
            ssl_mode,
            ssl_root_cert,
            application_name: application_name.map(|value| value.parse().unwrap()),
        }
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
                Err(ParseError::InvalidSslMode("invalid".to_string())),
            ),
            (
                "invalid_channel_binding",
                "postgres://user@localhost/mydb?channel_binding=invalid",
                Err(ParseError::InvalidChannelBinding("invalid".to_string())),
            ),
            (
                "invalid_hostaddr",
                "postgres://user@localhost/mydb?hostaddr=not-an-ip",
                Err(ParseError::InvalidHostAddr(
                    "invalid IP address".to_string(),
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
                Err(ParseError::UnsupportedParameter("channel_binding")),
            ),
            (
                "socket_with_hostaddr",
                "postgres://?host=/socket&user=user&dbname=mydb&hostaddr=127.0.0.1",
                Err(ParseError::UnsupportedParameter("hostaddr")),
            ),
        ];

        for (name, url_str, expected) in cases {
            let url = ::url::Url::parse(url_str).unwrap();
            let actual = parse(&url);

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
