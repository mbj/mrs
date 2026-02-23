//! Watch mode for continuous PR evaluation.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

use chrono::{DateTime, Utc};
use futures_util::StreamExt;

use crate::evaluate;
use crate::events;
use crate::github::{Client, PullRequestNumber, Repository};

/// Configuration for watch mode.
pub struct Config {
    /// How often to poll active PRs.
    pub poll_interval: Duration,
    /// How long a PR stays active after event activity.
    pub active_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_secs(20),
            active_timeout: Duration::from_secs(300),
        }
    }
}

/// Error during watch mode.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// GitHub API error.
    #[error("GitHub API error: {0}")]
    GitHub(#[from] crate::github::Error),
    /// Events stream error.
    #[error("Events error: {0}")]
    Events(#[from] events::Error),
    /// Evaluation error.
    #[error("Evaluation error: {0}")]
    Evaluate(#[from] evaluate::Error),
}

/// Tracks active PRs.
struct ActivePrs {
    /// Map from PR number to when it was last active.
    last_active: BTreeMap<PullRequestNumber, Instant>,
}

impl ActivePrs {
    fn new() -> Self {
        Self {
            last_active: BTreeMap::new(),
        }
    }

    /// Remove PRs that have timed out.
    fn prune(&mut self, active_timeout: Duration) {
        let now = Instant::now();
        self.last_active
            .retain(|_, last_active| now.duration_since(*last_active) <= active_timeout);
    }
}

/// Extract PR numbers from an event.
///
/// For PushEvent, resolves the branch to PRs via API call.
async fn extract_prs_from_event(
    client: &mut Client,
    repository: &Repository,
    event: &crate::github::Event,
) -> Vec<PullRequestNumber> {
    // Direct PR reference in payload
    if let Some(pr_number) = event
        .payload
        .get("pull_request")
        .and_then(|pr| pr.get("number"))
        .and_then(|n| n.as_u64())
    {
        return vec![pr_number.into()];
    }

    // PullRequestEvent has number at top level
    if event.event_type == "PullRequestEvent"
        && let Some(pr_number) = event.payload.get("number").and_then(|n| n.as_u64())
    {
        return vec![pr_number.into()];
    }

    // PushEvent: resolve branch to PRs
    if event.event_type == "PushEvent"
        && let Some(git_ref) = event.payload.get("ref").and_then(|r| r.as_str())
    {
        // ref is like "refs/heads/branch-name"
        if let Some(branch_name) = git_ref.strip_prefix("refs/heads/")
            && let Ok(branch) = branch_name.parse::<crate::github::Branch>()
        {
            let stream = typed_reqwest::link::paginate(
                client,
                crate::github::ListPullRequestsByHead {
                    repository: repository.clone(),
                    branch,
                },
            );

            let mut stream = std::pin::pin!(stream);
            let mut prs = Vec::new();

            while let Some(result) = stream.next().await {
                match result {
                    Ok(page) => {
                        prs.extend(page.iter().map(|pr| PullRequestNumber::from(pr.number)));
                    }
                    Err(error) => {
                        log::error!("Failed to resolve branch to PRs: {error}");
                        break;
                    }
                }
            }

            return prs;
        }
    }

    Vec::new()
}

/// Evaluate a single PR and create commit statuses.
async fn evaluate_pr(
    client: &mut Client,
    repository: &Repository,
    pr: PullRequestNumber,
    dry_run: bool,
) -> Result<(), evaluate::Error> {
    let (result, _) = evaluate::evaluate_pull_request(client, repository, pr).await?;
    log::info!("PR #{pr} evaluation: {:?}", result.status);

    let requests = evaluate::build_commit_statuses(repository, &result);

    if dry_run {
        log::info!("[dry-run] Would create {} commit statuses", requests.len());
    } else {
        evaluate::execute_commit_statuses(client, requests).await?;
    }

    Ok(())
}

/// Run watch mode.
pub async fn run(
    client: Client,
    repository: Repository,
    config: Config,
    dry_run: bool,
) -> Result<(), Error> {
    let mut client = client;

    // Initial evaluation of all open PRs
    log::info!("Running initial evaluation of all open PRs");
    let open_prs = evaluate::list_open_prs(&mut client, &repository).await?;
    log::info!("Found {} open PRs", open_prs.len());

    let results = futures_util::future::join_all(open_prs.iter().map(|pr| {
        let mut client = client.clone();
        let pr = *pr;
        let repository = repository.clone();
        async move {
            log::info!("Evaluating PR #{pr}");
            (pr, evaluate_pr(&mut client, &repository, pr, dry_run).await)
        }
    }))
    .await;

    for (pr, result) in results {
        if let Err(error) = result {
            log::error!("Failed to evaluate PR #{pr}: {error}");
        }
    }

    // Start watching events
    log::info!("Starting event watch");
    let mut active_prs = ActivePrs::new();
    let mut events_stream = std::pin::pin!(events::stream_new_events(
        client.clone(),
        repository.clone()
    ));

    let mut poll_ticker = tokio::time::interval(config.poll_interval);

    loop {
        tokio::select! {
            // Check for new events
            event_result = events_stream.next() => {
                match event_result {
                    Some(Ok(event)) => {
                        log::debug!("Event: {} {}", event.id, event.event_type);
                        let event_time: DateTime<Utc> = event.created_at.parse().unwrap();
                        let cutoff = Utc::now() - config.active_timeout;

                        if event_time >= cutoff {
                            let prs = extract_prs_from_event(&mut client, &repository, &event).await;
                            for pr in prs {
                                log::info!("PR #{pr} activity detected from event");
                                active_prs.last_active.insert(pr, Instant::now());
                                evaluate_pr(&mut client, &repository, pr, dry_run).await?;
                            }
                        }
                    }
                    Some(Err(error)) => {
                        log::error!("Event stream error: {error}");
                    }
                    None => {
                        log::warn!("Event stream desynced, restarting with full evaluation");
                        break;
                    }
                }
            }
            // Poll active PRs
            _ = poll_ticker.tick() => {
                active_prs.prune(config.active_timeout);
                let active: String = active_prs.last_active.keys().map(|pr| pr.to_string()).collect::<Vec<_>>().join(", ");
                log::info!("Active PRs: [{active}]");

                for pr in active_prs.last_active.keys().copied().collect::<Vec<_>>() {
                    log::info!("Evaluating active PR #{pr}");

                    if let Err(error) = evaluate_pr(&mut client, &repository, pr, dry_run).await {
                        log::error!("Failed to evaluate PR #{pr}: {error}");
                    }
                }
            }
        }
    }

    Ok(())
}
