use crate::secrets::Secret;
use stratosphere::cloudformation::aws::{iam, lambda, s3, secretsmanager};
use stack_deploy::instance_spec::{InstanceSpec, Registry};
use stack_deploy::secrets::SecretType;
use stack_deploy::types::{ParameterMap, StackName, Template, TemplateName};

pub mod artifacts {
    use super::*;
    use stratosphere::template::OutputExport;
    use stratosphere::value::ExpString;

    pub const STACK_NAME: &str = "greenhell-artifacts";
    pub const LAMBDA_BUCKET_EXPORT: &str = "greenhell-artifacts-lambda-bucket";
    pub const GITHUB_APP_SECRET_EXPORT: &str = "greenhell-artifacts-github-app-secret-arn";

    pub fn template() -> stratosphere::Template<'static> {
        stratosphere::Template::build(|template| {
            let lambda_bucket = template.resource("LambdaBucket", s3::Bucket! {});

            template.output(
                "LambdaBucketName",
                stratosphere::template::Output {
                    description: "Name of the Lambda artifacts bucket".into(),
                    value: ExpString::from(&lambda_bucket),
                    export: Some(OutputExport {
                        name: LAMBDA_BUCKET_EXPORT.into(),
                        value: None,
                    }),
                },
            );

            let github_app_secret = template.resource(
                "GitHubAppSecret",
                secretsmanager::Secret! {
                    description: "GitHub App credentials for greenhell",
                },
            );

            template.output(
                "GitHubAppSecretArn",
                stratosphere::template::Output {
                    description: "ARN of the GitHub App secret".into(),
                    value: ExpString::from(&github_app_secret),
                    export: Some(OutputExport {
                        name: GITHUB_APP_SECRET_EXPORT.into(),
                        value: None,
                    }),
                },
            );
        })
    }

    pub fn instance_spec() -> InstanceSpec {
        InstanceSpec {
            capabilities: Default::default(),
            stack_name: StackName::from(STACK_NAME),
            parameter_map: ParameterMap::default(),
            template: Template::Stratosphere {
                name: TemplateName::from(STACK_NAME),
                template: template(),
            },
        }
    }
}

pub mod webhook {
    use super::*;

    pub const STACK_NAME: &str = "greenhell-webhook";

    pub fn template() -> stratosphere::Template<'static> {
        stratosphere::Template::build(|template| {
            let lambda_bucket = stratosphere::fn_import_value!(artifacts::LAMBDA_BUCKET_EXPORT);
            let github_app_secret_arn =
                stratosphere::fn_import_value!(artifacts::GITHUB_APP_SECRET_EXPORT);

            let lambda_role = template.resource(
                "LambdaRole",
                iam::Role! {
                    assume_role_policy_document: serde_json::json!({
                        "Version": "2012-10-17",
                        "Statement": [{
                            "Effect": "Allow",
                            "Principal": {
                                "Service": "lambda.amazonaws.com"
                            },
                            "Action": "sts:AssumeRole"
                        }]
                    }),
                    managed_policy_arns: vec![
                        "arn:aws:iam::aws:policy/service-role/AWSLambdaBasicExecutionRole".into(),
                    ],
                    policies: vec![
                        iam::role::Policy! {
                            policy_name: "read-github-app-secret",
                            policy_document: serde_json::json!({
                                "Version": "2012-10-17",
                                "Statement": [{
                                    "Effect": "Allow",
                                    "Action": [
                                        "secretsmanager:GetSecretValue"
                                    ],
                                    "Resource": github_app_secret_arn
                                }]
                            }),
                        },
                    ],
                },
            );

            let lambda_function = template.resource(
                "WebhookFunction",
                lambda::Function! {
                    function_name: "greenhell-webhook",
                    runtime: "provided.al2023",
                    handler: "bootstrap",
                    role: stratosphere::fn_get_att_arn!(&lambda_role),
                    code: lambda::function::Code! {
                        s3_bucket: lambda_bucket.clone(),
                        s3_key: stratosphere::value::ExpString::Ref("LambdaS3Key".into()),
                    },
                    environment: lambda::function::Environment! {
                        variables: std::collections::BTreeMap::from([
                            (
                                Secret::GitHubApp.to_env_variable_name().to_string(),
                                github_app_secret_arn.clone(),
                            ),
                        ]),
                    },
                },
            );

            template.output(
                "WebhookFunctionArn",
                stratosphere::Output! {
                    description: "ARN of the webhook Lambda function",
                    value: stratosphere::fn_get_att_arn!(&lambda_function),
                },
            );
        })
    }

    pub fn instance_spec() -> InstanceSpec {
        InstanceSpec {
            capabilities: Default::default(),
            stack_name: StackName::from(STACK_NAME),
            parameter_map: ParameterMap::default(),
            template: Template::Stratosphere {
                name: TemplateName::from(STACK_NAME),
                template: template(),
            },
        }
    }
}

pub fn registry() -> Registry {
    Registry(vec![artifacts::instance_spec(), webhook::instance_spec()])
}
