pub mod apikey {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-apikey-stagekey.html
    pub struct StageKey_ {
        pub rest_api_id: Option<crate::value::ExpString>,
        pub stage_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_ApiKey_StageKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::ApiKey.StageKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_ApiKey_StageKey as StageKey;
    impl crate::value::ToValue for StageKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rest_api_id {
                properties.insert(
                    "RestApiId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stage_name {
                properties.insert(
                    "StageName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod deployment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-accesslogsetting.html
    pub struct AccessLogSetting_ {
        pub destination_arn: Option<crate::value::ExpString>,
        pub format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Deployment_AccessLogSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Deployment.AccessLogSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Deployment_AccessLogSetting as AccessLogSetting;
    impl crate::value::ToValue for AccessLogSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_arn {
                properties.insert(
                    "DestinationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.format {
                properties.insert("Format".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-canarysetting.html
    pub struct CanarySetting_ {
        pub percent_traffic: Option<f64>,
        pub stage_variable_overrides:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub use_stage_cache: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Deployment_CanarySetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Deployment.CanarySetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Deployment_CanarySetting as CanarySetting;
    impl crate::value::ToValue for CanarySetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.percent_traffic {
                properties.insert(
                    "PercentTraffic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stage_variable_overrides {
                properties.insert(
                    "StageVariableOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_stage_cache {
                properties.insert(
                    "UseStageCache".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-deploymentcanarysettings.html
    pub struct DeploymentCanarySettings_ {
        pub percent_traffic: Option<f64>,
        pub stage_variable_overrides:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub use_stage_cache: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Deployment_DeploymentCanarySettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Deployment.DeploymentCanarySettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Deployment_DeploymentCanarySettings as DeploymentCanarySettings;
    impl crate::value::ToValue for DeploymentCanarySettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.percent_traffic {
                properties.insert(
                    "PercentTraffic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stage_variable_overrides {
                properties.insert(
                    "StageVariableOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_stage_cache {
                properties.insert(
                    "UseStageCache".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-methodsetting.html
    pub struct MethodSetting_ {
        pub cache_data_encrypted: Option<crate::value::ExpBool>,
        pub cache_ttl_in_seconds: Option<i32>,
        pub caching_enabled: Option<crate::value::ExpBool>,
        pub data_trace_enabled: Option<crate::value::ExpBool>,
        pub http_method: Option<crate::value::ExpString>,
        pub logging_level: Option<crate::value::ExpString>,
        pub metrics_enabled: Option<crate::value::ExpBool>,
        pub resource_path: Option<crate::value::ExpString>,
        pub throttling_burst_limit: Option<i32>,
        pub throttling_rate_limit: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Deployment_MethodSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Deployment.MethodSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Deployment_MethodSetting as MethodSetting;
    impl crate::value::ToValue for MethodSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_data_encrypted {
                properties.insert(
                    "CacheDataEncrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_ttl_in_seconds {
                properties.insert(
                    "CacheTtlInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caching_enabled {
                properties.insert(
                    "CachingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_trace_enabled {
                properties.insert(
                    "DataTraceEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_method {
                properties.insert(
                    "HttpMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_level {
                properties.insert(
                    "LoggingLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics_enabled {
                properties.insert(
                    "MetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_path {
                properties.insert(
                    "ResourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throttling_burst_limit {
                properties.insert(
                    "ThrottlingBurstLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throttling_rate_limit {
                properties.insert(
                    "ThrottlingRateLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-deployment-stagedescription.html
    pub struct StageDescription_ {
        pub access_log_setting: Option<Box<AccessLogSetting_>>,
        pub cache_cluster_enabled: Option<crate::value::ExpBool>,
        pub cache_cluster_size: Option<crate::value::ExpString>,
        pub cache_data_encrypted: Option<crate::value::ExpBool>,
        pub cache_ttl_in_seconds: Option<i32>,
        pub caching_enabled: Option<crate::value::ExpBool>,
        pub canary_setting: Option<Box<CanarySetting_>>,
        pub client_certificate_id: Option<crate::value::ExpString>,
        pub data_trace_enabled: Option<crate::value::ExpBool>,
        pub description: Option<crate::value::ExpString>,
        pub documentation_version: Option<crate::value::ExpString>,
        pub logging_level: Option<crate::value::ExpString>,
        pub method_settings: Option<Vec<MethodSetting_>>,
        pub metrics_enabled: Option<crate::value::ExpBool>,
        pub tags: Option<Vec<crate::Tag_>>,
        pub throttling_burst_limit: Option<i32>,
        pub throttling_rate_limit: Option<f64>,
        pub tracing_enabled: Option<crate::value::ExpBool>,
        pub variables: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Deployment_StageDescription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Deployment.StageDescription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Deployment_StageDescription as StageDescription;
    impl crate::value::ToValue for StageDescription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_log_setting {
                properties.insert(
                    "AccessLogSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_cluster_enabled {
                properties.insert(
                    "CacheClusterEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_cluster_size {
                properties.insert(
                    "CacheClusterSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_data_encrypted {
                properties.insert(
                    "CacheDataEncrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_ttl_in_seconds {
                properties.insert(
                    "CacheTtlInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caching_enabled {
                properties.insert(
                    "CachingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.canary_setting {
                properties.insert(
                    "CanarySetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_certificate_id {
                properties.insert(
                    "ClientCertificateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_trace_enabled {
                properties.insert(
                    "DataTraceEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.documentation_version {
                properties.insert(
                    "DocumentationVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_level {
                properties.insert(
                    "LoggingLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.method_settings {
                properties.insert(
                    "MethodSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics_enabled {
                properties.insert(
                    "MetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.throttling_burst_limit {
                properties.insert(
                    "ThrottlingBurstLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throttling_rate_limit {
                properties.insert(
                    "ThrottlingRateLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tracing_enabled {
                properties.insert(
                    "TracingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.variables {
                properties.insert(
                    "Variables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod documentationpart {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-documentationpart-location.html
    pub struct Location_ {
        pub method: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
        pub status_code: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_DocumentationPart_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::DocumentationPart.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_DocumentationPart_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status_code {
                properties.insert(
                    "StatusCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod domainname {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainname-endpointconfiguration.html
    pub struct EndpointConfiguration_ {
        pub ip_address_type: Option<crate::value::ExpString>,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_DomainName_EndpointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::DomainName.EndpointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_DomainName_EndpointConfiguration as EndpointConfiguration;
    impl crate::value::ToValue for EndpointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainname-mutualtlsauthentication.html
    pub struct MutualTlsAuthentication_ {
        pub truststore_uri: Option<crate::value::ExpString>,
        pub truststore_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_DomainName_MutualTlsAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::DomainName.MutualTlsAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_DomainName_MutualTlsAuthentication as MutualTlsAuthentication;
    impl crate::value::ToValue for MutualTlsAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.truststore_uri {
                properties.insert(
                    "TruststoreUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.truststore_version {
                properties.insert(
                    "TruststoreVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod domainnamev2 {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-domainnamev2-endpointconfiguration.html
    pub struct EndpointConfiguration_ {
        pub ip_address_type: Option<crate::value::ExpString>,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_DomainNameV2_EndpointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::DomainNameV2.EndpointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_DomainNameV2_EndpointConfiguration as EndpointConfiguration;
    impl crate::value::ToValue for EndpointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod method {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-method-integration.html
    pub struct Integration_ {
        pub cache_key_parameters: Option<Vec<crate::value::ExpString>>,
        pub cache_namespace: Option<crate::value::ExpString>,
        pub connection_id: Option<crate::value::ExpString>,
        pub connection_type: Option<crate::value::ExpString>,
        pub content_handling: Option<crate::value::ExpString>,
        pub credentials: Option<crate::value::ExpString>,
        pub integration_http_method: Option<crate::value::ExpString>,
        pub integration_responses: Option<Vec<IntegrationResponse_>>,
        pub passthrough_behavior: Option<crate::value::ExpString>,
        pub request_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub request_templates: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub timeout_in_millis: Option<i32>,
        pub r#type: crate::value::ExpString,
        pub uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Method_Integration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Method.Integration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Method_Integration as Integration;
    impl crate::value::ToValue for Integration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_key_parameters {
                properties.insert(
                    "CacheKeyParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_namespace {
                properties.insert(
                    "CacheNamespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_id {
                properties.insert(
                    "ConnectionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_type {
                properties.insert(
                    "ConnectionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_handling {
                properties.insert(
                    "ContentHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.credentials {
                properties.insert(
                    "Credentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integration_http_method {
                properties.insert(
                    "IntegrationHttpMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integration_responses {
                properties.insert(
                    "IntegrationResponses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.passthrough_behavior {
                properties.insert(
                    "PassthroughBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.request_parameters {
                properties.insert(
                    "RequestParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.request_templates {
                properties.insert(
                    "RequestTemplates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_in_millis {
                properties.insert(
                    "TimeoutInMillis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.uri {
                properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-method-integrationresponse.html
    pub struct IntegrationResponse_ {
        pub content_handling: Option<crate::value::ExpString>,
        pub response_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub response_templates: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub selection_pattern: Option<crate::value::ExpString>,
        pub status_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Method_IntegrationResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Method.IntegrationResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Method_IntegrationResponse as IntegrationResponse;
    impl crate::value::ToValue for IntegrationResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_handling {
                properties.insert(
                    "ContentHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_parameters {
                properties.insert(
                    "ResponseParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_templates {
                properties.insert(
                    "ResponseTemplates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.selection_pattern {
                properties.insert(
                    "SelectionPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-method-methodresponse.html
    pub struct MethodResponse_ {
        pub response_models: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub response_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub status_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Method_MethodResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Method.MethodResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Method_MethodResponse as MethodResponse;
    impl crate::value::ToValue for MethodResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.response_models {
                properties.insert(
                    "ResponseModels".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_parameters {
                properties.insert(
                    "ResponseParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
}
pub mod restapi {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-endpointconfiguration.html
    pub struct EndpointConfiguration_ {
        pub ip_address_type: Option<crate::value::ExpString>,
        pub types: Option<Vec<crate::value::ExpString>>,
        pub vpc_endpoint_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_RestApi_EndpointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::RestApi.EndpointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_RestApi_EndpointConfiguration as EndpointConfiguration;
    impl crate::value::ToValue for EndpointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vpc_endpoint_ids {
                properties.insert(
                    "VpcEndpointIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-restapi-s3location.html
    pub struct S3Location_ {
        pub bucket: Option<crate::value::ExpString>,
        pub e_tag: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_RestApi_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::RestApi.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_RestApi_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket {
                properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.e_tag {
                properties.insert("ETag".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod stage {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-stage-accesslogsetting.html
    pub struct AccessLogSetting_ {
        pub destination_arn: Option<crate::value::ExpString>,
        pub format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Stage_AccessLogSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Stage.AccessLogSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Stage_AccessLogSetting as AccessLogSetting;
    impl crate::value::ToValue for AccessLogSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_arn {
                properties.insert(
                    "DestinationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.format {
                properties.insert("Format".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-stage-canarysetting.html
    pub struct CanarySetting_ {
        pub deployment_id: Option<crate::value::ExpString>,
        pub percent_traffic: Option<f64>,
        pub stage_variable_overrides:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub use_stage_cache: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Stage_CanarySetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Stage.CanarySetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Stage_CanarySetting as CanarySetting;
    impl crate::value::ToValue for CanarySetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deployment_id {
                properties.insert(
                    "DeploymentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.percent_traffic {
                properties.insert(
                    "PercentTraffic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stage_variable_overrides {
                properties.insert(
                    "StageVariableOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_stage_cache {
                properties.insert(
                    "UseStageCache".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-stage-methodsetting.html
    pub struct MethodSetting_ {
        pub cache_data_encrypted: Option<crate::value::ExpBool>,
        pub cache_ttl_in_seconds: Option<i32>,
        pub caching_enabled: Option<crate::value::ExpBool>,
        pub data_trace_enabled: Option<crate::value::ExpBool>,
        pub http_method: Option<crate::value::ExpString>,
        pub logging_level: Option<crate::value::ExpString>,
        pub metrics_enabled: Option<crate::value::ExpBool>,
        pub resource_path: Option<crate::value::ExpString>,
        pub throttling_burst_limit: Option<i32>,
        pub throttling_rate_limit: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_Stage_MethodSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::Stage.MethodSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_Stage_MethodSetting as MethodSetting;
    impl crate::value::ToValue for MethodSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_data_encrypted {
                properties.insert(
                    "CacheDataEncrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_ttl_in_seconds {
                properties.insert(
                    "CacheTtlInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.caching_enabled {
                properties.insert(
                    "CachingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_trace_enabled {
                properties.insert(
                    "DataTraceEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_method {
                properties.insert(
                    "HttpMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_level {
                properties.insert(
                    "LoggingLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics_enabled {
                properties.insert(
                    "MetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_path {
                properties.insert(
                    "ResourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throttling_burst_limit {
                properties.insert(
                    "ThrottlingBurstLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throttling_rate_limit {
                properties.insert(
                    "ThrottlingRateLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod usageplan {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-apistage.html
    pub struct ApiStage_ {
        pub api_id: Option<crate::value::ExpString>,
        pub stage: Option<crate::value::ExpString>,
        pub throttle: Option<std::collections::BTreeMap<String, ThrottleSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_UsagePlan_ApiStage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::UsagePlan.ApiStage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_UsagePlan_ApiStage as ApiStage;
    impl crate::value::ToValue for ApiStage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.api_id {
                properties.insert("ApiId".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.stage {
                properties.insert("Stage".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.throttle {
                properties.insert(
                    "Throttle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-quotasettings.html
    pub struct QuotaSettings_ {
        pub limit: Option<i32>,
        pub offset: Option<i32>,
        pub period: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_UsagePlan_QuotaSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::UsagePlan.QuotaSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_UsagePlan_QuotaSettings as QuotaSettings;
    impl crate::value::ToValue for QuotaSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.limit {
                properties.insert("Limit".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.offset {
                properties.insert("Offset".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.period {
                properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigateway-usageplan-throttlesettings.html
    pub struct ThrottleSettings_ {
        pub burst_limit: Option<i32>,
        pub rate_limit: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigateway_UsagePlan_ThrottleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ApiGateway::UsagePlan.ThrottleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigateway_UsagePlan_ThrottleSettings as ThrottleSettings;
    impl crate::value::ToValue for ThrottleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.burst_limit {
                properties.insert(
                    "BurstLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_limit {
                properties.insert(
                    "RateLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-account.html
pub struct Account_ {
    pub cloud_watch_role_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_Account {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::Account"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_Account as Account;
impl crate::template::ToResource for Account_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Account"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cloud_watch_role_arn {
            properties.insert(
                "CloudWatchRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-apikey.html
pub struct ApiKey_ {
    pub customer_id: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub generate_distinct_id: Option<crate::value::ExpBool>,
    pub name: Option<crate::value::ExpString>,
    pub stage_keys: Option<Vec<super::apigateway::apikey::StageKey_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub value: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_ApiKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::ApiKey"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_ApiKey as ApiKey;
impl crate::template::ToResource for ApiKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApiKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.customer_id {
            properties.insert(
                "CustomerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.generate_distinct_id {
            properties.insert(
                "GenerateDistinctId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.stage_keys {
            properties.insert(
                "StageKeys".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.value {
            properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-authorizer.html
pub struct Authorizer_ {
    pub auth_type: Option<crate::value::ExpString>,
    pub authorizer_credentials: Option<crate::value::ExpString>,
    pub authorizer_result_ttl_in_seconds: Option<i32>,
    pub authorizer_uri: Option<crate::value::ExpString>,
    pub identity_source: Option<crate::value::ExpString>,
    pub identity_validation_expression: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub provider_ar_ns: Option<Vec<crate::value::ExpString>>,
    pub rest_api_id: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_Authorizer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::Authorizer"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_Authorizer as Authorizer;
impl crate::template::ToResource for Authorizer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Authorizer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auth_type {
            properties.insert(
                "AuthType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorizer_credentials {
            properties.insert(
                "AuthorizerCredentials".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.identity_source {
            properties.insert(
                "IdentitySource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_validation_expression {
            properties.insert(
                "IdentityValidationExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.provider_ar_ns {
            properties.insert(
                "ProviderARNs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmapping.html
pub struct BasePathMapping_ {
    pub base_path: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub id: Option<crate::value::ExpString>,
    pub rest_api_id: Option<crate::value::ExpString>,
    pub stage: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_BasePathMapping {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::BasePathMapping"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_BasePathMapping as BasePathMapping;
impl crate::template::ToResource for BasePathMapping_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BasePathMapping"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.base_path {
            properties.insert(
                "BasePath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.id {
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.rest_api_id {
            properties.insert(
                "RestApiId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stage {
            properties.insert("Stage".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-basepathmappingv2.html
pub struct BasePathMappingV2_ {
    pub base_path: Option<crate::value::ExpString>,
    pub domain_name_arn: crate::value::ExpString,
    pub rest_api_id: crate::value::ExpString,
    pub stage: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_BasePathMappingV2 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::BasePathMappingV2"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_BasePathMappingV2 as BasePathMappingV2;
impl crate::template::ToResource for BasePathMappingV2_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BasePathMappingV2"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.base_path {
            properties.insert(
                "BasePath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainNameArn".to_string(),
            crate::value::ToValue::to_value(&self.domain_name_arn),
        );
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        if let Some(ref value) = self.stage {
            properties.insert("Stage".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-clientcertificate.html
pub struct ClientCertificate_ {
    pub description: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_ClientCertificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::ClientCertificate"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_ClientCertificate as ClientCertificate;
impl crate::template::ToResource for ClientCertificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ClientCertificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-deployment.html
pub struct Deployment_ {
    pub deployment_canary_settings:
        Option<super::apigateway::deployment::DeploymentCanarySettings_>,
    pub description: Option<crate::value::ExpString>,
    pub rest_api_id: crate::value::ExpString,
    pub stage_description: Option<super::apigateway::deployment::StageDescription_>,
    pub stage_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_Deployment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::Deployment"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_Deployment as Deployment;
impl crate::template::ToResource for Deployment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Deployment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deployment_canary_settings {
            properties.insert(
                "DeploymentCanarySettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        if let Some(ref value) = self.stage_description {
            properties.insert(
                "StageDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stage_name {
            properties.insert(
                "StageName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationpart.html
pub struct DocumentationPart_ {
    pub location: super::apigateway::documentationpart::Location_,
    pub properties: crate::value::ExpString,
    pub rest_api_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_DocumentationPart {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::DocumentationPart"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_DocumentationPart as DocumentationPart;
impl crate::template::ToResource for DocumentationPart_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DocumentationPart"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Location".to_string(),
            crate::value::ToValue::to_value(&self.location),
        );
        properties.insert(
            "Properties".to_string(),
            crate::value::ToValue::to_value(&self.properties),
        );
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-documentationversion.html
pub struct DocumentationVersion_ {
    pub description: Option<crate::value::ExpString>,
    pub documentation_version: crate::value::ExpString,
    pub rest_api_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_DocumentationVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::DocumentationVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_DocumentationVersion as DocumentationVersion;
impl crate::template::ToResource for DocumentationVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DocumentationVersion"),
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
            "DocumentationVersion".to_string(),
            crate::value::ToValue::to_value(&self.documentation_version),
        );
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainname.html
pub struct DomainName_ {
    pub certificate_arn: Option<crate::value::ExpString>,
    pub domain_name: Option<crate::value::ExpString>,
    pub endpoint_configuration: Option<super::apigateway::domainname::EndpointConfiguration_>,
    pub mutual_tls_authentication: Option<super::apigateway::domainname::MutualTlsAuthentication_>,
    pub ownership_verification_certificate_arn: Option<crate::value::ExpString>,
    pub regional_certificate_arn: Option<crate::value::ExpString>,
    pub routing_mode: Option<crate::value::ExpString>,
    pub security_policy: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_DomainName {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::DomainName"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_DomainName as DomainName;
impl crate::template::ToResource for DomainName_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainName"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_arn {
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_name {
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_configuration {
            properties.insert(
                "EndpointConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mutual_tls_authentication {
            properties.insert(
                "MutualTlsAuthentication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ownership_verification_certificate_arn {
            properties.insert(
                "OwnershipVerificationCertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.regional_certificate_arn {
            properties.insert(
                "RegionalCertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.routing_mode {
            properties.insert(
                "RoutingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_policy {
            properties.insert(
                "SecurityPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainnameaccessassociation.html
pub struct DomainNameAccessAssociation_ {
    pub access_association_source: crate::value::ExpString,
    pub access_association_source_type: crate::value::ExpString,
    pub domain_name_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_DomainNameAccessAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::DomainNameAccessAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_DomainNameAccessAssociation as DomainNameAccessAssociation;
impl crate::template::ToResource for DomainNameAccessAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "DomainNameAccessAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AccessAssociationSource".to_string(),
            crate::value::ToValue::to_value(&self.access_association_source),
        );
        properties.insert(
            "AccessAssociationSourceType".to_string(),
            crate::value::ToValue::to_value(&self.access_association_source_type),
        );
        properties.insert(
            "DomainNameArn".to_string(),
            crate::value::ToValue::to_value(&self.domain_name_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-domainnamev2.html
pub struct DomainNameV2_ {
    pub certificate_arn: Option<crate::value::ExpString>,
    pub domain_name: Option<crate::value::ExpString>,
    pub endpoint_configuration: Option<super::apigateway::domainnamev2::EndpointConfiguration_>,
    pub policy: Option<serde_json::Value>,
    pub routing_mode: Option<crate::value::ExpString>,
    pub security_policy: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_DomainNameV2 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::DomainNameV2"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_DomainNameV2 as DomainNameV2;
impl crate::template::ToResource for DomainNameV2_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainNameV2"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_arn {
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_name {
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_configuration {
            properties.insert(
                "EndpointConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.routing_mode {
            properties.insert(
                "RoutingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_policy {
            properties.insert(
                "SecurityPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-gatewayresponse.html
pub struct GatewayResponse_ {
    pub response_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub response_templates: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub response_type: crate::value::ExpString,
    pub rest_api_id: crate::value::ExpString,
    pub status_code: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_GatewayResponse {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::GatewayResponse"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_GatewayResponse as GatewayResponse;
impl crate::template::ToResource for GatewayResponse_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GatewayResponse"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.response_parameters {
            properties.insert(
                "ResponseParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.response_templates {
            properties.insert(
                "ResponseTemplates".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResponseType".to_string(),
            crate::value::ToValue::to_value(&self.response_type),
        );
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        if let Some(ref value) = self.status_code {
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-method.html
pub struct Method_ {
    pub api_key_required: Option<crate::value::ExpBool>,
    pub authorization_scopes: Option<Vec<crate::value::ExpString>>,
    pub authorization_type: Option<crate::value::ExpString>,
    pub authorizer_id: Option<crate::value::ExpString>,
    pub http_method: crate::value::ExpString,
    pub integration: Option<super::apigateway::method::Integration_>,
    pub method_responses: Option<Vec<super::apigateway::method::MethodResponse_>>,
    pub operation_name: Option<crate::value::ExpString>,
    pub request_models: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub request_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub request_validator_id: Option<crate::value::ExpString>,
    pub resource_id: crate::value::ExpString,
    pub rest_api_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_Method {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::Method"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_Method as Method;
impl crate::template::ToResource for Method_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Method"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.api_key_required {
            properties.insert(
                "ApiKeyRequired".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorization_scopes {
            properties.insert(
                "AuthorizationScopes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorization_type {
            properties.insert(
                "AuthorizationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorizer_id {
            properties.insert(
                "AuthorizerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "HttpMethod".to_string(),
            crate::value::ToValue::to_value(&self.http_method),
        );
        if let Some(ref value) = self.integration {
            properties.insert(
                "Integration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.method_responses {
            properties.insert(
                "MethodResponses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.operation_name {
            properties.insert(
                "OperationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.request_models {
            properties.insert(
                "RequestModels".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.request_parameters {
            properties.insert(
                "RequestParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.request_validator_id {
            properties.insert(
                "RequestValidatorId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceId".to_string(),
            crate::value::ToValue::to_value(&self.resource_id),
        );
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-model.html
pub struct Model_ {
    pub content_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub rest_api_id: crate::value::ExpString,
    pub schema: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_Model {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::Model"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_Model as Model;
impl crate::template::ToResource for Model_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Model"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.content_type {
            properties.insert(
                "ContentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        if let Some(ref value) = self.schema {
            properties.insert("Schema".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-requestvalidator.html
pub struct RequestValidator_ {
    pub name: Option<crate::value::ExpString>,
    pub rest_api_id: crate::value::ExpString,
    pub validate_request_body: Option<crate::value::ExpBool>,
    pub validate_request_parameters: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_RequestValidator {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::RequestValidator"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_RequestValidator as RequestValidator;
impl crate::template::ToResource for RequestValidator_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RequestValidator"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        if let Some(ref value) = self.validate_request_body {
            properties.insert(
                "ValidateRequestBody".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.validate_request_parameters {
            properties.insert(
                "ValidateRequestParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-resource.html
pub struct Resource_ {
    pub parent_id: crate::value::ExpString,
    pub path_part: crate::value::ExpString,
    pub rest_api_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_Resource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::Resource"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_Resource as Resource;
impl crate::template::ToResource for Resource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Resource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ParentId".to_string(),
            crate::value::ToValue::to_value(&self.parent_id),
        );
        properties.insert(
            "PathPart".to_string(),
            crate::value::ToValue::to_value(&self.path_part),
        );
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-restapi.html
pub struct RestApi_ {
    pub api_key_source_type: Option<crate::value::ExpString>,
    pub binary_media_types: Option<Vec<crate::value::ExpString>>,
    pub body: Option<serde_json::Value>,
    pub body_s3_location: Option<super::apigateway::restapi::S3Location_>,
    pub clone_from: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub disable_execute_api_endpoint: Option<crate::value::ExpBool>,
    pub endpoint_configuration: Option<super::apigateway::restapi::EndpointConfiguration_>,
    pub fail_on_warnings: Option<crate::value::ExpBool>,
    pub minimum_compression_size: Option<i32>,
    pub mode: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub policy: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_RestApi {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::RestApi"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_RestApi as RestApi;
impl crate::template::ToResource for RestApi_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RestApi"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.api_key_source_type {
            properties.insert(
                "ApiKeySourceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.binary_media_types {
            properties.insert(
                "BinaryMediaTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.body {
            properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.body_s3_location {
            properties.insert(
                "BodyS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.clone_from {
            properties.insert(
                "CloneFrom".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.disable_execute_api_endpoint {
            properties.insert(
                "DisableExecuteApiEndpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint_configuration {
            properties.insert(
                "EndpointConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fail_on_warnings {
            properties.insert(
                "FailOnWarnings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.minimum_compression_size {
            properties.insert(
                "MinimumCompressionSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mode {
            properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-stage.html
pub struct Stage_ {
    pub access_log_setting: Option<super::apigateway::stage::AccessLogSetting_>,
    pub cache_cluster_enabled: Option<crate::value::ExpBool>,
    pub cache_cluster_size: Option<crate::value::ExpString>,
    pub canary_setting: Option<super::apigateway::stage::CanarySetting_>,
    pub client_certificate_id: Option<crate::value::ExpString>,
    pub deployment_id: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub documentation_version: Option<crate::value::ExpString>,
    pub method_settings: Option<Vec<super::apigateway::stage::MethodSetting_>>,
    pub rest_api_id: crate::value::ExpString,
    pub stage_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tracing_enabled: Option<crate::value::ExpBool>,
    pub variables: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_Stage {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::Stage"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_Stage as Stage;
impl crate::template::ToResource for Stage_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stage"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_log_setting {
            properties.insert(
                "AccessLogSetting".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_cluster_enabled {
            properties.insert(
                "CacheClusterEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_cluster_size {
            properties.insert(
                "CacheClusterSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.canary_setting {
            properties.insert(
                "CanarySetting".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_certificate_id {
            properties.insert(
                "ClientCertificateId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_id {
            properties.insert(
                "DeploymentId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.documentation_version {
            properties.insert(
                "DocumentationVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.method_settings {
            properties.insert(
                "MethodSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RestApiId".to_string(),
            crate::value::ToValue::to_value(&self.rest_api_id),
        );
        if let Some(ref value) = self.stage_name {
            properties.insert(
                "StageName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tracing_enabled {
            properties.insert(
                "TracingEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.variables {
            properties.insert(
                "Variables".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplan.html
pub struct UsagePlan_ {
    pub api_stages: Option<Vec<super::apigateway::usageplan::ApiStage_>>,
    pub description: Option<crate::value::ExpString>,
    pub quota: Option<super::apigateway::usageplan::QuotaSettings_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub throttle: Option<super::apigateway::usageplan::ThrottleSettings_>,
    pub usage_plan_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_UsagePlan {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::UsagePlan"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_UsagePlan as UsagePlan;
impl crate::template::ToResource for UsagePlan_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UsagePlan"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.api_stages {
            properties.insert(
                "ApiStages".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.quota {
            properties.insert("Quota".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.throttle {
            properties.insert(
                "Throttle".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.usage_plan_name {
            properties.insert(
                "UsagePlanName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-usageplankey.html
pub struct UsagePlanKey_ {
    pub key_id: crate::value::ExpString,
    pub key_type: crate::value::ExpString,
    pub usage_plan_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_UsagePlanKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::UsagePlanKey"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_UsagePlanKey as UsagePlanKey;
impl crate::template::ToResource for UsagePlanKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UsagePlanKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "KeyId".to_string(),
            crate::value::ToValue::to_value(&self.key_id),
        );
        properties.insert(
            "KeyType".to_string(),
            crate::value::ToValue::to_value(&self.key_type),
        );
        properties.insert(
            "UsagePlanId".to_string(),
            crate::value::ToValue::to_value(&self.usage_plan_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigateway-vpclink.html
pub struct VpcLink_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arns: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigateway_VpcLink {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ApiGateway::VpcLink"
        $($field $value)*)
    };
}
pub use crate::__aws_apigateway_VpcLink as VpcLink;
impl crate::template::ToResource for VpcLink_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGateway"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcLink"),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetArns".to_string(),
            crate::value::ToValue::to_value(&self.target_arns),
        );
        properties
    }
}
