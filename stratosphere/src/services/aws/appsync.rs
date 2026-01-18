pub mod api {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-authmode.html
    pub struct AuthMode_ {
        pub auth_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_AuthMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.AuthMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_AuthMode as AuthMode;
    impl crate::value::ToValue for AuthMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_type {
                properties.insert(
                    "AuthType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-authprovider.html
    pub struct AuthProvider_ {
        pub auth_type: crate::value::ExpString,
        pub cognito_config: Option<Box<CognitoConfig_>>,
        pub lambda_authorizer_config: Option<Box<LambdaAuthorizerConfig_>>,
        pub open_id_connect_config: Option<Box<OpenIDConnectConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_AuthProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.AuthProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_AuthProvider as AuthProvider;
    impl crate::value::ToValue for AuthProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthType".to_string(),
                crate::value::ToValue::to_value(&self.auth_type),
            );
            if let Some(ref value) = self.cognito_config {
                properties.insert(
                    "CognitoConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_authorizer_config {
                properties.insert(
                    "LambdaAuthorizerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.open_id_connect_config {
                properties.insert(
                    "OpenIDConnectConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-cognitoconfig.html
    pub struct CognitoConfig_ {
        pub app_id_client_regex: Option<crate::value::ExpString>,
        pub aws_region: crate::value::ExpString,
        pub user_pool_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_CognitoConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.CognitoConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_CognitoConfig as CognitoConfig;
    impl crate::value::ToValue for CognitoConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_id_client_regex {
                properties.insert(
                    "AppIdClientRegex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AwsRegion".to_string(),
                crate::value::ToValue::to_value(&self.aws_region),
            );
            properties.insert(
                "UserPoolId".to_string(),
                crate::value::ToValue::to_value(&self.user_pool_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-dnsmap.html
    pub struct DnsMap_ {
        pub http: Option<crate::value::ExpString>,
        pub realtime: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_DnsMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.DnsMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_DnsMap as DnsMap;
    impl crate::value::ToValue for DnsMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http {
                properties.insert("Http".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.realtime {
                properties.insert(
                    "Realtime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-eventconfig.html
    pub struct EventConfig_ {
        pub auth_providers: Vec<AuthProvider_>,
        pub connection_auth_modes: Vec<AuthMode_>,
        pub default_publish_auth_modes: Vec<AuthMode_>,
        pub default_subscribe_auth_modes: Vec<AuthMode_>,
        pub log_config: Option<Box<EventLogConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_EventConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.EventConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_EventConfig as EventConfig;
    impl crate::value::ToValue for EventConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthProviders".to_string(),
                crate::value::ToValue::to_value(&self.auth_providers),
            );
            properties.insert(
                "ConnectionAuthModes".to_string(),
                crate::value::ToValue::to_value(&self.connection_auth_modes),
            );
            properties.insert(
                "DefaultPublishAuthModes".to_string(),
                crate::value::ToValue::to_value(&self.default_publish_auth_modes),
            );
            properties.insert(
                "DefaultSubscribeAuthModes".to_string(),
                crate::value::ToValue::to_value(&self.default_subscribe_auth_modes),
            );
            if let Some(ref value) = self.log_config {
                properties.insert(
                    "LogConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-eventlogconfig.html
    pub struct EventLogConfig_ {
        pub cloud_watch_logs_role_arn: crate::value::ExpString,
        pub log_level: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_EventLogConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.EventLogConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_EventLogConfig as EventLogConfig;
    impl crate::value::ToValue for EventLogConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchLogsRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_logs_role_arn),
            );
            properties.insert(
                "LogLevel".to_string(),
                crate::value::ToValue::to_value(&self.log_level),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-lambdaauthorizerconfig.html
    pub struct LambdaAuthorizerConfig_ {
        pub authorizer_result_ttl_in_seconds: Option<i64>,
        pub authorizer_uri: crate::value::ExpString,
        pub identity_validation_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_LambdaAuthorizerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.LambdaAuthorizerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_LambdaAuthorizerConfig as LambdaAuthorizerConfig;
    impl crate::value::ToValue for LambdaAuthorizerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorizer_result_ttl_in_seconds {
                properties.insert(
                    "AuthorizerResultTtlInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AuthorizerUri".to_string(),
                crate::value::ToValue::to_value(&self.authorizer_uri),
            );
            if let Some(ref value) = self.identity_validation_expression {
                properties.insert(
                    "IdentityValidationExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-api-openidconnectconfig.html
    pub struct OpenIDConnectConfig_ {
        pub auth_ttl: Option<f64>,
        pub client_id: Option<crate::value::ExpString>,
        pub iat_ttl: Option<f64>,
        pub issuer: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Api_OpenIDConnectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Api.OpenIDConnectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Api_OpenIDConnectConfig as OpenIDConnectConfig;
    impl crate::value::ToValue for OpenIDConnectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_ttl {
                properties.insert(
                    "AuthTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_id {
                properties.insert(
                    "ClientId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iat_ttl {
                properties.insert("IatTTL".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Issuer".to_string(),
                crate::value::ToValue::to_value(&self.issuer),
            );
            properties.into()
        }
    }
}
pub mod channelnamespace {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-channelnamespace-authmode.html
    pub struct AuthMode_ {
        pub auth_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_ChannelNamespace_AuthMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::ChannelNamespace.AuthMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_ChannelNamespace_AuthMode as AuthMode;
    impl crate::value::ToValue for AuthMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_type {
                properties.insert(
                    "AuthType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-channelnamespace-handlerconfig.html
    pub struct HandlerConfig_ {
        pub behavior: crate::value::ExpString,
        pub integration: Box<Integration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_ChannelNamespace_HandlerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::ChannelNamespace.HandlerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_ChannelNamespace_HandlerConfig as HandlerConfig;
    impl crate::value::ToValue for HandlerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Behavior".to_string(),
                crate::value::ToValue::to_value(&self.behavior),
            );
            properties.insert(
                "Integration".to_string(),
                crate::value::ToValue::to_value(&self.integration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-channelnamespace-handlerconfigs.html
    pub struct HandlerConfigs_ {
        pub on_publish: Option<Box<HandlerConfig_>>,
        pub on_subscribe: Option<Box<HandlerConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_ChannelNamespace_HandlerConfigs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::ChannelNamespace.HandlerConfigs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_ChannelNamespace_HandlerConfigs as HandlerConfigs;
    impl crate::value::ToValue for HandlerConfigs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_publish {
                properties.insert(
                    "OnPublish".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_subscribe {
                properties.insert(
                    "OnSubscribe".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-channelnamespace-integration.html
    pub struct Integration_ {
        pub data_source_name: crate::value::ExpString,
        pub lambda_config: Option<Box<LambdaConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_ChannelNamespace_Integration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::ChannelNamespace.Integration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_ChannelNamespace_Integration as Integration;
    impl crate::value::ToValue for Integration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceName".to_string(),
                crate::value::ToValue::to_value(&self.data_source_name),
            );
            if let Some(ref value) = self.lambda_config {
                properties.insert(
                    "LambdaConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-channelnamespace-lambdaconfig.html
    pub struct LambdaConfig_ {
        pub invoke_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_ChannelNamespace_LambdaConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::ChannelNamespace.LambdaConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_ChannelNamespace_LambdaConfig as LambdaConfig;
    impl crate::value::ToValue for LambdaConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InvokeType".to_string(),
                crate::value::ToValue::to_value(&self.invoke_type),
            );
            properties.into()
        }
    }
}
pub mod datasource {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-authorizationconfig.html
    pub struct AuthorizationConfig_ {
        pub authorization_type: crate::value::ExpString,
        pub aws_iam_config: Option<Box<AwsIamConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_AuthorizationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.AuthorizationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_AuthorizationConfig as AuthorizationConfig;
    impl crate::value::ToValue for AuthorizationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthorizationType".to_string(),
                crate::value::ToValue::to_value(&self.authorization_type),
            );
            if let Some(ref value) = self.aws_iam_config {
                properties.insert(
                    "AwsIamConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-awsiamconfig.html
    pub struct AwsIamConfig_ {
        pub signing_region: Option<crate::value::ExpString>,
        pub signing_service_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_AwsIamConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.AwsIamConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_AwsIamConfig as AwsIamConfig;
    impl crate::value::ToValue for AwsIamConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.signing_region {
                properties.insert(
                    "SigningRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.signing_service_name {
                properties.insert(
                    "SigningServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-deltasyncconfig.html
    pub struct DeltaSyncConfig_ {
        pub base_table_ttl: crate::value::ExpString,
        pub delta_sync_table_name: crate::value::ExpString,
        pub delta_sync_table_ttl: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_DeltaSyncConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.DeltaSyncConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_DeltaSyncConfig as DeltaSyncConfig;
    impl crate::value::ToValue for DeltaSyncConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BaseTableTTL".to_string(),
                crate::value::ToValue::to_value(&self.base_table_ttl),
            );
            properties.insert(
                "DeltaSyncTableName".to_string(),
                crate::value::ToValue::to_value(&self.delta_sync_table_name),
            );
            properties.insert(
                "DeltaSyncTableTTL".to_string(),
                crate::value::ToValue::to_value(&self.delta_sync_table_ttl),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-dynamodbconfig.html
    pub struct DynamoDBConfig_ {
        pub aws_region: crate::value::ExpString,
        pub delta_sync_config: Option<Box<DeltaSyncConfig_>>,
        pub table_name: crate::value::ExpString,
        pub use_caller_credentials: Option<crate::value::ExpBool>,
        pub versioned: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_DynamoDBConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.DynamoDBConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_DynamoDBConfig as DynamoDBConfig;
    impl crate::value::ToValue for DynamoDBConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AwsRegion".to_string(),
                crate::value::ToValue::to_value(&self.aws_region),
            );
            if let Some(ref value) = self.delta_sync_config {
                properties.insert(
                    "DeltaSyncConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            if let Some(ref value) = self.use_caller_credentials {
                properties.insert(
                    "UseCallerCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.versioned {
                properties.insert(
                    "Versioned".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-elasticsearchconfig.html
    pub struct ElasticsearchConfig_ {
        pub aws_region: crate::value::ExpString,
        pub endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_ElasticsearchConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.ElasticsearchConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_ElasticsearchConfig as ElasticsearchConfig;
    impl crate::value::ToValue for ElasticsearchConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AwsRegion".to_string(),
                crate::value::ToValue::to_value(&self.aws_region),
            );
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-eventbridgeconfig.html
    pub struct EventBridgeConfig_ {
        pub event_bus_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_EventBridgeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.EventBridgeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_EventBridgeConfig as EventBridgeConfig;
    impl crate::value::ToValue for EventBridgeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventBusArn".to_string(),
                crate::value::ToValue::to_value(&self.event_bus_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-httpconfig.html
    pub struct HttpConfig_ {
        pub authorization_config: Option<Box<AuthorizationConfig_>>,
        pub endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_HttpConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.HttpConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_HttpConfig as HttpConfig;
    impl crate::value::ToValue for HttpConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_config {
                properties.insert(
                    "AuthorizationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-lambdaconfig.html
    pub struct LambdaConfig_ {
        pub lambda_function_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_LambdaConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.LambdaConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_LambdaConfig as LambdaConfig;
    impl crate::value::ToValue for LambdaConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaFunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_function_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-opensearchserviceconfig.html
    pub struct OpenSearchServiceConfig_ {
        pub aws_region: crate::value::ExpString,
        pub endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_OpenSearchServiceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.OpenSearchServiceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_OpenSearchServiceConfig as OpenSearchServiceConfig;
    impl crate::value::ToValue for OpenSearchServiceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AwsRegion".to_string(),
                crate::value::ToValue::to_value(&self.aws_region),
            );
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-rdshttpendpointconfig.html
    pub struct RdsHttpEndpointConfig_ {
        pub aws_region: crate::value::ExpString,
        pub aws_secret_store_arn: crate::value::ExpString,
        pub database_name: Option<crate::value::ExpString>,
        pub db_cluster_identifier: crate::value::ExpString,
        pub schema: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_RdsHttpEndpointConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.RdsHttpEndpointConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_RdsHttpEndpointConfig as RdsHttpEndpointConfig;
    impl crate::value::ToValue for RdsHttpEndpointConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AwsRegion".to_string(),
                crate::value::ToValue::to_value(&self.aws_region),
            );
            properties.insert(
                "AwsSecretStoreArn".to_string(),
                crate::value::ToValue::to_value(&self.aws_secret_store_arn),
            );
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DbClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.db_cluster_identifier),
            );
            if let Some(ref value) = self.schema {
                properties.insert("Schema".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-datasource-relationaldatabaseconfig.html
    pub struct RelationalDatabaseConfig_ {
        pub rds_http_endpoint_config: Option<Box<RdsHttpEndpointConfig_>>,
        pub relational_database_source_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_DataSource_RelationalDatabaseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::DataSource.RelationalDatabaseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_DataSource_RelationalDatabaseConfig as RelationalDatabaseConfig;
    impl crate::value::ToValue for RelationalDatabaseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rds_http_endpoint_config {
                properties.insert(
                    "RdsHttpEndpointConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RelationalDatabaseSourceType".to_string(),
                crate::value::ToValue::to_value(&self.relational_database_source_type),
            );
            properties.into()
        }
    }
}
pub mod functionconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-appsyncruntime.html
    pub struct AppSyncRuntime_ {
        pub name: crate::value::ExpString,
        pub runtime_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_FunctionConfiguration_AppSyncRuntime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::FunctionConfiguration.AppSyncRuntime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_FunctionConfiguration_AppSyncRuntime as AppSyncRuntime;
    impl crate::value::ToValue for AppSyncRuntime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "RuntimeVersion".to_string(),
                crate::value::ToValue::to_value(&self.runtime_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-lambdaconflicthandlerconfig.html
    pub struct LambdaConflictHandlerConfig_ {
        pub lambda_conflict_handler_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_FunctionConfiguration_LambdaConflictHandlerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::FunctionConfiguration.LambdaConflictHandlerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_FunctionConfiguration_LambdaConflictHandlerConfig as LambdaConflictHandlerConfig;
    impl crate::value::ToValue for LambdaConflictHandlerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_conflict_handler_arn {
                properties.insert(
                    "LambdaConflictHandlerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-functionconfiguration-syncconfig.html
    pub struct SyncConfig_ {
        pub conflict_detection: crate::value::ExpString,
        pub conflict_handler: Option<crate::value::ExpString>,
        pub lambda_conflict_handler_config: Option<Box<LambdaConflictHandlerConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_FunctionConfiguration_SyncConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::FunctionConfiguration.SyncConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_FunctionConfiguration_SyncConfig as SyncConfig;
    impl crate::value::ToValue for SyncConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConflictDetection".to_string(),
                crate::value::ToValue::to_value(&self.conflict_detection),
            );
            if let Some(ref value) = self.conflict_handler {
                properties.insert(
                    "ConflictHandler".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_conflict_handler_config {
                properties.insert(
                    "LambdaConflictHandlerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod graphqlapi {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-additionalauthenticationprovider.html
    pub struct AdditionalAuthenticationProvider_ {
        pub authentication_type: crate::value::ExpString,
        pub lambda_authorizer_config: Option<Box<LambdaAuthorizerConfig_>>,
        pub open_id_connect_config: Option<Box<OpenIDConnectConfig_>>,
        pub user_pool_config: Option<Box<CognitoUserPoolConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_GraphQLApi_AdditionalAuthenticationProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::GraphQLApi.AdditionalAuthenticationProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_GraphQLApi_AdditionalAuthenticationProvider as AdditionalAuthenticationProvider;
    impl crate::value::ToValue for AdditionalAuthenticationProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(&self.authentication_type),
            );
            if let Some(ref value) = self.lambda_authorizer_config {
                properties.insert(
                    "LambdaAuthorizerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.open_id_connect_config {
                properties.insert(
                    "OpenIDConnectConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_pool_config {
                properties.insert(
                    "UserPoolConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-cognitouserpoolconfig.html
    pub struct CognitoUserPoolConfig_ {
        pub app_id_client_regex: Option<crate::value::ExpString>,
        pub aws_region: Option<crate::value::ExpString>,
        pub user_pool_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_GraphQLApi_CognitoUserPoolConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::GraphQLApi.CognitoUserPoolConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_GraphQLApi_CognitoUserPoolConfig as CognitoUserPoolConfig;
    impl crate::value::ToValue for CognitoUserPoolConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_id_client_regex {
                properties.insert(
                    "AppIdClientRegex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_region {
                properties.insert(
                    "AwsRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_pool_id {
                properties.insert(
                    "UserPoolId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-enhancedmetricsconfig.html
    pub struct EnhancedMetricsConfig_ {
        pub data_source_level_metrics_behavior: crate::value::ExpString,
        pub operation_level_metrics_config: crate::value::ExpString,
        pub resolver_level_metrics_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_GraphQLApi_EnhancedMetricsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::GraphQLApi.EnhancedMetricsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_GraphQLApi_EnhancedMetricsConfig as EnhancedMetricsConfig;
    impl crate::value::ToValue for EnhancedMetricsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceLevelMetricsBehavior".to_string(),
                crate::value::ToValue::to_value(&self.data_source_level_metrics_behavior),
            );
            properties.insert(
                "OperationLevelMetricsConfig".to_string(),
                crate::value::ToValue::to_value(&self.operation_level_metrics_config),
            );
            properties.insert(
                "ResolverLevelMetricsBehavior".to_string(),
                crate::value::ToValue::to_value(&self.resolver_level_metrics_behavior),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-lambdaauthorizerconfig.html
    pub struct LambdaAuthorizerConfig_ {
        pub authorizer_result_ttl_in_seconds: Option<f64>,
        pub authorizer_uri: Option<crate::value::ExpString>,
        pub identity_validation_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_GraphQLApi_LambdaAuthorizerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::GraphQLApi.LambdaAuthorizerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_GraphQLApi_LambdaAuthorizerConfig as LambdaAuthorizerConfig;
    impl crate::value::ToValue for LambdaAuthorizerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorizer_result_ttl_in_seconds {
                properties.insert(
                    "AuthorizerResultTtlInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.authorizer_uri {
                properties.insert(
                    "AuthorizerUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_validation_expression {
                properties.insert(
                    "IdentityValidationExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-logconfig.html
    pub struct LogConfig_ {
        pub cloud_watch_logs_role_arn: Option<crate::value::ExpString>,
        pub exclude_verbose_content: Option<crate::value::ExpBool>,
        pub field_log_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_GraphQLApi_LogConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::GraphQLApi.LogConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_GraphQLApi_LogConfig as LogConfig;
    impl crate::value::ToValue for LogConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_role_arn {
                properties.insert(
                    "CloudWatchLogsRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_verbose_content {
                properties.insert(
                    "ExcludeVerboseContent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_log_level {
                properties.insert(
                    "FieldLogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-openidconnectconfig.html
    pub struct OpenIDConnectConfig_ {
        pub auth_ttl: Option<f64>,
        pub client_id: Option<crate::value::ExpString>,
        pub iat_ttl: Option<f64>,
        pub issuer: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_GraphQLApi_OpenIDConnectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::GraphQLApi.OpenIDConnectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_GraphQLApi_OpenIDConnectConfig as OpenIDConnectConfig;
    impl crate::value::ToValue for OpenIDConnectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auth_ttl {
                properties.insert(
                    "AuthTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_id {
                properties.insert(
                    "ClientId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iat_ttl {
                properties.insert("IatTTL".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.issuer {
                properties.insert("Issuer".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-graphqlapi-userpoolconfig.html
    pub struct UserPoolConfig_ {
        pub app_id_client_regex: Option<crate::value::ExpString>,
        pub aws_region: Option<crate::value::ExpString>,
        pub default_action: Option<crate::value::ExpString>,
        pub user_pool_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_GraphQLApi_UserPoolConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::GraphQLApi.UserPoolConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_GraphQLApi_UserPoolConfig as UserPoolConfig;
    impl crate::value::ToValue for UserPoolConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_id_client_regex {
                properties.insert(
                    "AppIdClientRegex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_region {
                properties.insert(
                    "AwsRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_action {
                properties.insert(
                    "DefaultAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_pool_id {
                properties.insert(
                    "UserPoolId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod resolver {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-appsyncruntime.html
    pub struct AppSyncRuntime_ {
        pub name: crate::value::ExpString,
        pub runtime_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Resolver_AppSyncRuntime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Resolver.AppSyncRuntime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Resolver_AppSyncRuntime as AppSyncRuntime;
    impl crate::value::ToValue for AppSyncRuntime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "RuntimeVersion".to_string(),
                crate::value::ToValue::to_value(&self.runtime_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-cachingconfig.html
    pub struct CachingConfig_ {
        pub caching_keys: Option<Vec<crate::value::ExpString>>,
        pub ttl: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Resolver_CachingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Resolver.CachingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Resolver_CachingConfig as CachingConfig;
    impl crate::value::ToValue for CachingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.caching_keys {
                properties.insert(
                    "CachingKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Ttl".to_string(),
                crate::value::ToValue::to_value(&self.ttl),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-lambdaconflicthandlerconfig.html
    pub struct LambdaConflictHandlerConfig_ {
        pub lambda_conflict_handler_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Resolver_LambdaConflictHandlerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Resolver.LambdaConflictHandlerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Resolver_LambdaConflictHandlerConfig as LambdaConflictHandlerConfig;
    impl crate::value::ToValue for LambdaConflictHandlerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_conflict_handler_arn {
                properties.insert(
                    "LambdaConflictHandlerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-pipelineconfig.html
    pub struct PipelineConfig_ {
        pub functions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Resolver_PipelineConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Resolver.PipelineConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Resolver_PipelineConfig as PipelineConfig;
    impl crate::value::ToValue for PipelineConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.functions {
                properties.insert(
                    "Functions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-resolver-syncconfig.html
    pub struct SyncConfig_ {
        pub conflict_detection: crate::value::ExpString,
        pub conflict_handler: Option<crate::value::ExpString>,
        pub lambda_conflict_handler_config: Option<Box<LambdaConflictHandlerConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_Resolver_SyncConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::Resolver.SyncConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_Resolver_SyncConfig as SyncConfig;
    impl crate::value::ToValue for SyncConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConflictDetection".to_string(),
                crate::value::ToValue::to_value(&self.conflict_detection),
            );
            if let Some(ref value) = self.conflict_handler {
                properties.insert(
                    "ConflictHandler".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_conflict_handler_config {
                properties.insert(
                    "LambdaConflictHandlerConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod sourceapiassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-appsync-sourceapiassociation-sourceapiassociationconfig.html
    pub struct SourceApiAssociationConfig_ {
        pub merge_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_appsync_SourceApiAssociation_SourceApiAssociationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppSync::SourceApiAssociation.SourceApiAssociationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_appsync_SourceApiAssociation_SourceApiAssociationConfig as SourceApiAssociationConfig;
    impl crate::value::ToValue for SourceApiAssociationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.merge_type {
                properties.insert(
                    "MergeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-api.html
pub struct Api_ {
    pub event_config: Option<super::appsync::api::EventConfig_>,
    pub name: crate::value::ExpString,
    pub owner_contact: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_Api {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::Api" $($field
        $value)*)
    };
}
pub use crate::__aws_appsync_Api as Api;
impl crate::template::ToResource for Api_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Api"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.event_config {
            properties.insert(
                "EventConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.owner_contact {
            properties.insert(
                "OwnerContact".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apicache.html
pub struct ApiCache_ {
    pub api_caching_behavior: crate::value::ExpString,
    pub api_id: crate::value::ExpString,
    pub at_rest_encryption_enabled: Option<crate::value::ExpBool>,
    pub health_metrics_config: Option<crate::value::ExpString>,
    pub transit_encryption_enabled: Option<crate::value::ExpBool>,
    pub ttl: f64,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_ApiCache {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::ApiCache"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_ApiCache as ApiCache;
impl crate::template::ToResource for ApiCache_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApiCache"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiCachingBehavior".to_string(),
            crate::value::ToValue::to_value(&self.api_caching_behavior),
        );
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.at_rest_encryption_enabled {
            properties.insert(
                "AtRestEncryptionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_metrics_config {
            properties.insert(
                "HealthMetricsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.transit_encryption_enabled {
            properties.insert(
                "TransitEncryptionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Ttl".to_string(),
            crate::value::ToValue::to_value(&self.ttl),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-apikey.html
pub struct ApiKey_ {
    pub api_id: crate::value::ExpString,
    pub api_key_id: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub expires: Option<f64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_ApiKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::ApiKey" $($field
        $value)*)
    };
}
pub use crate::__aws_appsync_ApiKey as ApiKey;
impl crate::template::ToResource for ApiKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApiKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.api_key_id {
            properties.insert(
                "ApiKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expires {
            properties.insert(
                "Expires".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-channelnamespace.html
pub struct ChannelNamespace_ {
    pub api_id: crate::value::ExpString,
    pub code_handlers: Option<crate::value::ExpString>,
    pub code_s3_location: Option<crate::value::ExpString>,
    pub handler_configs: Option<super::appsync::channelnamespace::HandlerConfigs_>,
    pub name: crate::value::ExpString,
    pub publish_auth_modes: Option<Vec<super::appsync::channelnamespace::AuthMode_>>,
    pub subscribe_auth_modes: Option<Vec<super::appsync::channelnamespace::AuthMode_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_ChannelNamespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::ChannelNamespace"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_ChannelNamespace as ChannelNamespace;
impl crate::template::ToResource for ChannelNamespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ChannelNamespace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.code_handlers {
            properties.insert(
                "CodeHandlers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.code_s3_location {
            properties.insert(
                "CodeS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.handler_configs {
            properties.insert(
                "HandlerConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.publish_auth_modes {
            properties.insert(
                "PublishAuthModes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subscribe_auth_modes {
            properties.insert(
                "SubscribeAuthModes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-datasource.html
pub struct DataSource_ {
    pub api_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub dynamo_db_config: Option<super::appsync::datasource::DynamoDBConfig_>,
    pub elasticsearch_config: Option<super::appsync::datasource::ElasticsearchConfig_>,
    pub event_bridge_config: Option<super::appsync::datasource::EventBridgeConfig_>,
    pub http_config: Option<super::appsync::datasource::HttpConfig_>,
    pub lambda_config: Option<super::appsync::datasource::LambdaConfig_>,
    pub metrics_config: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub open_search_service_config: Option<super::appsync::datasource::OpenSearchServiceConfig_>,
    pub relational_database_config: Option<super::appsync::datasource::RelationalDatabaseConfig_>,
    pub service_role_arn: Option<crate::value::ExpString>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_DataSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::DataSource"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_DataSource as DataSource;
impl crate::template::ToResource for DataSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dynamo_db_config {
            properties.insert(
                "DynamoDBConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elasticsearch_config {
            properties.insert(
                "ElasticsearchConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_bridge_config {
            properties.insert(
                "EventBridgeConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.http_config {
            properties.insert(
                "HttpConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lambda_config {
            properties.insert(
                "LambdaConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metrics_config {
            properties.insert(
                "MetricsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.open_search_service_config {
            properties.insert(
                "OpenSearchServiceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.relational_database_config {
            properties.insert(
                "RelationalDatabaseConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_role_arn {
            properties.insert(
                "ServiceRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainname.html
pub struct DomainName_ {
    pub certificate_arn: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_DomainName {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::DomainName"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_DomainName as DomainName;
impl crate::template::ToResource for DomainName_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainName"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CertificateArn".to_string(),
            crate::value::ToValue::to_value(&self.certificate_arn),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-domainnameapiassociation.html
pub struct DomainNameApiAssociation_ {
    pub api_id: crate::value::ExpString,
    pub domain_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_DomainNameApiAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::DomainNameApiAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_DomainNameApiAssociation as DomainNameApiAssociation;
impl crate::template::ToResource for DomainNameApiAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainNameApiAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-functionconfiguration.html
pub struct FunctionConfiguration_ {
    pub api_id: crate::value::ExpString,
    pub code: Option<crate::value::ExpString>,
    pub code_s3_location: Option<crate::value::ExpString>,
    pub data_source_name: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub function_version: Option<crate::value::ExpString>,
    pub max_batch_size: Option<i64>,
    pub name: crate::value::ExpString,
    pub request_mapping_template: Option<crate::value::ExpString>,
    pub request_mapping_template_s3_location: Option<crate::value::ExpString>,
    pub response_mapping_template: Option<crate::value::ExpString>,
    pub response_mapping_template_s3_location: Option<crate::value::ExpString>,
    pub runtime: Option<super::appsync::functionconfiguration::AppSyncRuntime_>,
    pub sync_config: Option<super::appsync::functionconfiguration::SyncConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_FunctionConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::FunctionConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_FunctionConfiguration as FunctionConfiguration;
impl crate::template::ToResource for FunctionConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FunctionConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.code {
            properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.code_s3_location {
            properties.insert(
                "CodeS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataSourceName".to_string(),
            crate::value::ToValue::to_value(&self.data_source_name),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.function_version {
            properties.insert(
                "FunctionVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_batch_size {
            properties.insert(
                "MaxBatchSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.request_mapping_template {
            properties.insert(
                "RequestMappingTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.request_mapping_template_s3_location {
            properties.insert(
                "RequestMappingTemplateS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.response_mapping_template {
            properties.insert(
                "ResponseMappingTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.response_mapping_template_s3_location {
            properties.insert(
                "ResponseMappingTemplateS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.runtime {
            properties.insert(
                "Runtime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sync_config {
            properties.insert(
                "SyncConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlapi.html
pub struct GraphQLApi_ {
    pub additional_authentication_providers:
        Option<Vec<super::appsync::graphqlapi::AdditionalAuthenticationProvider_>>,
    pub api_type: Option<crate::value::ExpString>,
    pub authentication_type: crate::value::ExpString,
    pub enhanced_metrics_config: Option<super::appsync::graphqlapi::EnhancedMetricsConfig_>,
    pub environment_variables: Option<serde_json::Value>,
    pub introspection_config: Option<crate::value::ExpString>,
    pub lambda_authorizer_config: Option<super::appsync::graphqlapi::LambdaAuthorizerConfig_>,
    pub log_config: Option<super::appsync::graphqlapi::LogConfig_>,
    pub merged_api_execution_role_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub open_id_connect_config: Option<super::appsync::graphqlapi::OpenIDConnectConfig_>,
    pub owner_contact: Option<crate::value::ExpString>,
    pub query_depth_limit: Option<i64>,
    pub resolver_count_limit: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_pool_config: Option<super::appsync::graphqlapi::UserPoolConfig_>,
    pub visibility: Option<crate::value::ExpString>,
    pub xray_enabled: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_GraphQLApi {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::GraphQLApi"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_GraphQLApi as GraphQLApi;
impl crate::template::ToResource for GraphQLApi_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GraphQLApi"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_authentication_providers {
            properties.insert(
                "AdditionalAuthenticationProviders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.api_type {
            properties.insert(
                "ApiType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AuthenticationType".to_string(),
            crate::value::ToValue::to_value(&self.authentication_type),
        );
        if let Some(ref value) = self.enhanced_metrics_config {
            properties.insert(
                "EnhancedMetricsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_variables {
            properties.insert(
                "EnvironmentVariables".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.introspection_config {
            properties.insert(
                "IntrospectionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lambda_authorizer_config {
            properties.insert(
                "LambdaAuthorizerConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_config {
            properties.insert(
                "LogConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.merged_api_execution_role_arn {
            properties.insert(
                "MergedApiExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.open_id_connect_config {
            properties.insert(
                "OpenIDConnectConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.owner_contact {
            properties.insert(
                "OwnerContact".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.query_depth_limit {
            properties.insert(
                "QueryDepthLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resolver_count_limit {
            properties.insert(
                "ResolverCountLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_pool_config {
            properties.insert(
                "UserPoolConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.visibility {
            properties.insert(
                "Visibility".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.xray_enabled {
            properties.insert(
                "XrayEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-graphqlschema.html
pub struct GraphQLSchema_ {
    pub api_id: crate::value::ExpString,
    pub definition: Option<crate::value::ExpString>,
    pub definition_s3_location: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_GraphQLSchema {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::GraphQLSchema"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_GraphQLSchema as GraphQLSchema;
impl crate::template::ToResource for GraphQLSchema_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GraphQLSchema"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.definition {
            properties.insert(
                "Definition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_s3_location {
            properties.insert(
                "DefinitionS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-resolver.html
pub struct Resolver_ {
    pub api_id: crate::value::ExpString,
    pub caching_config: Option<super::appsync::resolver::CachingConfig_>,
    pub code: Option<crate::value::ExpString>,
    pub code_s3_location: Option<crate::value::ExpString>,
    pub data_source_name: Option<crate::value::ExpString>,
    pub field_name: crate::value::ExpString,
    pub kind: Option<crate::value::ExpString>,
    pub max_batch_size: Option<i64>,
    pub metrics_config: Option<crate::value::ExpString>,
    pub pipeline_config: Option<super::appsync::resolver::PipelineConfig_>,
    pub request_mapping_template: Option<crate::value::ExpString>,
    pub request_mapping_template_s3_location: Option<crate::value::ExpString>,
    pub response_mapping_template: Option<crate::value::ExpString>,
    pub response_mapping_template_s3_location: Option<crate::value::ExpString>,
    pub runtime: Option<super::appsync::resolver::AppSyncRuntime_>,
    pub sync_config: Option<super::appsync::resolver::SyncConfig_>,
    pub type_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_Resolver {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::Resolver"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_Resolver as Resolver;
impl crate::template::ToResource for Resolver_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Resolver"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.caching_config {
            properties.insert(
                "CachingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.code {
            properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.code_s3_location {
            properties.insert(
                "CodeS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_source_name {
            properties.insert(
                "DataSourceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FieldName".to_string(),
            crate::value::ToValue::to_value(&self.field_name),
        );
        if let Some(ref value) = self.kind {
            properties.insert("Kind".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.max_batch_size {
            properties.insert(
                "MaxBatchSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metrics_config {
            properties.insert(
                "MetricsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pipeline_config {
            properties.insert(
                "PipelineConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.request_mapping_template {
            properties.insert(
                "RequestMappingTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.request_mapping_template_s3_location {
            properties.insert(
                "RequestMappingTemplateS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.response_mapping_template {
            properties.insert(
                "ResponseMappingTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.response_mapping_template_s3_location {
            properties.insert(
                "ResponseMappingTemplateS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.runtime {
            properties.insert(
                "Runtime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sync_config {
            properties.insert(
                "SyncConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TypeName".to_string(),
            crate::value::ToValue::to_value(&self.type_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-appsync-sourceapiassociation.html
pub struct SourceApiAssociation_ {
    pub description: Option<crate::value::ExpString>,
    pub merged_api_identifier: Option<crate::value::ExpString>,
    pub source_api_association_config:
        Option<super::appsync::sourceapiassociation::SourceApiAssociationConfig_>,
    pub source_api_identifier: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_appsync_SourceApiAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppSync::SourceApiAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_appsync_SourceApiAssociation as SourceApiAssociation;
impl crate::template::ToResource for SourceApiAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SourceApiAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.merged_api_identifier {
            properties.insert(
                "MergedApiIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_api_association_config {
            properties.insert(
                "SourceApiAssociationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_api_identifier {
            properties.insert(
                "SourceApiIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
