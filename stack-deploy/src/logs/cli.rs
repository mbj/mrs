use super::tail;

#[derive(Clone, Debug, Eq, PartialEq, clap::Parser)]
pub enum Command {
    /// Stream logs in real-time using CloudWatch Live Tail
    Tail {
        /// Log group ARN
        #[arg(long)]
        log_group_arn: String,

        /// Filter to specific log stream names
        #[arg(long = "stream")]
        log_stream_names: Vec<String>,

        /// Filter pattern for log events
        #[arg(long)]
        filter: Option<String>,
    },
}

pub struct Config<'a> {
    pub client: &'a aws_sdk_cloudwatchlogs::Client,
}

impl Command {
    pub async fn run(&self, config: &Config<'_>) {
        match self {
            Self::Tail {
                log_group_arn,
                log_stream_names,
                filter,
            } => {
                let tail_config = tail::Config {
                    client: config.client,
                    log_group_arn,
                    log_stream_names: log_stream_names.clone(),
                    filter_pattern: filter.clone(),
                };

                if let Err(error) = tail::run(&tail_config).await {
                    log::error!("{error}");
                }
            }
        }
    }
}
