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

/// Parse the porcelain output of `git worktree list --porcelain` and return branches.
///
/// Example output:
/// ```text
/// worktree /path/to/bare
/// bare
///
/// worktree /path/to/worktree
/// HEAD abc1234
/// branch refs/heads/main
///
/// ```
pub fn parse_worktree_list(input: &str) -> Vec<crate::Branch> {
    let Ok((_, entries)) = many0(parse_worktree_entry).parse(input) else {
        return Vec::new();
    };

    entries.into_iter().flatten().collect()
}

fn parse_worktree_entry(input: &str) -> IResult<&str, Option<crate::Branch>, VerboseError<&str>> {
    let (remaining, _) = context("worktree prefix", tag("worktree ")).parse(input)?;
    let (remaining, _) = terminated(
        take_while1(|character: char| character != '\n'),
        char('\n'),
    )
    .parse(remaining)?;

    let (remaining, _bare) = opt(terminated(tag("bare"), char('\n'))).parse(remaining)?;

    if _bare.is_some() {
        let (remaining, _) = opt(char('\n')).parse(remaining)?;
        return Ok((remaining, None));
    }

    let (remaining, _) = context("HEAD line", preceded(tag("HEAD "), take_while1(|character: char| character != '\n'))).parse(remaining)?;
    let (remaining, _) = char('\n').parse(remaining)?;

    let (remaining, branch_str) = context(
        "branch line",
        preceded(tag("branch refs/heads/"), take_while1(|character: char| character != '\n')),
    )
    .parse(remaining)?;
    let (remaining, _) = char('\n').parse(remaining)?;

    let (remaining, _) = opt(char('\n')).parse(remaining)?;

    let branch = branch_str.parse().map_err(|_| {
        nom::Err::Error(VerboseError {
            errors: vec![(
                branch_str,
                nom_language::error::VerboseErrorKind::Context("invalid branch name"),
            )],
        })
    })?;

    Ok((remaining, Some(branch)))
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
        let input = "worktree /path/to/bare\nbare\n\nworktree /path/to/main\nHEAD abc123\nbranch refs/heads/main\n\nworktree /path/to/feature\nHEAD def456\nbranch refs/heads/feature\n\n";
        let result = parse_worktree_list(input);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].as_str(), "main");
        assert_eq!(result[1].as_str(), "feature");
    }

    #[test]
    fn test_parse_worktree_list_no_bare() {
        let input = "worktree /path/to/main\nHEAD abc123\nbranch refs/heads/main\n\n";
        let result = parse_worktree_list(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_str(), "main");
    }

    #[test]
    fn test_parse_worktree_list_nested_branch() {
        let input = "worktree /path/to/bare\nbare\n\nworktree /path/to/feature/login\nHEAD abc123\nbranch refs/heads/feature/login\n\n";
        let result = parse_worktree_list(input);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].as_str(), "feature/login");
    }
}
