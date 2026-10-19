use nom::{
    Finish as _, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_till, take_till1, take_while, take_while_m_n},
    character::complete::{char, one_of},
    combinator::{all_consuming, cut, eof, map, not, opt, peek, recognize, verify},
    multi::fold_many0,
    sequence::{delimited, pair, preceded, terminated},
};

use super::{DecodeError, Field, FieldError, FieldErrorCause, FieldSource, ParseError};
use crate::config::{ChannelBinding, Host, HostAddr, Password, Port};
use crate::libpq::{SslMode, SslRootCert};
use crate::{Database, User};

#[derive(Debug)]
enum Error<'a> {
    Syntax(&'a str),
    Semantic(ParseError),
}

impl<'a> nom::error::ParseError<&'a str> for Error<'a> {
    fn from_error_kind(input: &'a str, _kind: nom::error::ErrorKind) -> Self {
        Self::Syntax(input)
    }

    fn append(_input: &'a str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

type Result<'a, T> = IResult<&'a str, T, Error<'a>>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Target {
    Network(Host),
    Socket(std::path::PathBuf),
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Authority {
    pub(super) user: Option<User<'static>>,
    pub(super) password: Option<Password>,
    pub(super) host: Option<Target>,
    pub(super) port: Option<Port>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub(super) struct Query {
    pub(super) host: Option<Option<Target>>,
    pub(super) host_addr: Option<Option<HostAddr>>,
    pub(super) port: Option<Option<Port>>,
    pub(super) channel_binding: Option<Option<ChannelBinding>>,
    pub(super) user: Option<Option<User<'static>>>,
    pub(super) password: Option<Option<Password>>,
    pub(super) database: Option<Option<Database<'static>>>,
    pub(super) ssl_mode: Option<Option<SslMode>>,
    pub(super) ssl_root_cert: Option<Option<SslRootCert>>,
    pub(super) application_name: Option<Option<crate::parameter::Value>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Url {
    pub(super) authority: Authority,
    pub(super) database: Option<Database<'static>>,
    pub(super) query: Query,
}

pub(super) fn parse(input: &str) -> std::result::Result<Url, ParseError> {
    all_consuming(url)
        .parse(input)
        .finish()
        .map(|(_, url)| url)
        .map_err(|error| match error {
            Error::Syntax(remaining) => ParseError::InvalidUrl {
                offset: input.len() - remaining.len(),
            },
            Error::Semantic(error) => error,
        })
}

fn url(input: &str) -> Result<'_, Url> {
    let (input, ()) = scheme(input)?;
    let (input, (user, password)) = userinfo(input)?;
    let (input, host) = alt((bracketed_host, host)).parse(input)?;
    let (input, port) = opt(preceded(char(':'), port)).parse(input)?;
    let (input, ()) = single_host(input)?;
    let (input, database) = opt(preceded(char('/'), database)).parse(input)?;
    let (input, query) = opt(preceded(char('?'), query)).parse(input)?;

    Ok((
        input,
        Url {
            authority: Authority {
                user,
                password,
                host,
                port: port.flatten(),
            },
            database: database.flatten(),
            query: query.unwrap_or_default(),
        },
    ))
}

fn scheme(input: &str) -> Result<'_, ()> {
    let prefix: Result<'_, &str> = alt((tag("postgresql://"), tag("postgres://"))).parse(input);
    match prefix {
        Ok((input, _)) => Ok((input, ())),
        Err(_) => {
            let scheme = input.split(':').next().unwrap_or_default();
            Err(nom::Err::Failure(Error::Semantic(
                ParseError::InvalidScheme(scheme.to_string()),
            )))
        }
    }
}

fn userinfo(input: &str) -> Result<'_, (Option<User<'static>>, Option<Password>)> {
    semantic(
        opt(terminated(
            pair(
                take_till(|character| matches!(character, ':' | '@' | '/')),
                opt(preceded(
                    char(':'),
                    take_till(|character| matches!(character, '@' | '/')),
                )),
            ),
            char('@'),
        )),
        |userinfo| {
            let Some((user, password)) = userinfo else {
                return Ok((None, None));
            };
            let user = convert(FieldSource::Authority, Field::User, user, parse_identifier)?;
            let password = match password {
                Some(password) => convert(
                    FieldSource::Authority,
                    Field::Password,
                    password,
                    parse_value,
                )?,
                None => None,
            };
            Ok((user, password))
        },
    )
    .parse(input)
}

fn bracketed_host(input: &str) -> Result<'_, Option<Target>> {
    semantic(
        delimited(
            char('['),
            cut(verify(
                take_till(|character| character == ']'),
                |address: &str| !address.is_empty(),
            )),
            cut(terminated(
                char(']'),
                peek(alt((eof, recognize(one_of(":/?,"))))),
            )),
        ),
        |address| {
            let address = decode(address).map_err(|error| {
                field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidEncoding(error),
                )
            })?;
            address
                .parse::<std::net::Ipv6Addr>()
                .map(|address| Some(Target::Network(Host::IpAddr(address.into()))))
                .map_err(|error| {
                    field_error(
                        FieldSource::Authority,
                        Field::Host,
                        FieldErrorCause::InvalidValue(error.to_string()),
                    )
                })
        },
    )
    .parse(input)
}

fn host(input: &str) -> Result<'_, Option<Target>> {
    semantic(
        take_till(|character| matches!(character, ':' | '/' | '?' | ',')),
        |host| convert_target(FieldSource::Authority, host),
    )
    .parse(input)
}

fn port(input: &str) -> Result<'_, Option<Port>> {
    semantic(
        take_till(|character| matches!(character, '/' | '?' | ',')),
        |port| convert_port(FieldSource::Authority, port),
    )
    .parse(input)
}

fn single_host(input: &str) -> Result<'_, ()> {
    semantic(opt(char(',')), |separator| match separator {
        Some(_) => Err(ParseError::MultipleHosts),
        None => Ok(()),
    })
    .parse(input)
}

fn database(input: &str) -> Result<'_, Option<Database<'static>>> {
    semantic(take_till(|character| character == '?'), |database| {
        convert(
            FieldSource::Path,
            Field::Database,
            database,
            parse_identifier,
        )
    })
    .parse(input)
}

fn query(input: &str) -> Result<'_, Query> {
    fold_many0(
        preceded(not(eof), parameter),
        Query::default,
        |mut query, parameter| {
            match parameter {
                Parameter::Host(value) => query.host = Some(value),
                Parameter::HostAddr(value) => query.host_addr = Some(value),
                Parameter::Port(value) => query.port = Some(value),
                Parameter::ChannelBinding(value) => query.channel_binding = Some(value),
                Parameter::User(value) => query.user = Some(value),
                Parameter::Password(value) => query.password = Some(value),
                Parameter::Database(value) => query.database = Some(value),
                Parameter::SslMode(value) => query.ssl_mode = Some(value),
                Parameter::SslRootCert(value) => query.ssl_root_cert = Some(value),
                Parameter::ApplicationName(value) => query.application_name = Some(value),
            }
            query
        },
    )
    .parse(input)
}

enum Parameter {
    Host(Option<Target>),
    HostAddr(Option<HostAddr>),
    Port(Option<Port>),
    ChannelBinding(Option<ChannelBinding>),
    User(Option<User<'static>>),
    Password(Option<Password>),
    Database(Option<Database<'static>>),
    SslMode(Option<SslMode>),
    SslRootCert(Option<SslRootCert>),
    ApplicationName(Option<crate::parameter::Value>),
}

fn parameter(input: &str) -> Result<'_, Parameter> {
    semantic(
        terminated(
            pair(
                terminated(
                    take_till(|character| matches!(character, '=' | '&')),
                    cut(char('=')),
                ),
                take_till(|character| matches!(character, '=' | '&')),
            ),
            pair(cut(not(char('='))), opt(char('&'))),
        ),
        |(key, value)| {
            let key = decode(key).map_err(ParseError::InvalidQueryParameterEncoding)?;

            if key == "ssl" && decode(value).as_deref() == Ok("true") {
                return Ok(Parameter::SslMode(Some(SslMode::Require)));
            }

            let origin = FieldSource::QueryParam;

            match key.as_str() {
                "host" => convert_target(origin, value).map(Parameter::Host),
                "hostaddr" => {
                    convert(origin, Field::HostAddr, value, parse_value).map(Parameter::HostAddr)
                }
                "port" => convert_port(origin, value).map(Parameter::Port),
                "channel_binding" => convert(origin, Field::ChannelBinding, value, parse_echo)
                    .map(Parameter::ChannelBinding),
                "user" => {
                    convert(origin, Field::User, value, parse_identifier).map(Parameter::User)
                }
                "password" => {
                    convert(origin, Field::Password, value, parse_value).map(Parameter::Password)
                }
                "dbname" => convert(origin, Field::Database, value, parse_identifier)
                    .map(Parameter::Database),
                "sslmode" => {
                    convert(origin, Field::SslMode, value, parse_echo).map(Parameter::SslMode)
                }
                "sslrootcert" => convert(origin, Field::SslRootCert, value, parse_ssl_root_cert)
                    .map(Parameter::SslRootCert),
                "application_name" => convert(origin, Field::ApplicationName, value, parse_value)
                    .map(Parameter::ApplicationName),
                _ => Err(ParseError::InvalidQueryParameter(key)),
            }
        },
    )
    .parse(input)
}

fn convert_target(
    origin: FieldSource,
    raw: &str,
) -> std::result::Result<Option<Target>, ParseError> {
    let Some(value) = decode_field(origin.clone(), Field::Host, raw)? else {
        return Ok(None);
    };

    if value.contains(',') {
        Err(ParseError::MultipleHosts)
    } else if value.starts_with('/') || value.starts_with('@') {
        Ok(Some(Target::Socket(value.into())))
    } else {
        parse_value(&value)
            .map(|host| Some(Target::Network(host)))
            .map_err(|cause| field_error(origin, Field::Host, cause))
    }
}

fn convert_port(origin: FieldSource, raw: &str) -> std::result::Result<Option<Port>, ParseError> {
    let Some(value) = decode_field(origin.clone(), Field::Port, raw)? else {
        return Ok(None);
    };

    if value.contains(',') {
        Err(ParseError::MultipleHosts)
    } else {
        parse_value(&value)
            .map(Some)
            .map_err(|cause| field_error(origin, Field::Port, cause))
    }
}

fn parse_ssl_root_cert(value: &str) -> std::result::Result<SslRootCert, FieldErrorCause> {
    Ok(match value {
        "system" => SslRootCert::System,
        path => SslRootCert::File(path.into()),
    })
}

fn parse_identifier<T>(value: &str) -> std::result::Result<T, FieldErrorCause>
where
    T: std::str::FromStr<Err = crate::name::ParseError>,
{
    value.parse().map_err(FieldErrorCause::InvalidIdentifier)
}

fn parse_value<T>(value: &str) -> std::result::Result<T, FieldErrorCause>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    value
        .parse()
        .map_err(|error: T::Err| FieldErrorCause::InvalidValue(error.to_string()))
}

fn parse_echo<T: std::str::FromStr>(value: &str) -> std::result::Result<T, FieldErrorCause> {
    value
        .parse()
        .map_err(|_| FieldErrorCause::InvalidValue(value.to_string()))
}

fn convert<T>(
    origin: FieldSource,
    field: Field,
    raw: &str,
    parse: fn(&str) -> std::result::Result<T, FieldErrorCause>,
) -> std::result::Result<Option<T>, ParseError> {
    decode_field(origin.clone(), field.clone(), raw)?
        .map(|value| parse(&value).map_err(|cause| field_error(origin, field, cause)))
        .transpose()
}

fn decode_field(
    origin: FieldSource,
    field: Field,
    raw: &str,
) -> std::result::Result<Option<String>, ParseError> {
    let value = decode(raw)
        .map_err(|error| field_error(origin, field, FieldErrorCause::InvalidEncoding(error)))?;

    Ok(if value.is_empty() { None } else { Some(value) })
}

fn field_error(origin: FieldSource, field: Field, cause: FieldErrorCause) -> ParseError {
    FieldError {
        origin,
        field,
        cause,
    }
    .into()
}

fn semantic<'a, O, T>(
    mut parser: impl Parser<&'a str, Output = O, Error = Error<'a>>,
    convert: impl Fn(O) -> std::result::Result<T, ParseError>,
) -> impl Parser<&'a str, Output = T, Error = Error<'a>> {
    move |input: &'a str| {
        let (rest, output) = parser.parse(input)?;
        convert(output)
            .map(|converted| (rest, converted))
            .map_err(|error| nom::Err::Failure(Error::Semantic(error)))
    }
}

#[derive(Debug)]
enum DecodeFailure {
    Syntax,
    Decode(DecodeError),
}

impl nom::error::ParseError<&str> for DecodeFailure {
    fn from_error_kind(_input: &str, _kind: nom::error::ErrorKind) -> Self {
        Self::Syntax
    }

    fn append(_input: &str, _kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

enum Chunk<'a> {
    Literal(&'a str),
    Byte(u8),
}

fn decode(input: &str) -> std::result::Result<String, DecodeError> {
    let spaces = || take_while(|character| character == ' ');

    let bytes = all_consuming(delimited(
        spaces(),
        fold_many0(
            alt((
                map(
                    take_till1(|character| matches!(character, '%' | ' ')),
                    Chunk::Literal,
                ),
                map(percent_encoded, Chunk::Byte),
            )),
            Vec::new,
            |mut bytes, chunk| {
                match chunk {
                    Chunk::Literal(literal) => bytes.extend_from_slice(literal.as_bytes()),
                    Chunk::Byte(byte) => bytes.push(byte),
                }
                bytes
            },
        ),
        spaces(),
    ))
    .parse(input)
    .finish()
    .map(|(_, bytes)| bytes)
    .map_err(|error| match error {
        DecodeFailure::Syntax => DecodeError::UnexpectedSpaces,
        DecodeFailure::Decode(error) => error,
    })?;

    String::from_utf8(bytes).map_err(|error| DecodeError::InvalidUtf8(error.utf8_error()))
}

fn percent_encoded(input: &str) -> IResult<&str, u8, DecodeFailure> {
    let (input, _) = char('%').parse(input)?;
    let invalid = || nom::Err::Failure(DecodeFailure::Decode(DecodeError::InvalidPercentEncoding));
    let (input, hex) = take_while_m_n::<_, _, DecodeFailure>(2, 2, |character: char| {
        character.is_ascii_hexdigit()
    })
    .parse(input)
    .map_err(|_| invalid())?;

    match u8::from_str_radix(hex, 16) {
        Ok(0) => Err(nom::Err::Failure(DecodeFailure::Decode(DecodeError::Nul))),
        Ok(byte) => Ok((input, byte)),
        Err(_) => Err(invalid()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn render(url: Url) -> Vec<(String, String)> {
        let Url {
            authority,
            database,
            query,
        } = url;
        let target = |target: Target| match target {
            Target::Network(host) => host.pg_env_value(),
            Target::Socket(path) => path.to_str().unwrap().to_string(),
        };
        let mut fields: Vec<(&str, Option<String>)> = vec![
            (
                "user",
                query
                    .user
                    .unwrap_or(authority.user)
                    .map(|value| value.as_str().to_string()),
            ),
            (
                "password",
                query
                    .password
                    .unwrap_or(authority.password)
                    .map(|value| value.as_str().to_string()),
            ),
            (
                "dbname",
                query
                    .database
                    .unwrap_or(database)
                    .map(|value| value.as_str().to_string()),
            ),
            ("host", query.host.unwrap_or(authority.host).map(target)),
            (
                "port",
                query
                    .port
                    .unwrap_or(authority.port)
                    .map(|value| value.pg_env_value()),
            ),
            (
                "hostaddr",
                query
                    .host_addr
                    .flatten()
                    .map(|value| std::net::IpAddr::from(value).to_string()),
            ),
            (
                "channel_binding",
                query
                    .channel_binding
                    .flatten()
                    .map(|value| value.as_str().to_string()),
            ),
            (
                "sslmode",
                query.ssl_mode.flatten().map(|value| value.pg_env_value()),
            ),
            (
                "sslrootcert",
                query
                    .ssl_root_cert
                    .flatten()
                    .map(|value| value.pg_env_value()),
            ),
            (
                "application_name",
                query
                    .application_name
                    .flatten()
                    .map(|value| value.as_str().to_string()),
            ),
        ];
        fields.sort_by_key(|(key, _)| *key);
        fields
            .into_iter()
            .filter_map(|(key, value)| value.map(|value| (key.to_string(), value)))
            .collect()
    }

    type Expected = std::result::Result<&'static [(&'static str, &'static str)], ParseError>;

    // Inputs are the cases of libpq's URI regression suite
    // https://github.com/postgres/postgres/blob/REL_18_STABLE/src/interfaces/libpq/t/001_uri.pl
    // plus additional edge cases. Expected fields are the non-empty options
    // PQconninfoParse of libpq 18.6 reports for each input. Where libpq
    // accepts an input this parser rejects, libpq's result is noted above
    // the case.
    #[test]
    fn libpq_equivalence() {
        let cases: Vec<(&str, Expected)> = vec![
            (
                "postgresql://uri-user:secret@host:12345/db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "host"),
                    ("password", "secret"),
                    ("port", "12345"),
                    ("user", "uri-user"),
                ]),
            ),
            (
                "postgresql://uri-user@host:12345/db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "host"),
                    ("port", "12345"),
                    ("user", "uri-user"),
                ]),
            ),
            (
                "postgresql://uri-user@host/db",
                Ok(&[("dbname", "db"), ("host", "host"), ("user", "uri-user")]),
            ),
            (
                "postgresql://host:12345/db",
                Ok(&[("dbname", "db"), ("host", "host"), ("port", "12345")]),
            ),
            (
                "postgresql://host/db",
                Ok(&[("dbname", "db"), ("host", "host")]),
            ),
            (
                "postgresql://uri-user@host:12345/",
                Ok(&[("host", "host"), ("port", "12345"), ("user", "uri-user")]),
            ),
            (
                "postgresql://uri-user@host/",
                Ok(&[("host", "host"), ("user", "uri-user")]),
            ),
            ("postgresql://uri-user@", Ok(&[("user", "uri-user")])),
            (
                "postgresql://host:12345/",
                Ok(&[("host", "host"), ("port", "12345")]),
            ),
            (
                "postgresql://host:12345",
                Ok(&[("host", "host"), ("port", "12345")]),
            ),
            ("postgresql://host/", Ok(&[("host", "host")])),
            ("postgresql://host", Ok(&[("host", "host")])),
            ("postgresql://", Ok(&[])),
            (
                "postgresql://?hostaddr=127.0.0.1",
                Ok(&[("hostaddr", "127.0.0.1")]),
            ),
            (
                "postgresql://example.com?hostaddr=63.1.2.4",
                Ok(&[("host", "example.com"), ("hostaddr", "63.1.2.4")]),
            ),
            ("postgresql://%68ost/", Ok(&[("host", "host")])),
            (
                "postgresql://host/db?user=uri-user",
                Ok(&[("dbname", "db"), ("host", "host"), ("user", "uri-user")]),
            ),
            (
                "postgresql://host/db?user=uri-user&port=12345",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "host"),
                    ("port", "12345"),
                    ("user", "uri-user"),
                ]),
            ),
            (
                "postgresql://host/db?u%73er=someotheruser&port=12345",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "host"),
                    ("port", "12345"),
                    ("user", "someotheruser"),
                ]),
            ),
            (
                "postgresql://host/db?u%7aer=someotheruser&port=12345",
                Err(ParseError::InvalidQueryParameter("uzer".to_string())),
            ),
            (
                "postgresql://host:12345?user=uri-user",
                Ok(&[("host", "host"), ("port", "12345"), ("user", "uri-user")]),
            ),
            (
                "postgresql://host?user=uri-user",
                Ok(&[("host", "host"), ("user", "uri-user")]),
            ),
            (
                "postgresql://host?  user = uri-user & port  = 12345 ",
                Ok(&[("host", "host"), ("port", "12345"), ("user", "uri-user")]),
            ),
            (
                "postgresql://host?  user user  =  uri  & port = 12345 12 ",
                Err(ParseError::InvalidQueryParameterEncoding(
                    DecodeError::UnexpectedSpaces,
                )),
            ),
            (
                "postgresql://host?  user  =  uri-user  & port = 12345 12 ",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::Port,
                    FieldErrorCause::InvalidEncoding(DecodeError::UnexpectedSpaces),
                )),
            ),
            ("postgresql://host?", Ok(&[("host", "host")])),
            (
                "postgresql://[::1]:12345/db",
                Ok(&[("dbname", "db"), ("host", "::1"), ("port", "12345")]),
            ),
            (
                "postgresql://[::1]/db",
                Ok(&[("dbname", "db"), ("host", "::1")]),
            ),
            (
                "postgresql://[2001:db8::1234]/",
                Ok(&[("host", "2001:db8::1234")]),
            ),
            // libpq: OK host=200z:db8::1234
            (
                "postgresql://[200z:db8::1234]/",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("invalid IPv6 address syntax".to_string()),
                )),
            ),
            ("postgresql://[::1]", Ok(&[("host", "::1")])),
            ("postgres://", Ok(&[])),
            ("postgres:///", Ok(&[])),
            ("postgres:///db", Ok(&[("dbname", "db")])),
            (
                "postgres://uri-user@/db",
                Ok(&[("dbname", "db"), ("user", "uri-user")]),
            ),
            (
                "postgres://?host=/path/to/socket/dir",
                Ok(&[("host", "/path/to/socket/dir")]),
            ),
            (
                "postgresql://host?uzer=",
                Err(ParseError::InvalidQueryParameter("uzer".to_string())),
            ),
            (
                "postgre://",
                Err(ParseError::InvalidScheme("postgre".to_string())),
            ),
            (
                "postgres://[::1",
                Err(ParseError::InvalidUrl { offset: 15 }),
            ),
            ("postgres://[]", Err(ParseError::InvalidUrl { offset: 12 })),
            (
                "postgres://[::1]z",
                Err(ParseError::InvalidUrl { offset: 16 }),
            ),
            (
                "postgresql://host?zzz",
                Err(ParseError::InvalidUrl { offset: 21 }),
            ),
            (
                "postgresql://host?value1&value2",
                Err(ParseError::InvalidUrl { offset: 24 }),
            ),
            (
                "postgresql://host?key=key=value",
                Err(ParseError::InvalidUrl { offset: 25 }),
            ),
            (
                "postgres://host?dbname=%XXfoo",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::Database,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            (
                "postgresql://a%00b",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidEncoding(DecodeError::Nul),
                )),
            ),
            (
                "postgresql://%zz",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            (
                "postgresql://%1",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            (
                "postgresql://%",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            ("postgres://@host", Ok(&[("host", "host")])),
            ("postgres://host:/", Ok(&[("host", "host")])),
            ("postgres://:12345/", Ok(&[("port", "12345")])),
            (
                "postgres://otheruser@?host=/no/such/directory",
                Ok(&[("host", "/no/such/directory"), ("user", "otheruser")]),
            ),
            (
                "postgres://otheruser@/?host=/no/such/directory",
                Ok(&[("host", "/no/such/directory"), ("user", "otheruser")]),
            ),
            (
                "postgres://otheruser@:12345?host=/no/such/socket/path",
                Ok(&[
                    ("host", "/no/such/socket/path"),
                    ("port", "12345"),
                    ("user", "otheruser"),
                ]),
            ),
            (
                "postgres://otheruser@:12345/db?host=/path/to/socket",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "/path/to/socket"),
                    ("port", "12345"),
                    ("user", "otheruser"),
                ]),
            ),
            (
                "postgres://:12345/db?host=/path/to/socket",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "/path/to/socket"),
                    ("port", "12345"),
                ]),
            ),
            (
                "postgres://:12345?host=/path/to/socket",
                Ok(&[("host", "/path/to/socket"), ("port", "12345")]),
            ),
            (
                "postgres://%2Fvar%2Flib%2Fpostgresql/dbname",
                Ok(&[("dbname", "dbname"), ("host", "/var/lib/postgresql")]),
            ),
            (
                "postgresql://host?sslmode=disable",
                Ok(&[("host", "host"), ("sslmode", "disable")]),
            ),
            (
                "postgresql://host?sslmode=prefer",
                Ok(&[("host", "host"), ("sslmode", "prefer")]),
            ),
            (
                "postgresql://host?sslmode=verify-full",
                Ok(&[("host", "host"), ("sslmode", "verify-full")]),
            ),
            (
                "postgres://u:p@h:5432/db?sslmode=disable&application_name=app",
                Ok(&[
                    ("application_name", "app"),
                    ("dbname", "db"),
                    ("host", "h"),
                    ("password", "p"),
                    ("port", "5432"),
                    ("sslmode", "disable"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h/db?port=5433",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("port", "5433"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h:5432/db?port=5433",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("port", "5433"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@/db?host=/sock&port=5433",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "/sock"),
                    ("port", "5433"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@:5433/db?host=/sock",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "/sock"),
                    ("port", "5433"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@%2Fsock:5433/db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "/sock"),
                    ("port", "5433"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@%40abstract/db",
                Ok(&[("dbname", "db"), ("host", "@abstract"), ("user", "u")]),
            ),
            // libpq: OK dbname=db | host=h1,h2 | port=, | user=u
            ("postgres://u@h1,h2/db", Err(ParseError::MultipleHosts)),
            // libpq: OK dbname=db | host=h1,h2 | port=1,2 | user=u
            ("postgres://u@h1:1,h2:2/db", Err(ParseError::MultipleHosts)),
            // libpq: OK dbname=db | host=::1,::2 | port=, | user=u
            (
                "postgres://u@[::1],[::2]/db",
                Err(ParseError::MultipleHosts),
            ),
            // libpq: OK dbname=db | host=h,h2 | user=u
            ("postgres://u@h%2Ch2/db", Err(ParseError::MultipleHosts)),
            // libpq: OK dbname=db | host=h1,h2 | user=u
            (
                "postgres://u@h/db?host=h1,h2",
                Err(ParseError::MultipleHosts),
            ),
            // libpq: OK dbname=db | host=h | port=1,2 | user=u
            ("postgres://u@h/db?port=1,2", Err(ParseError::MultipleHosts)),
            (
                "postgres://u@h/db?ssl=true",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("sslmode", "require"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h/db?ssl=false",
                Err(ParseError::InvalidQueryParameter("ssl".to_string())),
            ),
            (
                "postgres://u@h/db?ssl=",
                Err(ParseError::InvalidQueryParameter("ssl".to_string())),
            ),
            (
                "postgres://u@h/db?sslmode=",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/db?user=",
                Ok(&[("dbname", "db"), ("host", "h")]),
            ),
            (
                "postgres://u@h/db?dbname=",
                Ok(&[("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/db?a=1&",
                Err(ParseError::InvalidQueryParameter("a".to_string())),
            ),
            (
                "postgres://u@h/db?&a=1",
                Err(ParseError::InvalidUrl { offset: 18 }),
            ),
            (
                "postgres://u@h/db?a=1&&b=2",
                Err(ParseError::InvalidQueryParameter("a".to_string())),
            ),
            (
                "postgres://u@h/db?user=a=b",
                Err(ParseError::InvalidUrl { offset: 24 }),
            ),
            (
                "postgres://u@h/db?=x",
                Err(ParseError::InvalidQueryParameter("".to_string())),
            ),
            (
                "postgres://u@h/db?",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/db??user=x",
                Err(ParseError::InvalidQueryParameter("?user".to_string())),
            ),
            (
                "postgres://u@h/db?user=x?y",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "x?y")]),
            ),
            (
                "postgres://u@h/db/extra",
                Ok(&[("dbname", "db/extra"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h//db",
                Ok(&[("dbname", "/db"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/db%2Fx",
                Ok(&[("dbname", "db/x"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/%20db%20",
                Ok(&[("dbname", " db "), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/ db ",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/d b",
                Err(field_error(
                    FieldSource::Path,
                    Field::Database,
                    FieldErrorCause::InvalidEncoding(DecodeError::UnexpectedSpaces),
                )),
            ),
            (
                "postgres:// u @h/db",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u%20x@h/db",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u x")]),
            ),
            (
                "postgres://u:@h/db",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://:p@h/db",
                Ok(&[("dbname", "db"), ("host", "h"), ("password", "p")]),
            ),
            (
                "postgres://u:p:q@h/db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("password", "p:q"),
                    ("user", "u"),
                ]),
            ),
            // libpq: OK dbname=db | host=h@x | user=u
            (
                "postgres://u@h@x/db",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("Not a socket address or FQDN".to_string()),
                )),
            ),
            (
                "postgres://u@h:/db",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u")]),
            ),
            // libpq: OK dbname=db | host=h | port=abc | user=u
            (
                "postgres://u@h:abc/db",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Port,
                    FieldErrorCause::InvalidValue("invalid postgresql port string".to_string()),
                )),
            ),
            (
                "postgres://u@h:0/db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("port", "0"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h:%35%34%33%32/db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("port", "5432"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h: 5432 /db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("port", "5432"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@[::1]:5432/db",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "::1"),
                    ("port", "5432"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@[::1]5432/db",
                Err(ParseError::InvalidUrl { offset: 18 }),
            ),
            (
                "postgres://u@[::1]?user=x",
                Ok(&[("host", "::1"), ("user", "x")]),
            ),
            // libpq: OK dbname=db | host=::1,h | port=, | user=u
            ("postgres://u@[::1],h/db", Err(ParseError::MultipleHosts)),
            (
                "postgres://u@[%3A%3A1]/db",
                Ok(&[("dbname", "db"), ("host", "::1"), ("user", "u")]),
            ),
            // libpq: OK dbname=db | host=fe80::1%eth0 | user=u
            (
                "postgres://u@[fe80::1%25eth0]/db",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("invalid IPv6 address syntax".to_string()),
                )),
            ),
            (
                "postgres://u@[]/db",
                Err(ParseError::InvalidUrl { offset: 14 }),
            ),
            (
                "postgres://u@[::1/db",
                Err(ParseError::InvalidUrl { offset: 20 }),
            ),
            // libpq: OK dbname=db | host=v1.fe80 | user=u
            (
                "postgres://u@[v1.fe80]/db",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("invalid IPv6 address syntax".to_string()),
                )),
            ),
            (
                "postgres://u@192.168.1.1/db",
                Ok(&[("dbname", "db"), ("host", "192.168.1.1"), ("user", "u")]),
            ),
            (
                "postgres://u@01.1.1.1/db",
                Ok(&[("dbname", "db"), ("host", "01.1.1.1"), ("user", "u")]),
            ),
            (
                "postgres://u@h/db#frag",
                Ok(&[("dbname", "db#frag"), ("host", "h"), ("user", "u")]),
            ),
            // libpq: OK dbname=db | host=h#frag | user=u
            (
                "postgres://u@h#frag/db",
                Err(field_error(
                    FieldSource::Authority,
                    Field::Host,
                    FieldErrorCause::InvalidValue("Not a socket address or FQDN".to_string()),
                )),
            ),
            (
                "postgres://u@h/db?application_name=a#b",
                Ok(&[
                    ("application_name", "a#b"),
                    ("dbname", "db"),
                    ("host", "h"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h/db?application_name=a%00b",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::ApplicationName,
                    FieldErrorCause::InvalidEncoding(DecodeError::Nul),
                )),
            ),
            (
                "postgres://u@h/db?sslrootcert=/a%00b",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::SslRootCert,
                    FieldErrorCause::InvalidEncoding(DecodeError::Nul),
                )),
            ),
            (
                "postgres://u@h/db?host=/a%00b",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::Host,
                    FieldErrorCause::InvalidEncoding(DecodeError::Nul),
                )),
            ),
            (
                "postgres://u@h/db?application_name=%",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::ApplicationName,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            (
                "postgres://u@h/db?application_name=%4",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::ApplicationName,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            (
                "postgres://u@h/db?application_name=%4g",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::ApplicationName,
                    FieldErrorCause::InvalidEncoding(DecodeError::InvalidPercentEncoding),
                )),
            ),
            (
                "postgres://u@h/db?application_name=%C3%A9",
                Ok(&[
                    ("application_name", "é"),
                    ("dbname", "db"),
                    ("host", "h"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h/db?application_name=é",
                Ok(&[
                    ("application_name", "é"),
                    ("dbname", "db"),
                    ("host", "h"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://ü@h/db",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "ü")]),
            ),
            (
                "postgres://u@h/db?%75ser=x",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "x")]),
            ),
            (
                "postgres://u@h/db?u%20ser=x",
                Err(ParseError::InvalidQueryParameter("u ser".to_string())),
            ),
            (
                "postgres://u@h/db? user =x",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "x")]),
            ),
            // libpq: OK dbname=db | host=h | sslmode=REQUIRE | user=u
            (
                "postgres://u@h/db?sslmode=REQUIRE",
                Err(field_error(
                    FieldSource::QueryParam,
                    Field::SslMode,
                    FieldErrorCause::InvalidValue("REQUIRE".to_string()),
                )),
            ),
            (
                "postgres://u@h/db?hostaddr=::1",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("hostaddr", "::1"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h/db?channel_binding=require&hostaddr=10.0.0.1",
                Ok(&[
                    ("channel_binding", "require"),
                    ("dbname", "db"),
                    ("host", "h"),
                    ("hostaddr", "10.0.0.1"),
                    ("user", "u"),
                ]),
            ),
            (
                "POSTGRES://u@h/db",
                Err(ParseError::InvalidScheme("POSTGRES".to_string())),
            ),
            (
                "postgresql:/u@h/db",
                Err(ParseError::InvalidScheme("postgresql".to_string())),
            ),
            (
                "postgres:/",
                Err(ParseError::InvalidScheme("postgres".to_string())),
            ),
            (
                "http://u@h/db",
                Err(ParseError::InvalidScheme("http".to_string())),
            ),
            (
                "postgres://u@h/db?sslmode=disable&sslmode=",
                Ok(&[("dbname", "db"), ("host", "h"), ("user", "u")]),
            ),
            (
                "postgres://u@h/db?application_name=a&application_name=b",
                Ok(&[
                    ("application_name", "b"),
                    ("dbname", "db"),
                    ("host", "h"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h/db?user=a&user=",
                Ok(&[("dbname", "db"), ("host", "h")]),
            ),
            (
                "postgres://u@h/db?ssl=true&sslmode=disable",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("sslmode", "disable"),
                    ("user", "u"),
                ]),
            ),
            (
                "postgres://u@h/db?sslmode=disable&ssl=true",
                Ok(&[
                    ("dbname", "db"),
                    ("host", "h"),
                    ("sslmode", "require"),
                    ("user", "u"),
                ]),
            ),
        ];

        for (input, expected) in cases {
            let expected = expected.map(|fields| {
                fields
                    .iter()
                    .map(|(key, value)| (key.to_string(), value.to_string()))
                    .collect::<Vec<_>>()
            });
            assert_eq!(parse(input).map(render), expected, "{input}");
        }
    }
}
