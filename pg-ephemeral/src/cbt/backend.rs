use super::command::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq, serde::Deserialize, clap::ValueEnum)]
#[serde(rename_all = "snake_case")]
pub enum Backend {
    Docker,
    Podman,
}

impl Backend {
    pub fn command(&self) -> Command {
        match self {
            Self::Docker => Self::docker_command(),
            Self::Podman => Self::podman_command(),
        }
    }

    fn docker_command() -> Command {
        Command::new("docker")
    }

    fn podman_command() -> Command {
        Command::new("podman")
    }
}

pub mod autodetect {
    use super::Backend;

    const ENV_VARIABLE_NAME: &str = "CBT_BACKEND";

    pub type Result = std::result::Result<super::Backend, Error>;

    #[derive(Clone, Debug, thiserror::Error, PartialEq)]
    pub enum Error {
        #[error(
            "Invalid env variable for {ENV_VARIABLE_NAME}, expected \"podman\" or \"docker\", got: {0}"
        )]
        InvalidEnvVariable(String),
        #[error("No container tool detected in $PATH, searched for podman and docker")]
        NoContainerToolDetected,
    }

    pub fn run() -> Result {
        match std::env::var(ENV_VARIABLE_NAME) {
            Err(std::env::VarError::NotPresent) => from_present_tool(),
            Err(std::env::VarError::NotUnicode(_)) => {
                panic!("{ENV_VARIABLE_NAME} env variable exist but is not unicode!")
            }
            Ok(value) => from_env_value(&value),
        }
    }

    fn from_env_value(value: &str) -> Result {
        if value == "docker" {
            Ok(Backend::Docker)
        } else if value == "podman" {
            Ok(Backend::Podman)
        } else {
            Err(Error::InvalidEnvVariable(value.to_string()))
        }
    }

    fn from_present_tool() -> Result {
        fn attempt(backend: Backend) -> Option<Backend> {
            match backend
                .command()
                .argument("--version")
                .capture_only_stdout_result()
            {
                Err(_) => None,
                Ok(_) => Some(backend),
            }
        }

        attempt(Backend::Podman)
            .or_else(|| attempt(Backend::Docker))
            .ok_or(Error::NoContainerToolDetected)
    }

    pub struct Lazy(std::cell::OnceCell<Result>);

    impl Default for Lazy {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Lazy {
        pub fn new() -> Self {
            Self(std::cell::OnceCell::new())
        }

        pub fn result(&self) -> &Result {
            self.0.get_or_init(run)
        }
    }
}
