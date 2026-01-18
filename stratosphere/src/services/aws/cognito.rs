pub mod identitypool {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitoidentityprovider.html
    pub struct CognitoIdentityProvider_ {
        pub client_id: crate::value::ExpString,
        pub provider_name: crate::value::ExpString,
        pub server_side_token_check: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_IdentityPool_CognitoIdentityProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::IdentityPool.CognitoIdentityProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_IdentityPool_CognitoIdentityProvider as CognitoIdentityProvider;
    impl crate::value::ToValue for CognitoIdentityProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            properties.insert(
                "ProviderName".to_string(),
                crate::value::ToValue::to_value(&self.provider_name),
            );
            if let Some(ref value) = self.server_side_token_check {
                properties.insert(
                    "ServerSideTokenCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-cognitostreams.html
    pub struct CognitoStreams_ {
        pub role_arn: Option<crate::value::ExpString>,
        pub stream_name: Option<crate::value::ExpString>,
        pub streaming_status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_IdentityPool_CognitoStreams {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::IdentityPool.CognitoStreams"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_IdentityPool_CognitoStreams as CognitoStreams;
    impl crate::value::ToValue for CognitoStreams_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stream_name {
                properties.insert(
                    "StreamName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.streaming_status {
                properties.insert(
                    "StreamingStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypool-pushsync.html
    pub struct PushSync_ {
        pub application_arns: Option<Vec<crate::value::ExpString>>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_IdentityPool_PushSync {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::IdentityPool.PushSync"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_IdentityPool_PushSync as PushSync;
    impl crate::value::ToValue for PushSync_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_arns {
                properties.insert(
                    "ApplicationArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod identitypoolroleattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-mappingrule.html
    pub struct MappingRule_ {
        pub claim: crate::value::ExpString,
        pub match_type: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_IdentityPoolRoleAttachment_MappingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::IdentityPoolRoleAttachment.MappingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_IdentityPoolRoleAttachment_MappingRule as MappingRule;
    impl crate::value::ToValue for MappingRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Claim".to_string(),
                crate::value::ToValue::to_value(&self.claim),
            );
            properties.insert(
                "MatchType".to_string(),
                crate::value::ToValue::to_value(&self.match_type),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rolemapping.html
    pub struct RoleMapping_ {
        pub ambiguous_role_resolution: Option<crate::value::ExpString>,
        pub identity_provider: Option<crate::value::ExpString>,
        pub rules_configuration: Option<Box<RulesConfigurationType_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_IdentityPoolRoleAttachment_RoleMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::IdentityPoolRoleAttachment.RoleMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_IdentityPoolRoleAttachment_RoleMapping as RoleMapping;
    impl crate::value::ToValue for RoleMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ambiguous_role_resolution {
                properties.insert(
                    "AmbiguousRoleResolution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_provider {
                properties.insert(
                    "IdentityProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rules_configuration {
                properties.insert(
                    "RulesConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-identitypoolroleattachment-rulesconfigurationtype.html
    pub struct RulesConfigurationType_ {
        pub rules: Vec<MappingRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_IdentityPoolRoleAttachment_RulesConfigurationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::IdentityPoolRoleAttachment.RulesConfigurationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_IdentityPoolRoleAttachment_RulesConfigurationType as RulesConfigurationType;
    impl crate::value::ToValue for RulesConfigurationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
}
pub mod logdeliveryconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-logdeliveryconfiguration-cloudwatchlogsconfiguration.html
    pub struct CloudWatchLogsConfiguration_ {
        pub log_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_LogDeliveryConfiguration_CloudWatchLogsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::LogDeliveryConfiguration.CloudWatchLogsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_LogDeliveryConfiguration_CloudWatchLogsConfiguration as CloudWatchLogsConfiguration;
    impl crate::value::ToValue for CloudWatchLogsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_arn {
                properties.insert(
                    "LogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-logdeliveryconfiguration-firehoseconfiguration.html
    pub struct FirehoseConfiguration_ {
        pub stream_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_LogDeliveryConfiguration_FirehoseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::LogDeliveryConfiguration.FirehoseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_LogDeliveryConfiguration_FirehoseConfiguration as FirehoseConfiguration;
    impl crate::value::ToValue for FirehoseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.stream_arn {
                properties.insert(
                    "StreamArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-logdeliveryconfiguration-logconfiguration.html
    pub struct LogConfiguration_ {
        pub cloud_watch_logs_configuration: Option<Box<CloudWatchLogsConfiguration_>>,
        pub event_source: Option<crate::value::ExpString>,
        pub firehose_configuration: Option<Box<FirehoseConfiguration_>>,
        pub log_level: Option<crate::value::ExpString>,
        pub s3_configuration: Option<Box<S3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_LogDeliveryConfiguration_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::LogDeliveryConfiguration.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_LogDeliveryConfiguration_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_configuration {
                properties.insert(
                    "CloudWatchLogsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_source {
                properties.insert(
                    "EventSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firehose_configuration {
                properties.insert(
                    "FirehoseConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_level {
                properties.insert(
                    "LogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_configuration {
                properties.insert(
                    "S3Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-logdeliveryconfiguration-s3configuration.html
    pub struct S3Configuration_ {
        pub bucket_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_LogDeliveryConfiguration_S3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::LogDeliveryConfiguration.S3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_LogDeliveryConfiguration_S3Configuration as S3Configuration;
    impl crate::value::ToValue for S3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_arn {
                properties.insert(
                    "BucketArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod managedloginbranding {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-managedloginbranding-assettype.html
    pub struct AssetType_ {
        pub bytes: Option<crate::value::ExpString>,
        pub category: crate::value::ExpString,
        pub color_mode: crate::value::ExpString,
        pub extension: crate::value::ExpString,
        pub resource_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_ManagedLoginBranding_AssetType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::ManagedLoginBranding.AssetType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_ManagedLoginBranding_AssetType as AssetType;
    impl crate::value::ToValue for AssetType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bytes {
                properties.insert("Bytes".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(&self.category),
            );
            properties.insert(
                "ColorMode".to_string(),
                crate::value::ToValue::to_value(&self.color_mode),
            );
            properties.insert(
                "Extension".to_string(),
                crate::value::ToValue::to_value(&self.extension),
            );
            if let Some(ref value) = self.resource_id {
                properties.insert(
                    "ResourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod userpool {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-accountrecoverysetting.html
    pub struct AccountRecoverySetting_ {
        pub recovery_mechanisms: Option<Vec<RecoveryOption_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_AccountRecoverySetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.AccountRecoverySetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_AccountRecoverySetting as AccountRecoverySetting;
    impl crate::value::ToValue for AccountRecoverySetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.recovery_mechanisms {
                properties.insert(
                    "RecoveryMechanisms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-admincreateuserconfig.html
    pub struct AdminCreateUserConfig_ {
        pub allow_admin_create_user_only: Option<crate::value::ExpBool>,
        pub invite_message_template: Option<Box<InviteMessageTemplate_>>,
        pub unused_account_validity_days: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_AdminCreateUserConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.AdminCreateUserConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_AdminCreateUserConfig as AdminCreateUserConfig;
    impl crate::value::ToValue for AdminCreateUserConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_admin_create_user_only {
                properties.insert(
                    "AllowAdminCreateUserOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.invite_message_template {
                properties.insert(
                    "InviteMessageTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unused_account_validity_days {
                properties.insert(
                    "UnusedAccountValidityDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-advancedsecurityadditionalflows.html
    pub struct AdvancedSecurityAdditionalFlows_ {
        pub custom_auth_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_AdvancedSecurityAdditionalFlows {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.AdvancedSecurityAdditionalFlows"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_AdvancedSecurityAdditionalFlows as AdvancedSecurityAdditionalFlows;
    impl crate::value::ToValue for AdvancedSecurityAdditionalFlows_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_auth_mode {
                properties.insert(
                    "CustomAuthMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customemailsender.html
    pub struct CustomEmailSender_ {
        pub lambda_arn: Option<crate::value::ExpString>,
        pub lambda_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_CustomEmailSender {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.CustomEmailSender"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_CustomEmailSender as CustomEmailSender;
    impl crate::value::ToValue for CustomEmailSender_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_arn {
                properties.insert(
                    "LambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_version {
                properties.insert(
                    "LambdaVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-customsmssender.html
    pub struct CustomSMSSender_ {
        pub lambda_arn: Option<crate::value::ExpString>,
        pub lambda_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_CustomSMSSender {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.CustomSMSSender"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_CustomSMSSender as CustomSMSSender;
    impl crate::value::ToValue for CustomSMSSender_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_arn {
                properties.insert(
                    "LambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_version {
                properties.insert(
                    "LambdaVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-deviceconfiguration.html
    pub struct DeviceConfiguration_ {
        pub challenge_required_on_new_device: Option<crate::value::ExpBool>,
        pub device_only_remembered_on_user_prompt: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_DeviceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.DeviceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_DeviceConfiguration as DeviceConfiguration;
    impl crate::value::ToValue for DeviceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.challenge_required_on_new_device {
                properties.insert(
                    "ChallengeRequiredOnNewDevice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_only_remembered_on_user_prompt {
                properties.insert(
                    "DeviceOnlyRememberedOnUserPrompt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-emailconfiguration.html
    pub struct EmailConfiguration_ {
        pub configuration_set: Option<crate::value::ExpString>,
        pub email_sending_account: Option<crate::value::ExpString>,
        pub from: Option<crate::value::ExpString>,
        pub reply_to_email_address: Option<crate::value::ExpString>,
        pub source_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_EmailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.EmailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_EmailConfiguration as EmailConfiguration;
    impl crate::value::ToValue for EmailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration_set {
                properties.insert(
                    "ConfigurationSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_sending_account {
                properties.insert(
                    "EmailSendingAccount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.from {
                properties.insert("From".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.reply_to_email_address {
                properties.insert(
                    "ReplyToEmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_arn {
                properties.insert(
                    "SourceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-invitemessagetemplate.html
    pub struct InviteMessageTemplate_ {
        pub email_message: Option<crate::value::ExpString>,
        pub email_subject: Option<crate::value::ExpString>,
        pub sms_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_InviteMessageTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.InviteMessageTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_InviteMessageTemplate as InviteMessageTemplate;
    impl crate::value::ToValue for InviteMessageTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email_message {
                properties.insert(
                    "EmailMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_subject {
                properties.insert(
                    "EmailSubject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sms_message {
                properties.insert(
                    "SMSMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-lambdaconfig.html
    pub struct LambdaConfig_ {
        pub create_auth_challenge: Option<crate::value::ExpString>,
        pub custom_email_sender: Option<Box<CustomEmailSender_>>,
        pub custom_message: Option<crate::value::ExpString>,
        pub custom_sms_sender: Option<Box<CustomSMSSender_>>,
        pub define_auth_challenge: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub post_authentication: Option<crate::value::ExpString>,
        pub post_confirmation: Option<crate::value::ExpString>,
        pub pre_authentication: Option<crate::value::ExpString>,
        pub pre_sign_up: Option<crate::value::ExpString>,
        pub pre_token_generation: Option<crate::value::ExpString>,
        pub pre_token_generation_config: Option<Box<PreTokenGenerationConfig_>>,
        pub user_migration: Option<crate::value::ExpString>,
        pub verify_auth_challenge_response: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_LambdaConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.LambdaConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_LambdaConfig as LambdaConfig;
    impl crate::value::ToValue for LambdaConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.create_auth_challenge {
                properties.insert(
                    "CreateAuthChallenge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_email_sender {
                properties.insert(
                    "CustomEmailSender".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_message {
                properties.insert(
                    "CustomMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_sms_sender {
                properties.insert(
                    "CustomSMSSender".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.define_auth_challenge {
                properties.insert(
                    "DefineAuthChallenge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KMSKeyID".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.post_authentication {
                properties.insert(
                    "PostAuthentication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.post_confirmation {
                properties.insert(
                    "PostConfirmation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_authentication {
                properties.insert(
                    "PreAuthentication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_sign_up {
                properties.insert(
                    "PreSignUp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_token_generation {
                properties.insert(
                    "PreTokenGeneration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pre_token_generation_config {
                properties.insert(
                    "PreTokenGenerationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_migration {
                properties.insert(
                    "UserMigration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.verify_auth_challenge_response {
                properties.insert(
                    "VerifyAuthChallengeResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-numberattributeconstraints.html
    pub struct NumberAttributeConstraints_ {
        pub max_value: Option<crate::value::ExpString>,
        pub min_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_NumberAttributeConstraints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.NumberAttributeConstraints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_NumberAttributeConstraints as NumberAttributeConstraints;
    impl crate::value::ToValue for NumberAttributeConstraints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_value {
                properties.insert(
                    "MaxValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_value {
                properties.insert(
                    "MinValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-passwordpolicy.html
    pub struct PasswordPolicy_ {
        pub minimum_length: Option<i64>,
        pub password_history_size: Option<i64>,
        pub require_lowercase: Option<crate::value::ExpBool>,
        pub require_numbers: Option<crate::value::ExpBool>,
        pub require_symbols: Option<crate::value::ExpBool>,
        pub require_uppercase: Option<crate::value::ExpBool>,
        pub temporary_password_validity_days: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_PasswordPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.PasswordPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_PasswordPolicy as PasswordPolicy;
    impl crate::value::ToValue for PasswordPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.minimum_length {
                properties.insert(
                    "MinimumLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password_history_size {
                properties.insert(
                    "PasswordHistorySize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_lowercase {
                properties.insert(
                    "RequireLowercase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_numbers {
                properties.insert(
                    "RequireNumbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_symbols {
                properties.insert(
                    "RequireSymbols".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_uppercase {
                properties.insert(
                    "RequireUppercase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temporary_password_validity_days {
                properties.insert(
                    "TemporaryPasswordValidityDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-policies.html
    pub struct Policies_ {
        pub password_policy: Option<Box<PasswordPolicy_>>,
        pub sign_in_policy: Option<Box<SignInPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_Policies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.Policies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_Policies as Policies;
    impl crate::value::ToValue for Policies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password_policy {
                properties.insert(
                    "PasswordPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sign_in_policy {
                properties.insert(
                    "SignInPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-pretokengenerationconfig.html
    pub struct PreTokenGenerationConfig_ {
        pub lambda_arn: Option<crate::value::ExpString>,
        pub lambda_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_PreTokenGenerationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.PreTokenGenerationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_PreTokenGenerationConfig as PreTokenGenerationConfig;
    impl crate::value::ToValue for PreTokenGenerationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_arn {
                properties.insert(
                    "LambdaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_version {
                properties.insert(
                    "LambdaVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-recoveryoption.html
    pub struct RecoveryOption_ {
        pub name: Option<crate::value::ExpString>,
        pub priority: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_RecoveryOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.RecoveryOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_RecoveryOption as RecoveryOption;
    impl crate::value::ToValue for RecoveryOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-schemaattribute.html
    pub struct SchemaAttribute_ {
        pub attribute_data_type: Option<crate::value::ExpString>,
        pub developer_only_attribute: Option<crate::value::ExpBool>,
        pub mutable: Option<crate::value::ExpBool>,
        pub name: Option<crate::value::ExpString>,
        pub number_attribute_constraints: Option<Box<NumberAttributeConstraints_>>,
        pub required: Option<crate::value::ExpBool>,
        pub string_attribute_constraints: Option<Box<StringAttributeConstraints_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_SchemaAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.SchemaAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_SchemaAttribute as SchemaAttribute;
    impl crate::value::ToValue for SchemaAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_data_type {
                properties.insert(
                    "AttributeDataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.developer_only_attribute {
                properties.insert(
                    "DeveloperOnlyAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mutable {
                properties.insert(
                    "Mutable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.number_attribute_constraints {
                properties.insert(
                    "NumberAttributeConstraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.required {
                properties.insert(
                    "Required".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_attribute_constraints {
                properties.insert(
                    "StringAttributeConstraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-signinpolicy.html
    pub struct SignInPolicy_ {
        pub allowed_first_auth_factors: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_SignInPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.SignInPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_SignInPolicy as SignInPolicy;
    impl crate::value::ToValue for SignInPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_first_auth_factors {
                properties.insert(
                    "AllowedFirstAuthFactors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-smsconfiguration.html
    pub struct SmsConfiguration_ {
        pub external_id: Option<crate::value::ExpString>,
        pub sns_caller_arn: Option<crate::value::ExpString>,
        pub sns_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_SmsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.SmsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_SmsConfiguration as SmsConfiguration;
    impl crate::value::ToValue for SmsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sns_caller_arn {
                properties.insert(
                    "SnsCallerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sns_region {
                properties.insert(
                    "SnsRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-stringattributeconstraints.html
    pub struct StringAttributeConstraints_ {
        pub max_length: Option<crate::value::ExpString>,
        pub min_length: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_StringAttributeConstraints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.StringAttributeConstraints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_StringAttributeConstraints as StringAttributeConstraints;
    impl crate::value::ToValue for StringAttributeConstraints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_length {
                properties.insert(
                    "MaxLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_length {
                properties.insert(
                    "MinLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-userattributeupdatesettings.html
    pub struct UserAttributeUpdateSettings_ {
        pub attributes_require_verification_before_update: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_UserAttributeUpdateSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.UserAttributeUpdateSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_UserAttributeUpdateSettings as UserAttributeUpdateSettings;
    impl crate::value::ToValue for UserAttributeUpdateSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributesRequireVerificationBeforeUpdate".to_string(),
                crate::value::ToValue::to_value(
                    &self.attributes_require_verification_before_update,
                ),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-userpooladdons.html
    pub struct UserPoolAddOns_ {
        pub advanced_security_additional_flows: Option<Box<AdvancedSecurityAdditionalFlows_>>,
        pub advanced_security_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_UserPoolAddOns {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.UserPoolAddOns"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_UserPoolAddOns as UserPoolAddOns;
    impl crate::value::ToValue for UserPoolAddOns_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.advanced_security_additional_flows {
                properties.insert(
                    "AdvancedSecurityAdditionalFlows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.advanced_security_mode {
                properties.insert(
                    "AdvancedSecurityMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-usernameconfiguration.html
    pub struct UsernameConfiguration_ {
        pub case_sensitive: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_UsernameConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.UsernameConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_UsernameConfiguration as UsernameConfiguration;
    impl crate::value::ToValue for UsernameConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.case_sensitive {
                properties.insert(
                    "CaseSensitive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpool-verificationmessagetemplate.html
    pub struct VerificationMessageTemplate_ {
        pub default_email_option: Option<crate::value::ExpString>,
        pub email_message: Option<crate::value::ExpString>,
        pub email_message_by_link: Option<crate::value::ExpString>,
        pub email_subject: Option<crate::value::ExpString>,
        pub email_subject_by_link: Option<crate::value::ExpString>,
        pub sms_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPool_VerificationMessageTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPool.VerificationMessageTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPool_VerificationMessageTemplate as VerificationMessageTemplate;
    impl crate::value::ToValue for VerificationMessageTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_email_option {
                properties.insert(
                    "DefaultEmailOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_message {
                properties.insert(
                    "EmailMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_message_by_link {
                properties.insert(
                    "EmailMessageByLink".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_subject {
                properties.insert(
                    "EmailSubject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_subject_by_link {
                properties.insert(
                    "EmailSubjectByLink".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sms_message {
                properties.insert(
                    "SmsMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod userpoolclient {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-analyticsconfiguration.html
    pub struct AnalyticsConfiguration_ {
        pub application_arn: Option<crate::value::ExpString>,
        pub application_id: Option<crate::value::ExpString>,
        pub external_id: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub user_data_shared: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolClient_AnalyticsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolClient.AnalyticsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolClient_AnalyticsConfiguration as AnalyticsConfiguration;
    impl crate::value::ToValue for AnalyticsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_arn {
                properties.insert(
                    "ApplicationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_id {
                properties.insert(
                    "ApplicationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_id {
                properties.insert(
                    "ExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_data_shared {
                properties.insert(
                    "UserDataShared".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-refreshtokenrotation.html
    pub struct RefreshTokenRotation_ {
        pub feature: Option<crate::value::ExpString>,
        pub retry_grace_period_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolClient_RefreshTokenRotation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolClient.RefreshTokenRotation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolClient_RefreshTokenRotation as RefreshTokenRotation;
    impl crate::value::ToValue for RefreshTokenRotation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.feature {
                properties.insert(
                    "Feature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_grace_period_seconds {
                properties.insert(
                    "RetryGracePeriodSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolclient-tokenvalidityunits.html
    pub struct TokenValidityUnits_ {
        pub access_token: Option<crate::value::ExpString>,
        pub id_token: Option<crate::value::ExpString>,
        pub refresh_token: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolClient_TokenValidityUnits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolClient.TokenValidityUnits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolClient_TokenValidityUnits as TokenValidityUnits;
    impl crate::value::ToValue for TokenValidityUnits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_token {
                properties.insert(
                    "AccessToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id_token {
                properties.insert(
                    "IdToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.refresh_token {
                properties.insert(
                    "RefreshToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod userpooldomain {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooldomain-customdomainconfigtype.html
    pub struct CustomDomainConfigType_ {
        pub certificate_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolDomain_CustomDomainConfigType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolDomain.CustomDomainConfigType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolDomain_CustomDomainConfigType as CustomDomainConfigType;
    impl crate::value::ToValue for CustomDomainConfigType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod userpoolresourceserver {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolresourceserver-resourceserverscopetype.html
    pub struct ResourceServerScopeType_ {
        pub scope_description: crate::value::ExpString,
        pub scope_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolResourceServer_ResourceServerScopeType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolResourceServer.ResourceServerScopeType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolResourceServer_ResourceServerScopeType as ResourceServerScopeType;
    impl crate::value::ToValue for ResourceServerScopeType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ScopeDescription".to_string(),
                crate::value::ToValue::to_value(&self.scope_description),
            );
            properties.insert(
                "ScopeName".to_string(),
                crate::value::ToValue::to_value(&self.scope_name),
            );
            properties.into()
        }
    }
}
pub mod userpoolriskconfigurationattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractiontype.html
    pub struct AccountTakeoverActionType_ {
        pub event_action: crate::value::ExpString,
        pub notify: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_AccountTakeoverActionType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.AccountTakeoverActionType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_AccountTakeoverActionType as AccountTakeoverActionType;
    impl crate::value::ToValue for AccountTakeoverActionType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventAction".to_string(),
                crate::value::ToValue::to_value(&self.event_action),
            );
            properties.insert(
                "Notify".to_string(),
                crate::value::ToValue::to_value(&self.notify),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoveractionstype.html
    pub struct AccountTakeoverActionsType_ {
        pub high_action: Option<Box<AccountTakeoverActionType_>>,
        pub low_action: Option<Box<AccountTakeoverActionType_>>,
        pub medium_action: Option<Box<AccountTakeoverActionType_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_AccountTakeoverActionsType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.AccountTakeoverActionsType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_AccountTakeoverActionsType as AccountTakeoverActionsType;
    impl crate::value::ToValue for AccountTakeoverActionsType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.high_action {
                properties.insert(
                    "HighAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.low_action {
                properties.insert(
                    "LowAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.medium_action {
                properties.insert(
                    "MediumAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-accounttakeoverriskconfigurationtype.html
    pub struct AccountTakeoverRiskConfigurationType_ {
        pub actions: Box<AccountTakeoverActionsType_>,
        pub notify_configuration: Option<Box<NotifyConfigurationType_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_AccountTakeoverRiskConfigurationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.AccountTakeoverRiskConfigurationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_AccountTakeoverRiskConfigurationType as AccountTakeoverRiskConfigurationType;
    impl crate::value::ToValue for AccountTakeoverRiskConfigurationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            if let Some(ref value) = self.notify_configuration {
                properties.insert(
                    "NotifyConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-compromisedcredentialsactionstype.html
    pub struct CompromisedCredentialsActionsType_ {
        pub event_action: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_CompromisedCredentialsActionsType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.CompromisedCredentialsActionsType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_CompromisedCredentialsActionsType as CompromisedCredentialsActionsType;
    impl crate::value::ToValue for CompromisedCredentialsActionsType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventAction".to_string(),
                crate::value::ToValue::to_value(&self.event_action),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-compromisedcredentialsriskconfigurationtype.html
    pub struct CompromisedCredentialsRiskConfigurationType_ {
        pub actions: Box<CompromisedCredentialsActionsType_>,
        pub event_filter: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_CompromisedCredentialsRiskConfigurationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.CompromisedCredentialsRiskConfigurationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_CompromisedCredentialsRiskConfigurationType as CompromisedCredentialsRiskConfigurationType;
    impl crate::value::ToValue for CompromisedCredentialsRiskConfigurationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            if let Some(ref value) = self.event_filter {
                properties.insert(
                    "EventFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyconfigurationtype.html
    pub struct NotifyConfigurationType_ {
        pub block_email: Option<Box<NotifyEmailType_>>,
        pub from: Option<crate::value::ExpString>,
        pub mfa_email: Option<Box<NotifyEmailType_>>,
        pub no_action_email: Option<Box<NotifyEmailType_>>,
        pub reply_to: Option<crate::value::ExpString>,
        pub source_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_NotifyConfigurationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.NotifyConfigurationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_NotifyConfigurationType as NotifyConfigurationType;
    impl crate::value::ToValue for NotifyConfigurationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_email {
                properties.insert(
                    "BlockEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.from {
                properties.insert("From".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mfa_email {
                properties.insert(
                    "MfaEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_action_email {
                properties.insert(
                    "NoActionEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reply_to {
                properties.insert(
                    "ReplyTo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceArn".to_string(),
                crate::value::ToValue::to_value(&self.source_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-notifyemailtype.html
    pub struct NotifyEmailType_ {
        pub html_body: Option<crate::value::ExpString>,
        pub subject: crate::value::ExpString,
        pub text_body: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_NotifyEmailType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.NotifyEmailType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_NotifyEmailType as NotifyEmailType;
    impl crate::value::ToValue for NotifyEmailType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.html_body {
                properties.insert(
                    "HtmlBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Subject".to_string(),
                crate::value::ToValue::to_value(&self.subject),
            );
            if let Some(ref value) = self.text_body {
                properties.insert(
                    "TextBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpoolriskconfigurationattachment-riskexceptionconfigurationtype.html
    pub struct RiskExceptionConfigurationType_ {
        pub blocked_ip_range_list: Option<Vec<crate::value::ExpString>>,
        pub skipped_ip_range_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment_RiskExceptionConfigurationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment.RiskExceptionConfigurationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment_RiskExceptionConfigurationType as RiskExceptionConfigurationType;
    impl crate::value::ToValue for RiskExceptionConfigurationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.blocked_ip_range_list {
                properties.insert(
                    "BlockedIPRangeList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skipped_ip_range_list {
                properties.insert(
                    "SkippedIPRangeList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod userpooluser {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cognito-userpooluser-attributetype.html
    pub struct AttributeType_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cognito_UserPoolUser_AttributeType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cognito::UserPoolUser.AttributeType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cognito_UserPoolUser_AttributeType as AttributeType;
    impl crate::value::ToValue for AttributeType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypool.html
pub struct IdentityPool_ {
    pub allow_classic_flow: Option<crate::value::ExpBool>,
    pub allow_unauthenticated_identities: crate::value::ExpBool,
    pub cognito_events: Option<serde_json::Value>,
    pub cognito_identity_providers:
        Option<Vec<super::cognito::identitypool::CognitoIdentityProvider_>>,
    pub cognito_streams: Option<super::cognito::identitypool::CognitoStreams_>,
    pub developer_provider_name: Option<crate::value::ExpString>,
    pub identity_pool_name: Option<crate::value::ExpString>,
    pub identity_pool_tags: Option<Vec<crate::Tag_>>,
    pub open_id_connect_provider_ar_ns: Option<Vec<crate::value::ExpString>>,
    pub push_sync: Option<super::cognito::identitypool::PushSync_>,
    pub saml_provider_ar_ns: Option<Vec<crate::value::ExpString>>,
    pub supported_login_providers: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_IdentityPool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::IdentityPool"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_IdentityPool as IdentityPool;
impl crate::template::ToResource for IdentityPool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdentityPool"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allow_classic_flow {
            properties.insert(
                "AllowClassicFlow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AllowUnauthenticatedIdentities".to_string(),
            crate::value::ToValue::to_value(&self.allow_unauthenticated_identities),
        );
        if let Some(ref value) = self.cognito_events {
            properties.insert(
                "CognitoEvents".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cognito_identity_providers {
            properties.insert(
                "CognitoIdentityProviders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cognito_streams {
            properties.insert(
                "CognitoStreams".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.developer_provider_name {
            properties.insert(
                "DeveloperProviderName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_pool_name {
            properties.insert(
                "IdentityPoolName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_pool_tags {
            properties.insert(
                "IdentityPoolTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.open_id_connect_provider_ar_ns {
            properties.insert(
                "OpenIdConnectProviderARNs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.push_sync {
            properties.insert(
                "PushSync".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.saml_provider_ar_ns {
            properties.insert(
                "SamlProviderARNs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.supported_login_providers {
            properties.insert(
                "SupportedLoginProviders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypoolprincipaltag.html
pub struct IdentityPoolPrincipalTag_ {
    pub identity_pool_id: crate::value::ExpString,
    pub identity_provider_name: crate::value::ExpString,
    pub principal_tags: Option<serde_json::Value>,
    pub use_defaults: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_IdentityPoolPrincipalTag {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::IdentityPoolPrincipalTag"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_IdentityPoolPrincipalTag as IdentityPoolPrincipalTag;
impl crate::template::ToResource for IdentityPoolPrincipalTag_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdentityPoolPrincipalTag"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IdentityPoolId".to_string(),
            crate::value::ToValue::to_value(&self.identity_pool_id),
        );
        properties.insert(
            "IdentityProviderName".to_string(),
            crate::value::ToValue::to_value(&self.identity_provider_name),
        );
        if let Some(ref value) = self.principal_tags {
            properties.insert(
                "PrincipalTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.use_defaults {
            properties.insert(
                "UseDefaults".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-identitypoolroleattachment.html
pub struct IdentityPoolRoleAttachment_ {
    pub identity_pool_id: crate::value::ExpString,
    pub role_mappings: Option<
        std::collections::BTreeMap<
            String,
            super::cognito::identitypoolroleattachment::RoleMapping_,
        >,
    >,
    pub roles: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_IdentityPoolRoleAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::IdentityPoolRoleAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_IdentityPoolRoleAttachment as IdentityPoolRoleAttachment;
impl crate::template::ToResource for IdentityPoolRoleAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "IdentityPoolRoleAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IdentityPoolId".to_string(),
            crate::value::ToValue::to_value(&self.identity_pool_id),
        );
        if let Some(ref value) = self.role_mappings {
            properties.insert(
                "RoleMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.roles {
            properties.insert("Roles".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-logdeliveryconfiguration.html
pub struct LogDeliveryConfiguration_ {
    pub log_configurations:
        Option<Vec<super::cognito::logdeliveryconfiguration::LogConfiguration_>>,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_LogDeliveryConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::LogDeliveryConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_LogDeliveryConfiguration as LogDeliveryConfiguration;
impl crate::template::ToResource for LogDeliveryConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LogDeliveryConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.log_configurations {
            properties.insert(
                "LogConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-managedloginbranding.html
pub struct ManagedLoginBranding_ {
    pub assets: Option<Vec<super::cognito::managedloginbranding::AssetType_>>,
    pub client_id: Option<crate::value::ExpString>,
    pub return_merged_resources: Option<crate::value::ExpBool>,
    pub settings: Option<serde_json::Value>,
    pub use_cognito_provided_values: Option<crate::value::ExpBool>,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_ManagedLoginBranding {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::ManagedLoginBranding"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_ManagedLoginBranding as ManagedLoginBranding;
impl crate::template::ToResource for ManagedLoginBranding_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ManagedLoginBranding"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.assets {
            properties.insert("Assets".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.client_id {
            properties.insert(
                "ClientId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.return_merged_resources {
            properties.insert(
                "ReturnMergedResources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.settings {
            properties.insert(
                "Settings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.use_cognito_provided_values {
            properties.insert(
                "UseCognitoProvidedValues".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpool.html
pub struct UserPool_ {
    pub account_recovery_setting: Option<super::cognito::userpool::AccountRecoverySetting_>,
    pub admin_create_user_config: Option<super::cognito::userpool::AdminCreateUserConfig_>,
    pub alias_attributes: Option<Vec<crate::value::ExpString>>,
    pub auto_verified_attributes: Option<Vec<crate::value::ExpString>>,
    pub deletion_protection: Option<crate::value::ExpString>,
    pub device_configuration: Option<super::cognito::userpool::DeviceConfiguration_>,
    pub email_authentication_message: Option<crate::value::ExpString>,
    pub email_authentication_subject: Option<crate::value::ExpString>,
    pub email_configuration: Option<super::cognito::userpool::EmailConfiguration_>,
    pub email_verification_message: Option<crate::value::ExpString>,
    pub email_verification_subject: Option<crate::value::ExpString>,
    pub enabled_mfas: Option<Vec<crate::value::ExpString>>,
    pub lambda_config: Option<super::cognito::userpool::LambdaConfig_>,
    pub mfa_configuration: Option<crate::value::ExpString>,
    pub policies: Option<super::cognito::userpool::Policies_>,
    pub schema: Option<Vec<super::cognito::userpool::SchemaAttribute_>>,
    pub sms_authentication_message: Option<crate::value::ExpString>,
    pub sms_configuration: Option<super::cognito::userpool::SmsConfiguration_>,
    pub sms_verification_message: Option<crate::value::ExpString>,
    pub user_attribute_update_settings:
        Option<super::cognito::userpool::UserAttributeUpdateSettings_>,
    pub user_pool_add_ons: Option<super::cognito::userpool::UserPoolAddOns_>,
    pub user_pool_name: Option<crate::value::ExpString>,
    pub user_pool_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub user_pool_tier: Option<crate::value::ExpString>,
    pub username_attributes: Option<Vec<crate::value::ExpString>>,
    pub username_configuration: Option<super::cognito::userpool::UsernameConfiguration_>,
    pub verification_message_template:
        Option<super::cognito::userpool::VerificationMessageTemplate_>,
    pub web_authn_relying_party_id: Option<crate::value::ExpString>,
    pub web_authn_user_verification: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPool"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPool as UserPool;
impl crate::template::ToResource for UserPool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPool"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.account_recovery_setting {
            properties.insert(
                "AccountRecoverySetting".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.admin_create_user_config {
            properties.insert(
                "AdminCreateUserConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alias_attributes {
            properties.insert(
                "AliasAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_verified_attributes {
            properties.insert(
                "AutoVerifiedAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.device_configuration {
            properties.insert(
                "DeviceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.email_authentication_message {
            properties.insert(
                "EmailAuthenticationMessage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.email_authentication_subject {
            properties.insert(
                "EmailAuthenticationSubject".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.email_configuration {
            properties.insert(
                "EmailConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.email_verification_message {
            properties.insert(
                "EmailVerificationMessage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.email_verification_subject {
            properties.insert(
                "EmailVerificationSubject".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled_mfas {
            properties.insert(
                "EnabledMfas".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lambda_config {
            properties.insert(
                "LambdaConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mfa_configuration {
            properties.insert(
                "MfaConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policies {
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schema {
            properties.insert("Schema".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.sms_authentication_message {
            properties.insert(
                "SmsAuthenticationMessage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sms_configuration {
            properties.insert(
                "SmsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sms_verification_message {
            properties.insert(
                "SmsVerificationMessage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_attribute_update_settings {
            properties.insert(
                "UserAttributeUpdateSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_pool_add_ons {
            properties.insert(
                "UserPoolAddOns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_pool_name {
            properties.insert(
                "UserPoolName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_pool_tags {
            properties.insert(
                "UserPoolTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_pool_tier {
            properties.insert(
                "UserPoolTier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.username_attributes {
            properties.insert(
                "UsernameAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.username_configuration {
            properties.insert(
                "UsernameConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.verification_message_template {
            properties.insert(
                "VerificationMessageTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.web_authn_relying_party_id {
            properties.insert(
                "WebAuthnRelyingPartyID".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.web_authn_user_verification {
            properties.insert(
                "WebAuthnUserVerification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolclient.html
pub struct UserPoolClient_ {
    pub access_token_validity: Option<i64>,
    pub allowed_o_auth_flows: Option<Vec<crate::value::ExpString>>,
    pub allowed_o_auth_flows_user_pool_client: Option<crate::value::ExpBool>,
    pub allowed_o_auth_scopes: Option<Vec<crate::value::ExpString>>,
    pub analytics_configuration: Option<super::cognito::userpoolclient::AnalyticsConfiguration_>,
    pub auth_session_validity: Option<i64>,
    pub callback_ur_ls: Option<Vec<crate::value::ExpString>>,
    pub client_name: Option<crate::value::ExpString>,
    pub default_redirect_uri: Option<crate::value::ExpString>,
    pub enable_propagate_additional_user_context_data: Option<crate::value::ExpBool>,
    pub enable_token_revocation: Option<crate::value::ExpBool>,
    pub explicit_auth_flows: Option<Vec<crate::value::ExpString>>,
    pub generate_secret: Option<crate::value::ExpBool>,
    pub id_token_validity: Option<i64>,
    pub logout_ur_ls: Option<Vec<crate::value::ExpString>>,
    pub prevent_user_existence_errors: Option<crate::value::ExpString>,
    pub read_attributes: Option<Vec<crate::value::ExpString>>,
    pub refresh_token_rotation: Option<super::cognito::userpoolclient::RefreshTokenRotation_>,
    pub refresh_token_validity: Option<i64>,
    pub supported_identity_providers: Option<Vec<crate::value::ExpString>>,
    pub token_validity_units: Option<super::cognito::userpoolclient::TokenValidityUnits_>,
    pub user_pool_id: crate::value::ExpString,
    pub write_attributes: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolClient {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolClient"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolClient as UserPoolClient;
impl crate::template::ToResource for UserPoolClient_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPoolClient"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_token_validity {
            properties.insert(
                "AccessTokenValidity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allowed_o_auth_flows {
            properties.insert(
                "AllowedOAuthFlows".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allowed_o_auth_flows_user_pool_client {
            properties.insert(
                "AllowedOAuthFlowsUserPoolClient".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allowed_o_auth_scopes {
            properties.insert(
                "AllowedOAuthScopes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.analytics_configuration {
            properties.insert(
                "AnalyticsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auth_session_validity {
            properties.insert(
                "AuthSessionValidity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.callback_ur_ls {
            properties.insert(
                "CallbackURLs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_name {
            properties.insert(
                "ClientName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_redirect_uri {
            properties.insert(
                "DefaultRedirectURI".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_propagate_additional_user_context_data {
            properties.insert(
                "EnablePropagateAdditionalUserContextData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_token_revocation {
            properties.insert(
                "EnableTokenRevocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.explicit_auth_flows {
            properties.insert(
                "ExplicitAuthFlows".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.generate_secret {
            properties.insert(
                "GenerateSecret".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.id_token_validity {
            properties.insert(
                "IdTokenValidity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logout_ur_ls {
            properties.insert(
                "LogoutURLs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.prevent_user_existence_errors {
            properties.insert(
                "PreventUserExistenceErrors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.read_attributes {
            properties.insert(
                "ReadAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.refresh_token_rotation {
            properties.insert(
                "RefreshTokenRotation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.refresh_token_validity {
            properties.insert(
                "RefreshTokenValidity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.supported_identity_providers {
            properties.insert(
                "SupportedIdentityProviders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.token_validity_units {
            properties.insert(
                "TokenValidityUnits".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        if let Some(ref value) = self.write_attributes {
            properties.insert(
                "WriteAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooldomain.html
pub struct UserPoolDomain_ {
    pub custom_domain_config: Option<super::cognito::userpooldomain::CustomDomainConfigType_>,
    pub domain: crate::value::ExpString,
    pub managed_login_version: Option<i64>,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolDomain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolDomain"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolDomain as UserPoolDomain;
impl crate::template::ToResource for UserPoolDomain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPoolDomain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.custom_domain_config {
            properties.insert(
                "CustomDomainConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Domain".to_string(),
            crate::value::ToValue::to_value(&self.domain),
        );
        if let Some(ref value) = self.managed_login_version {
            properties.insert(
                "ManagedLoginVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolgroup.html
pub struct UserPoolGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub group_name: Option<crate::value::ExpString>,
    pub precedence: Option<i64>,
    pub role_arn: Option<crate::value::ExpString>,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolGroup as UserPoolGroup;
impl crate::template::ToResource for UserPoolGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPoolGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.group_name {
            properties.insert(
                "GroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.precedence {
            properties.insert(
                "Precedence".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolidentityprovider.html
pub struct UserPoolIdentityProvider_ {
    pub attribute_mapping: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub idp_identifiers: Option<Vec<crate::value::ExpString>>,
    pub provider_details: std::collections::BTreeMap<String, crate::value::ExpString>,
    pub provider_name: crate::value::ExpString,
    pub provider_type: crate::value::ExpString,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolIdentityProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolIdentityProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolIdentityProvider as UserPoolIdentityProvider;
impl crate::template::ToResource for UserPoolIdentityProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPoolIdentityProvider"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attribute_mapping {
            properties.insert(
                "AttributeMapping".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.idp_identifiers {
            properties.insert(
                "IdpIdentifiers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProviderDetails".to_string(),
            crate::value::ToValue::to_value(&self.provider_details),
        );
        properties.insert(
            "ProviderName".to_string(),
            crate::value::ToValue::to_value(&self.provider_name),
        );
        properties.insert(
            "ProviderType".to_string(),
            crate::value::ToValue::to_value(&self.provider_type),
        );
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolresourceserver.html
pub struct UserPoolResourceServer_ {
    pub identifier: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub scopes: Option<Vec<super::cognito::userpoolresourceserver::ResourceServerScopeType_>>,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolResourceServer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolResourceServer"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolResourceServer as UserPoolResourceServer;
impl crate::template::ToResource for UserPoolResourceServer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPoolResourceServer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Identifier".to_string(),
            crate::value::ToValue::to_value(&self.identifier),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.scopes {
            properties.insert("Scopes".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolriskconfigurationattachment.html
pub struct UserPoolRiskConfigurationAttachment_ {
    pub account_takeover_risk_configuration: Option<
        super::cognito::userpoolriskconfigurationattachment::AccountTakeoverRiskConfigurationType_,
    >,
    pub client_id: crate::value::ExpString,
    pub compromised_credentials_risk_configuration: Option<
        super::cognito::userpoolriskconfigurationattachment::CompromisedCredentialsRiskConfigurationType_,
    >,
    pub risk_exception_configuration: Option<
        super::cognito::userpoolriskconfigurationattachment::RiskExceptionConfigurationType_,
    >,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolRiskConfigurationAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolRiskConfigurationAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolRiskConfigurationAttachment as UserPoolRiskConfigurationAttachment;
impl crate::template::ToResource for UserPoolRiskConfigurationAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "UserPoolRiskConfigurationAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.account_takeover_risk_configuration {
            properties.insert(
                "AccountTakeoverRiskConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClientId".to_string(),
            crate::value::ToValue::to_value(&self.client_id),
        );
        if let Some(ref value) = self.compromised_credentials_risk_configuration {
            properties.insert(
                "CompromisedCredentialsRiskConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.risk_exception_configuration {
            properties.insert(
                "RiskExceptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluicustomizationattachment.html
pub struct UserPoolUICustomizationAttachment_ {
    pub css: Option<crate::value::ExpString>,
    pub client_id: crate::value::ExpString,
    pub user_pool_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolUICustomizationAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolUICustomizationAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolUICustomizationAttachment as UserPoolUICustomizationAttachment;
impl crate::template::ToResource for UserPoolUICustomizationAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "UserPoolUICustomizationAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.css {
            properties.insert("CSS".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ClientId".to_string(),
            crate::value::ToValue::to_value(&self.client_id),
        );
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpooluser.html
pub struct UserPoolUser_ {
    pub client_metadata: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub desired_delivery_mediums: Option<Vec<crate::value::ExpString>>,
    pub force_alias_creation: Option<crate::value::ExpBool>,
    pub message_action: Option<crate::value::ExpString>,
    pub user_attributes: Option<Vec<super::cognito::userpooluser::AttributeType_>>,
    pub user_pool_id: crate::value::ExpString,
    pub username: Option<crate::value::ExpString>,
    pub validation_data: Option<Vec<super::cognito::userpooluser::AttributeType_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolUser {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolUser"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolUser as UserPoolUser;
impl crate::template::ToResource for UserPoolUser_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserPoolUser"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.client_metadata {
            properties.insert(
                "ClientMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.desired_delivery_mediums {
            properties.insert(
                "DesiredDeliveryMediums".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force_alias_creation {
            properties.insert(
                "ForceAliasCreation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.message_action {
            properties.insert(
                "MessageAction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_attributes {
            properties.insert(
                "UserAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        if let Some(ref value) = self.username {
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.validation_data {
            properties.insert(
                "ValidationData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cognito-userpoolusertogroupattachment.html
pub struct UserPoolUserToGroupAttachment_ {
    pub group_name: crate::value::ExpString,
    pub user_pool_id: crate::value::ExpString,
    pub username: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cognito_UserPoolUserToGroupAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cognito::UserPoolUserToGroupAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_cognito_UserPoolUserToGroupAttachment as UserPoolUserToGroupAttachment;
impl crate::template::ToResource for UserPoolUserToGroupAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cognito"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "UserPoolUserToGroupAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GroupName".to_string(),
            crate::value::ToValue::to_value(&self.group_name),
        );
        properties.insert(
            "UserPoolId".to_string(),
            crate::value::ToValue::to_value(&self.user_pool_id),
        );
        properties.insert(
            "Username".to_string(),
            crate::value::ToValue::to_value(&self.username),
        );
        properties
    }
}
