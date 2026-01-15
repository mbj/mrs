pub mod api {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-bodys3location.html
    pub struct BodyS3Location_ {
        pub bucket: Option<crate::value::ExpString>,
        pub etag: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Api_BodyS3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Api.BodyS3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Api_BodyS3Location as BodyS3Location;
    impl crate::value::ToValue for BodyS3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket {
                properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.etag {
                properties.insert("Etag".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-api-cors.html
    pub struct Cors_ {
        pub allow_credentials: Option<crate::value::ExpBool>,
        pub allow_headers: Option<Vec<crate::value::ExpString>>,
        pub allow_methods: Option<Vec<crate::value::ExpString>>,
        pub allow_origins: Option<Vec<crate::value::ExpString>>,
        pub expose_headers: Option<Vec<crate::value::ExpString>>,
        pub max_age: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Api_Cors {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Api.Cors"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Api_Cors as Cors;
    impl crate::value::ToValue for Cors_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_credentials {
                properties.insert(
                    "AllowCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allow_headers {
                properties.insert(
                    "AllowHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allow_methods {
                properties.insert(
                    "AllowMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allow_origins {
                properties.insert(
                    "AllowOrigins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expose_headers {
                properties.insert(
                    "ExposeHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_age {
                properties.insert("MaxAge".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod apigatewaymanagedoverrides {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-accesslogsettings.html
    pub struct AccessLogSettings_ {
        pub destination_arn: Option<crate::value::ExpString>,
        pub format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_ApiGatewayManagedOverrides_AccessLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::ApiGatewayManagedOverrides.AccessLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_ApiGatewayManagedOverrides_AccessLogSettings as AccessLogSettings;
    impl crate::value::ToValue for AccessLogSettings_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-integrationoverrides.html
    pub struct IntegrationOverrides_ {
        pub description: Option<crate::value::ExpString>,
        pub integration_method: Option<crate::value::ExpString>,
        pub payload_format_version: Option<crate::value::ExpString>,
        pub timeout_in_millis: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_ApiGatewayManagedOverrides_IntegrationOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::ApiGatewayManagedOverrides.IntegrationOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_ApiGatewayManagedOverrides_IntegrationOverrides as IntegrationOverrides;
    impl crate::value::ToValue for IntegrationOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integration_method {
                properties.insert(
                    "IntegrationMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload_format_version {
                properties.insert(
                    "PayloadFormatVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_in_millis {
                properties.insert(
                    "TimeoutInMillis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routeoverrides.html
    pub struct RouteOverrides_ {
        pub authorization_scopes: Option<Vec<crate::value::ExpString>>,
        pub authorization_type: Option<crate::value::ExpString>,
        pub authorizer_id: Option<crate::value::ExpString>,
        pub operation_name: Option<crate::value::ExpString>,
        pub target: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_ApiGatewayManagedOverrides_RouteOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::ApiGatewayManagedOverrides.RouteOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_ApiGatewayManagedOverrides_RouteOverrides as RouteOverrides;
    impl crate::value::ToValue for RouteOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.operation_name {
                properties.insert(
                    "OperationName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-routesettings.html
    pub struct RouteSettings_ {
        pub data_trace_enabled: Option<crate::value::ExpBool>,
        pub detailed_metrics_enabled: Option<crate::value::ExpBool>,
        pub logging_level: Option<crate::value::ExpString>,
        pub throttling_burst_limit: Option<i64>,
        pub throttling_rate_limit: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_ApiGatewayManagedOverrides_RouteSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::ApiGatewayManagedOverrides.RouteSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_ApiGatewayManagedOverrides_RouteSettings as RouteSettings;
    impl crate::value::ToValue for RouteSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_trace_enabled {
                properties.insert(
                    "DataTraceEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.detailed_metrics_enabled {
                properties.insert(
                    "DetailedMetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_level {
                properties.insert(
                    "LoggingLevel".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-apigatewaymanagedoverrides-stageoverrides.html
    pub struct StageOverrides_ {
        pub access_log_settings: Option<Box<AccessLogSettings_>>,
        pub auto_deploy: Option<crate::value::ExpBool>,
        pub default_route_settings: Option<Box<RouteSettings_>>,
        pub description: Option<crate::value::ExpString>,
        pub route_settings: Option<serde_json::Value>,
        pub stage_variables: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_ApiGatewayManagedOverrides_StageOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::ApiGatewayManagedOverrides.StageOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_ApiGatewayManagedOverrides_StageOverrides as StageOverrides;
    impl crate::value::ToValue for StageOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_log_settings {
                properties.insert(
                    "AccessLogSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auto_deploy {
                properties.insert(
                    "AutoDeploy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_route_settings {
                properties.insert(
                    "DefaultRouteSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route_settings {
                properties.insert(
                    "RouteSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stage_variables {
                properties.insert(
                    "StageVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod authorizer {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-authorizer-jwtconfiguration.html
    pub struct JWTConfiguration_ {
        pub audience: Option<Vec<crate::value::ExpString>>,
        pub issuer: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Authorizer_JWTConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Authorizer.JWTConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Authorizer_JWTConfiguration as JWTConfiguration;
    impl crate::value::ToValue for JWTConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audience {
                properties.insert(
                    "Audience".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.issuer {
                properties.insert("Issuer".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod domainname {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-domainnameconfiguration.html
    pub struct DomainNameConfiguration_ {
        pub certificate_arn: Option<crate::value::ExpString>,
        pub certificate_name: Option<crate::value::ExpString>,
        pub endpoint_type: Option<crate::value::ExpString>,
        pub ip_address_type: Option<crate::value::ExpString>,
        pub ownership_verification_certificate_arn: Option<crate::value::ExpString>,
        pub security_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_DomainName_DomainNameConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::DomainName.DomainNameConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_DomainName_DomainNameConfiguration as DomainNameConfiguration;
    impl crate::value::ToValue for DomainNameConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_arn {
                properties.insert(
                    "CertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.certificate_name {
                properties.insert(
                    "CertificateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_type {
                properties.insert(
                    "EndpointType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ownership_verification_certificate_arn {
                properties.insert(
                    "OwnershipVerificationCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_policy {
                properties.insert(
                    "SecurityPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-domainname-mutualtlsauthentication.html
    pub struct MutualTlsAuthentication_ {
        pub truststore_uri: Option<crate::value::ExpString>,
        pub truststore_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_DomainName_MutualTlsAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::DomainName.MutualTlsAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_DomainName_MutualTlsAuthentication as MutualTlsAuthentication;
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
pub mod integration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-responseparameter.html
    pub struct ResponseParameter_ {
        pub destination: Option<crate::value::ExpString>,
        pub source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Integration_ResponseParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Integration.ResponseParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Integration_ResponseParameter as ResponseParameter;
    impl crate::value::ToValue for ResponseParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-responseparametermap.html
    pub struct ResponseParameterMap_ {
        pub response_parameters: Option<Vec<ResponseParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Integration_ResponseParameterMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Integration.ResponseParameterMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Integration_ResponseParameterMap as ResponseParameterMap;
    impl crate::value::ToValue for ResponseParameterMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.response_parameters {
                properties.insert(
                    "ResponseParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-integration-tlsconfig.html
    pub struct TlsConfig_ {
        pub server_name_to_verify: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Integration_TlsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Integration.TlsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Integration_TlsConfig as TlsConfig;
    impl crate::value::ToValue for TlsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.server_name_to_verify {
                properties.insert(
                    "ServerNameToVerify".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod routeresponse {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routeresponse-parameterconstraints.html
    pub struct ParameterConstraints_ {
        pub required: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_RouteResponse_ParameterConstraints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::RouteResponse.ParameterConstraints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_RouteResponse_ParameterConstraints as ParameterConstraints;
    impl crate::value::ToValue for ParameterConstraints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Required".to_string(),
                crate::value::ToValue::to_value(&self.required),
            );
            properties.into()
        }
    }
}
pub mod routingrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routingrule-action.html
    pub struct Action_ {
        pub invoke_api: Box<ActionInvokeApi_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_RoutingRule_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::RoutingRule.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_RoutingRule_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InvokeApi".to_string(),
                crate::value::ToValue::to_value(&self.invoke_api),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routingrule-actioninvokeapi.html
    pub struct ActionInvokeApi_ {
        pub api_id: crate::value::ExpString,
        pub stage: crate::value::ExpString,
        pub strip_base_path: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_RoutingRule_ActionInvokeApi {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::RoutingRule.ActionInvokeApi"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_RoutingRule_ActionInvokeApi as ActionInvokeApi;
    impl crate::value::ToValue for ActionInvokeApi_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiId".to_string(),
                crate::value::ToValue::to_value(&self.api_id),
            );
            properties.insert(
                "Stage".to_string(),
                crate::value::ToValue::to_value(&self.stage),
            );
            if let Some(ref value) = self.strip_base_path {
                properties.insert(
                    "StripBasePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routingrule-condition.html
    pub struct Condition_ {
        pub match_base_paths: Option<Box<MatchBasePaths_>>,
        pub match_headers: Option<Box<MatchHeaders_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_RoutingRule_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::RoutingRule.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_RoutingRule_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.match_base_paths {
                properties.insert(
                    "MatchBasePaths".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_headers {
                properties.insert(
                    "MatchHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routingrule-matchbasepaths.html
    pub struct MatchBasePaths_ {
        pub any_of: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_RoutingRule_MatchBasePaths {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::RoutingRule.MatchBasePaths"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_RoutingRule_MatchBasePaths as MatchBasePaths;
    impl crate::value::ToValue for MatchBasePaths_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AnyOf".to_string(),
                crate::value::ToValue::to_value(&self.any_of),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routingrule-matchheadervalue.html
    pub struct MatchHeaderValue_ {
        pub header: crate::value::ExpString,
        pub value_glob: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_RoutingRule_MatchHeaderValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::RoutingRule.MatchHeaderValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_RoutingRule_MatchHeaderValue as MatchHeaderValue;
    impl crate::value::ToValue for MatchHeaderValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Header".to_string(),
                crate::value::ToValue::to_value(&self.header),
            );
            properties.insert(
                "ValueGlob".to_string(),
                crate::value::ToValue::to_value(&self.value_glob),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-routingrule-matchheaders.html
    pub struct MatchHeaders_ {
        pub any_of: Vec<MatchHeaderValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_RoutingRule_MatchHeaders {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::RoutingRule.MatchHeaders"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_RoutingRule_MatchHeaders as MatchHeaders;
    impl crate::value::ToValue for MatchHeaders_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AnyOf".to_string(),
                crate::value::ToValue::to_value(&self.any_of),
            );
            properties.into()
        }
    }
}
pub mod stage {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-accesslogsettings.html
    pub struct AccessLogSettings_ {
        pub destination_arn: Option<crate::value::ExpString>,
        pub format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Stage_AccessLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Stage.AccessLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Stage_AccessLogSettings as AccessLogSettings;
    impl crate::value::ToValue for AccessLogSettings_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apigatewayv2-stage-routesettings.html
    pub struct RouteSettings_ {
        pub data_trace_enabled: Option<crate::value::ExpBool>,
        pub detailed_metrics_enabled: Option<crate::value::ExpBool>,
        pub logging_level: Option<crate::value::ExpString>,
        pub throttling_burst_limit: Option<i64>,
        pub throttling_rate_limit: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apigatewayv2_Stage_RouteSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ApiGatewayV2::Stage.RouteSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apigatewayv2_Stage_RouteSettings as RouteSettings;
    impl crate::value::ToValue for RouteSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_trace_enabled {
                properties.insert(
                    "DataTraceEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.detailed_metrics_enabled {
                properties.insert(
                    "DetailedMetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging_level {
                properties.insert(
                    "LoggingLevel".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-api.html
pub struct Api_ {
    pub api_key_selection_expression: Option<crate::value::ExpString>,
    pub base_path: Option<crate::value::ExpString>,
    pub body: Option<serde_json::Value>,
    pub body_s3_location: Option<super::apigatewayv2::api::BodyS3Location_>,
    pub cors_configuration: Option<super::apigatewayv2::api::Cors_>,
    pub credentials_arn: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub disable_execute_api_endpoint: Option<crate::value::ExpBool>,
    pub disable_schema_validation: Option<crate::value::ExpBool>,
    pub fail_on_warnings: Option<crate::value::ExpBool>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub protocol_type: Option<crate::value::ExpString>,
    pub route_key: Option<crate::value::ExpString>,
    pub route_selection_expression: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub target: Option<crate::value::ExpString>,
    pub version: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_Api {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::Api"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_Api as Api;
impl crate::template::ToResource for Api_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Api"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.api_key_selection_expression {
            properties.insert(
                "ApiKeySelectionExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.base_path {
            properties.insert(
                "BasePath".to_string(),
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
        if let Some(ref value) = self.cors_configuration {
            properties.insert(
                "CorsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.credentials_arn {
            properties.insert(
                "CredentialsArn".to_string(),
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
        if let Some(ref value) = self.disable_schema_validation {
            properties.insert(
                "DisableSchemaValidation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fail_on_warnings {
            properties.insert(
                "FailOnWarnings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.protocol_type {
            properties.insert(
                "ProtocolType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.route_key {
            properties.insert(
                "RouteKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.route_selection_expression {
            properties.insert(
                "RouteSelectionExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target {
            properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.version {
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apigatewaymanagedoverrides.html
pub struct ApiGatewayManagedOverrides_ {
    pub api_id: crate::value::ExpString,
    pub integration: Option<super::apigatewayv2::apigatewaymanagedoverrides::IntegrationOverrides_>,
    pub route: Option<super::apigatewayv2::apigatewaymanagedoverrides::RouteOverrides_>,
    pub stage: Option<super::apigatewayv2::apigatewaymanagedoverrides::StageOverrides_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_ApiGatewayManagedOverrides {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::ApiGatewayManagedOverrides"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_ApiGatewayManagedOverrides as ApiGatewayManagedOverrides;
impl crate::template::ToResource for ApiGatewayManagedOverrides_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ApiGatewayManagedOverrides",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.integration {
            properties.insert(
                "Integration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.route {
            properties.insert("Route".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.stage {
            properties.insert("Stage".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-apimapping.html
pub struct ApiMapping_ {
    pub api_id: crate::value::ExpString,
    pub api_mapping_key: Option<crate::value::ExpString>,
    pub domain_name: crate::value::ExpString,
    pub stage: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_ApiMapping {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::ApiMapping"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_ApiMapping as ApiMapping;
impl crate::template::ToResource for ApiMapping_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApiMapping"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.api_mapping_key {
            properties.insert(
                "ApiMappingKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        properties.insert(
            "Stage".to_string(),
            crate::value::ToValue::to_value(&self.stage),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-authorizer.html
pub struct Authorizer_ {
    pub api_id: crate::value::ExpString,
    pub authorizer_credentials_arn: Option<crate::value::ExpString>,
    pub authorizer_payload_format_version: Option<crate::value::ExpString>,
    pub authorizer_result_ttl_in_seconds: Option<i64>,
    pub authorizer_type: crate::value::ExpString,
    pub authorizer_uri: Option<crate::value::ExpString>,
    pub enable_simple_responses: Option<crate::value::ExpBool>,
    pub identity_source: Option<Vec<crate::value::ExpString>>,
    pub identity_validation_expression: Option<crate::value::ExpString>,
    pub jwt_configuration: Option<super::apigatewayv2::authorizer::JWTConfiguration_>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_Authorizer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::Authorizer"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_Authorizer as Authorizer;
impl crate::template::ToResource for Authorizer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Authorizer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.authorizer_credentials_arn {
            properties.insert(
                "AuthorizerCredentialsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorizer_payload_format_version {
            properties.insert(
                "AuthorizerPayloadFormatVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authorizer_result_ttl_in_seconds {
            properties.insert(
                "AuthorizerResultTtlInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AuthorizerType".to_string(),
            crate::value::ToValue::to_value(&self.authorizer_type),
        );
        if let Some(ref value) = self.authorizer_uri {
            properties.insert(
                "AuthorizerUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_simple_responses {
            properties.insert(
                "EnableSimpleResponses".to_string(),
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
        if let Some(ref value) = self.jwt_configuration {
            properties.insert(
                "JwtConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-deployment.html
pub struct Deployment_ {
    pub api_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub stage_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_Deployment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::Deployment"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_Deployment as Deployment;
impl crate::template::ToResource for Deployment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Deployment"),
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
        if let Some(ref value) = self.stage_name {
            properties.insert(
                "StageName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-domainname.html
pub struct DomainName_ {
    pub domain_name: crate::value::ExpString,
    pub domain_name_configurations:
        Option<Vec<super::apigatewayv2::domainname::DomainNameConfiguration_>>,
    pub mutual_tls_authentication:
        Option<super::apigatewayv2::domainname::MutualTlsAuthentication_>,
    pub routing_mode: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_DomainName {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::DomainName"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_DomainName as DomainName;
impl crate::template::ToResource for DomainName_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainName"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.domain_name_configurations {
            properties.insert(
                "DomainNameConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mutual_tls_authentication {
            properties.insert(
                "MutualTlsAuthentication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.routing_mode {
            properties.insert(
                "RoutingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integration.html
pub struct Integration_ {
    pub api_id: crate::value::ExpString,
    pub connection_id: Option<crate::value::ExpString>,
    pub connection_type: Option<crate::value::ExpString>,
    pub content_handling_strategy: Option<crate::value::ExpString>,
    pub credentials_arn: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub integration_method: Option<crate::value::ExpString>,
    pub integration_subtype: Option<crate::value::ExpString>,
    pub integration_type: crate::value::ExpString,
    pub integration_uri: Option<crate::value::ExpString>,
    pub passthrough_behavior: Option<crate::value::ExpString>,
    pub payload_format_version: Option<crate::value::ExpString>,
    pub request_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub request_templates: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub response_parameters: Option<
        std::collections::BTreeMap<String, super::apigatewayv2::integration::ResponseParameterMap_>,
    >,
    pub template_selection_expression: Option<crate::value::ExpString>,
    pub timeout_in_millis: Option<i64>,
    pub tls_config: Option<super::apigatewayv2::integration::TlsConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_Integration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::Integration"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_Integration as Integration;
impl crate::template::ToResource for Integration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Integration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
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
        if let Some(ref value) = self.content_handling_strategy {
            properties.insert(
                "ContentHandlingStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.credentials_arn {
            properties.insert(
                "CredentialsArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.integration_method {
            properties.insert(
                "IntegrationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.integration_subtype {
            properties.insert(
                "IntegrationSubtype".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IntegrationType".to_string(),
            crate::value::ToValue::to_value(&self.integration_type),
        );
        if let Some(ref value) = self.integration_uri {
            properties.insert(
                "IntegrationUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.passthrough_behavior {
            properties.insert(
                "PassthroughBehavior".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.payload_format_version {
            properties.insert(
                "PayloadFormatVersion".to_string(),
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
        if let Some(ref value) = self.response_parameters {
            properties.insert(
                "ResponseParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.template_selection_expression {
            properties.insert(
                "TemplateSelectionExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.timeout_in_millis {
            properties.insert(
                "TimeoutInMillis".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tls_config {
            properties.insert(
                "TlsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-integrationresponse.html
pub struct IntegrationResponse_ {
    pub api_id: crate::value::ExpString,
    pub content_handling_strategy: Option<crate::value::ExpString>,
    pub integration_id: crate::value::ExpString,
    pub integration_response_key: crate::value::ExpString,
    pub response_parameters: Option<serde_json::Value>,
    pub response_templates: Option<serde_json::Value>,
    pub template_selection_expression: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_IntegrationResponse {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::IntegrationResponse"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_IntegrationResponse as IntegrationResponse;
impl crate::template::ToResource for IntegrationResponse_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IntegrationResponse"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.content_handling_strategy {
            properties.insert(
                "ContentHandlingStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IntegrationId".to_string(),
            crate::value::ToValue::to_value(&self.integration_id),
        );
        properties.insert(
            "IntegrationResponseKey".to_string(),
            crate::value::ToValue::to_value(&self.integration_response_key),
        );
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
        if let Some(ref value) = self.template_selection_expression {
            properties.insert(
                "TemplateSelectionExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-model.html
pub struct Model_ {
    pub api_id: crate::value::ExpString,
    pub content_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub schema: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_Model {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::Model"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_Model as Model;
impl crate::template::ToResource for Model_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Model"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
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
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Schema".to_string(),
            crate::value::ToValue::to_value(&self.schema),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-route.html
pub struct Route_ {
    pub api_id: crate::value::ExpString,
    pub api_key_required: Option<crate::value::ExpBool>,
    pub authorization_scopes: Option<Vec<crate::value::ExpString>>,
    pub authorization_type: Option<crate::value::ExpString>,
    pub authorizer_id: Option<crate::value::ExpString>,
    pub model_selection_expression: Option<crate::value::ExpString>,
    pub operation_name: Option<crate::value::ExpString>,
    pub request_models: Option<serde_json::Value>,
    pub request_parameters: Option<serde_json::Value>,
    pub route_key: crate::value::ExpString,
    pub route_response_selection_expression: Option<crate::value::ExpString>,
    pub target: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_Route {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::Route"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_Route as Route;
impl crate::template::ToResource for Route_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Route"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
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
        if let Some(ref value) = self.model_selection_expression {
            properties.insert(
                "ModelSelectionExpression".to_string(),
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
        properties.insert(
            "RouteKey".to_string(),
            crate::value::ToValue::to_value(&self.route_key),
        );
        if let Some(ref value) = self.route_response_selection_expression {
            properties.insert(
                "RouteResponseSelectionExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target {
            properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routeresponse.html
pub struct RouteResponse_ {
    pub api_id: crate::value::ExpString,
    pub model_selection_expression: Option<crate::value::ExpString>,
    pub response_models: Option<serde_json::Value>,
    pub response_parameters: Option<
        std::collections::BTreeMap<
            String,
            super::apigatewayv2::routeresponse::ParameterConstraints_,
        >,
    >,
    pub route_id: crate::value::ExpString,
    pub route_response_key: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_RouteResponse {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::RouteResponse"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_RouteResponse as RouteResponse;
impl crate::template::ToResource for RouteResponse_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteResponse"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.model_selection_expression {
            properties.insert(
                "ModelSelectionExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
            "RouteId".to_string(),
            crate::value::ToValue::to_value(&self.route_id),
        );
        properties.insert(
            "RouteResponseKey".to_string(),
            crate::value::ToValue::to_value(&self.route_response_key),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-routingrule.html
pub struct RoutingRule_ {
    pub actions: Vec<super::apigatewayv2::routingrule::Action_>,
    pub conditions: Vec<super::apigatewayv2::routingrule::Condition_>,
    pub domain_name_arn: crate::value::ExpString,
    pub priority: i64,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_RoutingRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::RoutingRule"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_RoutingRule as RoutingRule;
impl crate::template::ToResource for RoutingRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RoutingRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        properties.insert(
            "Conditions".to_string(),
            crate::value::ToValue::to_value(&self.conditions),
        );
        properties.insert(
            "DomainNameArn".to_string(),
            crate::value::ToValue::to_value(&self.domain_name_arn),
        );
        properties.insert(
            "Priority".to_string(),
            crate::value::ToValue::to_value(&self.priority),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-stage.html
pub struct Stage_ {
    pub access_log_settings: Option<super::apigatewayv2::stage::AccessLogSettings_>,
    pub access_policy_id: Option<crate::value::ExpString>,
    pub api_id: crate::value::ExpString,
    pub auto_deploy: Option<crate::value::ExpBool>,
    pub client_certificate_id: Option<crate::value::ExpString>,
    pub default_route_settings: Option<super::apigatewayv2::stage::RouteSettings_>,
    pub deployment_id: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub route_settings: Option<serde_json::Value>,
    pub stage_name: crate::value::ExpString,
    pub stage_variables: Option<serde_json::Value>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_Stage {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::Stage"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_Stage as Stage;
impl crate::template::ToResource for Stage_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stage"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_log_settings {
            properties.insert(
                "AccessLogSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.access_policy_id {
            properties.insert(
                "AccessPolicyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ApiId".to_string(),
            crate::value::ToValue::to_value(&self.api_id),
        );
        if let Some(ref value) = self.auto_deploy {
            properties.insert(
                "AutoDeploy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_certificate_id {
            properties.insert(
                "ClientCertificateId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_route_settings {
            properties.insert(
                "DefaultRouteSettings".to_string(),
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
        if let Some(ref value) = self.route_settings {
            properties.insert(
                "RouteSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StageName".to_string(),
            crate::value::ToValue::to_value(&self.stage_name),
        );
        if let Some(ref value) = self.stage_variables {
            properties.insert(
                "StageVariables".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apigatewayv2-vpclink.html
pub struct VpcLink_ {
    pub name: crate::value::ExpString,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apigatewayv2_VpcLink {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ApiGatewayV2::VpcLink"
        $($field $value)*)
    };
}
pub use crate::__aws_apigatewayv2_VpcLink as VpcLink;
impl crate::template::ToResource for VpcLink_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ApiGatewayV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcLink"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
