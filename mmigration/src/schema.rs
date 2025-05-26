use crate::types::Schema;

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
pub fn remove_version_details(schema: &Schema) -> Schema {
    let mut output = String::new();

    let pattern =
        regex_lite::Regex::new(r#"\A(?<prefix>-- Dumped from database version \d+.\d+)"#).unwrap();

    for line in <Schema as AsRef<str>>::as_ref(schema).lines() {
        match pattern.captures(line) {
            None => {
                output.push_str(line);
            }
            Some(captures) => output.push_str(&captures["prefix"]),
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
pub fn noop(schema: &Schema) -> Schema {
    schema.clone()
}
