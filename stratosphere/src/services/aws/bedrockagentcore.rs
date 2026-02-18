pub mod browsercustom {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-browsercustom-browsernetworkconfiguration.html>
    pub struct BrowserNetworkConfiguration_ {
        pub network_mode: crate::value::ExpString,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_BrowserCustom_BrowserNetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::BrowserCustom.BrowserNetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_BrowserCustom_BrowserNetworkConfiguration as BrowserNetworkConfiguration;
    impl crate::value::ToValue for BrowserNetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkMode".to_string(),
                crate::value::ToValue::to_value(&self.network_mode),
            );
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-browsercustom-browsersigning.html>
    pub struct BrowserSigning_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_BrowserCustom_BrowserSigning {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::BrowserCustom.BrowserSigning"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_BrowserCustom_BrowserSigning as BrowserSigning;
    impl crate::value::ToValue for BrowserSigning_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-browsercustom-recordingconfig.html>
    pub struct RecordingConfig_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub s3_location: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_BrowserCustom_RecordingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::BrowserCustom.RecordingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_BrowserCustom_RecordingConfig as RecordingConfig;
    impl crate::value::ToValue for RecordingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_location {
                properties.insert(
                    "S3Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-browsercustom-s3location.html>
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_BrowserCustom_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::BrowserCustom.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_BrowserCustom_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Prefix".to_string(),
                crate::value::ToValue::to_value(&self.prefix),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-browsercustom-vpcconfig.html>
    pub struct VpcConfig_ {
        pub security_groups: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_BrowserCustom_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::BrowserCustom.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_BrowserCustom_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(&self.security_groups),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod codeinterpretercustom {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-codeinterpretercustom-codeinterpreternetworkconfiguration.html>
    pub struct CodeInterpreterNetworkConfiguration_ {
        pub network_mode: crate::value::ExpString,
        pub vpc_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_CodeInterpreterCustom_CodeInterpreterNetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::CodeInterpreterCustom.CodeInterpreterNetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_CodeInterpreterCustom_CodeInterpreterNetworkConfiguration as CodeInterpreterNetworkConfiguration;
    impl crate::value::ToValue for CodeInterpreterNetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkMode".to_string(),
                crate::value::ToValue::to_value(&self.network_mode),
            );
            if let Some(ref value) = self.vpc_config {
                properties.insert(
                    "VpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-codeinterpretercustom-vpcconfig.html>
    pub struct VpcConfig_ {
        pub security_groups: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_CodeInterpreterCustom_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::CodeInterpreterCustom.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_CodeInterpreterCustom_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(&self.security_groups),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
}
pub mod gateway {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-authorizerconfiguration.html>
    pub struct AuthorizerConfiguration_ {
        pub custom_jwt_authorizer: Box<CustomJWTAuthorizerConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_AuthorizerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.AuthorizerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_AuthorizerConfiguration as AuthorizerConfiguration;
    impl crate::value::ToValue for AuthorizerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CustomJWTAuthorizer".to_string(),
                crate::value::ToValue::to_value(&self.custom_jwt_authorizer),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-authorizingclaimmatchvaluetype.html>
    pub struct AuthorizingClaimMatchValueType_ {
        pub claim_match_operator: crate::value::ExpString,
        pub claim_match_value: Box<ClaimMatchValueType_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_AuthorizingClaimMatchValueType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.AuthorizingClaimMatchValueType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_AuthorizingClaimMatchValueType as AuthorizingClaimMatchValueType;
    impl crate::value::ToValue for AuthorizingClaimMatchValueType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClaimMatchOperator".to_string(),
                crate::value::ToValue::to_value(&self.claim_match_operator),
            );
            properties.insert(
                "ClaimMatchValue".to_string(),
                crate::value::ToValue::to_value(&self.claim_match_value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-claimmatchvaluetype.html>
    pub struct ClaimMatchValueType_ {
        pub match_value_string: Option<crate::value::ExpString>,
        pub match_value_string_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_ClaimMatchValueType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.ClaimMatchValueType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_ClaimMatchValueType as ClaimMatchValueType;
    impl crate::value::ToValue for ClaimMatchValueType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.match_value_string {
                properties.insert(
                    "MatchValueString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_value_string_list {
                properties.insert(
                    "MatchValueStringList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-customclaimvalidationtype.html>
    pub struct CustomClaimValidationType_ {
        pub authorizing_claim_match_value: Box<AuthorizingClaimMatchValueType_>,
        pub inbound_token_claim_name: crate::value::ExpString,
        pub inbound_token_claim_value_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_CustomClaimValidationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.CustomClaimValidationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_CustomClaimValidationType as CustomClaimValidationType;
    impl crate::value::ToValue for CustomClaimValidationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthorizingClaimMatchValue".to_string(),
                crate::value::ToValue::to_value(&self.authorizing_claim_match_value),
            );
            properties.insert(
                "InboundTokenClaimName".to_string(),
                crate::value::ToValue::to_value(&self.inbound_token_claim_name),
            );
            properties.insert(
                "InboundTokenClaimValueType".to_string(),
                crate::value::ToValue::to_value(&self.inbound_token_claim_value_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-customjwtauthorizerconfiguration.html>
    pub struct CustomJWTAuthorizerConfiguration_ {
        pub allowed_audience: Option<Vec<crate::value::ExpString>>,
        pub allowed_clients: Option<Vec<crate::value::ExpString>>,
        pub allowed_scopes: Option<Vec<crate::value::ExpString>>,
        pub custom_claims: Option<Vec<CustomClaimValidationType_>>,
        pub discovery_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_CustomJWTAuthorizerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.CustomJWTAuthorizerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_CustomJWTAuthorizerConfiguration as CustomJWTAuthorizerConfiguration;
    impl crate::value::ToValue for CustomJWTAuthorizerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_audience {
                properties.insert(
                    "AllowedAudience".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_clients {
                properties.insert(
                    "AllowedClients".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_scopes {
                properties.insert(
                    "AllowedScopes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_claims {
                properties.insert(
                    "CustomClaims".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DiscoveryUrl".to_string(),
                crate::value::ToValue::to_value(&self.discovery_url),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-gatewayinterceptorconfiguration.html>
    pub struct GatewayInterceptorConfiguration_ {
        pub input_configuration: Option<Box<InterceptorInputConfiguration_>>,
        pub interception_points: Vec<crate::value::ExpString>,
        pub interceptor: Box<InterceptorConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_GatewayInterceptorConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.GatewayInterceptorConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_GatewayInterceptorConfiguration as GatewayInterceptorConfiguration;
    impl crate::value::ToValue for GatewayInterceptorConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_configuration {
                properties.insert(
                    "InputConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InterceptionPoints".to_string(),
                crate::value::ToValue::to_value(&self.interception_points),
            );
            properties.insert(
                "Interceptor".to_string(),
                crate::value::ToValue::to_value(&self.interceptor),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-gatewayprotocolconfiguration.html>
    pub struct GatewayProtocolConfiguration_ {
        pub mcp: Box<MCPGatewayConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_GatewayProtocolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.GatewayProtocolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_GatewayProtocolConfiguration as GatewayProtocolConfiguration;
    impl crate::value::ToValue for GatewayProtocolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mcp".to_string(),
                crate::value::ToValue::to_value(&self.mcp),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-interceptorconfiguration.html>
    pub struct InterceptorConfiguration_ {
        pub lambda: Box<LambdaInterceptorConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_InterceptorConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.InterceptorConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_InterceptorConfiguration as InterceptorConfiguration;
    impl crate::value::ToValue for InterceptorConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Lambda".to_string(),
                crate::value::ToValue::to_value(&self.lambda),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-interceptorinputconfiguration.html>
    pub struct InterceptorInputConfiguration_ {
        pub pass_request_headers: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_InterceptorInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.InterceptorInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_InterceptorInputConfiguration as InterceptorInputConfiguration;
    impl crate::value::ToValue for InterceptorInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PassRequestHeaders".to_string(),
                crate::value::ToValue::to_value(&self.pass_request_headers),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-lambdainterceptorconfiguration.html>
    pub struct LambdaInterceptorConfiguration_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_LambdaInterceptorConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.LambdaInterceptorConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_LambdaInterceptorConfiguration as LambdaInterceptorConfiguration;
    impl crate::value::ToValue for LambdaInterceptorConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-mcpgatewayconfiguration.html>
    pub struct MCPGatewayConfiguration_ {
        pub instructions: Option<crate::value::ExpString>,
        pub search_type: Option<crate::value::ExpString>,
        pub supported_versions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_MCPGatewayConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.MCPGatewayConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_MCPGatewayConfiguration as MCPGatewayConfiguration;
    impl crate::value::ToValue for MCPGatewayConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instructions {
                properties.insert(
                    "Instructions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.search_type {
                properties.insert(
                    "SearchType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supported_versions {
                properties.insert(
                    "SupportedVersions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gateway-workloadidentitydetails.html>
    pub struct WorkloadIdentityDetails_ {
        pub workload_identity_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Gateway_WorkloadIdentityDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Gateway.WorkloadIdentityDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Gateway_WorkloadIdentityDetails as WorkloadIdentityDetails;
    impl crate::value::ToValue for WorkloadIdentityDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkloadIdentityArn".to_string(),
                crate::value::ToValue::to_value(&self.workload_identity_arn),
            );
            properties.into()
        }
    }
}
pub mod gatewaytarget {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-apigatewaytargetconfiguration.html>
    pub struct ApiGatewayTargetConfiguration_ {
        pub api_gateway_tool_configuration: Box<ApiGatewayToolConfiguration_>,
        pub rest_api_id: crate::value::ExpString,
        pub stage: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ApiGatewayTargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ApiGatewayTargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ApiGatewayTargetConfiguration as ApiGatewayTargetConfiguration;
    impl crate::value::ToValue for ApiGatewayTargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiGatewayToolConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.api_gateway_tool_configuration),
            );
            properties.insert(
                "RestApiId".to_string(),
                crate::value::ToValue::to_value(&self.rest_api_id),
            );
            properties.insert(
                "Stage".to_string(),
                crate::value::ToValue::to_value(&self.stage),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-apigatewaytoolconfiguration.html>
    pub struct ApiGatewayToolConfiguration_ {
        pub tool_filters: Vec<ApiGatewayToolFilter_>,
        pub tool_overrides: Option<Vec<ApiGatewayToolOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ApiGatewayToolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ApiGatewayToolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ApiGatewayToolConfiguration as ApiGatewayToolConfiguration;
    impl crate::value::ToValue for ApiGatewayToolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ToolFilters".to_string(),
                crate::value::ToValue::to_value(&self.tool_filters),
            );
            if let Some(ref value) = self.tool_overrides {
                properties.insert(
                    "ToolOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-apigatewaytoolfilter.html>
    pub struct ApiGatewayToolFilter_ {
        pub filter_path: crate::value::ExpString,
        pub methods: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ApiGatewayToolFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ApiGatewayToolFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ApiGatewayToolFilter as ApiGatewayToolFilter;
    impl crate::value::ToValue for ApiGatewayToolFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FilterPath".to_string(),
                crate::value::ToValue::to_value(&self.filter_path),
            );
            properties.insert(
                "Methods".to_string(),
                crate::value::ToValue::to_value(&self.methods),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-apigatewaytooloverride.html>
    pub struct ApiGatewayToolOverride_ {
        pub description: Option<crate::value::ExpString>,
        pub method: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ApiGatewayToolOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ApiGatewayToolOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ApiGatewayToolOverride as ApiGatewayToolOverride;
    impl crate::value::ToValue for ApiGatewayToolOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Method".to_string(),
                crate::value::ToValue::to_value(&self.method),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-apikeycredentialprovider.html>
    pub struct ApiKeyCredentialProvider_ {
        pub credential_location: Option<crate::value::ExpString>,
        pub credential_parameter_name: Option<crate::value::ExpString>,
        pub credential_prefix: Option<crate::value::ExpString>,
        pub provider_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ApiKeyCredentialProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ApiKeyCredentialProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ApiKeyCredentialProvider as ApiKeyCredentialProvider;
    impl crate::value::ToValue for ApiKeyCredentialProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.credential_location {
                properties.insert(
                    "CredentialLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.credential_parameter_name {
                properties.insert(
                    "CredentialParameterName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.credential_prefix {
                properties.insert(
                    "CredentialPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProviderArn".to_string(),
                crate::value::ToValue::to_value(&self.provider_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-apischemaconfiguration.html>
    pub struct ApiSchemaConfiguration_ {
        pub inline_payload: Option<crate::value::ExpString>,
        pub s3: Option<Box<S3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ApiSchemaConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ApiSchemaConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ApiSchemaConfiguration as ApiSchemaConfiguration;
    impl crate::value::ToValue for ApiSchemaConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inline_payload {
                properties.insert(
                    "InlinePayload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-credentialprovider.html>
    pub struct CredentialProvider_ {
        pub api_key_credential_provider: Option<Box<ApiKeyCredentialProvider_>>,
        pub oauth_credential_provider: Option<Box<OAuthCredentialProvider_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_CredentialProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.CredentialProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_CredentialProvider as CredentialProvider;
    impl crate::value::ToValue for CredentialProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.api_key_credential_provider {
                properties.insert(
                    "ApiKeyCredentialProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.oauth_credential_provider {
                properties.insert(
                    "OauthCredentialProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-credentialproviderconfiguration.html>
    pub struct CredentialProviderConfiguration_ {
        pub credential_provider: Option<Box<CredentialProvider_>>,
        pub credential_provider_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_CredentialProviderConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.CredentialProviderConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_CredentialProviderConfiguration as CredentialProviderConfiguration;
    impl crate::value::ToValue for CredentialProviderConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.credential_provider {
                properties.insert(
                    "CredentialProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CredentialProviderType".to_string(),
                crate::value::ToValue::to_value(&self.credential_provider_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-mcplambdatargetconfiguration.html>
    pub struct McpLambdaTargetConfiguration_ {
        pub lambda_arn: crate::value::ExpString,
        pub tool_schema: Box<ToolSchema_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_McpLambdaTargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.McpLambdaTargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_McpLambdaTargetConfiguration as McpLambdaTargetConfiguration;
    impl crate::value::ToValue for McpLambdaTargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.insert(
                "ToolSchema".to_string(),
                crate::value::ToValue::to_value(&self.tool_schema),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-mcpservertargetconfiguration.html>
    pub struct McpServerTargetConfiguration_ {
        pub endpoint: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_McpServerTargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.McpServerTargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_McpServerTargetConfiguration as McpServerTargetConfiguration;
    impl crate::value::ToValue for McpServerTargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-mcptargetconfiguration.html>
    pub struct McpTargetConfiguration_ {
        pub api_gateway: Option<Box<ApiGatewayTargetConfiguration_>>,
        pub lambda: Option<Box<McpLambdaTargetConfiguration_>>,
        pub mcp_server: Option<Box<McpServerTargetConfiguration_>>,
        pub open_api_schema: Option<Box<ApiSchemaConfiguration_>>,
        pub smithy_model: Option<Box<ApiSchemaConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_McpTargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.McpTargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_McpTargetConfiguration as McpTargetConfiguration;
    impl crate::value::ToValue for McpTargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.api_gateway {
                properties.insert(
                    "ApiGateway".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mcp_server {
                properties.insert(
                    "McpServer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.open_api_schema {
                properties.insert(
                    "OpenApiSchema".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.smithy_model {
                properties.insert(
                    "SmithyModel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-metadataconfiguration.html>
    pub struct MetadataConfiguration_ {
        pub allowed_query_parameters: Option<Vec<crate::value::ExpString>>,
        pub allowed_request_headers: Option<Vec<crate::value::ExpString>>,
        pub allowed_response_headers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_MetadataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.MetadataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_MetadataConfiguration as MetadataConfiguration;
    impl crate::value::ToValue for MetadataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_query_parameters {
                properties.insert(
                    "AllowedQueryParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_request_headers {
                properties.insert(
                    "AllowedRequestHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_response_headers {
                properties.insert(
                    "AllowedResponseHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-oauthcredentialprovider.html>
    pub struct OAuthCredentialProvider_ {
        pub custom_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub default_return_url: Option<crate::value::ExpString>,
        pub grant_type: Option<crate::value::ExpString>,
        pub provider_arn: crate::value::ExpString,
        pub scopes: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_OAuthCredentialProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.OAuthCredentialProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_OAuthCredentialProvider as OAuthCredentialProvider;
    impl crate::value::ToValue for OAuthCredentialProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_parameters {
                properties.insert(
                    "CustomParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_return_url {
                properties.insert(
                    "DefaultReturnUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.grant_type {
                properties.insert(
                    "GrantType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProviderArn".to_string(),
                crate::value::ToValue::to_value(&self.provider_arn),
            );
            properties.insert(
                "Scopes".to_string(),
                crate::value::ToValue::to_value(&self.scopes),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-s3configuration.html>
    pub struct S3Configuration_ {
        pub bucket_owner_account_id: Option<crate::value::ExpString>,
        pub uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_S3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.S3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_S3Configuration as S3Configuration;
    impl crate::value::ToValue for S3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_owner_account_id {
                properties.insert(
                    "BucketOwnerAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri {
                properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-schemadefinition.html>
    pub struct SchemaDefinition_ {
        pub description: Option<crate::value::ExpString>,
        pub items: Option<Box<SchemaDefinition_>>,
        pub properties: Option<std::collections::BTreeMap<String, SchemaDefinition_>>,
        pub required: Option<Vec<crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_SchemaDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.SchemaDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_SchemaDefinition as SchemaDefinition;
    impl crate::value::ToValue for SchemaDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.items {
                properties.insert("Items".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.required {
                properties.insert(
                    "Required".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-targetconfiguration.html>
    pub struct TargetConfiguration_ {
        pub mcp: Box<McpTargetConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_TargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.TargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_TargetConfiguration as TargetConfiguration;
    impl crate::value::ToValue for TargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mcp".to_string(),
                crate::value::ToValue::to_value(&self.mcp),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-tooldefinition.html>
    pub struct ToolDefinition_ {
        pub description: crate::value::ExpString,
        pub input_schema: Box<SchemaDefinition_>,
        pub name: crate::value::ExpString,
        pub output_schema: Option<Box<SchemaDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ToolDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ToolDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ToolDefinition as ToolDefinition;
    impl crate::value::ToValue for ToolDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(&self.description),
            );
            properties.insert(
                "InputSchema".to_string(),
                crate::value::ToValue::to_value(&self.input_schema),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.output_schema {
                properties.insert(
                    "OutputSchema".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-gatewaytarget-toolschema.html>
    pub struct ToolSchema_ {
        pub inline_payload: Option<Vec<ToolDefinition_>>,
        pub s3: Option<Box<S3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_GatewayTarget_ToolSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::GatewayTarget.ToolSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_GatewayTarget_ToolSchema as ToolSchema;
    impl crate::value::ToValue for ToolSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inline_payload {
                properties.insert(
                    "InlinePayload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod memory {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-customconfigurationinput.html>
    pub struct CustomConfigurationInput_ {
        pub episodic_override: Option<Box<EpisodicOverride_>>,
        pub self_managed_configuration: Option<Box<SelfManagedConfiguration_>>,
        pub semantic_override: Option<Box<SemanticOverride_>>,
        pub summary_override: Option<Box<SummaryOverride_>>,
        pub user_preference_override: Option<Box<UserPreferenceOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_CustomConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.CustomConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_CustomConfigurationInput as CustomConfigurationInput;
    impl crate::value::ToValue for CustomConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.episodic_override {
                properties.insert(
                    "EpisodicOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_managed_configuration {
                properties.insert(
                    "SelfManagedConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.semantic_override {
                properties.insert(
                    "SemanticOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.summary_override {
                properties.insert(
                    "SummaryOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_preference_override {
                properties.insert(
                    "UserPreferenceOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-custommemorystrategy.html>
    pub struct CustomMemoryStrategy_ {
        pub configuration: Option<Box<CustomConfigurationInput_>>,
        pub created_at: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub namespaces: Option<Vec<crate::value::ExpString>>,
        pub status: Option<crate::value::ExpString>,
        pub strategy_id: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub updated_at: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_CustomMemoryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.CustomMemoryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_CustomMemoryStrategy as CustomMemoryStrategy;
    impl crate::value::ToValue for CustomMemoryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.created_at {
                properties.insert(
                    "CreatedAt".to_string(),
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
            if let Some(ref value) = self.namespaces {
                properties.insert(
                    "Namespaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.strategy_id {
                properties.insert(
                    "StrategyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-episodicmemorystrategy.html>
    pub struct EpisodicMemoryStrategy_ {
        pub created_at: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub namespaces: Option<Vec<crate::value::ExpString>>,
        pub reflection_configuration: Option<Box<EpisodicReflectionConfigurationInput_>>,
        pub status: Option<crate::value::ExpString>,
        pub strategy_id: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub updated_at: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_EpisodicMemoryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.EpisodicMemoryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_EpisodicMemoryStrategy as EpisodicMemoryStrategy;
    impl crate::value::ToValue for EpisodicMemoryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.created_at {
                properties.insert(
                    "CreatedAt".to_string(),
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
            if let Some(ref value) = self.namespaces {
                properties.insert(
                    "Namespaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reflection_configuration {
                properties.insert(
                    "ReflectionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.strategy_id {
                properties.insert(
                    "StrategyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-episodicoverride.html>
    pub struct EpisodicOverride_ {
        pub consolidation: Option<Box<EpisodicOverrideConsolidationConfigurationInput_>>,
        pub extraction: Option<Box<EpisodicOverrideExtractionConfigurationInput_>>,
        pub reflection: Option<Box<EpisodicOverrideReflectionConfigurationInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_EpisodicOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.EpisodicOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_EpisodicOverride as EpisodicOverride;
    impl crate::value::ToValue for EpisodicOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consolidation {
                properties.insert(
                    "Consolidation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extraction {
                properties.insert(
                    "Extraction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reflection {
                properties.insert(
                    "Reflection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-episodicoverrideconsolidationconfigurationinput.html>
    pub struct EpisodicOverrideConsolidationConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_EpisodicOverrideConsolidationConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.EpisodicOverrideConsolidationConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_EpisodicOverrideConsolidationConfigurationInput as EpisodicOverrideConsolidationConfigurationInput;
    impl crate::value::ToValue for EpisodicOverrideConsolidationConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-episodicoverrideextractionconfigurationinput.html>
    pub struct EpisodicOverrideExtractionConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_EpisodicOverrideExtractionConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.EpisodicOverrideExtractionConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_EpisodicOverrideExtractionConfigurationInput as EpisodicOverrideExtractionConfigurationInput;
    impl crate::value::ToValue for EpisodicOverrideExtractionConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-episodicoverridereflectionconfigurationinput.html>
    pub struct EpisodicOverrideReflectionConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
        pub namespaces: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_EpisodicOverrideReflectionConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.EpisodicOverrideReflectionConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_EpisodicOverrideReflectionConfigurationInput as EpisodicOverrideReflectionConfigurationInput;
    impl crate::value::ToValue for EpisodicOverrideReflectionConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            if let Some(ref value) = self.namespaces {
                properties.insert(
                    "Namespaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-episodicreflectionconfigurationinput.html>
    pub struct EpisodicReflectionConfigurationInput_ {
        pub namespaces: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_EpisodicReflectionConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.EpisodicReflectionConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_EpisodicReflectionConfigurationInput as EpisodicReflectionConfigurationInput;
    impl crate::value::ToValue for EpisodicReflectionConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespaces".to_string(),
                crate::value::ToValue::to_value(&self.namespaces),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-invocationconfigurationinput.html>
    pub struct InvocationConfigurationInput_ {
        pub payload_delivery_bucket_name: Option<crate::value::ExpString>,
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_InvocationConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.InvocationConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_InvocationConfigurationInput as InvocationConfigurationInput;
    impl crate::value::ToValue for InvocationConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload_delivery_bucket_name {
                properties.insert(
                    "PayloadDeliveryBucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic_arn {
                properties.insert(
                    "TopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-memorystrategy.html>
    pub struct MemoryStrategy_ {
        pub custom_memory_strategy: Option<Box<CustomMemoryStrategy_>>,
        pub episodic_memory_strategy: Option<Box<EpisodicMemoryStrategy_>>,
        pub semantic_memory_strategy: Option<Box<SemanticMemoryStrategy_>>,
        pub summary_memory_strategy: Option<Box<SummaryMemoryStrategy_>>,
        pub user_preference_memory_strategy: Option<Box<UserPreferenceMemoryStrategy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_MemoryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.MemoryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_MemoryStrategy as MemoryStrategy;
    impl crate::value::ToValue for MemoryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_memory_strategy {
                properties.insert(
                    "CustomMemoryStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.episodic_memory_strategy {
                properties.insert(
                    "EpisodicMemoryStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.semantic_memory_strategy {
                properties.insert(
                    "SemanticMemoryStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.summary_memory_strategy {
                properties.insert(
                    "SummaryMemoryStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_preference_memory_strategy {
                properties.insert(
                    "UserPreferenceMemoryStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-messagebasedtriggerinput.html>
    pub struct MessageBasedTriggerInput_ {
        pub message_count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_MessageBasedTriggerInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.MessageBasedTriggerInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_MessageBasedTriggerInput as MessageBasedTriggerInput;
    impl crate::value::ToValue for MessageBasedTriggerInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message_count {
                properties.insert(
                    "MessageCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-selfmanagedconfiguration.html>
    pub struct SelfManagedConfiguration_ {
        pub historical_context_window_size: Option<i32>,
        pub invocation_configuration: Option<Box<InvocationConfigurationInput_>>,
        pub trigger_conditions: Option<Vec<TriggerConditionInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SelfManagedConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SelfManagedConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SelfManagedConfiguration as SelfManagedConfiguration;
    impl crate::value::ToValue for SelfManagedConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.historical_context_window_size {
                properties.insert(
                    "HistoricalContextWindowSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.invocation_configuration {
                properties.insert(
                    "InvocationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trigger_conditions {
                properties.insert(
                    "TriggerConditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-semanticmemorystrategy.html>
    pub struct SemanticMemoryStrategy_ {
        pub created_at: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub namespaces: Option<Vec<crate::value::ExpString>>,
        pub status: Option<crate::value::ExpString>,
        pub strategy_id: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub updated_at: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SemanticMemoryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SemanticMemoryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SemanticMemoryStrategy as SemanticMemoryStrategy;
    impl crate::value::ToValue for SemanticMemoryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.created_at {
                properties.insert(
                    "CreatedAt".to_string(),
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
            if let Some(ref value) = self.namespaces {
                properties.insert(
                    "Namespaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.strategy_id {
                properties.insert(
                    "StrategyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-semanticoverride.html>
    pub struct SemanticOverride_ {
        pub consolidation: Option<Box<SemanticOverrideConsolidationConfigurationInput_>>,
        pub extraction: Option<Box<SemanticOverrideExtractionConfigurationInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SemanticOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SemanticOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SemanticOverride as SemanticOverride;
    impl crate::value::ToValue for SemanticOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consolidation {
                properties.insert(
                    "Consolidation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extraction {
                properties.insert(
                    "Extraction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-semanticoverrideconsolidationconfigurationinput.html>
    pub struct SemanticOverrideConsolidationConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SemanticOverrideConsolidationConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SemanticOverrideConsolidationConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SemanticOverrideConsolidationConfigurationInput as SemanticOverrideConsolidationConfigurationInput;
    impl crate::value::ToValue for SemanticOverrideConsolidationConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-semanticoverrideextractionconfigurationinput.html>
    pub struct SemanticOverrideExtractionConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SemanticOverrideExtractionConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SemanticOverrideExtractionConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SemanticOverrideExtractionConfigurationInput as SemanticOverrideExtractionConfigurationInput;
    impl crate::value::ToValue for SemanticOverrideExtractionConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-summarymemorystrategy.html>
    pub struct SummaryMemoryStrategy_ {
        pub created_at: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub namespaces: Option<Vec<crate::value::ExpString>>,
        pub status: Option<crate::value::ExpString>,
        pub strategy_id: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub updated_at: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SummaryMemoryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SummaryMemoryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SummaryMemoryStrategy as SummaryMemoryStrategy;
    impl crate::value::ToValue for SummaryMemoryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.created_at {
                properties.insert(
                    "CreatedAt".to_string(),
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
            if let Some(ref value) = self.namespaces {
                properties.insert(
                    "Namespaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.strategy_id {
                properties.insert(
                    "StrategyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-summaryoverride.html>
    pub struct SummaryOverride_ {
        pub consolidation: Option<Box<SummaryOverrideConsolidationConfigurationInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SummaryOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SummaryOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SummaryOverride as SummaryOverride;
    impl crate::value::ToValue for SummaryOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consolidation {
                properties.insert(
                    "Consolidation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-summaryoverrideconsolidationconfigurationinput.html>
    pub struct SummaryOverrideConsolidationConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_SummaryOverrideConsolidationConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.SummaryOverrideConsolidationConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_SummaryOverrideConsolidationConfigurationInput as SummaryOverrideConsolidationConfigurationInput;
    impl crate::value::ToValue for SummaryOverrideConsolidationConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-timebasedtriggerinput.html>
    pub struct TimeBasedTriggerInput_ {
        pub idle_session_timeout: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_TimeBasedTriggerInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.TimeBasedTriggerInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_TimeBasedTriggerInput as TimeBasedTriggerInput;
    impl crate::value::ToValue for TimeBasedTriggerInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_session_timeout {
                properties.insert(
                    "IdleSessionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-tokenbasedtriggerinput.html>
    pub struct TokenBasedTriggerInput_ {
        pub token_count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_TokenBasedTriggerInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.TokenBasedTriggerInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_TokenBasedTriggerInput as TokenBasedTriggerInput;
    impl crate::value::ToValue for TokenBasedTriggerInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.token_count {
                properties.insert(
                    "TokenCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-triggerconditioninput.html>
    pub struct TriggerConditionInput_ {
        pub message_based_trigger: Option<Box<MessageBasedTriggerInput_>>,
        pub time_based_trigger: Option<Box<TimeBasedTriggerInput_>>,
        pub token_based_trigger: Option<Box<TokenBasedTriggerInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_TriggerConditionInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.TriggerConditionInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_TriggerConditionInput as TriggerConditionInput;
    impl crate::value::ToValue for TriggerConditionInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message_based_trigger {
                properties.insert(
                    "MessageBasedTrigger".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_based_trigger {
                properties.insert(
                    "TimeBasedTrigger".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.token_based_trigger {
                properties.insert(
                    "TokenBasedTrigger".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-userpreferencememorystrategy.html>
    pub struct UserPreferenceMemoryStrategy_ {
        pub created_at: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub namespaces: Option<Vec<crate::value::ExpString>>,
        pub status: Option<crate::value::ExpString>,
        pub strategy_id: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub updated_at: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_UserPreferenceMemoryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.UserPreferenceMemoryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_UserPreferenceMemoryStrategy as UserPreferenceMemoryStrategy;
    impl crate::value::ToValue for UserPreferenceMemoryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.created_at {
                properties.insert(
                    "CreatedAt".to_string(),
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
            if let Some(ref value) = self.namespaces {
                properties.insert(
                    "Namespaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.strategy_id {
                properties.insert(
                    "StrategyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.updated_at {
                properties.insert(
                    "UpdatedAt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-userpreferenceoverride.html>
    pub struct UserPreferenceOverride_ {
        pub consolidation: Option<Box<UserPreferenceOverrideConsolidationConfigurationInput_>>,
        pub extraction: Option<Box<UserPreferenceOverrideExtractionConfigurationInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_UserPreferenceOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.UserPreferenceOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_UserPreferenceOverride as UserPreferenceOverride;
    impl crate::value::ToValue for UserPreferenceOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consolidation {
                properties.insert(
                    "Consolidation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extraction {
                properties.insert(
                    "Extraction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-userpreferenceoverrideconsolidationconfigurationinput.html>
    pub struct UserPreferenceOverrideConsolidationConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_UserPreferenceOverrideConsolidationConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.UserPreferenceOverrideConsolidationConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_UserPreferenceOverrideConsolidationConfigurationInput as UserPreferenceOverrideConsolidationConfigurationInput;
    impl crate::value::ToValue for UserPreferenceOverrideConsolidationConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-memory-userpreferenceoverrideextractionconfigurationinput.html>
    pub struct UserPreferenceOverrideExtractionConfigurationInput_ {
        pub append_to_prompt: crate::value::ExpString,
        pub model_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Memory_UserPreferenceOverrideExtractionConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Memory.UserPreferenceOverrideExtractionConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Memory_UserPreferenceOverrideExtractionConfigurationInput as UserPreferenceOverrideExtractionConfigurationInput;
    impl crate::value::ToValue for UserPreferenceOverrideExtractionConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppendToPrompt".to_string(),
                crate::value::ToValue::to_value(&self.append_to_prompt),
            );
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.into()
        }
    }
}
pub mod runtime {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-agentruntimeartifact.html>
    pub struct AgentRuntimeArtifact_ {
        pub code_configuration: Option<Box<CodeConfiguration_>>,
        pub container_configuration: Option<Box<ContainerConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_AgentRuntimeArtifact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.AgentRuntimeArtifact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_AgentRuntimeArtifact as AgentRuntimeArtifact;
    impl crate::value::ToValue for AgentRuntimeArtifact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_configuration {
                properties.insert(
                    "CodeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_configuration {
                properties.insert(
                    "ContainerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-authorizerconfiguration.html>
    pub struct AuthorizerConfiguration_ {
        pub custom_jwt_authorizer: Option<Box<CustomJWTAuthorizerConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_AuthorizerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.AuthorizerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_AuthorizerConfiguration as AuthorizerConfiguration;
    impl crate::value::ToValue for AuthorizerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_jwt_authorizer {
                properties.insert(
                    "CustomJWTAuthorizer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-authorizingclaimmatchvaluetype.html>
    pub struct AuthorizingClaimMatchValueType_ {
        pub claim_match_operator: crate::value::ExpString,
        pub claim_match_value: Box<ClaimMatchValueType_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_AuthorizingClaimMatchValueType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.AuthorizingClaimMatchValueType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_AuthorizingClaimMatchValueType as AuthorizingClaimMatchValueType;
    impl crate::value::ToValue for AuthorizingClaimMatchValueType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClaimMatchOperator".to_string(),
                crate::value::ToValue::to_value(&self.claim_match_operator),
            );
            properties.insert(
                "ClaimMatchValue".to_string(),
                crate::value::ToValue::to_value(&self.claim_match_value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-claimmatchvaluetype.html>
    pub struct ClaimMatchValueType_ {
        pub match_value_string: Option<crate::value::ExpString>,
        pub match_value_string_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_ClaimMatchValueType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.ClaimMatchValueType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_ClaimMatchValueType as ClaimMatchValueType;
    impl crate::value::ToValue for ClaimMatchValueType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.match_value_string {
                properties.insert(
                    "MatchValueString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_value_string_list {
                properties.insert(
                    "MatchValueStringList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-code.html>
    pub struct Code_ {
        pub s3: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_Code {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.Code"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_Code as Code;
    impl crate::value::ToValue for Code_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-codeconfiguration.html>
    pub struct CodeConfiguration_ {
        pub code: Box<Code_>,
        pub entry_point: Vec<crate::value::ExpString>,
        pub runtime: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_CodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.CodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_CodeConfiguration as CodeConfiguration;
    impl crate::value::ToValue for CodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Code".to_string(),
                crate::value::ToValue::to_value(&self.code),
            );
            properties.insert(
                "EntryPoint".to_string(),
                crate::value::ToValue::to_value(&self.entry_point),
            );
            properties.insert(
                "Runtime".to_string(),
                crate::value::ToValue::to_value(&self.runtime),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-containerconfiguration.html>
    pub struct ContainerConfiguration_ {
        pub container_uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_ContainerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.ContainerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_ContainerConfiguration as ContainerConfiguration;
    impl crate::value::ToValue for ContainerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContainerUri".to_string(),
                crate::value::ToValue::to_value(&self.container_uri),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-customclaimvalidationtype.html>
    pub struct CustomClaimValidationType_ {
        pub authorizing_claim_match_value: Box<AuthorizingClaimMatchValueType_>,
        pub inbound_token_claim_name: crate::value::ExpString,
        pub inbound_token_claim_value_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_CustomClaimValidationType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.CustomClaimValidationType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_CustomClaimValidationType as CustomClaimValidationType;
    impl crate::value::ToValue for CustomClaimValidationType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthorizingClaimMatchValue".to_string(),
                crate::value::ToValue::to_value(&self.authorizing_claim_match_value),
            );
            properties.insert(
                "InboundTokenClaimName".to_string(),
                crate::value::ToValue::to_value(&self.inbound_token_claim_name),
            );
            properties.insert(
                "InboundTokenClaimValueType".to_string(),
                crate::value::ToValue::to_value(&self.inbound_token_claim_value_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-customjwtauthorizerconfiguration.html>
    pub struct CustomJWTAuthorizerConfiguration_ {
        pub allowed_audience: Option<Vec<crate::value::ExpString>>,
        pub allowed_clients: Option<Vec<crate::value::ExpString>>,
        pub allowed_scopes: Option<Vec<crate::value::ExpString>>,
        pub custom_claims: Option<Vec<CustomClaimValidationType_>>,
        pub discovery_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_CustomJWTAuthorizerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.CustomJWTAuthorizerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_CustomJWTAuthorizerConfiguration as CustomJWTAuthorizerConfiguration;
    impl crate::value::ToValue for CustomJWTAuthorizerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_audience {
                properties.insert(
                    "AllowedAudience".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_clients {
                properties.insert(
                    "AllowedClients".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.allowed_scopes {
                properties.insert(
                    "AllowedScopes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_claims {
                properties.insert(
                    "CustomClaims".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DiscoveryUrl".to_string(),
                crate::value::ToValue::to_value(&self.discovery_url),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-lifecycleconfiguration.html>
    pub struct LifecycleConfiguration_ {
        pub idle_runtime_session_timeout: Option<i32>,
        pub max_lifetime: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_LifecycleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.LifecycleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_LifecycleConfiguration as LifecycleConfiguration;
    impl crate::value::ToValue for LifecycleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_runtime_session_timeout {
                properties.insert(
                    "IdleRuntimeSessionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_lifetime {
                properties.insert(
                    "MaxLifetime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-networkconfiguration.html>
    pub struct NetworkConfiguration_ {
        pub network_mode: crate::value::ExpString,
        pub network_mode_config: Option<Box<VpcConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkMode".to_string(),
                crate::value::ToValue::to_value(&self.network_mode),
            );
            if let Some(ref value) = self.network_mode_config {
                properties.insert(
                    "NetworkModeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-requestheaderconfiguration.html>
    pub struct RequestHeaderConfiguration_ {
        pub request_header_allowlist: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_RequestHeaderConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.RequestHeaderConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_RequestHeaderConfiguration as RequestHeaderConfiguration;
    impl crate::value::ToValue for RequestHeaderConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.request_header_allowlist {
                properties.insert(
                    "RequestHeaderAllowlist".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-s3location.html>
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub prefix: crate::value::ExpString,
        pub version_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Prefix".to_string(),
                crate::value::ToValue::to_value(&self.prefix),
            );
            if let Some(ref value) = self.version_id {
                properties.insert(
                    "VersionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-vpcconfig.html>
    pub struct VpcConfig_ {
        pub security_groups: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(&self.security_groups),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrockagentcore-runtime-workloadidentitydetails.html>
    pub struct WorkloadIdentityDetails_ {
        pub workload_identity_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrockagentcore_Runtime_WorkloadIdentityDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BedrockAgentCore::Runtime.WorkloadIdentityDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrockagentcore_Runtime_WorkloadIdentityDetails as WorkloadIdentityDetails;
    impl crate::value::ToValue for WorkloadIdentityDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkloadIdentityArn".to_string(),
                crate::value::ToValue::to_value(&self.workload_identity_arn),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-browsercustom.html>
pub struct BrowserCustom_ {
    pub browser_signing: Option<super::bedrockagentcore::browsercustom::BrowserSigning_>,
    pub description: Option<crate::value::ExpString>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub network_configuration: super::bedrockagentcore::browsercustom::BrowserNetworkConfiguration_,
    pub recording_config: Option<super::bedrockagentcore::browsercustom::RecordingConfig_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_BrowserCustom {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::BrowserCustom"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_BrowserCustom as BrowserCustom;
impl crate::template::ToResource for BrowserCustom_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BrowserCustom"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.browser_signing {
            properties.insert(
                "BrowserSigning".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "NetworkConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.network_configuration),
        );
        if let Some(ref value) = self.recording_config {
            properties.insert(
                "RecordingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-codeinterpretercustom.html>
pub struct CodeInterpreterCustom_ {
    pub description: Option<crate::value::ExpString>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub network_configuration:
        super::bedrockagentcore::codeinterpretercustom::CodeInterpreterNetworkConfiguration_,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_CodeInterpreterCustom {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::CodeInterpreterCustom"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_CodeInterpreterCustom as CodeInterpreterCustom;
impl crate::template::ToResource for CodeInterpreterCustom_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CodeInterpreterCustom"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.execution_role_arn {
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "NetworkConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.network_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-gateway.html>
pub struct Gateway_ {
    pub authorizer_configuration:
        Option<super::bedrockagentcore::gateway::AuthorizerConfiguration_>,
    pub authorizer_type: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub exception_level: Option<crate::value::ExpString>,
    pub interceptor_configurations:
        Option<Vec<super::bedrockagentcore::gateway::GatewayInterceptorConfiguration_>>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub protocol_configuration:
        Option<super::bedrockagentcore::gateway::GatewayProtocolConfiguration_>,
    pub protocol_type: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_Gateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::Gateway"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_Gateway as Gateway;
impl crate::template::ToResource for Gateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Gateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.authorizer_configuration {
            properties.insert(
                "AuthorizerConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AuthorizerType".to_string(),
            crate::value::ToValue::to_value(&self.authorizer_type),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.exception_level {
            properties.insert(
                "ExceptionLevel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.interceptor_configurations {
            properties.insert(
                "InterceptorConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.protocol_configuration {
            properties.insert(
                "ProtocolConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProtocolType".to_string(),
            crate::value::ToValue::to_value(&self.protocol_type),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-gatewaytarget.html>
pub struct GatewayTarget_ {
    pub credential_provider_configurations:
        Option<Vec<super::bedrockagentcore::gatewaytarget::CredentialProviderConfiguration_>>,
    pub description: Option<crate::value::ExpString>,
    pub gateway_identifier: Option<crate::value::ExpString>,
    pub metadata_configuration:
        Option<super::bedrockagentcore::gatewaytarget::MetadataConfiguration_>,
    pub name: crate::value::ExpString,
    pub target_configuration: super::bedrockagentcore::gatewaytarget::TargetConfiguration_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_GatewayTarget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::GatewayTarget"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_GatewayTarget as GatewayTarget;
impl crate::template::ToResource for GatewayTarget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GatewayTarget"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.credential_provider_configurations {
            properties.insert(
                "CredentialProviderConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gateway_identifier {
            properties.insert(
                "GatewayIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metadata_configuration {
            properties.insert(
                "MetadataConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "TargetConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.target_configuration),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-memory.html>
pub struct Memory_ {
    pub description: Option<crate::value::ExpString>,
    pub encryption_key_arn: Option<crate::value::ExpString>,
    pub event_expiry_duration: i32,
    pub memory_execution_role_arn: Option<crate::value::ExpString>,
    pub memory_strategies: Option<Vec<super::bedrockagentcore::memory::MemoryStrategy_>>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_Memory {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::Memory"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_Memory as Memory;
impl crate::template::ToResource for Memory_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Memory"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_key_arn {
            properties.insert(
                "EncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventExpiryDuration".to_string(),
            crate::value::ToValue::to_value(&self.event_expiry_duration),
        );
        if let Some(ref value) = self.memory_execution_role_arn {
            properties.insert(
                "MemoryExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.memory_strategies {
            properties.insert(
                "MemoryStrategies".to_string(),
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
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-runtime.html>
pub struct Runtime_ {
    pub agent_runtime_artifact: super::bedrockagentcore::runtime::AgentRuntimeArtifact_,
    pub agent_runtime_name: crate::value::ExpString,
    pub authorizer_configuration:
        Option<super::bedrockagentcore::runtime::AuthorizerConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub environment_variables: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub lifecycle_configuration: Option<super::bedrockagentcore::runtime::LifecycleConfiguration_>,
    pub network_configuration: super::bedrockagentcore::runtime::NetworkConfiguration_,
    pub protocol_configuration: Option<crate::value::ExpString>,
    pub request_header_configuration:
        Option<super::bedrockagentcore::runtime::RequestHeaderConfiguration_>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_Runtime {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::Runtime"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_Runtime as Runtime;
impl crate::template::ToResource for Runtime_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Runtime"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AgentRuntimeArtifact".to_string(),
            crate::value::ToValue::to_value(&self.agent_runtime_artifact),
        );
        properties.insert(
            "AgentRuntimeName".to_string(),
            crate::value::ToValue::to_value(&self.agent_runtime_name),
        );
        if let Some(ref value) = self.authorizer_configuration {
            properties.insert(
                "AuthorizerConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_variables {
            properties.insert(
                "EnvironmentVariables".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_configuration {
            properties.insert(
                "LifecycleConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NetworkConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.network_configuration),
        );
        if let Some(ref value) = self.protocol_configuration {
            properties.insert(
                "ProtocolConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.request_header_configuration {
            properties.insert(
                "RequestHeaderConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-runtimeendpoint.html>
pub struct RuntimeEndpoint_ {
    pub agent_runtime_id: crate::value::ExpString,
    pub agent_runtime_version: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_RuntimeEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::RuntimeEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_RuntimeEndpoint as RuntimeEndpoint;
impl crate::template::ToResource for RuntimeEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RuntimeEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AgentRuntimeId".to_string(),
            crate::value::ToValue::to_value(&self.agent_runtime_id),
        );
        if let Some(ref value) = self.agent_runtime_version {
            properties.insert(
                "AgentRuntimeVersion".to_string(),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrockagentcore-workloadidentity.html>
pub struct WorkloadIdentity_ {
    pub allowed_resource_oauth2_return_urls: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrockagentcore_WorkloadIdentity {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BedrockAgentCore::WorkloadIdentity"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrockagentcore_WorkloadIdentity as WorkloadIdentity;
impl crate::template::ToResource for WorkloadIdentity_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BedrockAgentCore"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WorkloadIdentity"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_resource_oauth2_return_urls {
            properties.insert(
                "AllowedResourceOauth2ReturnUrls".to_string(),
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
        properties
    }
}
