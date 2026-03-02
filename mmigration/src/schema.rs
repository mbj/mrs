use crate::types::Schema;
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::recognize,
};

fn dumped_from_database_version_prefix_parser(input: &str) -> IResult<&str, &str> {
    recognize((
        tag("-- Dumped from database version "),
        digit1,
        char('.'),
        digit1,
    ))
    .parse(input)
}

fn dumped_from_database_version_prefix(line: &str) -> Option<&str> {
    dumped_from_database_version_prefix_parser(line)
        .ok()
        .map(|(_, prefix)| prefix)
}

/// Schema normalizer to remove detailed version information from schema dumps
///
/// This is useful when the version string used in production cannot be reproduced exactly locally.
///
/// # Example
///
/// ```
/// # use mmigration::types::*;
/// # use mmigration::schema::*;
/// let schema: Schema = r#"
/// --
/// -- PostgreSQL database dump
/// --
/// -- Dumped from database version 16.9 (Debian 16.9-1.pgdg120+1)
/// -- Dumped by pg_dump version 16.9 (Debian 16.9-1.pgdg120+1)
/// REGULAR STATEMENTS;
/// "#
/// .into();
///
/// let expected: Schema = r#"
/// --
/// -- PostgreSQL database dump
/// --
/// -- Dumped from database version 16.9
/// -- Dumped by pg_dump version 16.9 (Debian 16.9-1.pgdg120+1)
/// REGULAR STATEMENTS;
/// "#
/// .into();
///
/// assert_eq!(expected, remove_version_details(&schema));
/// ```
#[must_use]
pub fn remove_version_details(schema: &Schema) -> Schema {
    let mut output = String::new();

    for line in <Schema as AsRef<str>>::as_ref(schema).lines() {
        match dumped_from_database_version_prefix(line) {
            None => output.push_str(line),
            Some(prefix) => output.push_str(prefix),
        }
        output.push('\n');
    }

    output.into()
}

/// Noop schema normalizer
///
/// This is useful when there is no need to use a normalizer
///
/// # Example
///
/// ```
/// # use mmigration::types::*;
/// # use mmigration::schema::*;
/// let schema: Schema = r#"
/// --
/// -- PostgreSQL database dump
/// --
/// -- Dumped from database version 16.9 (Debian 16.9-1.pgdg120+1)
/// -- Dumped by pg_dump version 16.9 (Debian 16.9-1.pgdg120+1)
/// REGULAR STATEMENTS;
/// "#
/// .into();
///
/// assert_eq!(schema, noop(&schema));
/// ```
#[must_use]
pub fn noop(schema: &Schema) -> Schema {
    schema.clone()
}
