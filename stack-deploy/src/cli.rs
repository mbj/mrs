use crate::instance_spec::{Registry, TemplateUploader};

#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

impl App {
    pub async fn run(&self, config: &Config<'_>) {
        self.command.run(config).await
    }
}

pub struct Config<'a> {
    pub cloudformation: &'a aws_sdk_cloudformation::client::Client,
    pub cloudwatchlogs: &'a aws_sdk_cloudwatchlogs::Client,
    pub registry: &'a Registry,
    pub template_uploader: Option<&'a TemplateUploader<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    Instance(Box<crate::cli::instance::App>),
    Logs {
        #[clap(subcommand)]
        command: crate::logs::cli::Command,
    },
}

impl Command {
    pub async fn run(&self, config: &Config<'_>) {
        match self {
            Self::Instance(command) => command.run(config).await,
            Self::Logs { command } => {
                let logs_config = crate::logs::cli::Config {
                    client: config.cloudwatchlogs,
                };
                command.run(&logs_config).await
            }
        }
    }
}

mod instance {
    use crate::instance_spec::ReviewChangeSet;
    use crate::types::*;

    #[derive(Clone, Debug, Eq, PartialEq, clap::Parser)]
    pub struct App {
        #[clap(subcommand)]
        command: Command,
    }

    impl App {
        pub async fn run(&self, config: &super::Config<'_>) {
            self.command.run(config).await
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq, clap::Parser)]
    pub enum ChangeSetCommand {
        Create {
            #[arg(long)]
            change_set_name: ChangeSetName,
            #[arg(long = "parameter")]
            parameters: Vec<Parameter>,
        },
        Delete {
            #[arg(long)]
            change_set_name: ChangeSetName,
        },
        Describe {
            #[arg(long)]
            change_set_name: ChangeSetName,
        },
        List,
    }

    #[derive(Clone, Debug, Eq, PartialEq, clap::Parser)]
    pub enum Command {
        ChangeSet {
            #[arg(long = "stack-name")]
            name: StackName,
            #[clap(subcommand)]
            command: ChangeSetCommand,
        },
        /// Delete stack instance
        Delete {
            #[arg(long = "stack-name")]
            name: StackName,
        },
        /// List registered stack instances
        List,
        /// Update stack template, fails if absent
        Update {
            #[arg(long = "stack-name")]
            name: StackName,
            #[arg(long = "parameter")]
            parameters: Vec<Parameter>,
            #[arg(long, default_value = "interactive")]
            review_change_set: ReviewChangeSet,
        },
        /// Update stack template, fails if absent
        /// Sync stack instance, creates if absent, template updates is missing
        Sync {
            #[arg(long = "stack-name")]
            name: StackName,
            #[arg(long = "parameter")]
            parameters: Vec<Parameter>,
            #[arg(long, default_value = "interactive")]
            review_change_set: ReviewChangeSet,
        },
        /// Watch stack events
        Watch {
            #[arg(long = "stack-name")]
            name: StackName,
        },
    }

    impl Command {
        pub async fn run(&self, config: &super::Config<'_>) {
            let fetch_context = |name| {
                config
                    .registry
                    .find(name)
                    .expect("registered instance spec")
                    .context(config.cloudformation, config.template_uploader)
            };

            match self {
                Self::ChangeSet {
                    name,
                    command:
                        ChangeSetCommand::Create {
                            change_set_name,
                            parameters,
                        },
                } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();
                    fetch_context(name)
                        .create_change_set(change_set_name, &parameter_map)
                        .await;
                }
                Self::ChangeSet {
                    name,
                    command: ChangeSetCommand::Delete { change_set_name },
                } => {
                    fetch_context(name).delete_change_set(change_set_name).await;
                }
                Self::ChangeSet {
                    name,
                    command: ChangeSetCommand::Describe { change_set_name },
                } => {
                    fetch_context(name)
                        .describe_change_set(change_set_name)
                        .await;
                }
                Self::ChangeSet {
                    name,
                    command: ChangeSetCommand::List,
                } => fetch_context(name).list_change_sets().await,
                Self::Delete { name } => fetch_context(name).delete().await,
                Self::List => config
                    .registry
                    .0
                    .iter()
                    .for_each(|instance_spec| println!("{}", instance_spec.stack_name.0)),
                Self::Sync {
                    name,
                    parameters,
                    review_change_set,
                } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();

                    fetch_context(name)
                        .sync(review_change_set, &parameter_map)
                        .await
                }
                Self::Update {
                    name,
                    parameters,
                    review_change_set,
                } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();

                    fetch_context(name)
                        .update(review_change_set, &parameter_map)
                        .await
                }
                Self::Watch { name } => {
                    crate::instance_spec::InstanceSpec::watch(
                        config.cloudformation,
                        crate::stack::load_stack_id(config.cloudformation, name).await,
                    )
                    .await
                }
            }
        }
    }
}
