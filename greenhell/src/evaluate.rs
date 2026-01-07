//! Evaluation logic for greenhell status checks.

use std::sync::LazyLock;

use futures_util::StreamExt;
use tower_service::Service;

use crate::github::{
    Branch, CheckName, CheckRun, CheckRunConclusion, CheckRunStatus, Client, CombinedStatus,
    CombinedStatusState, CommitStatusState, CompareCommits, CreateCommitStatus, GetCombinedStatus,
    GetRepository, ListCheckRuns, ListPullRequestCommits, PullRequest, Repository,
};

/// The greenhell check name.
pub static GREENHELL: LazyLock<CheckName> = LazyLock::new(|| CheckName::new("greenhell"));

/// Our evaluation status with additional states beyond GitHub's.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EvaluationStatus {
    /// All checks passed.
    Success,
    /// At least one check is still pending.
    Pending,
    /// At least one check failed.
    Failure,
    /// No checks configured.
    NoChecks,
}

impl EvaluationStatus {
    /// Project to GitHub commit status.
    #[must_use]
    pub fn to_commit_status(self) -> CommitStatus {
        match self {
            Self::Success => CommitStatus::Success,
            Self::Pending => CommitStatus::Pending,
            Self::Failure => CommitStatus::Failure,
            Self::NoChecks => CommitStatus::Failure,
        }
    }
}

/// GitHub commit status.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CommitStatus {
    /// All checks passed.
    Success,
    /// At least one check is still pending.
    Pending,
    /// At least one check failed.
    Failure,
}

/// Result of evaluating a commit.
#[derive(Debug, Clone, PartialEq)]
pub struct CommitResult {
    /// The commit SHA.
    pub sha: String,
    /// The aggregated status.
    pub status: EvaluationStatus,
}

/// Result of evaluating all commits.
#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationResult {
    /// Results for each commit.
    pub commits: Vec<CommitResult>,
    /// Overall status.
    pub status: EvaluationStatus,
}

impl EvaluationResult {
    /// Compute the overall status from commit results.
    #[must_use]
    pub fn from_commits(commits: Vec<CommitResult>) -> Self {
        let status = Self::compute_status(&commits);
        Self { commits, status }
    }

    fn compute_status(commits: &[CommitResult]) -> EvaluationStatus {
        let mut has_pending = false;
        let mut has_no_checks = false;

        for commit in commits {
            match commit.status {
                EvaluationStatus::Failure => return EvaluationStatus::Failure,
                EvaluationStatus::NoChecks => has_no_checks = true,
                EvaluationStatus::Pending => has_pending = true,
                EvaluationStatus::Success => {}
            }
        }

        if has_no_checks {
            EvaluationStatus::NoChecks
        } else if has_pending {
            EvaluationStatus::Pending
        } else {
            EvaluationStatus::Success
        }
    }
}

/// Evaluate a single commit's status from its check runs and combined status.
///
/// Filters out the greenhell check itself to avoid self-referential loops.
/// Returns NoChecks if there are no checks at all.
#[must_use]
pub fn evaluate_commit(
    sha: &str,
    check_runs: &[CheckRun],
    combined_status: &CombinedStatus,
) -> CommitResult {
    let check_run_status = evaluate_check_runs(check_runs);
    let commit_status_status = evaluate_combined_status(combined_status);

    let status = match (check_run_status, commit_status_status) {
        (EvaluationStatus::Failure, _) => EvaluationStatus::Failure,
        (_, EvaluationStatus::Failure) => EvaluationStatus::Failure,
        (EvaluationStatus::Pending, _) => EvaluationStatus::Pending,
        (_, EvaluationStatus::Pending) => EvaluationStatus::Pending,
        (EvaluationStatus::NoChecks, EvaluationStatus::NoChecks) => EvaluationStatus::NoChecks,
        (EvaluationStatus::NoChecks, EvaluationStatus::Success) => EvaluationStatus::Success,
        (EvaluationStatus::Success, EvaluationStatus::NoChecks) => EvaluationStatus::Success,
        (EvaluationStatus::Success, EvaluationStatus::Success) => EvaluationStatus::Success,
    };

    CommitResult {
        sha: sha.to_string(),
        status,
    }
}

/// Error during evaluation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// GitHub API error.
    #[error("GitHub API error: {0}")]
    GitHub(#[from] crate::github::Error),
    /// Pagination error.
    #[error("Pagination error: {0}")]
    Paginate(#[from] mhttp::link::PaginateError<crate::github::Error>),
}

/// Evaluate a list of commits by their SHAs.
///
/// Returns the evaluation result and the list of commit SHAs (in order).
pub async fn evaluate_shas(
    client: &mut Client,
    repository: &Repository,
    shas: Vec<String>,
) -> Result<(EvaluationResult, Vec<String>), Error> {
    // Fetch check-runs and combined status for all commits in parallel
    let commit_results = try_join_map(shas.iter(), |sha| {
        let sha = sha.clone();
        let repository = repository.clone();
        let mut client = client.clone();

        async move {
            let mut status_client = client.clone();

            let check_runs_stream = mhttp::link::paginate(
                &mut client,
                ListCheckRuns {
                    repository: repository.clone(),
                    git_ref: sha.parse().unwrap(),
                },
            );

            let check_runs_future = async {
                collect_pages(check_runs_stream.map(|result| result.map(|list| list.check_runs)))
                    .await
                    .map_err(Error::from)
            };

            let combined_status_future = async {
                status_client
                    .call(GetCombinedStatus {
                        repository,
                        git_ref: sha.parse().unwrap(),
                    })
                    .await
                    .map_err(Error::from)
            };

            let (check_runs, combined_status) =
                tokio::try_join!(check_runs_future, combined_status_future)?;

            Ok::<_, Error>(evaluate_commit(&sha, &check_runs, &combined_status))
        }
    })
    .await?;

    Ok((EvaluationResult::from_commits(commit_results), shas))
}

/// Evaluate all commits in a pull request.
///
/// Returns the evaluation result and the list of commit SHAs (in order).
pub async fn evaluate_pull_request(
    client: &mut Client,
    repository: &Repository,
    pull_request: &PullRequest,
) -> Result<(EvaluationResult, Vec<String>), Error> {
    let commits = collect_pages(mhttp::link::paginate(
        &mut *client,
        ListPullRequestCommits {
            repository: repository.clone(),
            pull_request: pull_request.clone(),
        },
    ))
    .await?;

    let shas: Vec<String> = commits.iter().map(|commit| commit.sha.clone()).collect();

    evaluate_shas(client, repository, shas).await
}

/// Evaluate a branch by comparing it to the default branch.
///
/// Fetches the repository's default branch, then compares the given branch
/// against it to find commits ahead. Evaluates all commits in the comparison.
///
/// Returns the evaluation result and the list of commit SHAs (in order).
pub async fn evaluate_branch(
    client: &mut Client,
    repository: &Repository,
    branch: &Branch,
) -> Result<(EvaluationResult, Vec<String>), Error> {
    // Get the default branch for this repository
    let repo_info = client
        .call(GetRepository {
            repository: repository.clone(),
        })
        .await?;

    let default_branch: Branch = repo_info.default_branch.parse().unwrap();

    // Compare the branch against the default branch (with pagination)
    let comparison_stream = mhttp::link::paginate(
        &mut *client,
        CompareCommits {
            repository: repository.clone(),
            base: default_branch.into(),
            head: branch.clone().into(),
        },
    );

    let commits =
        collect_pages(comparison_stream.map(|result| result.map(|comparison| comparison.commits)))
            .await?;

    let shas: Vec<String> = commits.iter().map(|commit| commit.sha.clone()).collect();

    evaluate_shas(client, repository, shas).await
}

async fn collect_pages<T, E>(
    stream: impl futures_util::Stream<Item = Result<Vec<T>, E>>,
) -> Result<Vec<T>, E> {
    futures_util::pin_mut!(stream);
    let mut items = Vec::new();
    while let Some(result) = stream.next().await {
        items.extend(result?);
    }
    Ok(items)
}

async fn try_join_map<I, T, F, Fut, R, E>(items: I, map: F) -> Result<Vec<R>, E>
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> Fut,
    Fut: std::future::Future<Output = Result<R, E>>,
{
    let futures: Vec<_> = items.into_iter().map(map).collect();
    futures_util::future::try_join_all(futures).await
}

/// Build commit status requests for all commits in the evaluation.
///
/// All commits except the last (head) are marked as success.
/// Only the head commit reflects the actual aggregated status.
#[must_use]
pub fn build_commit_statuses(
    repository: &Repository,
    shas: &[String],
    result: &EvaluationResult,
) -> Vec<CreateCommitStatus> {
    if shas.is_empty() {
        return Vec::new();
    }

    let mut requests = Vec::with_capacity(shas.len());

    // All commits except the last get marked as success
    for sha in &shas[..shas.len() - 1] {
        requests.push(CreateCommitStatus {
            repository: repository.clone(),
            sha: sha.clone(),
            state: CommitStatusState::Success,
            context: GREENHELL.clone(),
            description: Some("Intermediate commit".to_string()),
        });
    }

    // Head commit gets the actual result
    let head_sha = &shas[shas.len() - 1];

    let state = match result.status {
        EvaluationStatus::Success => CommitStatusState::Success,
        EvaluationStatus::Failure | EvaluationStatus::NoChecks => CommitStatusState::Failure,
        EvaluationStatus::Pending => CommitStatusState::Pending,
    };

    requests.push(CreateCommitStatus {
        repository: repository.clone(),
        sha: head_sha.clone(),
        state,
        context: GREENHELL.clone(),
        description: Some(format_title(result)),
    });

    requests
}

/// Execute commit status requests in parallel.
pub async fn execute_commit_statuses(
    client: &mut Client,
    requests: Vec<CreateCommitStatus>,
) -> Result<(), Error> {
    try_join_map(requests, |request| {
        let mut client = client.clone();
        async move { client.call(request).await.map_err(Error::from) }
    })
    .await?;
    Ok(())
}

fn format_title(result: &EvaluationResult) -> String {
    match result.status {
        EvaluationStatus::Success => "All checks on all commits passed".to_string(),
        EvaluationStatus::Failure => "Some checks failed".to_string(),
        EvaluationStatus::NoChecks => "No checks configured".to_string(),
        EvaluationStatus::Pending => "Some checks pending".to_string(),
    }
}

fn evaluate_check_runs(check_runs: &[CheckRun]) -> EvaluationStatus {
    let mut has_pending = false;
    let mut has_checks = false;

    for check_run in check_runs {
        // Skip our own check to avoid self-referential status
        if check_run.name.is_greenhell() {
            continue;
        }

        has_checks = true;

        match check_run.status {
            CheckRunStatus::Queued => {
                has_pending = true;
            }
            CheckRunStatus::InProgress => {
                has_pending = true;
            }
            CheckRunStatus::Completed => match check_run.conclusion {
                Some(CheckRunConclusion::Success) => {}
                Some(CheckRunConclusion::Neutral) => {}
                Some(CheckRunConclusion::Skipped) => {}
                Some(CheckRunConclusion::Failure) => {
                    return EvaluationStatus::Failure;
                }
                Some(CheckRunConclusion::Cancelled) => {
                    return EvaluationStatus::Failure;
                }
                Some(CheckRunConclusion::TimedOut) => {
                    return EvaluationStatus::Failure;
                }
                Some(CheckRunConclusion::ActionRequired) => {
                    return EvaluationStatus::Failure;
                }
                Some(CheckRunConclusion::Stale) => {
                    return EvaluationStatus::Failure;
                }
                None => {
                    // Completed without conclusion is unexpected, treat as pending
                    has_pending = true;
                }
            },
        }
    }

    if !has_checks {
        EvaluationStatus::NoChecks
    } else if has_pending {
        EvaluationStatus::Pending
    } else {
        EvaluationStatus::Success
    }
}

fn evaluate_combined_status(combined_status: &CombinedStatus) -> EvaluationStatus {
    let has_non_greenhell = combined_status
        .statuses
        .iter()
        .any(|status| !status.context.is_greenhell());

    if !has_non_greenhell {
        return EvaluationStatus::NoChecks;
    }

    match combined_status.state {
        CombinedStatusState::Success => EvaluationStatus::Success,
        CombinedStatusState::Pending => EvaluationStatus::Pending,
        CombinedStatusState::Failure => EvaluationStatus::Failure,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::CommitStatus as ApiCommitStatus;

    fn check_run(
        name: CheckName,
        status: CheckRunStatus,
        conclusion: Option<CheckRunConclusion>,
    ) -> CheckRun {
        CheckRun {
            id: 1,
            name,
            status,
            conclusion,
        }
    }

    fn combined_status(state: CombinedStatusState, contexts: Vec<CheckName>) -> CombinedStatus {
        CombinedStatus {
            state,
            total_count: contexts.len() as u64,
            statuses: contexts
                .into_iter()
                .map(|context| ApiCommitStatus {
                    id: 1,
                    state: "success".to_string(),
                    context,
                    description: None,
                    target_url: None,
                })
                .collect(),
            sha: "abc123".to_string(),
        }
    }

    mod evaluate_check_runs_tests {
        use super::*;

        #[test]
        fn all_success() {
            let check_runs = vec![
                check_run(
                    "build".into(),
                    CheckRunStatus::Completed,
                    Some(CheckRunConclusion::Success),
                ),
                check_run(
                    "test".into(),
                    CheckRunStatus::Completed,
                    Some(CheckRunConclusion::Success),
                ),
            ];
            assert_eq!(evaluate_check_runs(&check_runs), EvaluationStatus::Success);
        }

        #[test]
        fn one_pending() {
            let check_runs = vec![
                check_run(
                    "build".into(),
                    CheckRunStatus::Completed,
                    Some(CheckRunConclusion::Success),
                ),
                check_run("test".into(), CheckRunStatus::InProgress, None),
            ];
            assert_eq!(evaluate_check_runs(&check_runs), EvaluationStatus::Pending);
        }

        #[test]
        fn one_failure() {
            let check_runs = vec![
                check_run(
                    "build".into(),
                    CheckRunStatus::Completed,
                    Some(CheckRunConclusion::Success),
                ),
                check_run(
                    "test".into(),
                    CheckRunStatus::Completed,
                    Some(CheckRunConclusion::Failure),
                ),
            ];
            assert_eq!(evaluate_check_runs(&check_runs), EvaluationStatus::Failure);
        }

        #[test]
        fn skips_greenhell_check() {
            let check_runs = vec![
                check_run(
                    GREENHELL.clone(),
                    CheckRunStatus::Completed,
                    Some(CheckRunConclusion::Failure),
                ),
                check_run(
                    "build".into(),
                    CheckRunStatus::Completed,
                    Some(CheckRunConclusion::Success),
                ),
            ];
            assert_eq!(evaluate_check_runs(&check_runs), EvaluationStatus::Success);
        }

        #[test]
        fn neutral_is_success() {
            let check_runs = vec![check_run(
                "lint".into(),
                CheckRunStatus::Completed,
                Some(CheckRunConclusion::Neutral),
            )];
            assert_eq!(evaluate_check_runs(&check_runs), EvaluationStatus::Success);
        }

        #[test]
        fn skipped_is_success() {
            let check_runs = vec![check_run(
                "optional".into(),
                CheckRunStatus::Completed,
                Some(CheckRunConclusion::Skipped),
            )];
            assert_eq!(evaluate_check_runs(&check_runs), EvaluationStatus::Success);
        }

        #[test]
        fn empty_is_no_checks() {
            let check_runs: Vec<CheckRun> = vec![];
            assert_eq!(evaluate_check_runs(&check_runs), EvaluationStatus::NoChecks);
        }
    }

    mod evaluate_combined_status_tests {
        use super::*;

        #[test]
        fn success() {
            let status = combined_status(CombinedStatusState::Success, vec!["ci/build".into()]);
            assert_eq!(evaluate_combined_status(&status), EvaluationStatus::Success);
        }

        #[test]
        fn pending() {
            let status = combined_status(CombinedStatusState::Pending, vec!["ci/build".into()]);
            assert_eq!(evaluate_combined_status(&status), EvaluationStatus::Pending);
        }

        #[test]
        fn failure() {
            let status = combined_status(CombinedStatusState::Failure, vec!["ci/build".into()]);
            assert_eq!(evaluate_combined_status(&status), EvaluationStatus::Failure);
        }

        #[test]
        fn ignores_greenhell_only() {
            let status = combined_status(CombinedStatusState::Failure, vec![GREENHELL.clone()]);
            assert_eq!(
                evaluate_combined_status(&status),
                EvaluationStatus::NoChecks
            );
        }

        #[test]
        fn empty_statuses_is_no_checks() {
            let status = combined_status(CombinedStatusState::Pending, vec![]);
            assert_eq!(
                evaluate_combined_status(&status),
                EvaluationStatus::NoChecks
            );
        }
    }

    mod evaluation_result_tests {
        use super::*;

        #[test]
        fn all_success() {
            let commits = vec![
                CommitResult {
                    sha: "a".to_string(),
                    status: EvaluationStatus::Success,
                },
                CommitResult {
                    sha: "b".to_string(),
                    status: EvaluationStatus::Success,
                },
            ];
            let result = EvaluationResult::from_commits(commits);
            assert_eq!(result.status, EvaluationStatus::Success);
        }

        #[test]
        fn one_pending() {
            let commits = vec![
                CommitResult {
                    sha: "a".to_string(),
                    status: EvaluationStatus::Success,
                },
                CommitResult {
                    sha: "b".to_string(),
                    status: EvaluationStatus::Pending,
                },
            ];
            let result = EvaluationResult::from_commits(commits);
            assert_eq!(result.status, EvaluationStatus::Pending);
        }

        #[test]
        fn one_failure() {
            let commits = vec![
                CommitResult {
                    sha: "a".to_string(),
                    status: EvaluationStatus::Success,
                },
                CommitResult {
                    sha: "b".to_string(),
                    status: EvaluationStatus::Failure,
                },
            ];
            let result = EvaluationResult::from_commits(commits);
            assert_eq!(result.status, EvaluationStatus::Failure);
        }

        #[test]
        fn failure_takes_precedence_over_pending() {
            let commits = vec![
                CommitResult {
                    sha: "a".to_string(),
                    status: EvaluationStatus::Pending,
                },
                CommitResult {
                    sha: "b".to_string(),
                    status: EvaluationStatus::Failure,
                },
            ];
            let result = EvaluationResult::from_commits(commits);
            assert_eq!(result.status, EvaluationStatus::Failure);
        }

        #[test]
        fn no_checks_takes_precedence_over_pending() {
            let commits = vec![
                CommitResult {
                    sha: "a".to_string(),
                    status: EvaluationStatus::Pending,
                },
                CommitResult {
                    sha: "b".to_string(),
                    status: EvaluationStatus::NoChecks,
                },
            ];
            let result = EvaluationResult::from_commits(commits);
            assert_eq!(result.status, EvaluationStatus::NoChecks);
        }

        #[test]
        fn empty_is_success() {
            let commits: Vec<CommitResult> = vec![];
            let result = EvaluationResult::from_commits(commits);
            assert_eq!(result.status, EvaluationStatus::Success);
        }
    }
}
