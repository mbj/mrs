pub mod cli;
pub mod cli_token;
pub mod evaluate;
pub mod events;
pub mod github;
pub mod parse;
pub mod secrets;
pub mod stack;
pub mod watch;
pub mod webhook;

stratosphere::generator::services!("AWS::IAM", "AWS::Lambda", "AWS::S3", "AWS::SecretsManager");
