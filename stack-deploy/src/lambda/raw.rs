#[derive(Clone, Debug, clap::Parser)]
pub struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionName(String);

impl std::str::FromStr for FunctionName {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<FunctionName, Self::Err> {
        Ok(Self(String::from(input)))
    }
}

impl From<&FunctionName> for String {
    fn from(value: &FunctionName) -> Self {
        value.0.clone()
    }
}

impl App {
    pub async fn run(&self, lambda: &aws_sdk_lambda::client::Client) {
        self.command.run(lambda).await
    }
}

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum OutputFormat {
    JSON,
    RAW,
}

impl OutputFormat {
    fn print(&self, body: aws_sdk_lambda::primitives::Blob) {
        match self {
            Self::JSON => Self::print_json(body),
            Self::RAW => Self::print_raw(body),
        }
    }

    fn print_raw(body: aws_sdk_lambda::primitives::Blob) {
        println!("{body:#?}")
    }

    fn print_json(body: aws_sdk_lambda::primitives::Blob) {
        println!(
            "{}",
            serde_json::from_slice::<serde_json::Value>(body.into_inner().as_ref()).unwrap()
        )
    }
}

#[derive(Clone, Debug, clap::Parser)]
pub enum Command {
    /// Invoke raw lambda function
    Invoke {
        /// Function name to execute
        #[arg(long = "function-name")]
        function_name: FunctionName,
        /// Output format
        #[arg(long = "output-format")]
        output_format: OutputFormat,
    },
}

impl Command {
    pub async fn run(&self, lambda: &aws_sdk_lambda::client::Client) {
        match self {
            Self::Invoke {
                function_name,
                output_format,
            } => invoke(lambda, function_name, output_format).await,
        }
    }
}

#[must_use]
pub fn decode_log(log_result: Option<String>) -> String {
    log_result.map_or_else(
        || String::from("Log field empty!"),
        |result| {
            base64::Engine::decode(&base64::engine::general_purpose::STANDARD, result).map_or_else(
                |error| format!("Log base64 decode failed!: {error:#?}"),
                |bytes| {
                    String::from_utf8(bytes)
                        .unwrap_or_else(|error| format!("Log utf8 decode failed!: {error:#?}"))
                },
            )
        },
    )
}

async fn invoke(
    lambda: &aws_sdk_lambda::client::Client,
    function_name: &FunctionName,
    output_format: &OutputFormat,
) {
    let response = lambda
        .invoke()
        .function_name(function_name)
        .log_type(aws_sdk_lambda::types::LogType::Tail)
        .send()
        .await;

    match response {
        Err(error) => panic!(
            "Lambda function failed to invoke: {:#?}",
            error.into_service_error()
        ),
        Ok(output) => {
            if let Some(error) = output.function_error {
                panic!(
                    "Lambda invoked but errored: Function Error: {:#?}, log: {}",
                    error,
                    decode_log(output.log_result)
                )
            } else {
                output_format.print(output.payload.unwrap())
            }
        }
    }
}
