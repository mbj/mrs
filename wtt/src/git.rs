use nom::{
    IResult, Parser,
    bytes::complete::{tag, take_till, take_while1},
    character::complete::char,
    combinator::opt,
    error::context,
    multi::many0,
    sequence::{preceded, terminated},
};
use nom_language::error::VerboseError;

use crate::Base;

/// Parse the output of `git ls-remote --symref origin HEAD` to extract the default branch.
///
/// Example output:
/// ```text
/// ref: refs/heads/main    HEAD
/// abc123...    HEAD
/// ```
///
/// Returns the branch name (e.g., "main") from the symref line.
pub fn parse_default_branch(input: &str) -> Result<Base, ParseError> {
    let (_, branch) = parse_symref_output(input).map_err(|_| ParseError::DefaultBranch)?;

    branch.ok_or(ParseError::DefaultBranch)
}

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Failed to parse default branch from git ls-remote output")]
    DefaultBranch,
}

fn parse_symref_output(input: &str) -> IResult<&str, Option<Base>, VerboseError<&str>> {
    let (remaining, lines) = many0(parse_line).parse(input)?;

    let branch = lines.into_iter().find_map(|line| match line {
        Line::SymRef(branch) => Some(branch),
        Line::Other => None,
    });

    Ok((remaining, branch))
}

enum Line {
    SymRef(Base),
    Other,
}

fn parse_line(input: &str) -> IResult<&str, Line, VerboseError<&str>> {
    let (remaining, line) =
        terminated(take_till(|character| character == '\n'), opt(char('\n'))).parse(input)?;

    if line.is_empty() && remaining.is_empty() {
        return Err(nom::Err::Error(VerboseError {
            errors: vec![(
                input,
                nom_language::error::VerboseErrorKind::Context("empty input"),
            )],
        }));
    }

    let result = parse_symref_line(line);

    match result {
        Ok((_, branch)) => Ok((remaining, Line::SymRef(branch))),
        Err(_) => Ok((remaining, Line::Other)),
    }
}

fn parse_symref_line(input: &str) -> IResult<&str, Base, VerboseError<&str>> {
    let (remaining, _) = context("symref prefix", tag("ref: refs/heads/")).parse(input)?;
    let (remaining, branch) = context(
        "branch name",
        take_while1(|character: char| character != '\t' && character != '\n'),
    )
    .parse(remaining)?;
    let (remaining, _) = context("tab separator", char('\t')).parse(remaining)?;
    let (remaining, _) = context("HEAD suffix", tag("HEAD")).parse(remaining)?;

    let base: Base = branch.parse().map_err(|_| {
        nom::Err::Error(VerboseError {
            errors: vec![(
                branch,
                nom_language::error::VerboseErrorKind::Context("invalid branch name"),
            )],
        })
    })?;

    Ok((remaining, base))
}

/// Parse the output of `git worktree list` and filter out bare repository entries.
///
/// Example output:
/// ```text
/// /path/to/bare  (bare)
/// /path/to/worktree  abc1234 [branch]
/// ```
pub fn parse_worktree_list(input: &str) -> Vec<&str> {
    input
        .lines()
        .filter(|line| !is_bare_worktree(line))
        .collect()
}

fn is_bare_worktree(input: &str) -> bool {
    parse_bare_marker(input).is_ok()
}

fn parse_bare_marker(input: &str) -> IResult<&str, (), VerboseError<&str>> {
    let (remaining, _) = take_till(|character| character == '(').parse(input)?;
    let (remaining, _) = preceded(char('('), tag("bare")).parse(remaining)?;

    Ok((remaining, ()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_default_branch_main() {
        let input = "ref: refs/heads/main\tHEAD\nabc123def456\tHEAD\n";
        let result = parse_default_branch(input).unwrap();
        assert_eq!(result.as_str(), "main");
    }

    #[test]
    fn test_parse_default_branch_master() {
        let input = "ref: refs/heads/master\tHEAD\nabc123def456\tHEAD\n";
        let result = parse_default_branch(input).unwrap();
        assert_eq!(result.as_str(), "master");
    }

    #[test]
    fn test_parse_default_branch_no_symref() {
        let input = "abc123def456\tHEAD\n";
        let result = parse_default_branch(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_worktree_list_filters_bare() {
        let input = "/path/to/bare  (bare)\n/path/to/main  abc123 [main]\n/path/to/feature  def456 [feature]\n";
        let result = parse_worktree_list(input);
        assert_eq!(result.len(), 2);
        assert!(result[0].contains("/path/to/main"));
        assert!(result[1].contains("/path/to/feature"));
    }

    #[test]
    fn test_parse_worktree_list_no_bare() {
        let input = "/path/to/main  abc123 [main]\n";
        let result = parse_worktree_list(input);
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_is_bare_worktree() {
        assert!(is_bare_worktree("/path/to/repo  (bare)"));
        assert!(!is_bare_worktree("/path/to/worktree  abc123 [main]"));
    }
}
