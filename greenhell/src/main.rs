use clap::error::ErrorKind;
use futures_util::StreamExt;
use greenhell::github::{Branch, PullRequestNumber, Ref, Repository};

#[derive(Debug, Eq, PartialEq, clap::Parser)]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Eq, PartialEq, clap::Parser)]
enum Command {
    /// Discover and print the GitHub token using CLI resolution order
    ///
    /// Checks sources in order:
    /// 1. GH_TOKEN environment variable
    /// 2. GITHUB_TOKEN environment variable
    /// 3. `gh auth token` command
    ///
    /// Prints the token and its source to stdout. Useful for debugging
    /// authentication issues or piping to other commands.
    ///
    /// This command uses CLI token discovery only. For GitHub App
    /// authentication (used in hosted deployments), different mechanisms apply.
    #[command(name = "cli-token")]
    CliToken,
    /// Evaluate status checks for a single branch or pull request
    Evaluate {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,

        #[clap(flatten)]
        target: EvaluateTarget,

        /// Print actions without executing
        #[clap(long)]
        dry_run: bool,
    },
    /// Evaluate status checks for all open pull requests
    EvaluateAll {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,

        /// Print actions without executing
        #[clap(long)]
        dry_run: bool,
    },
    /// Direct GitHub API operations
    #[command(name = "github")]
    GitHub {
        #[clap(subcommand)]
        command: greenhell::cli::GitHub,
    },
    /// Watch repository events and print them
    #[command(name = "watch-events")]
    WatchEvents {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,
    },
    /// Watch for PR activity and continuously evaluate
    Watch {
        /// Target repository (owner/repo)
        #[clap(long)]
        repository: Repository,

        /// Poll interval for active PRs in seconds
        #[clap(long, default_value = "20")]
        poll_interval: u64,

        /// How long a PR stays active after event activity in seconds
        #[clap(long, default_value = "300")]
        active_timeout: u64,

        /// Print actions without executing
        #[clap(long)]
        dry_run: bool,
    },
    /// Push commits individually to trigger CI builds for each
    ///
    /// GitHub Actions only builds the HEAD commit when multiple commits are pushed.
    /// This command identifies commits without check runs and pushes them individually.
    ///
    /// By default, uses the upstream tracking branch as the base. If no upstream is
    /// configured, you must provide --base explicitly.
    #[command(name = "push-each")]
    PushEach {
        /// Base ref to compare against (default: upstream tracking branch)
        #[clap(long)]
        base: Option<Ref>,

        /// Git remote name
        #[clap(long, default_value = "origin")]
        remote: String,

        /// Print actions without executing
        #[clap(long)]
        dry_run: bool,
    },
}

/// Target for evaluation: either a branch or a pull request number.
#[derive(Debug, Eq, PartialEq)]
enum EvaluateTarget {
    Branch(Branch),
    PullRequest(PullRequestNumber),
}

impl clap::FromArgMatches for EvaluateTarget {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let branch = matches.get_one::<Branch>("branch");
        let pull_request = matches.get_one::<PullRequestNumber>("pull_request");

        match (branch, pull_request) {
            (Some(branch), None) => Ok(Self::Branch(branch.clone())),
            (None, Some(pull_request)) => Ok(Self::PullRequest(*pull_request)),
            (Some(_), Some(_)) => Err(clap::Error::raw(
                ErrorKind::ArgumentConflict,
                "--branch and --pull-request are mutually exclusive\n",
            )),
            (None, None) => Err(clap::Error::raw(
                ErrorKind::MissingRequiredArgument,
                "either --branch or --pull-request is required\n",
            )),
        }
    }

    fn update_from_arg_matches(&mut self, matches: &clap::ArgMatches) -> Result<(), clap::Error> {
        *self = Self::from_arg_matches(matches)?;
        Ok(())
    }
}

impl clap::Args for EvaluateTarget {
    fn augment_args(command: clap::Command) -> clap::Command {
        command
            .arg(
                clap::Arg::new("branch")
                    .long("branch")
                    .value_name("NAME")
                    .help("Branch to evaluate")
                    .value_parser(clap::value_parser!(Branch)),
            )
            .arg(
                clap::Arg::new("pull_request")
                    .long("pull-request")
                    .value_name("NUMBER")
                    .help("Pull request number to evaluate")
                    .value_parser(clap::value_parser!(PullRequestNumber)),
            )
            .group(
                clap::ArgGroup::new("target")
                    .args(["branch", "pull_request"])
                    .required(true),
            )
    }

    fn augment_args_for_update(command: clap::Command) -> clap::Command {
        Self::augment_args(command)
    }
}

impl App {
    async fn run(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        match &self.command {
            Command::CliToken => match greenhell::cli_token::discover() {
                Ok(discovery) => {
                    log::info!("Token source: {}", discovery.source);
                    println!("{}", discovery.token.as_str());
                }
                Err(error) => {
                    log::error!("{error}");
                    std::process::exit(1);
                }
            },
            Command::Evaluate {
                repository,
                target,
                dry_run,
            } => {
                let target_display = match target {
                    EvaluateTarget::Branch(branch) => format!("branch {branch}"),
                    EvaluateTarget::PullRequest(pull_request) => {
                        format!("pull request #{pull_request}")
                    }
                };

                log::info!("Evaluating {target_display} on repository {repository}");

                let token = greenhell::cli_token::discover()?;
                let mut client = greenhell::github::Client::new(token.token);

                let (result, _) = match target {
                    EvaluateTarget::PullRequest(pull_request) => {
                        greenhell::evaluate::evaluate_pull_request(
                            &mut client,
                            repository,
                            *pull_request,
                        )
                        .await?
                    }
                    EvaluateTarget::Branch(branch) => {
                        greenhell::evaluate::evaluate_branch(&mut client, repository, branch)
                            .await?
                    }
                };

                log::info!("Evaluation result: {:?}", result.status);

                let requests = greenhell::evaluate::build_commit_statuses(repository, &result);

                if *dry_run {
                    log::info!("[dry-run] Would execute {} requests:", requests.len());
                    for request in &requests {
                        log::info!("[dry-run] {request:#?}");
                    }
                } else {
                    greenhell::evaluate::execute_commit_statuses(&mut client, requests).await?;
                    log::info!(
                        "Created commit statuses on {} commits",
                        result.commits.len()
                    );
                }
            }
            Command::EvaluateAll {
                repository,
                dry_run,
            } => {
                if *dry_run {
                    log::info!(
                        "[dry-run] Would evaluate all open pull requests on repository {repository}"
                    );
                } else {
                    log::info!("Evaluating all open pull requests on repository {repository}");
                }
            }
            Command::GitHub { command } => {
                let token = greenhell::cli_token::discover()?;
                let mut client = greenhell::github::Client::new(token.token);
                command.run(&mut client).await?;
            }
            Command::WatchEvents { repository } => {
                log::info!("Watching events on repository {repository}");

                let token = greenhell::cli_token::discover()?;
                let client = greenhell::github::Client::new(token.token);

                let mut stream = std::pin::pin!(greenhell::events::stream_new_events(
                    client,
                    repository.clone()
                ));

                while let Some(result) = stream.next().await {
                    match result {
                        Ok(event) => {
                            println!("{}: {}", event.id, event.event_type);
                        }
                        Err(error) => {
                            log::error!("Event stream error: {error}");
                            break;
                        }
                    }
                }

                log::warn!("Event stream desynced, full evaluation required");
            }
            Command::Watch {
                repository,
                poll_interval,
                active_timeout,
                dry_run,
            } => {
                let token = greenhell::cli_token::discover()?;
                let client = greenhell::github::Client::new(token.token);

                let config = greenhell::watch::Config {
                    poll_interval: std::time::Duration::from_secs(*poll_interval),
                    active_timeout: std::time::Duration::from_secs(*active_timeout),
                };

                greenhell::watch::run(client, repository.clone(), config, *dry_run).await?;
            }
            Command::PushEach {
                base,
                remote,
                dry_run,
            } => {
                use greenhell::github::git;
                use tower_service::Service;

                let repository = git::get_github_repository(remote)?;
                let branch = git::get_current_branch()?;

                let base_ref = match base {
                    Some(base) => base.clone(),
                    None => git::get_upstream()?
                        .ok_or("No upstream branch detected. Use --base to specify a base ref.")?,
                };

                log::info!("Checking commits from {base_ref} to HEAD on {repository}/{branch}");

                let commits = git::list_commits(&base_ref)?;

                if commits.is_empty() {
                    log::info!("No commits to push");
                    return Ok(());
                }

                log::info!("Found {} commits", commits.len());

                let mut client = greenhell::github::Client::discover()?;

                let mut remaining_unpushed = false;

                for sha in &commits {
                    if remaining_unpushed {
                        if *dry_run {
                            log::info!("  {} - [dry-run] would push", sha.abbrev());
                        } else {
                            log::info!("  {} - pushing", sha.abbrev());
                            git::force_push_commit(remote, sha, &branch)?;
                        }
                        continue;
                    }

                    let request =
                        greenhell::github::TryListCheckRuns(greenhell::github::ListCheckRuns {
                            repository: repository.clone(),
                            git_ref: sha.into(),
                        });

                    let check_runs = match client.call(request).await? {
                        Some(list) => list,
                        None => {
                            remaining_unpushed = true;
                            if *dry_run {
                                log::info!("  {} - [dry-run] would push", sha.abbrev());
                            } else {
                                log::info!("  {} - pushing", sha.abbrev());
                                git::force_push_commit(remote, sha, &branch)?;
                            }
                            continue;
                        }
                    };

                    if !check_runs.check_runs.is_empty() {
                        log::info!("  {} - has check runs, skipping", sha.abbrev());
                        continue;
                    }

                    if *dry_run {
                        log::info!("  {} - [dry-run] would push", sha.abbrev());
                    } else {
                        log::info!("  {} - pushing", sha.abbrev());
                        git::force_push_commit(remote, sha, &branch)?;
                    }
                }
            }
        }
        Ok(())
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let app = <App as clap::Parser>::parse();
    if let Err(error) = app.run().await {
        log::error!("{error}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    fn parse(args: &[&str]) -> Result<App, clap::Error> {
        App::try_parse_from(std::iter::once("greenhell").chain(args.iter().copied()))
    }

    mod evaluate {
        use super::*;

        #[test]
        fn with_branch() {
            assert_eq!(
                parse(&[
                    "evaluate",
                    "--repository",
                    "owner/repo",
                    "--branch",
                    "feature/test",
                ])
                .unwrap(),
                App {
                    command: Command::Evaluate {
                        repository: "owner/repo".parse().unwrap(),
                        target: EvaluateTarget::Branch("feature/test".parse().unwrap()),
                        dry_run: false,
                    },
                }
            );
        }

        #[test]
        fn with_pull_request() {
            assert_eq!(
                parse(&[
                    "evaluate",
                    "--repository",
                    "owner/repo",
                    "--pull-request",
                    "123",
                ])
                .unwrap(),
                App {
                    command: Command::Evaluate {
                        repository: "owner/repo".parse().unwrap(),
                        target: EvaluateTarget::PullRequest("123".parse().unwrap()),
                        dry_run: false,
                    },
                }
            );
        }

        #[test]
        fn with_dry_run() {
            assert_eq!(
                parse(&[
                    "evaluate",
                    "--repository",
                    "owner/repo",
                    "--branch",
                    "main",
                    "--dry-run",
                ])
                .unwrap(),
                App {
                    command: Command::Evaluate {
                        repository: "owner/repo".parse().unwrap(),
                        target: EvaluateTarget::Branch("main".parse().unwrap()),
                        dry_run: true,
                    },
                }
            );
        }

        #[test]
        fn rejects_both_branch_and_pull_request() {
            let result = parse(&[
                "evaluate",
                "--repository",
                "owner/repo",
                "--branch",
                "main",
                "--pull-request",
                "123",
            ]);

            assert!(result.is_err());
            assert_eq!(result.unwrap_err().kind(), ErrorKind::ArgumentConflict);
        }

        #[test]
        fn rejects_neither_branch_nor_pull_request() {
            let result = parse(&["evaluate", "--repository", "owner/repo"]);

            assert!(result.is_err());
            assert_eq!(
                result.unwrap_err().kind(),
                ErrorKind::MissingRequiredArgument
            );
        }

        #[test]
        fn rejects_missing_repository() {
            assert!(parse(&["evaluate", "--branch", "main"]).is_err());
        }

        #[test]
        fn rejects_invalid_repository() {
            assert!(parse(&["evaluate", "--repository", "invalid", "--branch", "main"]).is_err());
        }

        #[test]
        fn rejects_invalid_branch() {
            assert!(
                parse(&[
                    "evaluate",
                    "--repository",
                    "owner/repo",
                    "--branch",
                    ".invalid",
                ])
                .is_err()
            );
        }

        #[test]
        fn rejects_invalid_pull_request() {
            assert!(
                parse(&[
                    "evaluate",
                    "--repository",
                    "owner/repo",
                    "--pull-request",
                    "abc",
                ])
                .is_err()
            );
        }
    }

    mod evaluate_all {
        use super::*;

        #[test]
        fn basic() {
            assert_eq!(
                parse(&["evaluate-all", "--repository", "owner/repo"]).unwrap(),
                App {
                    command: Command::EvaluateAll {
                        repository: "owner/repo".parse().unwrap(),
                        dry_run: false,
                    },
                }
            );
        }

        #[test]
        fn with_dry_run() {
            assert_eq!(
                parse(&["evaluate-all", "--repository", "owner/repo", "--dry-run"]).unwrap(),
                App {
                    command: Command::EvaluateAll {
                        repository: "owner/repo".parse().unwrap(),
                        dry_run: true,
                    },
                }
            );
        }

        #[test]
        fn rejects_missing_repository() {
            assert!(parse(&["evaluate-all"]).is_err());
        }

        #[test]
        fn rejects_invalid_repository() {
            assert!(parse(&["evaluate-all", "--repository", "invalid"]).is_err());
        }
    }

    #[test]
    fn cli_token() {
        assert_eq!(
            parse(&["cli-token"]).unwrap(),
            App {
                command: Command::CliToken,
            }
        );
    }
}
