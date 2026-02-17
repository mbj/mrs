pub mod connection {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-apikeyauthparameters.html>
    pub struct ApiKeyAuthParameters_ {
        pub api_key_name: crate::value::ExpString,
        pub api_key_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_ApiKeyAuthParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.ApiKeyAuthParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_ApiKeyAuthParameters as ApiKeyAuthParameters;
    impl crate::value::ToValue for ApiKeyAuthParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiKeyName".to_string(),
                crate::value::ToValue::to_value(&self.api_key_name),
            );
            properties.insert(
                "ApiKeyValue".to_string(),
                crate::value::ToValue::to_value(&self.api_key_value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-authparameters.html>
    pub struct AuthParameters_ {
        pub api_key_auth_parameters: Option<Box<ApiKeyAuthParameters_>>,
        pub basic_auth_parameters: Option<Box<BasicAuthParameters_>>,
        pub connectivity_parameters: Option<Box<ConnectivityParameters_>>,
        pub invocation_http_parameters: Option<Box<ConnectionHttpParameters_>>,
        pub o_auth_parameters: Option<Box<OAuthParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_AuthParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.AuthParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_AuthParameters as AuthParameters;
    impl crate::value::ToValue for AuthParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.api_key_auth_parameters {
                properties.insert(
                    "ApiKeyAuthParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.basic_auth_parameters {
                properties.insert(
                    "BasicAuthParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connectivity_parameters {
                properties.insert(
                    "ConnectivityParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.invocation_http_parameters {
                properties.insert(
                    "InvocationHttpParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.o_auth_parameters {
                properties.insert(
                    "OAuthParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-basicauthparameters.html>
    pub struct BasicAuthParameters_ {
        pub password: crate::value::ExpString,
        pub username: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_BasicAuthParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.BasicAuthParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_BasicAuthParameters as BasicAuthParameters;
    impl crate::value::ToValue for BasicAuthParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(&self.username),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-clientparameters.html>
    pub struct ClientParameters_ {
        pub client_id: crate::value::ExpString,
        pub client_secret: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_ClientParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.ClientParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_ClientParameters as ClientParameters;
    impl crate::value::ToValue for ClientParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientID".to_string(),
                crate::value::ToValue::to_value(&self.client_id),
            );
            properties.insert(
                "ClientSecret".to_string(),
                crate::value::ToValue::to_value(&self.client_secret),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-connectionhttpparameters.html>
    pub struct ConnectionHttpParameters_ {
        pub body_parameters: Option<Vec<Parameter_>>,
        pub header_parameters: Option<Vec<Parameter_>>,
        pub query_string_parameters: Option<Vec<Parameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_ConnectionHttpParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.ConnectionHttpParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_ConnectionHttpParameters as ConnectionHttpParameters;
    impl crate::value::ToValue for ConnectionHttpParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.body_parameters {
                properties.insert(
                    "BodyParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_parameters {
                properties.insert(
                    "HeaderParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_string_parameters {
                properties.insert(
                    "QueryStringParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-connectivityparameters.html>
    pub struct ConnectivityParameters_ {
        pub resource_parameters: Box<ResourceParameters_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_ConnectivityParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.ConnectivityParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_ConnectivityParameters as ConnectivityParameters;
    impl crate::value::ToValue for ConnectivityParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceParameters".to_string(),
                crate::value::ToValue::to_value(&self.resource_parameters),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-invocationconnectivityparameters.html>
    pub struct InvocationConnectivityParameters_ {
        pub resource_parameters: Box<ResourceParameters_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_InvocationConnectivityParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.InvocationConnectivityParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_InvocationConnectivityParameters as InvocationConnectivityParameters;
    impl crate::value::ToValue for InvocationConnectivityParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceParameters".to_string(),
                crate::value::ToValue::to_value(&self.resource_parameters),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-oauthparameters.html>
    pub struct OAuthParameters_ {
        pub authorization_endpoint: crate::value::ExpString,
        pub client_parameters: Box<ClientParameters_>,
        pub http_method: crate::value::ExpString,
        pub o_auth_http_parameters: Option<Box<ConnectionHttpParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_OAuthParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.OAuthParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_OAuthParameters as OAuthParameters;
    impl crate::value::ToValue for OAuthParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthorizationEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.authorization_endpoint),
            );
            properties.insert(
                "ClientParameters".to_string(),
                crate::value::ToValue::to_value(&self.client_parameters),
            );
            properties.insert(
                "HttpMethod".to_string(),
                crate::value::ToValue::to_value(&self.http_method),
            );
            if let Some(ref value) = self.o_auth_http_parameters {
                properties.insert(
                    "OAuthHttpParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-parameter.html>
    pub struct Parameter_ {
        pub is_value_secret: Option<crate::value::ExpBool>,
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_Parameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.Parameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_Parameter as Parameter;
    impl crate::value::ToValue for Parameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_value_secret {
                properties.insert(
                    "IsValueSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-connection-resourceparameters.html>
    pub struct ResourceParameters_ {
        pub resource_association_arn: Option<crate::value::ExpString>,
        pub resource_configuration_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Connection_ResourceParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Connection.ResourceParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Connection_ResourceParameters as ResourceParameters;
    impl crate::value::ToValue for ResourceParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_association_arn {
                properties.insert(
                    "ResourceAssociationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceConfigurationArn".to_string(),
                crate::value::ToValue::to_value(&self.resource_configuration_arn),
            );
            properties.into()
        }
    }
}
pub mod endpoint {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-endpoint-endpointeventbus.html>
    pub struct EndpointEventBus_ {
        pub event_bus_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Endpoint_EndpointEventBus {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Endpoint.EndpointEventBus"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Endpoint_EndpointEventBus as EndpointEventBus;
    impl crate::value::ToValue for EndpointEventBus_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventBusArn".to_string(),
                crate::value::ToValue::to_value(&self.event_bus_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-endpoint-failoverconfig.html>
    pub struct FailoverConfig_ {
        pub primary: Box<Primary_>,
        pub secondary: Box<Secondary_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Endpoint_FailoverConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Endpoint.FailoverConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Endpoint_FailoverConfig as FailoverConfig;
    impl crate::value::ToValue for FailoverConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Primary".to_string(),
                crate::value::ToValue::to_value(&self.primary),
            );
            properties.insert(
                "Secondary".to_string(),
                crate::value::ToValue::to_value(&self.secondary),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-endpoint-primary.html>
    pub struct Primary_ {
        pub health_check: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Endpoint_Primary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Endpoint.Primary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Endpoint_Primary as Primary;
    impl crate::value::ToValue for Primary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HealthCheck".to_string(),
                crate::value::ToValue::to_value(&self.health_check),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-endpoint-replicationconfig.html>
    pub struct ReplicationConfig_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Endpoint_ReplicationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Endpoint.ReplicationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Endpoint_ReplicationConfig as ReplicationConfig;
    impl crate::value::ToValue for ReplicationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-endpoint-routingconfig.html>
    pub struct RoutingConfig_ {
        pub failover_config: Box<FailoverConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Endpoint_RoutingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Endpoint.RoutingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Endpoint_RoutingConfig as RoutingConfig;
    impl crate::value::ToValue for RoutingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailoverConfig".to_string(),
                crate::value::ToValue::to_value(&self.failover_config),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-endpoint-secondary.html>
    pub struct Secondary_ {
        pub route: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Endpoint_Secondary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Endpoint.Secondary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Endpoint_Secondary as Secondary;
    impl crate::value::ToValue for Secondary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Route".to_string(),
                crate::value::ToValue::to_value(&self.route),
            );
            properties.into()
        }
    }
}
pub mod eventbus {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-eventbus-deadletterconfig.html>
    pub struct DeadLetterConfig_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_EventBus_DeadLetterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::EventBus.DeadLetterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_EventBus_DeadLetterConfig as DeadLetterConfig;
    impl crate::value::ToValue for DeadLetterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-eventbus-logconfig.html>
    pub struct LogConfig_ {
        pub include_detail: Option<crate::value::ExpString>,
        pub level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_EventBus_LogConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::EventBus.LogConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_EventBus_LogConfig as LogConfig;
    impl crate::value::ToValue for LogConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.include_detail {
                properties.insert(
                    "IncludeDetail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level {
                properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod rule {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-appsyncparameters.html>
    pub struct AppSyncParameters_ {
        pub graph_ql_operation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_AppSyncParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.AppSyncParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_AppSyncParameters as AppSyncParameters;
    impl crate::value::ToValue for AppSyncParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GraphQLOperation".to_string(),
                crate::value::ToValue::to_value(&self.graph_ql_operation),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-awsvpcconfiguration.html>
    pub struct AwsVpcConfiguration_ {
        pub assign_public_ip: Option<crate::value::ExpString>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_AwsVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.AwsVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_AwsVpcConfiguration as AwsVpcConfiguration;
    impl crate::value::ToValue for AwsVpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.assign_public_ip {
                properties.insert(
                    "AssignPublicIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batcharrayproperties.html>
    pub struct BatchArrayProperties_ {
        pub size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_BatchArrayProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.BatchArrayProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_BatchArrayProperties as BatchArrayProperties;
    impl crate::value::ToValue for BatchArrayProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.size {
                properties.insert("Size".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchparameters.html>
    pub struct BatchParameters_ {
        pub array_properties: Option<Box<BatchArrayProperties_>>,
        pub job_definition: crate::value::ExpString,
        pub job_name: crate::value::ExpString,
        pub retry_strategy: Option<Box<BatchRetryStrategy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_BatchParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.BatchParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_BatchParameters as BatchParameters;
    impl crate::value::ToValue for BatchParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.array_properties {
                properties.insert(
                    "ArrayProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "JobDefinition".to_string(),
                crate::value::ToValue::to_value(&self.job_definition),
            );
            properties.insert(
                "JobName".to_string(),
                crate::value::ToValue::to_value(&self.job_name),
            );
            if let Some(ref value) = self.retry_strategy {
                properties.insert(
                    "RetryStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-batchretrystrategy.html>
    pub struct BatchRetryStrategy_ {
        pub attempts: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_BatchRetryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.BatchRetryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_BatchRetryStrategy as BatchRetryStrategy;
    impl crate::value::ToValue for BatchRetryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attempts {
                properties.insert(
                    "Attempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-capacityproviderstrategyitem.html>
    pub struct CapacityProviderStrategyItem_ {
        pub base: Option<i32>,
        pub capacity_provider: crate::value::ExpString,
        pub weight: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_CapacityProviderStrategyItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.CapacityProviderStrategyItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_CapacityProviderStrategyItem as CapacityProviderStrategyItem;
    impl crate::value::ToValue for CapacityProviderStrategyItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base {
                properties.insert("Base".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "CapacityProvider".to_string(),
                crate::value::ToValue::to_value(&self.capacity_provider),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-deadletterconfig.html>
    pub struct DeadLetterConfig_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_DeadLetterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.DeadLetterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_DeadLetterConfig as DeadLetterConfig;
    impl crate::value::ToValue for DeadLetterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-ecsparameters.html>
    pub struct EcsParameters_ {
        pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem_>>,
        pub enable_ecs_managed_tags: Option<crate::value::ExpBool>,
        pub enable_execute_command: Option<crate::value::ExpBool>,
        pub group: Option<crate::value::ExpString>,
        pub launch_type: Option<crate::value::ExpString>,
        pub network_configuration: Option<Box<NetworkConfiguration_>>,
        pub placement_constraints: Option<Vec<PlacementConstraint_>>,
        pub placement_strategies: Option<Vec<PlacementStrategy_>>,
        pub platform_version: Option<crate::value::ExpString>,
        pub propagate_tags: Option<crate::value::ExpString>,
        pub reference_id: Option<crate::value::ExpString>,
        pub tag_list: Option<Vec<crate::Tag_>>,
        pub task_count: Option<i32>,
        pub task_definition_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_EcsParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.EcsParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_EcsParameters as EcsParameters;
    impl crate::value::ToValue for EcsParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_provider_strategy {
                properties.insert(
                    "CapacityProviderStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_ecs_managed_tags {
                properties.insert(
                    "EnableECSManagedTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_execute_command {
                properties.insert(
                    "EnableExecuteCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group {
                properties.insert("Group".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.launch_type {
                properties.insert(
                    "LaunchType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_configuration {
                properties.insert(
                    "NetworkConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement_constraints {
                properties.insert(
                    "PlacementConstraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement_strategies {
                properties.insert(
                    "PlacementStrategies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.platform_version {
                properties.insert(
                    "PlatformVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.propagate_tags {
                properties.insert(
                    "PropagateTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reference_id {
                properties.insert(
                    "ReferenceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_list {
                properties.insert(
                    "TagList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_count {
                properties.insert(
                    "TaskCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TaskDefinitionArn".to_string(),
                crate::value::ToValue::to_value(&self.task_definition_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-httpparameters.html>
    pub struct HttpParameters_ {
        pub header_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub path_parameter_values: Option<Vec<crate::value::ExpString>>,
        pub query_string_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_HttpParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.HttpParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_HttpParameters as HttpParameters;
    impl crate::value::ToValue for HttpParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header_parameters {
                properties.insert(
                    "HeaderParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path_parameter_values {
                properties.insert(
                    "PathParameterValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_string_parameters {
                properties.insert(
                    "QueryStringParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-inputtransformer.html>
    pub struct InputTransformer_ {
        pub input_paths_map: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub input_template: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_InputTransformer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.InputTransformer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_InputTransformer as InputTransformer;
    impl crate::value::ToValue for InputTransformer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_paths_map {
                properties.insert(
                    "InputPathsMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputTemplate".to_string(),
                crate::value::ToValue::to_value(&self.input_template),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-kinesisparameters.html>
    pub struct KinesisParameters_ {
        pub partition_key_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_KinesisParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.KinesisParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_KinesisParameters as KinesisParameters;
    impl crate::value::ToValue for KinesisParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PartitionKeyPath".to_string(),
                crate::value::ToValue::to_value(&self.partition_key_path),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-networkconfiguration.html>
    pub struct NetworkConfiguration_ {
        pub aws_vpc_configuration: Option<Box<AwsVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_vpc_configuration {
                properties.insert(
                    "AwsVpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-placementconstraint.html>
    pub struct PlacementConstraint_ {
        pub expression: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_PlacementConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.PlacementConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_PlacementConstraint as PlacementConstraint;
    impl crate::value::ToValue for PlacementConstraint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-placementstrategy.html>
    pub struct PlacementStrategy_ {
        pub field: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_PlacementStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.PlacementStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_PlacementStrategy as PlacementStrategy;
    impl crate::value::ToValue for PlacementStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-redshiftdataparameters.html>
    pub struct RedshiftDataParameters_ {
        pub database: crate::value::ExpString,
        pub db_user: Option<crate::value::ExpString>,
        pub secret_manager_arn: Option<crate::value::ExpString>,
        pub sql: Option<crate::value::ExpString>,
        pub sqls: Option<Vec<crate::value::ExpString>>,
        pub statement_name: Option<crate::value::ExpString>,
        pub with_event: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_RedshiftDataParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.RedshiftDataParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_RedshiftDataParameters as RedshiftDataParameters;
    impl crate::value::ToValue for RedshiftDataParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Database".to_string(),
                crate::value::ToValue::to_value(&self.database),
            );
            if let Some(ref value) = self.db_user {
                properties.insert("DbUser".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.secret_manager_arn {
                properties.insert(
                    "SecretManagerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sql {
                properties.insert("Sql".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sqls {
                properties.insert("Sqls".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.statement_name {
                properties.insert(
                    "StatementName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.with_event {
                properties.insert(
                    "WithEvent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-retrypolicy.html>
    pub struct RetryPolicy_ {
        pub maximum_event_age_in_seconds: Option<i32>,
        pub maximum_retry_attempts: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_RetryPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.RetryPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_RetryPolicy as RetryPolicy;
    impl crate::value::ToValue for RetryPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_event_age_in_seconds {
                properties.insert(
                    "MaximumEventAgeInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_retry_attempts {
                properties.insert(
                    "MaximumRetryAttempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandparameters.html>
    pub struct RunCommandParameters_ {
        pub run_command_targets: Vec<RunCommandTarget_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_RunCommandParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.RunCommandParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_RunCommandParameters as RunCommandParameters;
    impl crate::value::ToValue for RunCommandParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RunCommandTargets".to_string(),
                crate::value::ToValue::to_value(&self.run_command_targets),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-runcommandtarget.html>
    pub struct RunCommandTarget_ {
        pub key: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_RunCommandTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.RunCommandTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_RunCommandTarget as RunCommandTarget;
    impl crate::value::ToValue for RunCommandTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-sagemakerpipelineparameter.html>
    pub struct SageMakerPipelineParameter_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_SageMakerPipelineParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.SageMakerPipelineParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_SageMakerPipelineParameter as SageMakerPipelineParameter;
    impl crate::value::ToValue for SageMakerPipelineParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-sagemakerpipelineparameters.html>
    pub struct SageMakerPipelineParameters_ {
        pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_SageMakerPipelineParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.SageMakerPipelineParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_SageMakerPipelineParameters as SageMakerPipelineParameters;
    impl crate::value::ToValue for SageMakerPipelineParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pipeline_parameter_list {
                properties.insert(
                    "PipelineParameterList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-sqsparameters.html>
    pub struct SqsParameters_ {
        pub message_group_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_SqsParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.SqsParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_SqsParameters as SqsParameters;
    impl crate::value::ToValue for SqsParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MessageGroupId".to_string(),
                crate::value::ToValue::to_value(&self.message_group_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-events-rule-target.html>
    pub struct Target_ {
        pub app_sync_parameters: Option<Box<AppSyncParameters_>>,
        pub arn: crate::value::ExpString,
        pub batch_parameters: Option<Box<BatchParameters_>>,
        pub dead_letter_config: Option<Box<DeadLetterConfig_>>,
        pub ecs_parameters: Option<Box<EcsParameters_>>,
        pub http_parameters: Option<Box<HttpParameters_>>,
        pub id: crate::value::ExpString,
        pub input: Option<crate::value::ExpString>,
        pub input_path: Option<crate::value::ExpString>,
        pub input_transformer: Option<Box<InputTransformer_>>,
        pub kinesis_parameters: Option<Box<KinesisParameters_>>,
        pub redshift_data_parameters: Option<Box<RedshiftDataParameters_>>,
        pub retry_policy: Option<Box<RetryPolicy_>>,
        pub role_arn: Option<crate::value::ExpString>,
        pub run_command_parameters: Option<Box<RunCommandParameters_>>,
        pub sage_maker_pipeline_parameters: Option<Box<SageMakerPipelineParameters_>>,
        pub sqs_parameters: Option<Box<SqsParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_events_Rule_Target {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Events::Rule.Target"
            $($field $value)*)
        };
    }
    pub use crate::__aws_events_Rule_Target as Target;
    impl crate::value::ToValue for Target_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_sync_parameters {
                properties.insert(
                    "AppSyncParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.batch_parameters {
                properties.insert(
                    "BatchParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dead_letter_config {
                properties.insert(
                    "DeadLetterConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecs_parameters {
                properties.insert(
                    "EcsParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_parameters {
                properties.insert(
                    "HttpParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.input {
                properties.insert("Input".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.input_path {
                properties.insert(
                    "InputPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_transformer {
                properties.insert(
                    "InputTransformer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_parameters {
                properties.insert(
                    "KinesisParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_data_parameters {
                properties.insert(
                    "RedshiftDataParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_policy {
                properties.insert(
                    "RetryPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.run_command_parameters {
                properties.insert(
                    "RunCommandParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_pipeline_parameters {
                properties.insert(
                    "SageMakerPipelineParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sqs_parameters {
                properties.insert(
                    "SqsParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-apidestination.html>
pub struct ApiDestination_ {
    pub connection_arn: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub http_method: crate::value::ExpString,
    pub invocation_endpoint: crate::value::ExpString,
    pub invocation_rate_limit_per_second: Option<i32>,
    pub name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_events_ApiDestination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Events::ApiDestination"
        $($field $value)*)
    };
}
pub use crate::__aws_events_ApiDestination as ApiDestination;
impl crate::template::ToResource for ApiDestination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Events"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApiDestination"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConnectionArn".to_string(),
            crate::value::ToValue::to_value(&self.connection_arn),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "HttpMethod".to_string(),
            crate::value::ToValue::to_value(&self.http_method),
        );
        properties.insert(
            "InvocationEndpoint".to_string(),
            crate::value::ToValue::to_value(&self.invocation_endpoint),
        );
        if let Some(ref value) = self.invocation_rate_limit_per_second {
            properties.insert(
                "InvocationRateLimitPerSecond".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-archive.html>
pub struct Archive_ {
    pub archive_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub event_pattern: Option<serde_json::Value>,
    pub kms_key_identifier: Option<crate::value::ExpString>,
    pub retention_days: Option<i32>,
    pub source_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_events_Archive {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Events::Archive" $($field
        $value)*)
    };
}
pub use crate::__aws_events_Archive as Archive;
impl crate::template::ToResource for Archive_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Events"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Archive"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.archive_name {
            properties.insert(
                "ArchiveName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_pattern {
            properties.insert(
                "EventPattern".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_identifier {
            properties.insert(
                "KmsKeyIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_days {
            properties.insert(
                "RetentionDays".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceArn".to_string(),
            crate::value::ToValue::to_value(&self.source_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-connection.html>
pub struct Connection_ {
    pub auth_parameters: Option<super::events::connection::AuthParameters_>,
    pub authorization_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub invocation_connectivity_parameters:
        Option<super::events::connection::InvocationConnectivityParameters_>,
    pub kms_key_identifier: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_events_Connection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Events::Connection"
        $($field $value)*)
    };
}
pub use crate::__aws_events_Connection as Connection;
impl crate::template::ToResource for Connection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Events"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auth_parameters {
            properties.insert(
                "AuthParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorization_type {
            properties.insert(
                "AuthorizationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.invocation_connectivity_parameters {
            properties.insert(
                "InvocationConnectivityParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_identifier {
            properties.insert(
                "KmsKeyIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-endpoint.html>
pub struct Endpoint_ {
    pub description: Option<crate::value::ExpString>,
    pub event_buses: Vec<super::events::endpoint::EndpointEventBus_>,
    pub name: Option<crate::value::ExpString>,
    pub replication_config: Option<super::events::endpoint::ReplicationConfig_>,
    pub role_arn: Option<crate::value::ExpString>,
    pub routing_config: super::events::endpoint::RoutingConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_events_Endpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Events::Endpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_events_Endpoint as Endpoint;
impl crate::template::ToResource for Endpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Events"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Endpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventBuses".to_string(),
            crate::value::ToValue::to_value(&self.event_buses),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.replication_config {
            properties.insert(
                "ReplicationConfig".to_string(),
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
            "RoutingConfig".to_string(),
            crate::value::ToValue::to_value(&self.routing_config),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbus.html>
pub struct EventBus_ {
    pub dead_letter_config: Option<super::events::eventbus::DeadLetterConfig_>,
    pub description: Option<crate::value::ExpString>,
    pub event_source_name: Option<crate::value::ExpString>,
    pub kms_key_identifier: Option<crate::value::ExpString>,
    pub log_config: Option<super::events::eventbus::LogConfig_>,
    pub name: crate::value::ExpString,
    pub policy: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_events_EventBus {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Events::EventBus"
        $($field $value)*)
    };
}
pub use crate::__aws_events_EventBus as EventBus;
impl crate::template::ToResource for EventBus_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Events"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventBus"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dead_letter_config {
            properties.insert(
                "DeadLetterConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_source_name {
            properties.insert(
                "EventSourceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_identifier {
            properties.insert(
                "KmsKeyIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_config {
            properties.insert(
                "LogConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-eventbuspolicy.html>
pub struct EventBusPolicy_ {
    pub event_bus_name: Option<crate::value::ExpString>,
    pub statement: Option<serde_json::Value>,
    pub statement_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_events_EventBusPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Events::EventBusPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_events_EventBusPolicy as EventBusPolicy;
impl crate::template::ToResource for EventBusPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Events"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventBusPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.event_bus_name {
            properties.insert(
                "EventBusName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.statement {
            properties.insert(
                "Statement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StatementId".to_string(),
            crate::value::ToValue::to_value(&self.statement_id),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-events-rule.html>
pub struct Rule_ {
    pub description: Option<crate::value::ExpString>,
    pub event_bus_name: Option<crate::value::ExpString>,
    pub event_pattern: Option<serde_json::Value>,
    pub name: Option<crate::value::ExpString>,
    pub role_arn: Option<crate::value::ExpString>,
    pub schedule_expression: Option<crate::value::ExpString>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub targets: Option<Vec<super::events::rule::Target_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_events_Rule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Events::Rule" $($field
        $value)*)
    };
}
pub use crate::__aws_events_Rule as Rule;
impl crate::template::ToResource for Rule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Events"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Rule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_bus_name {
            properties.insert(
                "EventBusName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_pattern {
            properties.insert(
                "EventPattern".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule_expression {
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.targets {
            properties.insert(
                "Targets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
