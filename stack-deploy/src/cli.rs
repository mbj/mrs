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
    pub registry: &'a Registry,
    pub template_uploader: Option<&'a TemplateUploader<'a>>,
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    Instance(Box<crate::cli::instance::App>),
}

impl Command {
    pub async fn run(&self, config: &Config<'_>) {
        match self {
            Self::Instance(command) => command.run(config).await,
        }
    }
}

mod instance {
    use crate::types::*;

    #[derive(Clone, Debug, clap::Parser)]
    pub struct App {
        #[clap(subcommand)]
        command: Command,
    }

    impl App {
        pub async fn run(&self, config: &super::Config<'_>) {
            self.command.run(config).await
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
        pub async fn run(&self, config: &super::Config<'_>) {
            let fetch_context = |name| {
                config
                    .registry
                    .find(name)
                    .expect("registered instance spec")
                    .context(config.cloudformation, config.template_uploader)
            };

            match self {
                Self::Delete { name } => fetch_context(name).delete().await,
                Self::List => config
                    .registry
                    .0
                    .iter()
                    .for_each(|instance_spec| println!("{}", instance_spec.stack_name.0)),
                Self::Sync { name, parameters } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();

                    fetch_context(name).sync(&parameter_map).await
                }
                Self::Update { name, parameters } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();

                    fetch_context(name).update(&parameter_map).await
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
