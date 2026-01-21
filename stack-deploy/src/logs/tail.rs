use aws_sdk_cloudwatchlogs::types::StartLiveTailResponseStream;

pub struct Config<'a> {
    pub client: &'a aws_sdk_cloudwatchlogs::Client,
    pub log_group_arn: &'a str,
    pub log_stream_names: Vec<String>,
    pub filter_pattern: Option<String>,
}

pub async fn run(config: &Config<'_>) -> Result<(), Error> {
    let mut request = config
        .client
        .start_live_tail()
        .log_group_identifiers(config.log_group_arn);

    if !config.log_stream_names.is_empty() {
        request = request.set_log_stream_names(Some(config.log_stream_names.clone()));
    }

    if let Some(ref pattern) = config.filter_pattern {
        request = request.log_event_filter_pattern(pattern);
    }

    let response = request.send().await.map_err(Error::StartLiveTail)?;

    let mut stream = response.response_stream;

    loop {
        match stream.recv().await {
            Ok(Some(event)) => handle_event(event)?,
            Ok(None) => {
                log::info!("Stream ended");
                break;
            }
            Err(error) => {
                return Err(Error::Stream(error.into()));
            }
        }
    }

    Ok(())
}

fn handle_event(event: StartLiveTailResponseStream) -> Result<(), Error> {
    match event {
        StartLiveTailResponseStream::SessionStart(start) => {
            log::info!(
                "Live tail session started: {}",
                start.session_id.as_deref().unwrap_or("unknown")
            );
        }
        StartLiveTailResponseStream::SessionUpdate(update) => {
            if let Some(results) = update.session_results {
                for result in results {
                    let timestamp = result
                        .timestamp
                        .map(|t| {
                            chrono::DateTime::from_timestamp_millis(t)
                                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string())
                                .unwrap_or_else(|| t.to_string())
                        })
                        .unwrap_or_default();

                    let stream = result.log_stream_name.as_deref().unwrap_or("unknown");
                    let message = result.message.as_deref().unwrap_or("");

                    println!("{timestamp} [{stream}] {message}");
                }
            }
        }
        _ => {
            log::warn!("Received unknown event type");
        }
    }

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to start live tail: {0:#?}")]
    StartLiveTail(
        #[source]
        aws_sdk_cloudwatchlogs::error::SdkError<
            aws_sdk_cloudwatchlogs::operation::start_live_tail::StartLiveTailError,
        >,
    ),
    #[error("Stream error: {0:#?}")]
    Stream(#[source] Box<dyn std::error::Error + Send + Sync>),
}
