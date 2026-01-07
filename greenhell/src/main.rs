use clap::error::ErrorKind;
use greenhell::github::{Branch, PullRequest, Repository};

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
}

/// Target for evaluation: either a branch or a pull request number.
#[derive(Debug, Eq, PartialEq)]
enum EvaluateTarget {
    Branch(Branch),
    PullRequest(PullRequest),
}

impl clap::FromArgMatches for EvaluateTarget {
    fn from_arg_matches(matches: &clap::ArgMatches) -> Result<Self, clap::Error> {
        let branch = matches.get_one::<Branch>("branch");
        let pull_request = matches.get_one::<PullRequest>("pull_request");

        match (branch, pull_request) {
            (Some(branch), None) => Ok(Self::Branch(branch.clone())),
            (None, Some(pull_request)) => Ok(Self::PullRequest(pull_request.clone())),
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
                    .value_parser(clap::value_parser!(PullRequest)),
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

                let (result, shas) = match target {
                    EvaluateTarget::PullRequest(pull_request) => {
                        greenhell::evaluate::evaluate_pull_request(
                            &mut client,
                            repository,
                            pull_request,
                        )
                        .await?
                    }
                    EvaluateTarget::Branch(branch) => {
                        greenhell::evaluate::evaluate_branch(&mut client, repository, branch)
                            .await?
                    }
                };

                log::info!("Evaluation result: {:?}", result.status);

                let requests =
                    greenhell::evaluate::build_commit_statuses(repository, &shas, &result);

                if *dry_run {
                    log::info!("[dry-run] Would execute {} requests:", requests.len());
                    for request in &requests {
                        log::info!("[dry-run] {request:#?}");
                    }
                } else {
                    greenhell::evaluate::execute_commit_statuses(&mut client, requests).await?;
                    log::info!("Created commit statuses on {} commits", shas.len());
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
