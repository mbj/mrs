//! CLI interface for partitioned index operations.

use core::num::NonZeroU16;

use std::collections::BTreeSet;

use nom::{
    Finish as _, IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while, take_while1},
    character::complete::{char, multispace0},
    combinator::{all_consuming, map, value},
    multi::{many0, separated_list1},
    sequence::{delimited, pair, preceded},
};
use nom_language::error::VerboseError;

use super::{ConcurrentlyConfig, Error, FillFactor, SqlFragment};
use crate::identifier::{AccessMethod, Index, Schema, Table};

/// Partitioned index operations.
#[derive(Debug, clap::Subcommand)]
pub enum Command {
    /// Create an index on a partitioned table.
    Create(Create),
    /// Garbage collect incomplete index creation state.
    Gc(Gc),
}

/// Output of a partitioned index operation.
#[derive(Debug)]
pub enum Output {
    /// Output of index creation.
    Create(super::create::Result),
    /// Output of garbage collection.
    Gc(super::gc::Result),
}

impl std::fmt::Display for Output {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Create(result) => result.fmt(f),
            Self::Gc(result) => result.fmt(f),
        }
    }
}

impl Command {
    /// Run the partitioned index operation using the provided database configuration.
    pub async fn run(self, config: &crate::Config) -> Result<Output, Error> {
        let output = match self {
            Self::Create(create) => create.run(config).await.map(Output::Create),
            Self::Gc(gc) => gc.run(config).await.map(Output::Gc),
        }?;

        log::info!("{output}");

        Ok(output)
    }
}

/// Create an index on a partitioned PostgreSQL table.
#[derive(Debug, clap::Args)]
pub struct Create {
    /// Parent partitioned table name.
    #[arg(long)]
    table: Table,
    /// Index name for the parent index.
    #[arg(long)]
    index: Index,
    /// Index key expression without surrounding parentheses (e.g. "created_at" or "lower(email), account_id").
    #[arg(long)]
    key_expression: SqlFragment,
    /// Schema name.
    #[arg(long, default_value = "public")]
    schema: Schema,
    /// Create a unique index.
    #[arg(long)]
    unique: bool,
    /// Index access method (e.g. btree, hash).
    #[arg(long, default_value = "btree")]
    method: AccessMethod,
    /// INCLUDE clause for covering indexes without INCLUDE keyword or parentheses (e.g. "col1, col2").
    #[arg(long)]
    include: Option<SqlFragment>,
    /// WHERE clause filter (without the WHERE keyword).
    #[arg(long)]
    where_clause: Option<SqlFragment>,
    /// Storage parameter for fillfactor (1-100).
    #[arg(long)]
    fillfactor: Option<FillFactor>,
    /// Use CREATE INDEX CONCURRENTLY on partitions.
    #[arg(
        long,
        default_value = "none",
        default_missing_value = "all",
        num_args = 0..=1,
        value_name = "MODE",
        value_parser = parse_concurrently,
    )]
    concurrently: ConcurrentlyConfig,
    /// Number of parallel workers for partition index creation.
    #[arg(long, default_value = "1")]
    jobs: NonZeroU16,
    /// Print SQL statements without executing them.
    #[arg(long)]
    dry_run: bool,
}

impl Create {
    /// Run the partitioned index creation using the provided database configuration.
    pub async fn run(self, config: &crate::Config) -> Result<super::create::Result, Error> {
        let input = super::create::Input {
            schema: self.schema,
            table: self.table,
            index: self.index,
            key_expression: self.key_expression,
            unique: self.unique,
            method: self.method,
            include: self.include,
            where_clause: self.where_clause,
            fillfactor: self.fillfactor,
            concurrently: self.concurrently,
        };

        super::create::run(config, &input, self.jobs, self.dry_run).await
    }
}

fn parse_concurrently(value: &str) -> Result<ConcurrentlyConfig, String> {
    let parsed = all_consuming(delimited(multispace0, concurrently_spec, multispace0))
        .parse(value)
        .finish();

    match parsed {
        Ok(("", concurrently_config)) => Ok(concurrently_config),
        Ok((remaining, _)) => Err(format!("unexpected trailing input: '{remaining}'")),
        Err(error) => Err(nom_language::error::convert_error(value, error)),
    }
}

fn parse_table_set(values: Vec<String>) -> Result<BTreeSet<Table>, String> {
    let mut tables = BTreeSet::new();
    for value in values {
        let table: Table = value
            .parse()
            .map_err(|error| format!("invalid table identifier '{value}': {error}"))?;
        if !tables.insert(table.clone()) {
            return Err(format!("duplicate table identifier '{table}'"));
        }
    }
    Ok(tables)
}

fn concurrently_spec(input: &str) -> IResult<&str, ConcurrentlyConfig, VerboseError<&str>> {
    alt((
        value(ConcurrentlyConfig::All, tag("all")),
        value(ConcurrentlyConfig::None, tag("none")),
        value(ConcurrentlyConfig::None, tag("disabled")),
        map(
            preceded(
                tag("except:"),
                delimited(multispace0, table_set, multispace0),
            ),
            ConcurrentlyConfig::Except,
        ),
    ))
    .parse(input)
}

fn table_identifier(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    alt((quoted_identifier, unquoted_identifier)).parse(input)
}

fn table_list(input: &str) -> IResult<&str, Vec<String>, VerboseError<&str>> {
    separated_list1(
        delimited(multispace0, char(','), multispace0),
        table_identifier,
    )
    .parse(input)
}

fn table_set(input: &str) -> IResult<&str, BTreeSet<Table>, VerboseError<&str>> {
    nom::combinator::map_res(table_list, parse_table_set).parse(input)
}

fn quoted_identifier(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    let (input, parts) = delimited(
        char('"'),
        many0(alt((
            value("\"", tag("\"\"")),
            take_while1(|character: char| character != '"'),
        ))),
        char('"'),
    )
    .parse(input)?;

    let mut value = String::new();
    for part in parts {
        value.push_str(part);
    }

    Ok((input, value))
}

fn unquoted_identifier(input: &str) -> IResult<&str, String, VerboseError<&str>> {
    let (input, value) = nom::combinator::recognize(pair(
        take_while1(|character: char| character.is_ascii_alphabetic() || character == '_'),
        take_while(|character: char| {
            character.is_ascii_alphanumeric() || character == '_' || character == '$'
        }),
    ))
    .parse(input)?;

    Ok((input, value.to_ascii_lowercase()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_concurrently_none() {
        assert_eq!(
            parse_concurrently("none").unwrap(),
            ConcurrentlyConfig::None
        );
    }

    #[test]
    fn parse_concurrently_except_quoted_with_comma() {
        let mut tables = BTreeSet::new();
        tables.insert("foo,bar".parse::<Table>().unwrap());
        tables.insert("baz".parse::<Table>().unwrap());

        assert_eq!(
            parse_concurrently("except:\"foo,bar\", baz").unwrap(),
            ConcurrentlyConfig::Except(tables)
        );
    }

    #[test]
    fn parse_concurrently_except_quoted_with_escape() {
        let mut tables = BTreeSet::new();
        tables.insert("foo\"bar".parse::<Table>().unwrap());

        assert_eq!(
            parse_concurrently("except:\"foo\"\"bar\"").unwrap(),
            ConcurrentlyConfig::Except(tables)
        );
    }

    #[test]
    fn parse_concurrently_except_empty_fails() {
        assert!(parse_concurrently("except:").is_err());
    }

    #[test]
    fn parse_concurrently_except_unquoted() {
        let mut tables = BTreeSet::new();
        tables.insert("events_2024".parse::<Table>().unwrap());
        tables.insert("foobar".parse::<Table>().unwrap());

        assert_eq!(
            parse_concurrently("except:events_2024, FooBar").unwrap(),
            ConcurrentlyConfig::Except(tables)
        );
    }

    #[test]
    fn parse_concurrently_except_duplicate_fails() {
        assert!(parse_concurrently("except:events_2024, events_2024").is_err());
    }
}

/// Garbage collect incomplete partitioned index creation state.
#[derive(Debug, clap::Args)]
pub struct Gc {
    /// Index name for the parent index.
    #[arg(long)]
    index: Index,
    /// Schema name.
    #[arg(long, default_value = "public")]
    schema: Schema,
    /// Number of parallel workers for partition index deletion.
    #[arg(long, default_value = "1")]
    jobs: NonZeroU16,
    /// Print SQL statements without executing them.
    #[arg(long)]
    dry_run: bool,
}

impl Gc {
    /// Run the garbage collection using the provided database configuration.
    pub async fn run(self, config: &crate::Config) -> Result<super::gc::Result, Error> {
        let input = super::gc::Input {
            schema: self.schema,
            index: self.index,
        };

        super::gc::run(config, &input, self.jobs, self.dry_run).await
    }
}
