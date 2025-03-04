#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

impl App {
    pub async fn run<T: crate::instance_spec::Registry>(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        registry: T,
    ) {
        self.command.run(cloudformation, registry).await
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    Instance(Box<crate::cli::instance::App>),
}

impl Command {
    pub async fn run<T: crate::instance_spec::Registry>(
        &self,
        cloudformation: &aws_sdk_cloudformation::client::Client,
        registry: T,
    ) {
        match self {
            Self::Instance(command) => command.run(cloudformation, registry).await,
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
        pub async fn run<T: crate::instance_spec::Registry>(
            &self,
            cloudformation: &aws_sdk_cloudformation::client::Client,
            registry: T,
        ) {
            self.command.run(cloudformation, registry).await
        }
    }

    #[derive(Clone, Debug, clap::Parser)]
    pub enum Command {
        Sync {
            #[arg(long = "name")]
            name: StackName,
            #[arg(long = "parameter")]
            parameters: Vec<Parameter>,
        },
    }

    impl Command {
        pub async fn run<T: crate::instance_spec::Registry>(
            &self,
            cloudformation: &aws_sdk_cloudformation::client::Client,
            registry: T,
        ) {
            match self {
                Self::Sync { name, parameters } => {
                    let parameter_map = ParameterMap::parse(parameters).unwrap();

                    registry
                        .find(name)
                        .expect("registered instance spec")
                        .sync(cloudformation, &parameter_map)
                        .await
                }
            }
        }
    }
}
