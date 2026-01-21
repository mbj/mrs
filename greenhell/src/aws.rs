pub struct Clients {
    pub cloudformation: aws_sdk_cloudformation::Client,
    pub cloudwatchlogs: aws_sdk_cloudwatchlogs::Client,
    pub s3: aws_sdk_s3::Client,
    pub secretsmanager: aws_sdk_secretsmanager::Client,
}

impl Clients {
    pub async fn load() -> Self {
        let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;

        Self {
            cloudformation: aws_sdk_cloudformation::Client::new(&config),
            cloudwatchlogs: aws_sdk_cloudwatchlogs::Client::new(&config),
            s3: aws_sdk_s3::Client::new(&config),
            secretsmanager: aws_sdk_secretsmanager::Client::new(&config),
        }
    }
}
