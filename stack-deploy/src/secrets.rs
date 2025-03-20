pub struct SecretId(pub String);

pub trait SecretType:
    Clone
    + std::fmt::Display
    + std::marker::Send
    + std::marker::Sync
    + std::str::FromStr<Err = strum::ParseError>
    + strum::IntoEnumIterator
{
    fn to_arn_output_key(&self) -> crate::types::OutputKey;
    fn to_env_variable_name(&self) -> &str;
}

pub async fn read_stack_json<T: for<'a> serde::Deserialize<'a>, S: SecretType>(
    secretsmanager: &aws_sdk_secretsmanager::client::Client,
    stack: &aws_sdk_cloudformation::types::Stack,
    secret: S,
) -> T {
    serde_json::from_str(
        &read_secret_value_string(
            secretsmanager,
            &SecretId(crate::stack::fetch_stack_output(
                stack,
                &secret.to_arn_output_key(),
            )),
        )
        .await,
    )
    .unwrap()
}

pub async fn read_env_json<T: for<'a> serde::Deserialize<'a>, S: SecretType>(
    secretsmanager: &aws_sdk_secretsmanager::client::Client,
    secret: S,
) -> T {
    let secret_id = SecretId(
        std::env::var(secret.to_env_variable_name())
            .unwrap()
            .to_string(),
    );

    serde_json::from_str(&read_secret_value_string(secretsmanager, &secret_id).await).unwrap()
}

pub async fn read_secret_value_string(
    secretsmanager: &aws_sdk_secretsmanager::client::Client,
    secret_id: &SecretId,
) -> String {
    eprintln!("Reading secret: {}", secret_id.0);

    secretsmanager
        .get_secret_value()
        .secret_id(&secret_id.0)
        .send()
        .await
        .unwrap()
        .secret_string()
        .unwrap()
        .to_string()
}

pub async fn put_secret_value_string(
    secretsmanager: &aws_sdk_secretsmanager::client::Client,
    secret_id: &SecretId,
    secret_string: &str,
) {
    eprintln!("Writing secret: {}", secret_id.0);

    secretsmanager
        .put_secret_value()
        .secret_id(&secret_id.0)
        .secret_string(secret_string)
        .send()
        .await
        .unwrap();
}

pub mod cli {
    use crate::secrets::{SecretId, SecretType};

    #[derive(Clone, Debug, clap::Parser)]
    pub struct App<T: SecretType + 'static> {
        #[clap(subcommand)]
        command: Command<T>,
        #[clap(skip)]
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T: SecretType> App<T> {
        pub async fn run(
            &self,
            cloudformation: &aws_sdk_cloudformation::client::Client,
            secretsmanager: &aws_sdk_secretsmanager::client::Client,
        ) {
            self.command.run(cloudformation, secretsmanager).await
        }
    }

    #[derive(Clone, Debug, clap::Parser)]
    pub enum Command<T: SecretType + 'static> {
        /// List registered secrets
        List,
        /// Write string to secret
        PutSecretString {
            /// Name of the secret to print
            #[clap(long = "secret")]
            secret: T,
            /// Name of the stack providing the secret
            #[clap(long = "stack-name")]
            stack_name: crate::types::StackName,
        },
        /// Print secret string
        PrintSecretString {
            /// Name of the secret to print
            #[clap(long = "secret")]
            secret: T,
            /// Name of the stack providing the secret
            #[clap(long = "stack-name")]
            stack_name: crate::types::StackName,
        },
    }

    impl<T: SecretType> Command<T> {
        pub async fn run(
            &self,
            cloudformation: &aws_sdk_cloudformation::client::Client,
            secretsmanager: &aws_sdk_secretsmanager::client::Client,
        ) {
            match self {
                Self::List => list::<T>(),
                Self::PrintSecretString { secret, stack_name } => {
                    print_secret_string::<T>(cloudformation, secretsmanager, stack_name, secret)
                        .await
                }
                Self::PutSecretString { secret, stack_name } => {
                    put_secret_string::<T>(cloudformation, secretsmanager, stack_name, secret).await
                }
            }
        }
    }

    fn list<T: SecretType>() {
        for secret in T::iter() {
            println!("{}", secret)
        }
    }

    async fn put_secret_string<T: SecretType>(
        cloudformation: &aws_sdk_cloudformation::client::Client,
        secretsmanager: &aws_sdk_secretsmanager::client::Client,
        stack_name: &crate::types::StackName,
        secret: &T,
    ) {
        let secret_id = SecretId(
            crate::stack::read_stack_output(
                cloudformation,
                stack_name,
                &secret.to_arn_output_key(),
            )
            .await,
        );

        let mut secret = String::new();

        eprintln!(
            "Enter secret, newline terminates reading, last character(\\n) will be ignored. Secret will be echoded:"
        );

        match std::io::stdin().read_line(&mut secret) {
            Ok(_) => {
                secret.truncate(secret.len() - 1);
                crate::secrets::put_secret_value_string(secretsmanager, &secret_id, &secret).await;
            }
            Err(error) => panic!("Error: {error}"),
        }
    }

    async fn print_secret_string<T: SecretType>(
        cloudformation: &aws_sdk_cloudformation::client::Client,
        secretsmanager: &aws_sdk_secretsmanager::client::Client,
        stack_name: &crate::types::StackName,
        secret: &T,
    ) {
        let secret_id = SecretId(
            crate::stack::read_stack_output(
                cloudformation,
                stack_name,
                &secret.to_arn_output_key(),
            )
            .await,
        );

        println!(
            "{}",
            crate::secrets::read_secret_value_string(secretsmanager, &secret_id).await
        );
    }
}
