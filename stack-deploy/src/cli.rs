use crate::instance_spec::Registry;

#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

impl App {
    pub async fn run(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        registry: Registry,
    ) {
        self.command.run(cloudformation, registry).await
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    Instance(Box<crate::cli::instance::App>),
}

impl Command {
    pub async fn run(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        registry: Registry,
    ) {
        match self {
            Self::Instance(command) => command.run(cloudformation, registry).await,
        }
    }
}

mod instance {
    use crate::instance_spec::Registry;
    use crate::types::*;

    #[derive(Clone, Debug, clap::Parser)]
    pub struct App {
        #[clap(subcommand)]
        command: Command,
    }

    impl App {
        pub async fn run(
            &self,
            cloudformation: &aws_sdk_cloudformation::client::Client,
            registry: Registry,
        ) {
            self.command.run(cloudformation, registry).await
        }
    }

    #[derive(Clone, Debug, clap::Parser)]
    pub enum Command {
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
        },
        /// Sync stack instance, creates if absent, template updates is missing
        Sync {
            #[arg(long = "stack-name")]
            name: StackName,
            #[arg(long = "parameter")]
            parameters: Vec<Parameter>,
        },
        /// Watch stack events
        Watch {
            #[arg(long = "stack-name")]
            name: StackName,
        },
    }

    impl Command {
        pub async fn run(
            &self,
            cloudformation: &aws_sdk_cloudformation::client::Client,
            registry: Registry,
        ) {
            match self {
                Self::Delete { name } => {
                    registry
                        .find(name)
                        .expect("registered instance spec")
                        .delete(cloudformation)
                        .await
                }
                Self::List => registry
                    .0
                    .into_iter()
                    .for_each(|instance_spec| eprintln!("{}", instance_spec.stack_name.0)),
                Self::Sync { name, parameters } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();

                    registry
                        .find(name)
                        .expect("registered instance spec")
                        .sync(cloudformation, &parameter_map)
                        .await
                }
                Self::Update { name, parameters } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();

                    registry
                        .find(name)
                        .expect("registered instance spec")
                        .update(cloudformation, &parameter_map)
                        .await
                }
                Self::Watch { name } => {
                    crate::instance_spec::InstanceSpec::watch(
                        cloudformation,
                        crate::stack::load_stack_id(cloudformation, name).await,
                    )
                    .await
                }
            }
        }
    }
}
