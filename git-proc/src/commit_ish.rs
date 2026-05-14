//! Commit-ish target type and reusable typestate markers shared by command
//! builders that take a "thing that names a tree state" — `git checkout`,
//! `git reset`, etc.

use std::ffi::OsStr;

use crate::branch::Branch;
use crate::commit_id::CommitId;
use crate::tag::Tag;

/// Anything that names a git commit-ish: a branch tip, a tag, or a specific
/// commit object ID.
///
/// This is a typed subset of git's full commit-ish concept — only validated
/// newtypes from this crate are accepted, no free-form revspecs.
#[derive(Clone, Copy, Debug)]
pub enum CommitIsh<'a> {
    Branch(&'a Branch),
    Tag(&'a Tag),
    Commit(&'a CommitId),
}

impl AsRef<OsStr> for CommitIsh<'_> {
    fn as_ref(&self) -> &OsStr {
        match self {
            Self::Branch(branch) => branch.as_ref(),
            Self::Tag(tag) => tag.as_ref(),
            Self::Commit(commit) => commit.as_ref(),
        }
    }
}

impl<'a> From<&'a Branch> for CommitIsh<'a> {
    fn from(value: &'a Branch) -> Self {
        Self::Branch(value)
    }
}

impl<'a> From<&'a Tag> for CommitIsh<'a> {
    fn from(value: &'a Tag) -> Self {
        Self::Tag(value)
    }
}

impl<'a> From<&'a CommitId> for CommitIsh<'a> {
    fn from(value: &'a CommitId) -> Self {
        Self::Commit(value)
    }
}

/// Initial typestate: no commit-ish target chosen yet.
#[derive(Debug)]
pub struct NoTarget;

/// Typestate after a target has been selected.
#[derive(Debug)]
pub struct WithTarget<'a> {
    pub(crate) target: CommitIsh<'a>,
}
