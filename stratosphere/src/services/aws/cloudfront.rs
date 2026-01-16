pub mod anycastiplist {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-anycastiplist-anycastiplist.html
    pub struct AnycastIpList_ {
        pub anycast_ips: Vec<crate::value::ExpString>,
        pub arn: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub ip_count: i64,
        pub last_modified_time: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_AnycastIpList_AnycastIpList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::AnycastIpList.AnycastIpList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_AnycastIpList_AnycastIpList as AnycastIpList;
    impl crate::value::ToValue for AnycastIpList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AnycastIps".to_string(),
                crate::value::ToValue::to_value(&self.anycast_ips),
            );
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "IpCount".to_string(),
                crate::value::ToValue::to_value(&self.ip_count),
            );
            properties.insert(
                "LastModifiedTime".to_string(),
                crate::value::ToValue::to_value(&self.last_modified_time),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-anycastiplist-tags.html
    pub struct Tags_ {
        pub items: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_AnycastIpList_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::AnycastIpList.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_AnycastIpList_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.items {
                properties.insert("Items".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod cachepolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cachepolicyconfig.html
    pub struct CachePolicyConfig_ {
        pub comment: Option<crate::value::ExpString>,
        pub default_ttl: f64,
        pub max_ttl: f64,
        pub min_ttl: f64,
        pub name: crate::value::ExpString,
        pub parameters_in_cache_key_and_forwarded_to_origin:
            Box<ParametersInCacheKeyAndForwardedToOrigin_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_CachePolicy_CachePolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::CachePolicy.CachePolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_CachePolicy_CachePolicyConfig as CachePolicyConfig;
    impl crate::value::ToValue for CachePolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DefaultTTL".to_string(),
                crate::value::ToValue::to_value(&self.default_ttl),
            );
            properties.insert(
                "MaxTTL".to_string(),
                crate::value::ToValue::to_value(&self.max_ttl),
            );
            properties.insert(
                "MinTTL".to_string(),
                crate::value::ToValue::to_value(&self.min_ttl),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "ParametersInCacheKeyAndForwardedToOrigin".to_string(),
                crate::value::ToValue::to_value(
                    &self.parameters_in_cache_key_and_forwarded_to_origin,
                ),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-cookiesconfig.html
    pub struct CookiesConfig_ {
        pub cookie_behavior: crate::value::ExpString,
        pub cookies: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_CachePolicy_CookiesConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::CachePolicy.CookiesConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_CachePolicy_CookiesConfig as CookiesConfig;
    impl crate::value::ToValue for CookiesConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CookieBehavior".to_string(),
                crate::value::ToValue::to_value(&self.cookie_behavior),
            );
            if let Some(ref value) = self.cookies {
                properties.insert(
                    "Cookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-headersconfig.html
    pub struct HeadersConfig_ {
        pub header_behavior: crate::value::ExpString,
        pub headers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_CachePolicy_HeadersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::CachePolicy.HeadersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_CachePolicy_HeadersConfig as HeadersConfig;
    impl crate::value::ToValue for HeadersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HeaderBehavior".to_string(),
                crate::value::ToValue::to_value(&self.header_behavior),
            );
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-parametersincachekeyandforwardedtoorigin.html
    pub struct ParametersInCacheKeyAndForwardedToOrigin_ {
        pub cookies_config: Box<CookiesConfig_>,
        pub enable_accept_encoding_brotli: Option<crate::value::ExpBool>,
        pub enable_accept_encoding_gzip: crate::value::ExpBool,
        pub headers_config: Box<HeadersConfig_>,
        pub query_strings_config: Box<QueryStringsConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_CachePolicy_ParametersInCacheKeyAndForwardedToOrigin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::CachePolicy.ParametersInCacheKeyAndForwardedToOrigin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_CachePolicy_ParametersInCacheKeyAndForwardedToOrigin as ParametersInCacheKeyAndForwardedToOrigin;
    impl crate::value::ToValue for ParametersInCacheKeyAndForwardedToOrigin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CookiesConfig".to_string(),
                crate::value::ToValue::to_value(&self.cookies_config),
            );
            if let Some(ref value) = self.enable_accept_encoding_brotli {
                properties.insert(
                    "EnableAcceptEncodingBrotli".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EnableAcceptEncodingGzip".to_string(),
                crate::value::ToValue::to_value(&self.enable_accept_encoding_gzip),
            );
            properties.insert(
                "HeadersConfig".to_string(),
                crate::value::ToValue::to_value(&self.headers_config),
            );
            properties.insert(
                "QueryStringsConfig".to_string(),
                crate::value::ToValue::to_value(&self.query_strings_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cachepolicy-querystringsconfig.html
    pub struct QueryStringsConfig_ {
        pub query_string_behavior: crate::value::ExpString,
        pub query_strings: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_CachePolicy_QueryStringsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::CachePolicy.QueryStringsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_CachePolicy_QueryStringsConfig as QueryStringsConfig;
    impl crate::value::ToValue for QueryStringsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QueryStringBehavior".to_string(),
                crate::value::ToValue::to_value(&self.query_string_behavior),
            );
            if let Some(ref value) = self.query_strings {
                properties.insert(
                    "QueryStrings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod cloudfrontoriginaccessidentity {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-cloudfrontoriginaccessidentity-cloudfrontoriginaccessidentityconfig.html
    pub struct CloudFrontOriginAccessIdentityConfig_ {
        pub comment: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_CloudFrontOriginAccessIdentity_CloudFrontOriginAccessIdentityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::CloudFrontOriginAccessIdentity.CloudFrontOriginAccessIdentityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_CloudFrontOriginAccessIdentity_CloudFrontOriginAccessIdentityConfig as CloudFrontOriginAccessIdentityConfig;
    impl crate::value::ToValue for CloudFrontOriginAccessIdentityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comment".to_string(),
                crate::value::ToValue::to_value(&self.comment),
            );
            properties.into()
        }
    }
}
pub mod continuousdeploymentpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-continuousdeploymentpolicy-continuousdeploymentpolicyconfig.html
    pub struct ContinuousDeploymentPolicyConfig_ {
        pub enabled: crate::value::ExpBool,
        pub single_header_policy_config: Option<Box<SingleHeaderPolicyConfig_>>,
        pub single_weight_policy_config: Option<Box<SingleWeightPolicyConfig_>>,
        pub staging_distribution_dns_names: Vec<crate::value::ExpString>,
        pub traffic_config: Option<Box<TrafficConfig_>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy_ContinuousDeploymentPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ContinuousDeploymentPolicy.ContinuousDeploymentPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy_ContinuousDeploymentPolicyConfig as ContinuousDeploymentPolicyConfig;
    impl crate::value::ToValue for ContinuousDeploymentPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.single_header_policy_config {
                properties.insert(
                    "SingleHeaderPolicyConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_weight_policy_config {
                properties.insert(
                    "SingleWeightPolicyConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StagingDistributionDnsNames".to_string(),
                crate::value::ToValue::to_value(&self.staging_distribution_dns_names),
            );
            if let Some(ref value) = self.traffic_config {
                properties.insert(
                    "TrafficConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-continuousdeploymentpolicy-sessionstickinessconfig.html
    pub struct SessionStickinessConfig_ {
        pub idle_ttl: i64,
        pub maximum_ttl: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy_SessionStickinessConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ContinuousDeploymentPolicy.SessionStickinessConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy_SessionStickinessConfig as SessionStickinessConfig;
    impl crate::value::ToValue for SessionStickinessConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdleTTL".to_string(),
                crate::value::ToValue::to_value(&self.idle_ttl),
            );
            properties.insert(
                "MaximumTTL".to_string(),
                crate::value::ToValue::to_value(&self.maximum_ttl),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-continuousdeploymentpolicy-singleheaderconfig.html
    pub struct SingleHeaderConfig_ {
        pub header: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy_SingleHeaderConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ContinuousDeploymentPolicy.SingleHeaderConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy_SingleHeaderConfig as SingleHeaderConfig;
    impl crate::value::ToValue for SingleHeaderConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Header".to_string(),
                crate::value::ToValue::to_value(&self.header),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-continuousdeploymentpolicy-singleheaderpolicyconfig.html
    pub struct SingleHeaderPolicyConfig_ {
        pub header: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy_SingleHeaderPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ContinuousDeploymentPolicy.SingleHeaderPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy_SingleHeaderPolicyConfig as SingleHeaderPolicyConfig;
    impl crate::value::ToValue for SingleHeaderPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Header".to_string(),
                crate::value::ToValue::to_value(&self.header),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-continuousdeploymentpolicy-singleweightconfig.html
    pub struct SingleWeightConfig_ {
        pub session_stickiness_config: Option<Box<SessionStickinessConfig_>>,
        pub weight: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy_SingleWeightConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ContinuousDeploymentPolicy.SingleWeightConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy_SingleWeightConfig as SingleWeightConfig;
    impl crate::value::ToValue for SingleWeightConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.session_stickiness_config {
                properties.insert(
                    "SessionStickinessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Weight".to_string(),
                crate::value::ToValue::to_value(&self.weight),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-continuousdeploymentpolicy-singleweightpolicyconfig.html
    pub struct SingleWeightPolicyConfig_ {
        pub session_stickiness_config: Option<Box<SessionStickinessConfig_>>,
        pub weight: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy_SingleWeightPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ContinuousDeploymentPolicy.SingleWeightPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy_SingleWeightPolicyConfig as SingleWeightPolicyConfig;
    impl crate::value::ToValue for SingleWeightPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.session_stickiness_config {
                properties.insert(
                    "SessionStickinessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Weight".to_string(),
                crate::value::ToValue::to_value(&self.weight),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-continuousdeploymentpolicy-trafficconfig.html
    pub struct TrafficConfig_ {
        pub single_header_config: Option<Box<SingleHeaderConfig_>>,
        pub single_weight_config: Option<Box<SingleWeightConfig_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy_TrafficConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ContinuousDeploymentPolicy.TrafficConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy_TrafficConfig as TrafficConfig;
    impl crate::value::ToValue for TrafficConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.single_header_config {
                properties.insert(
                    "SingleHeaderConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_weight_config {
                properties.insert(
                    "SingleWeightConfig".to_string(),
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
}
pub mod distribution {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cachebehavior.html
    pub struct CacheBehavior_ {
        pub allowed_methods: Option<Vec<crate::value::ExpString>>,
        pub cache_policy_id: Option<crate::value::ExpString>,
        pub cached_methods: Option<Vec<crate::value::ExpString>>,
        pub compress: Option<crate::value::ExpBool>,
        pub default_ttl: Option<f64>,
        pub field_level_encryption_id: Option<crate::value::ExpString>,
        pub forwarded_values: Option<Box<ForwardedValues_>>,
        pub function_associations: Option<Vec<FunctionAssociation_>>,
        pub grpc_config: Option<Box<GrpcConfig_>>,
        pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation_>>,
        pub max_ttl: Option<f64>,
        pub min_ttl: Option<f64>,
        pub origin_request_policy_id: Option<crate::value::ExpString>,
        pub path_pattern: crate::value::ExpString,
        pub realtime_log_config_arn: Option<crate::value::ExpString>,
        pub response_headers_policy_id: Option<crate::value::ExpString>,
        pub smooth_streaming: Option<crate::value::ExpBool>,
        pub target_origin_id: crate::value::ExpString,
        pub trusted_key_groups: Option<Vec<crate::value::ExpString>>,
        pub trusted_signers: Option<Vec<crate::value::ExpString>>,
        pub viewer_protocol_policy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_CacheBehavior {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.CacheBehavior"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_CacheBehavior as CacheBehavior;
    impl crate::value::ToValue for CacheBehavior_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_methods {
                properties.insert(
                    "AllowedMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_policy_id {
                properties.insert(
                    "CachePolicyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cached_methods {
                properties.insert(
                    "CachedMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compress {
                properties.insert(
                    "Compress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_ttl {
                properties.insert(
                    "DefaultTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_level_encryption_id {
                properties.insert(
                    "FieldLevelEncryptionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_values {
                properties.insert(
                    "ForwardedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.function_associations {
                properties.insert(
                    "FunctionAssociations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.grpc_config {
                properties.insert(
                    "GrpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_associations {
                properties.insert(
                    "LambdaFunctionAssociations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_ttl {
                properties.insert("MaxTTL".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min_ttl {
                properties.insert("MinTTL".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.origin_request_policy_id {
                properties.insert(
                    "OriginRequestPolicyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PathPattern".to_string(),
                crate::value::ToValue::to_value(&self.path_pattern),
            );
            if let Some(ref value) = self.realtime_log_config_arn {
                properties.insert(
                    "RealtimeLogConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_headers_policy_id {
                properties.insert(
                    "ResponseHeadersPolicyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.smooth_streaming {
                properties.insert(
                    "SmoothStreaming".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetOriginId".to_string(),
                crate::value::ToValue::to_value(&self.target_origin_id),
            );
            if let Some(ref value) = self.trusted_key_groups {
                properties.insert(
                    "TrustedKeyGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trusted_signers {
                properties.insert(
                    "TrustedSigners".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ViewerProtocolPolicy".to_string(),
                crate::value::ToValue::to_value(&self.viewer_protocol_policy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-cookies.html
    pub struct Cookies_ {
        pub forward: crate::value::ExpString,
        pub whitelisted_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_Cookies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.Cookies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_Cookies as Cookies;
    impl crate::value::ToValue for Cookies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Forward".to_string(),
                crate::value::ToValue::to_value(&self.forward),
            );
            if let Some(ref value) = self.whitelisted_names {
                properties.insert(
                    "WhitelistedNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customerrorresponse.html
    pub struct CustomErrorResponse_ {
        pub error_caching_min_ttl: Option<f64>,
        pub error_code: i64,
        pub response_code: Option<i64>,
        pub response_page_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_CustomErrorResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.CustomErrorResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_CustomErrorResponse as CustomErrorResponse;
    impl crate::value::ToValue for CustomErrorResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_caching_min_ttl {
                properties.insert(
                    "ErrorCachingMinTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ErrorCode".to_string(),
                crate::value::ToValue::to_value(&self.error_code),
            );
            if let Some(ref value) = self.response_code {
                properties.insert(
                    "ResponseCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_page_path {
                properties.insert(
                    "ResponsePagePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-customoriginconfig.html
    pub struct CustomOriginConfig_ {
        pub http_port: Option<i64>,
        pub https_port: Option<i64>,
        pub ip_address_type: Option<crate::value::ExpString>,
        pub origin_keepalive_timeout: Option<i64>,
        pub origin_protocol_policy: crate::value::ExpString,
        pub origin_read_timeout: Option<i64>,
        pub origin_ssl_protocols: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_CustomOriginConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.CustomOriginConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_CustomOriginConfig as CustomOriginConfig;
    impl crate::value::ToValue for CustomOriginConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http_port {
                properties.insert(
                    "HTTPPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.https_port {
                properties.insert(
                    "HTTPSPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_keepalive_timeout {
                properties.insert(
                    "OriginKeepaliveTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OriginProtocolPolicy".to_string(),
                crate::value::ToValue::to_value(&self.origin_protocol_policy),
            );
            if let Some(ref value) = self.origin_read_timeout {
                properties.insert(
                    "OriginReadTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_ssl_protocols {
                properties.insert(
                    "OriginSSLProtocols".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-defaultcachebehavior.html
    pub struct DefaultCacheBehavior_ {
        pub allowed_methods: Option<Vec<crate::value::ExpString>>,
        pub cache_policy_id: Option<crate::value::ExpString>,
        pub cached_methods: Option<Vec<crate::value::ExpString>>,
        pub compress: Option<crate::value::ExpBool>,
        pub default_ttl: Option<f64>,
        pub field_level_encryption_id: Option<crate::value::ExpString>,
        pub forwarded_values: Option<Box<ForwardedValues_>>,
        pub function_associations: Option<Vec<FunctionAssociation_>>,
        pub grpc_config: Option<Box<GrpcConfig_>>,
        pub lambda_function_associations: Option<Vec<LambdaFunctionAssociation_>>,
        pub max_ttl: Option<f64>,
        pub min_ttl: Option<f64>,
        pub origin_request_policy_id: Option<crate::value::ExpString>,
        pub realtime_log_config_arn: Option<crate::value::ExpString>,
        pub response_headers_policy_id: Option<crate::value::ExpString>,
        pub smooth_streaming: Option<crate::value::ExpBool>,
        pub target_origin_id: crate::value::ExpString,
        pub trusted_key_groups: Option<Vec<crate::value::ExpString>>,
        pub trusted_signers: Option<Vec<crate::value::ExpString>>,
        pub viewer_protocol_policy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_DefaultCacheBehavior {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.DefaultCacheBehavior"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_DefaultCacheBehavior as DefaultCacheBehavior;
    impl crate::value::ToValue for DefaultCacheBehavior_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_methods {
                properties.insert(
                    "AllowedMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cache_policy_id {
                properties.insert(
                    "CachePolicyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cached_methods {
                properties.insert(
                    "CachedMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compress {
                properties.insert(
                    "Compress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_ttl {
                properties.insert(
                    "DefaultTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field_level_encryption_id {
                properties.insert(
                    "FieldLevelEncryptionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_values {
                properties.insert(
                    "ForwardedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.function_associations {
                properties.insert(
                    "FunctionAssociations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.grpc_config {
                properties.insert(
                    "GrpcConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_associations {
                properties.insert(
                    "LambdaFunctionAssociations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_ttl {
                properties.insert("MaxTTL".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min_ttl {
                properties.insert("MinTTL".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.origin_request_policy_id {
                properties.insert(
                    "OriginRequestPolicyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.realtime_log_config_arn {
                properties.insert(
                    "RealtimeLogConfigArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_headers_policy_id {
                properties.insert(
                    "ResponseHeadersPolicyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.smooth_streaming {
                properties.insert(
                    "SmoothStreaming".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetOriginId".to_string(),
                crate::value::ToValue::to_value(&self.target_origin_id),
            );
            if let Some(ref value) = self.trusted_key_groups {
                properties.insert(
                    "TrustedKeyGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trusted_signers {
                properties.insert(
                    "TrustedSigners".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ViewerProtocolPolicy".to_string(),
                crate::value::ToValue::to_value(&self.viewer_protocol_policy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-definition.html
    pub struct Definition_ {
        pub string_schema: Option<Box<StringSchema_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_Definition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.Definition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_Definition as Definition;
    impl crate::value::ToValue for Definition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.string_schema {
                properties.insert(
                    "StringSchema".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-distributionconfig.html
    pub struct DistributionConfig_ {
        pub aliases: Option<Vec<crate::value::ExpString>>,
        pub anycast_ip_list_id: Option<crate::value::ExpString>,
        pub cnam_es: Option<Vec<crate::value::ExpString>>,
        pub cache_behaviors: Option<Vec<CacheBehavior_>>,
        pub comment: Option<crate::value::ExpString>,
        pub connection_mode: Option<crate::value::ExpString>,
        pub continuous_deployment_policy_id: Option<crate::value::ExpString>,
        pub custom_error_responses: Option<Vec<CustomErrorResponse_>>,
        pub custom_origin: Option<Box<LegacyCustomOrigin_>>,
        pub default_cache_behavior: Box<DefaultCacheBehavior_>,
        pub default_root_object: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
        pub http_version: Option<crate::value::ExpString>,
        pub ipv6_enabled: Option<crate::value::ExpBool>,
        pub logging: Option<Box<Logging_>>,
        pub origin_groups: Option<Box<OriginGroups_>>,
        pub origins: Option<Vec<Origin_>>,
        pub price_class: Option<crate::value::ExpString>,
        pub restrictions: Option<Box<Restrictions_>>,
        pub s3_origin: Option<Box<LegacyS3Origin_>>,
        pub staging: Option<crate::value::ExpBool>,
        pub tenant_config: Option<Box<TenantConfig_>>,
        pub viewer_certificate: Option<Box<ViewerCertificate_>>,
        pub web_acl_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_DistributionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.DistributionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_DistributionConfig as DistributionConfig;
    impl crate::value::ToValue for DistributionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aliases {
                properties.insert(
                    "Aliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.anycast_ip_list_id {
                properties.insert(
                    "AnycastIpListId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cnam_es {
                properties.insert("CNAMEs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cache_behaviors {
                properties.insert(
                    "CacheBehaviors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_mode {
                properties.insert(
                    "ConnectionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.continuous_deployment_policy_id {
                properties.insert(
                    "ContinuousDeploymentPolicyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_error_responses {
                properties.insert(
                    "CustomErrorResponses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_origin {
                properties.insert(
                    "CustomOrigin".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DefaultCacheBehavior".to_string(),
                crate::value::ToValue::to_value(&self.default_cache_behavior),
            );
            if let Some(ref value) = self.default_root_object {
                properties.insert(
                    "DefaultRootObject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.http_version {
                properties.insert(
                    "HttpVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_enabled {
                properties.insert(
                    "IPV6Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging {
                properties.insert(
                    "Logging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_groups {
                properties.insert(
                    "OriginGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origins {
                properties.insert(
                    "Origins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.price_class {
                properties.insert(
                    "PriceClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restrictions {
                properties.insert(
                    "Restrictions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_origin {
                properties.insert(
                    "S3Origin".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.staging {
                properties.insert(
                    "Staging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tenant_config {
                properties.insert(
                    "TenantConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.viewer_certificate {
                properties.insert(
                    "ViewerCertificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.web_acl_id {
                properties.insert(
                    "WebACLId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-forwardedvalues.html
    pub struct ForwardedValues_ {
        pub cookies: Option<Box<Cookies_>>,
        pub headers: Option<Vec<crate::value::ExpString>>,
        pub query_string: crate::value::ExpBool,
        pub query_string_cache_keys: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_ForwardedValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.ForwardedValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_ForwardedValues as ForwardedValues;
    impl crate::value::ToValue for ForwardedValues_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cookies {
                properties.insert(
                    "Cookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueryString".to_string(),
                crate::value::ToValue::to_value(&self.query_string),
            );
            if let Some(ref value) = self.query_string_cache_keys {
                properties.insert(
                    "QueryStringCacheKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-functionassociation.html
    pub struct FunctionAssociation_ {
        pub event_type: Option<crate::value::ExpString>,
        pub function_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_FunctionAssociation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.FunctionAssociation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_FunctionAssociation as FunctionAssociation;
    impl crate::value::ToValue for FunctionAssociation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.event_type {
                properties.insert(
                    "EventType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.function_arn {
                properties.insert(
                    "FunctionARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-georestriction.html
    pub struct GeoRestriction_ {
        pub locations: Option<Vec<crate::value::ExpString>>,
        pub restriction_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_GeoRestriction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.GeoRestriction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_GeoRestriction as GeoRestriction;
    impl crate::value::ToValue for GeoRestriction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.locations {
                properties.insert(
                    "Locations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RestrictionType".to_string(),
                crate::value::ToValue::to_value(&self.restriction_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-grpcconfig.html
    pub struct GrpcConfig_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_GrpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.GrpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_GrpcConfig as GrpcConfig;
    impl crate::value::ToValue for GrpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-lambdafunctionassociation.html
    pub struct LambdaFunctionAssociation_ {
        pub event_type: Option<crate::value::ExpString>,
        pub include_body: Option<crate::value::ExpBool>,
        pub lambda_function_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_LambdaFunctionAssociation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.LambdaFunctionAssociation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_LambdaFunctionAssociation as LambdaFunctionAssociation;
    impl crate::value::ToValue for LambdaFunctionAssociation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.event_type {
                properties.insert(
                    "EventType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_body {
                properties.insert(
                    "IncludeBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_arn {
                properties.insert(
                    "LambdaFunctionARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacycustomorigin.html
    pub struct LegacyCustomOrigin_ {
        pub dns_name: crate::value::ExpString,
        pub http_port: Option<i64>,
        pub https_port: Option<i64>,
        pub origin_protocol_policy: crate::value::ExpString,
        pub origin_ssl_protocols: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_LegacyCustomOrigin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.LegacyCustomOrigin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_LegacyCustomOrigin as LegacyCustomOrigin;
    impl crate::value::ToValue for LegacyCustomOrigin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DNSName".to_string(),
                crate::value::ToValue::to_value(&self.dns_name),
            );
            if let Some(ref value) = self.http_port {
                properties.insert(
                    "HTTPPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.https_port {
                properties.insert(
                    "HTTPSPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OriginProtocolPolicy".to_string(),
                crate::value::ToValue::to_value(&self.origin_protocol_policy),
            );
            properties.insert(
                "OriginSSLProtocols".to_string(),
                crate::value::ToValue::to_value(&self.origin_ssl_protocols),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-legacys3origin.html
    pub struct LegacyS3Origin_ {
        pub dns_name: crate::value::ExpString,
        pub origin_access_identity: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_LegacyS3Origin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.LegacyS3Origin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_LegacyS3Origin as LegacyS3Origin;
    impl crate::value::ToValue for LegacyS3Origin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DNSName".to_string(),
                crate::value::ToValue::to_value(&self.dns_name),
            );
            if let Some(ref value) = self.origin_access_identity {
                properties.insert(
                    "OriginAccessIdentity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-logging.html
    pub struct Logging_ {
        pub bucket: Option<crate::value::ExpString>,
        pub include_cookies: Option<crate::value::ExpBool>,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_Logging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.Logging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_Logging as Logging;
    impl crate::value::ToValue for Logging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket {
                properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.include_cookies {
                properties.insert(
                    "IncludeCookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origin.html
    pub struct Origin_ {
        pub connection_attempts: Option<i64>,
        pub connection_timeout: Option<i64>,
        pub custom_origin_config: Option<Box<CustomOriginConfig_>>,
        pub domain_name: crate::value::ExpString,
        pub id: crate::value::ExpString,
        pub origin_access_control_id: Option<crate::value::ExpString>,
        pub origin_custom_headers: Option<Vec<OriginCustomHeader_>>,
        pub origin_path: Option<crate::value::ExpString>,
        pub origin_shield: Option<Box<OriginShield_>>,
        pub response_completion_timeout: Option<i64>,
        pub s3_origin_config: Option<Box<S3OriginConfig_>>,
        pub vpc_origin_config: Option<Box<VpcOriginConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_Origin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.Origin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_Origin as Origin;
    impl crate::value::ToValue for Origin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_attempts {
                properties.insert(
                    "ConnectionAttempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connection_timeout {
                properties.insert(
                    "ConnectionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_origin_config {
                properties.insert(
                    "CustomOriginConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(&self.domain_name),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.origin_access_control_id {
                properties.insert(
                    "OriginAccessControlId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_custom_headers {
                properties.insert(
                    "OriginCustomHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_path {
                properties.insert(
                    "OriginPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_shield {
                properties.insert(
                    "OriginShield".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_completion_timeout {
                properties.insert(
                    "ResponseCompletionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_origin_config {
                properties.insert(
                    "S3OriginConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_origin_config {
                properties.insert(
                    "VpcOriginConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origincustomheader.html
    pub struct OriginCustomHeader_ {
        pub header_name: crate::value::ExpString,
        pub header_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_OriginCustomHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.OriginCustomHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_OriginCustomHeader as OriginCustomHeader;
    impl crate::value::ToValue for OriginCustomHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HeaderName".to_string(),
                crate::value::ToValue::to_value(&self.header_name),
            );
            properties.insert(
                "HeaderValue".to_string(),
                crate::value::ToValue::to_value(&self.header_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroup.html
    pub struct OriginGroup_ {
        pub failover_criteria: Box<OriginGroupFailoverCriteria_>,
        pub id: crate::value::ExpString,
        pub members: Box<OriginGroupMembers_>,
        pub selection_criteria: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_OriginGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.OriginGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_OriginGroup as OriginGroup;
    impl crate::value::ToValue for OriginGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailoverCriteria".to_string(),
                crate::value::ToValue::to_value(&self.failover_criteria),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Members".to_string(),
                crate::value::ToValue::to_value(&self.members),
            );
            if let Some(ref value) = self.selection_criteria {
                properties.insert(
                    "SelectionCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupfailovercriteria.html
    pub struct OriginGroupFailoverCriteria_ {
        pub status_codes: Box<StatusCodes_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_OriginGroupFailoverCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.OriginGroupFailoverCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_OriginGroupFailoverCriteria as OriginGroupFailoverCriteria;
    impl crate::value::ToValue for OriginGroupFailoverCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StatusCodes".to_string(),
                crate::value::ToValue::to_value(&self.status_codes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmember.html
    pub struct OriginGroupMember_ {
        pub origin_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_OriginGroupMember {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.OriginGroupMember"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_OriginGroupMember as OriginGroupMember;
    impl crate::value::ToValue for OriginGroupMember_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OriginId".to_string(),
                crate::value::ToValue::to_value(&self.origin_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroupmembers.html
    pub struct OriginGroupMembers_ {
        pub items: Vec<OriginGroupMember_>,
        pub quantity: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_OriginGroupMembers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.OriginGroupMembers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_OriginGroupMembers as OriginGroupMembers;
    impl crate::value::ToValue for OriginGroupMembers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.insert(
                "Quantity".to_string(),
                crate::value::ToValue::to_value(&self.quantity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-origingroups.html
    pub struct OriginGroups_ {
        pub items: Option<Vec<OriginGroup_>>,
        pub quantity: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_OriginGroups {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.OriginGroups"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_OriginGroups as OriginGroups;
    impl crate::value::ToValue for OriginGroups_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.items {
                properties.insert("Items".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Quantity".to_string(),
                crate::value::ToValue::to_value(&self.quantity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-originshield.html
    pub struct OriginShield_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub origin_shield_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_OriginShield {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.OriginShield"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_OriginShield as OriginShield;
    impl crate::value::ToValue for OriginShield_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_shield_region {
                properties.insert(
                    "OriginShieldRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-parameterdefinition.html
    pub struct ParameterDefinition_ {
        pub definition: Box<Definition_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_ParameterDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.ParameterDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_ParameterDefinition as ParameterDefinition;
    impl crate::value::ToValue for ParameterDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Definition".to_string(),
                crate::value::ToValue::to_value(&self.definition),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-restrictions.html
    pub struct Restrictions_ {
        pub geo_restriction: Box<GeoRestriction_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_Restrictions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.Restrictions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_Restrictions as Restrictions;
    impl crate::value::ToValue for Restrictions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GeoRestriction".to_string(),
                crate::value::ToValue::to_value(&self.geo_restriction),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-s3originconfig.html
    pub struct S3OriginConfig_ {
        pub origin_access_identity: Option<crate::value::ExpString>,
        pub origin_read_timeout: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_S3OriginConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.S3OriginConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_S3OriginConfig as S3OriginConfig;
    impl crate::value::ToValue for S3OriginConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.origin_access_identity {
                properties.insert(
                    "OriginAccessIdentity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_read_timeout {
                properties.insert(
                    "OriginReadTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-statuscodes.html
    pub struct StatusCodes_ {
        pub items: Vec<i64>,
        pub quantity: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_StatusCodes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.StatusCodes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_StatusCodes as StatusCodes;
    impl crate::value::ToValue for StatusCodes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.insert(
                "Quantity".to_string(),
                crate::value::ToValue::to_value(&self.quantity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-stringschema.html
    pub struct StringSchema_ {
        pub comment: Option<crate::value::ExpString>,
        pub default_value: Option<crate::value::ExpString>,
        pub required: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_StringSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.StringSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_StringSchema as StringSchema;
    impl crate::value::ToValue for StringSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Required".to_string(),
                crate::value::ToValue::to_value(&self.required),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-tenantconfig.html
    pub struct TenantConfig_ {
        pub parameter_definitions: Option<Vec<ParameterDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_TenantConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.TenantConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_TenantConfig as TenantConfig;
    impl crate::value::ToValue for TenantConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameter_definitions {
                properties.insert(
                    "ParameterDefinitions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-viewercertificate.html
    pub struct ViewerCertificate_ {
        pub acm_certificate_arn: Option<crate::value::ExpString>,
        pub cloud_front_default_certificate: Option<crate::value::ExpBool>,
        pub iam_certificate_id: Option<crate::value::ExpString>,
        pub minimum_protocol_version: Option<crate::value::ExpString>,
        pub ssl_support_method: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_ViewerCertificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.ViewerCertificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_ViewerCertificate as ViewerCertificate;
    impl crate::value::ToValue for ViewerCertificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acm_certificate_arn {
                properties.insert(
                    "AcmCertificateArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_front_default_certificate {
                properties.insert(
                    "CloudFrontDefaultCertificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_certificate_id {
                properties.insert(
                    "IamCertificateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_protocol_version {
                properties.insert(
                    "MinimumProtocolVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_support_method {
                properties.insert(
                    "SslSupportMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distribution-vpcoriginconfig.html
    pub struct VpcOriginConfig_ {
        pub origin_keepalive_timeout: Option<i64>,
        pub origin_read_timeout: Option<i64>,
        pub vpc_origin_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Distribution_VpcOriginConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Distribution.VpcOriginConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Distribution_VpcOriginConfig as VpcOriginConfig;
    impl crate::value::ToValue for VpcOriginConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.origin_keepalive_timeout {
                properties.insert(
                    "OriginKeepaliveTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_read_timeout {
                properties.insert(
                    "OriginReadTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VpcOriginId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_origin_id),
            );
            properties.into()
        }
    }
}
pub mod distributiontenant {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distributiontenant-certificate.html
    pub struct Certificate_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_DistributionTenant_Certificate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::DistributionTenant.Certificate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_DistributionTenant_Certificate as Certificate;
    impl crate::value::ToValue for Certificate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distributiontenant-customizations.html
    pub struct Customizations_ {
        pub certificate: Option<Box<Certificate_>>,
        pub geo_restrictions: Option<Box<GeoRestrictionCustomization_>>,
        pub web_acl: Option<Box<WebAclCustomization_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_DistributionTenant_Customizations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::DistributionTenant.Customizations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_DistributionTenant_Customizations as Customizations;
    impl crate::value::ToValue for Customizations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate {
                properties.insert(
                    "Certificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.geo_restrictions {
                properties.insert(
                    "GeoRestrictions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.web_acl {
                properties.insert("WebAcl".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distributiontenant-domainresult.html
    pub struct DomainResult_ {
        pub domain: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_DistributionTenant_DomainResult {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::DistributionTenant.DomainResult"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_DistributionTenant_DomainResult as DomainResult;
    impl crate::value::ToValue for DomainResult_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain {
                properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distributiontenant-georestrictioncustomization.html
    pub struct GeoRestrictionCustomization_ {
        pub locations: Option<Vec<crate::value::ExpString>>,
        pub restriction_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_DistributionTenant_GeoRestrictionCustomization {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::DistributionTenant.GeoRestrictionCustomization"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_DistributionTenant_GeoRestrictionCustomization as GeoRestrictionCustomization;
    impl crate::value::ToValue for GeoRestrictionCustomization_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.locations {
                properties.insert(
                    "Locations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restriction_type {
                properties.insert(
                    "RestrictionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distributiontenant-managedcertificaterequest.html
    pub struct ManagedCertificateRequest_ {
        pub certificate_transparency_logging_preference: Option<crate::value::ExpString>,
        pub primary_domain_name: Option<crate::value::ExpString>,
        pub validation_token_host: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_DistributionTenant_ManagedCertificateRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::DistributionTenant.ManagedCertificateRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_DistributionTenant_ManagedCertificateRequest as ManagedCertificateRequest;
    impl crate::value::ToValue for ManagedCertificateRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_transparency_logging_preference {
                properties.insert(
                    "CertificateTransparencyLoggingPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.primary_domain_name {
                properties.insert(
                    "PrimaryDomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.validation_token_host {
                properties.insert(
                    "ValidationTokenHost".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distributiontenant-parameter.html
    pub struct Parameter_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_DistributionTenant_Parameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::DistributionTenant.Parameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_DistributionTenant_Parameter as Parameter;
    impl crate::value::ToValue for Parameter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-distributiontenant-webaclcustomization.html
    pub struct WebAclCustomization_ {
        pub action: Option<crate::value::ExpString>,
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_DistributionTenant_WebAclCustomization {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::DistributionTenant.WebAclCustomization"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_DistributionTenant_WebAclCustomization as WebAclCustomization;
    impl crate::value::ToValue for WebAclCustomization_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod function {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionconfig.html
    pub struct FunctionConfig_ {
        pub comment: crate::value::ExpString,
        pub key_value_store_associations: Option<Vec<KeyValueStoreAssociation_>>,
        pub runtime: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Function_FunctionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Function.FunctionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Function_FunctionConfig as FunctionConfig;
    impl crate::value::ToValue for FunctionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Comment".to_string(),
                crate::value::ToValue::to_value(&self.comment),
            );
            if let Some(ref value) = self.key_value_store_associations {
                properties.insert(
                    "KeyValueStoreAssociations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Runtime".to_string(),
                crate::value::ToValue::to_value(&self.runtime),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-functionmetadata.html
    pub struct FunctionMetadata_ {
        pub function_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Function_FunctionMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Function.FunctionMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Function_FunctionMetadata as FunctionMetadata;
    impl crate::value::ToValue for FunctionMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.function_arn {
                properties.insert(
                    "FunctionARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-function-keyvaluestoreassociation.html
    pub struct KeyValueStoreAssociation_ {
        pub key_value_store_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_Function_KeyValueStoreAssociation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::Function.KeyValueStoreAssociation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_Function_KeyValueStoreAssociation as KeyValueStoreAssociation;
    impl crate::value::ToValue for KeyValueStoreAssociation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KeyValueStoreARN".to_string(),
                crate::value::ToValue::to_value(&self.key_value_store_arn),
            );
            properties.into()
        }
    }
}
pub mod keygroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keygroup-keygroupconfig.html
    pub struct KeyGroupConfig_ {
        pub comment: Option<crate::value::ExpString>,
        pub items: Vec<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_KeyGroup_KeyGroupConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::KeyGroup.KeyGroupConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_KeyGroup_KeyGroupConfig as KeyGroupConfig;
    impl crate::value::ToValue for KeyGroupConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod keyvaluestore {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-keyvaluestore-importsource.html
    pub struct ImportSource_ {
        pub source_arn: crate::value::ExpString,
        pub source_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_KeyValueStore_ImportSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::KeyValueStore.ImportSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_KeyValueStore_ImportSource as ImportSource;
    impl crate::value::ToValue for ImportSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceArn".to_string(),
                crate::value::ToValue::to_value(&self.source_arn),
            );
            properties.insert(
                "SourceType".to_string(),
                crate::value::ToValue::to_value(&self.source_type),
            );
            properties.into()
        }
    }
}
pub mod monitoringsubscription {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-monitoringsubscription-monitoringsubscription.html
    pub struct MonitoringSubscription_ {
        pub realtime_metrics_subscription_config: Option<Box<RealtimeMetricsSubscriptionConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_MonitoringSubscription_MonitoringSubscription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::MonitoringSubscription.MonitoringSubscription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_MonitoringSubscription_MonitoringSubscription as MonitoringSubscription;
    impl crate::value::ToValue for MonitoringSubscription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.realtime_metrics_subscription_config {
                properties.insert(
                    "RealtimeMetricsSubscriptionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-monitoringsubscription-realtimemetricssubscriptionconfig.html
    pub struct RealtimeMetricsSubscriptionConfig_ {
        pub realtime_metrics_subscription_status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_MonitoringSubscription_RealtimeMetricsSubscriptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::MonitoringSubscription.RealtimeMetricsSubscriptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_MonitoringSubscription_RealtimeMetricsSubscriptionConfig as RealtimeMetricsSubscriptionConfig;
    impl crate::value::ToValue for RealtimeMetricsSubscriptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RealtimeMetricsSubscriptionStatus".to_string(),
                crate::value::ToValue::to_value(&self.realtime_metrics_subscription_status),
            );
            properties.into()
        }
    }
}
pub mod originaccesscontrol {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originaccesscontrol-originaccesscontrolconfig.html
    pub struct OriginAccessControlConfig_ {
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub origin_access_control_origin_type: crate::value::ExpString,
        pub signing_behavior: crate::value::ExpString,
        pub signing_protocol: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_OriginAccessControl_OriginAccessControlConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::OriginAccessControl.OriginAccessControlConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_OriginAccessControl_OriginAccessControlConfig as OriginAccessControlConfig;
    impl crate::value::ToValue for OriginAccessControlConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
                "OriginAccessControlOriginType".to_string(),
                crate::value::ToValue::to_value(&self.origin_access_control_origin_type),
            );
            properties.insert(
                "SigningBehavior".to_string(),
                crate::value::ToValue::to_value(&self.signing_behavior),
            );
            properties.insert(
                "SigningProtocol".to_string(),
                crate::value::ToValue::to_value(&self.signing_protocol),
            );
            properties.into()
        }
    }
}
pub mod originrequestpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-cookiesconfig.html
    pub struct CookiesConfig_ {
        pub cookie_behavior: crate::value::ExpString,
        pub cookies: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_OriginRequestPolicy_CookiesConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::OriginRequestPolicy.CookiesConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_OriginRequestPolicy_CookiesConfig as CookiesConfig;
    impl crate::value::ToValue for CookiesConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CookieBehavior".to_string(),
                crate::value::ToValue::to_value(&self.cookie_behavior),
            );
            if let Some(ref value) = self.cookies {
                properties.insert(
                    "Cookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-headersconfig.html
    pub struct HeadersConfig_ {
        pub header_behavior: crate::value::ExpString,
        pub headers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_OriginRequestPolicy_HeadersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::OriginRequestPolicy.HeadersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_OriginRequestPolicy_HeadersConfig as HeadersConfig;
    impl crate::value::ToValue for HeadersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HeaderBehavior".to_string(),
                crate::value::ToValue::to_value(&self.header_behavior),
            );
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-originrequestpolicyconfig.html
    pub struct OriginRequestPolicyConfig_ {
        pub comment: Option<crate::value::ExpString>,
        pub cookies_config: Box<CookiesConfig_>,
        pub headers_config: Box<HeadersConfig_>,
        pub name: crate::value::ExpString,
        pub query_strings_config: Box<QueryStringsConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_OriginRequestPolicy_OriginRequestPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::OriginRequestPolicy.OriginRequestPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_OriginRequestPolicy_OriginRequestPolicyConfig as OriginRequestPolicyConfig;
    impl crate::value::ToValue for OriginRequestPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CookiesConfig".to_string(),
                crate::value::ToValue::to_value(&self.cookies_config),
            );
            properties.insert(
                "HeadersConfig".to_string(),
                crate::value::ToValue::to_value(&self.headers_config),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "QueryStringsConfig".to_string(),
                crate::value::ToValue::to_value(&self.query_strings_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-originrequestpolicy-querystringsconfig.html
    pub struct QueryStringsConfig_ {
        pub query_string_behavior: crate::value::ExpString,
        pub query_strings: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_OriginRequestPolicy_QueryStringsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::OriginRequestPolicy.QueryStringsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_OriginRequestPolicy_QueryStringsConfig as QueryStringsConfig;
    impl crate::value::ToValue for QueryStringsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QueryStringBehavior".to_string(),
                crate::value::ToValue::to_value(&self.query_string_behavior),
            );
            if let Some(ref value) = self.query_strings {
                properties.insert(
                    "QueryStrings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod publickey {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-publickey-publickeyconfig.html
    pub struct PublicKeyConfig_ {
        pub caller_reference: crate::value::ExpString,
        pub comment: Option<crate::value::ExpString>,
        pub encoded_key: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_PublicKey_PublicKeyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::PublicKey.PublicKeyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_PublicKey_PublicKeyConfig as PublicKeyConfig;
    impl crate::value::ToValue for PublicKeyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CallerReference".to_string(),
                crate::value::ToValue::to_value(&self.caller_reference),
            );
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EncodedKey".to_string(),
                crate::value::ToValue::to_value(&self.encoded_key),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod realtimelogconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-endpoint.html
    pub struct EndPoint_ {
        pub kinesis_stream_config: Box<KinesisStreamConfig_>,
        pub stream_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_RealtimeLogConfig_EndPoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::RealtimeLogConfig.EndPoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_RealtimeLogConfig_EndPoint as EndPoint;
    impl crate::value::ToValue for EndPoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KinesisStreamConfig".to_string(),
                crate::value::ToValue::to_value(&self.kinesis_stream_config),
            );
            properties.insert(
                "StreamType".to_string(),
                crate::value::ToValue::to_value(&self.stream_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-realtimelogconfig-kinesisstreamconfig.html
    pub struct KinesisStreamConfig_ {
        pub role_arn: crate::value::ExpString,
        pub stream_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_RealtimeLogConfig_KinesisStreamConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::RealtimeLogConfig.KinesisStreamConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_RealtimeLogConfig_KinesisStreamConfig as KinesisStreamConfig;
    impl crate::value::ToValue for KinesisStreamConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "StreamArn".to_string(),
                crate::value::ToValue::to_value(&self.stream_arn),
            );
            properties.into()
        }
    }
}
pub mod responseheaderspolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolallowheaders.html
    pub struct AccessControlAllowHeaders_ {
        pub items: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_AccessControlAllowHeaders {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.AccessControlAllowHeaders"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_AccessControlAllowHeaders as AccessControlAllowHeaders;
    impl crate::value::ToValue for AccessControlAllowHeaders_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolallowmethods.html
    pub struct AccessControlAllowMethods_ {
        pub items: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_AccessControlAllowMethods {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.AccessControlAllowMethods"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_AccessControlAllowMethods as AccessControlAllowMethods;
    impl crate::value::ToValue for AccessControlAllowMethods_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolalloworigins.html
    pub struct AccessControlAllowOrigins_ {
        pub items: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_AccessControlAllowOrigins {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.AccessControlAllowOrigins"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_AccessControlAllowOrigins as AccessControlAllowOrigins;
    impl crate::value::ToValue for AccessControlAllowOrigins_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-accesscontrolexposeheaders.html
    pub struct AccessControlExposeHeaders_ {
        pub items: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_AccessControlExposeHeaders {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.AccessControlExposeHeaders"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_AccessControlExposeHeaders as AccessControlExposeHeaders;
    impl crate::value::ToValue for AccessControlExposeHeaders_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-contentsecuritypolicy.html
    pub struct ContentSecurityPolicy_ {
        pub content_security_policy: crate::value::ExpString,
        pub r#override: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_ContentSecurityPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.ContentSecurityPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_ContentSecurityPolicy as ContentSecurityPolicy;
    impl crate::value::ToValue for ContentSecurityPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContentSecurityPolicy".to_string(),
                crate::value::ToValue::to_value(&self.content_security_policy),
            );
            properties.insert(
                "Override".to_string(),
                crate::value::ToValue::to_value(&self.r#override),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-contenttypeoptions.html
    pub struct ContentTypeOptions_ {
        pub r#override: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_ContentTypeOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.ContentTypeOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_ContentTypeOptions as ContentTypeOptions;
    impl crate::value::ToValue for ContentTypeOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Override".to_string(),
                crate::value::ToValue::to_value(&self.r#override),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-corsconfig.html
    pub struct CorsConfig_ {
        pub access_control_allow_credentials: crate::value::ExpBool,
        pub access_control_allow_headers: Box<AccessControlAllowHeaders_>,
        pub access_control_allow_methods: Box<AccessControlAllowMethods_>,
        pub access_control_allow_origins: Box<AccessControlAllowOrigins_>,
        pub access_control_expose_headers: Option<Box<AccessControlExposeHeaders_>>,
        pub access_control_max_age_sec: Option<i64>,
        pub origin_override: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_CorsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.CorsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_CorsConfig as CorsConfig;
    impl crate::value::ToValue for CorsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccessControlAllowCredentials".to_string(),
                crate::value::ToValue::to_value(&self.access_control_allow_credentials),
            );
            properties.insert(
                "AccessControlAllowHeaders".to_string(),
                crate::value::ToValue::to_value(&self.access_control_allow_headers),
            );
            properties.insert(
                "AccessControlAllowMethods".to_string(),
                crate::value::ToValue::to_value(&self.access_control_allow_methods),
            );
            properties.insert(
                "AccessControlAllowOrigins".to_string(),
                crate::value::ToValue::to_value(&self.access_control_allow_origins),
            );
            if let Some(ref value) = self.access_control_expose_headers {
                properties.insert(
                    "AccessControlExposeHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.access_control_max_age_sec {
                properties.insert(
                    "AccessControlMaxAgeSec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OriginOverride".to_string(),
                crate::value::ToValue::to_value(&self.origin_override),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheader.html
    pub struct CustomHeader_ {
        pub header: crate::value::ExpString,
        pub r#override: crate::value::ExpBool,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_CustomHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.CustomHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_CustomHeader as CustomHeader;
    impl crate::value::ToValue for CustomHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Header".to_string(),
                crate::value::ToValue::to_value(&self.header),
            );
            properties.insert(
                "Override".to_string(),
                crate::value::ToValue::to_value(&self.r#override),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-customheadersconfig.html
    pub struct CustomHeadersConfig_ {
        pub items: Vec<CustomHeader_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_CustomHeadersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.CustomHeadersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_CustomHeadersConfig as CustomHeadersConfig;
    impl crate::value::ToValue for CustomHeadersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-frameoptions.html
    pub struct FrameOptions_ {
        pub frame_option: crate::value::ExpString,
        pub r#override: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_FrameOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.FrameOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_FrameOptions as FrameOptions;
    impl crate::value::ToValue for FrameOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FrameOption".to_string(),
                crate::value::ToValue::to_value(&self.frame_option),
            );
            properties.insert(
                "Override".to_string(),
                crate::value::ToValue::to_value(&self.r#override),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-referrerpolicy.html
    pub struct ReferrerPolicy_ {
        pub r#override: crate::value::ExpBool,
        pub referrer_policy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_ReferrerPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.ReferrerPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_ReferrerPolicy as ReferrerPolicy;
    impl crate::value::ToValue for ReferrerPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Override".to_string(),
                crate::value::ToValue::to_value(&self.r#override),
            );
            properties.insert(
                "ReferrerPolicy".to_string(),
                crate::value::ToValue::to_value(&self.referrer_policy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-removeheader.html
    pub struct RemoveHeader_ {
        pub header: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_RemoveHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.RemoveHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_RemoveHeader as RemoveHeader;
    impl crate::value::ToValue for RemoveHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Header".to_string(),
                crate::value::ToValue::to_value(&self.header),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-removeheadersconfig.html
    pub struct RemoveHeadersConfig_ {
        pub items: Vec<RemoveHeader_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_RemoveHeadersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.RemoveHeadersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_RemoveHeadersConfig as RemoveHeadersConfig;
    impl crate::value::ToValue for RemoveHeadersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Items".to_string(),
                crate::value::ToValue::to_value(&self.items),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-responseheaderspolicyconfig.html
    pub struct ResponseHeadersPolicyConfig_ {
        pub comment: Option<crate::value::ExpString>,
        pub cors_config: Option<Box<CorsConfig_>>,
        pub custom_headers_config: Option<Box<CustomHeadersConfig_>>,
        pub name: crate::value::ExpString,
        pub remove_headers_config: Option<Box<RemoveHeadersConfig_>>,
        pub security_headers_config: Option<Box<SecurityHeadersConfig_>>,
        pub server_timing_headers_config: Option<Box<ServerTimingHeadersConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_ResponseHeadersPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.ResponseHeadersPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_ResponseHeadersPolicyConfig as ResponseHeadersPolicyConfig;
    impl crate::value::ToValue for ResponseHeadersPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cors_config {
                properties.insert(
                    "CorsConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_headers_config {
                properties.insert(
                    "CustomHeadersConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.remove_headers_config {
                properties.insert(
                    "RemoveHeadersConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_headers_config {
                properties.insert(
                    "SecurityHeadersConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_timing_headers_config {
                properties.insert(
                    "ServerTimingHeadersConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-securityheadersconfig.html
    pub struct SecurityHeadersConfig_ {
        pub content_security_policy: Option<Box<ContentSecurityPolicy_>>,
        pub content_type_options: Option<Box<ContentTypeOptions_>>,
        pub frame_options: Option<Box<FrameOptions_>>,
        pub referrer_policy: Option<Box<ReferrerPolicy_>>,
        pub strict_transport_security: Option<Box<StrictTransportSecurity_>>,
        pub xss_protection: Option<Box<XSSProtection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_SecurityHeadersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.SecurityHeadersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_SecurityHeadersConfig as SecurityHeadersConfig;
    impl crate::value::ToValue for SecurityHeadersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_security_policy {
                properties.insert(
                    "ContentSecurityPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_type_options {
                properties.insert(
                    "ContentTypeOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frame_options {
                properties.insert(
                    "FrameOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.referrer_policy {
                properties.insert(
                    "ReferrerPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strict_transport_security {
                properties.insert(
                    "StrictTransportSecurity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.xss_protection {
                properties.insert(
                    "XSSProtection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-servertimingheadersconfig.html
    pub struct ServerTimingHeadersConfig_ {
        pub enabled: crate::value::ExpBool,
        pub sampling_rate: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_ServerTimingHeadersConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.ServerTimingHeadersConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_ServerTimingHeadersConfig as ServerTimingHeadersConfig;
    impl crate::value::ToValue for ServerTimingHeadersConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.sampling_rate {
                properties.insert(
                    "SamplingRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-stricttransportsecurity.html
    pub struct StrictTransportSecurity_ {
        pub access_control_max_age_sec: i64,
        pub include_subdomains: Option<crate::value::ExpBool>,
        pub r#override: crate::value::ExpBool,
        pub preload: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_StrictTransportSecurity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.StrictTransportSecurity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_StrictTransportSecurity as StrictTransportSecurity;
    impl crate::value::ToValue for StrictTransportSecurity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccessControlMaxAgeSec".to_string(),
                crate::value::ToValue::to_value(&self.access_control_max_age_sec),
            );
            if let Some(ref value) = self.include_subdomains {
                properties.insert(
                    "IncludeSubdomains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Override".to_string(),
                crate::value::ToValue::to_value(&self.r#override),
            );
            if let Some(ref value) = self.preload {
                properties.insert(
                    "Preload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-responseheaderspolicy-xssprotection.html
    pub struct XSSProtection_ {
        pub mode_block: Option<crate::value::ExpBool>,
        pub r#override: crate::value::ExpBool,
        pub protection: crate::value::ExpBool,
        pub report_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_ResponseHeadersPolicy_XSSProtection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::ResponseHeadersPolicy.XSSProtection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_ResponseHeadersPolicy_XSSProtection as XSSProtection;
    impl crate::value::ToValue for XSSProtection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mode_block {
                properties.insert(
                    "ModeBlock".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Override".to_string(),
                crate::value::ToValue::to_value(&self.r#override),
            );
            properties.insert(
                "Protection".to_string(),
                crate::value::ToValue::to_value(&self.protection),
            );
            if let Some(ref value) = self.report_uri {
                properties.insert(
                    "ReportUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod streamingdistribution {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-logging.html
    pub struct Logging_ {
        pub bucket: crate::value::ExpString,
        pub enabled: crate::value::ExpBool,
        pub prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_StreamingDistribution_Logging {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::StreamingDistribution.Logging"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_StreamingDistribution_Logging as Logging;
    impl crate::value::ToValue for Logging_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.insert(
                "Prefix".to_string(),
                crate::value::ToValue::to_value(&self.prefix),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-s3origin.html
    pub struct S3Origin_ {
        pub domain_name: crate::value::ExpString,
        pub origin_access_identity: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_StreamingDistribution_S3Origin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::StreamingDistribution.S3Origin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_StreamingDistribution_S3Origin as S3Origin;
    impl crate::value::ToValue for S3Origin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(&self.domain_name),
            );
            properties.insert(
                "OriginAccessIdentity".to_string(),
                crate::value::ToValue::to_value(&self.origin_access_identity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-streamingdistributionconfig.html
    pub struct StreamingDistributionConfig_ {
        pub aliases: Option<Vec<crate::value::ExpString>>,
        pub comment: crate::value::ExpString,
        pub enabled: crate::value::ExpBool,
        pub logging: Option<Box<Logging_>>,
        pub price_class: Option<crate::value::ExpString>,
        pub s3_origin: Box<S3Origin_>,
        pub trusted_signers: Box<TrustedSigners_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_StreamingDistribution_StreamingDistributionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::StreamingDistribution.StreamingDistributionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_StreamingDistribution_StreamingDistributionConfig as StreamingDistributionConfig;
    impl crate::value::ToValue for StreamingDistributionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aliases {
                properties.insert(
                    "Aliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Comment".to_string(),
                crate::value::ToValue::to_value(&self.comment),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.logging {
                properties.insert(
                    "Logging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.price_class {
                properties.insert(
                    "PriceClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Origin".to_string(),
                crate::value::ToValue::to_value(&self.s3_origin),
            );
            properties.insert(
                "TrustedSigners".to_string(),
                crate::value::ToValue::to_value(&self.trusted_signers),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-streamingdistribution-trustedsigners.html
    pub struct TrustedSigners_ {
        pub aws_account_numbers: Option<Vec<crate::value::ExpString>>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_StreamingDistribution_TrustedSigners {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::StreamingDistribution.TrustedSigners"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_StreamingDistribution_TrustedSigners as TrustedSigners;
    impl crate::value::ToValue for TrustedSigners_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_account_numbers {
                properties.insert(
                    "AwsAccountNumbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
}
pub mod vpcorigin {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudfront-vpcorigin-vpcoriginendpointconfig.html
    pub struct VpcOriginEndpointConfig_ {
        pub arn: crate::value::ExpString,
        pub http_port: Option<i64>,
        pub https_port: Option<i64>,
        pub name: crate::value::ExpString,
        pub origin_protocol_policy: Option<crate::value::ExpString>,
        pub origin_ssl_protocols: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudfront_VpcOrigin_VpcOriginEndpointConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::CloudFront::VpcOrigin.VpcOriginEndpointConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudfront_VpcOrigin_VpcOriginEndpointConfig as VpcOriginEndpointConfig;
    impl crate::value::ToValue for VpcOriginEndpointConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.http_port {
                properties.insert(
                    "HTTPPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.https_port {
                properties.insert(
                    "HTTPSPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.origin_protocol_policy {
                properties.insert(
                    "OriginProtocolPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_ssl_protocols {
                properties.insert(
                    "OriginSSLProtocols".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-anycastiplist.html
pub struct AnycastIpList_ {
    pub ip_count: i64,
    pub name: crate::value::ExpString,
    pub tags: Option<super::cloudfront::anycastiplist::Tags_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_AnycastIpList {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::AnycastIpList"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_AnycastIpList as AnycastIpList;
impl crate::template::ToResource for AnycastIpList_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnycastIpList"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IpCount".to_string(),
            crate::value::ToValue::to_value(&self.ip_count),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cachepolicy.html
pub struct CachePolicy_ {
    pub cache_policy_config: super::cloudfront::cachepolicy::CachePolicyConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_CachePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::CachePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_CachePolicy as CachePolicy;
impl crate::template::ToResource for CachePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CachePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CachePolicyConfig".to_string(),
            crate::value::ToValue::to_value(&self.cache_policy_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-cloudfrontoriginaccessidentity.html
pub struct CloudFrontOriginAccessIdentity_ {
    pub cloud_front_origin_access_identity_config:
        super::cloudfront::cloudfrontoriginaccessidentity::CloudFrontOriginAccessIdentityConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_CloudFrontOriginAccessIdentity {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::CloudFrontOriginAccessIdentity"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_CloudFrontOriginAccessIdentity as CloudFrontOriginAccessIdentity;
impl crate::template::ToResource for CloudFrontOriginAccessIdentity_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CloudFrontOriginAccessIdentity",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CloudFrontOriginAccessIdentityConfig".to_string(),
            crate::value::ToValue::to_value(&self.cloud_front_origin_access_identity_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-connectiongroup.html
pub struct ConnectionGroup_ {
    pub anycast_ip_list_id: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub ipv6_enabled: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_ConnectionGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::ConnectionGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_ConnectionGroup as ConnectionGroup;
impl crate::template::ToResource for ConnectionGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConnectionGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.anycast_ip_list_id {
            properties.insert(
                "AnycastIpListId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_enabled {
            properties.insert(
                "Ipv6Enabled".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-continuousdeploymentpolicy.html
pub struct ContinuousDeploymentPolicy_ {
    pub continuous_deployment_policy_config:
        super::cloudfront::continuousdeploymentpolicy::ContinuousDeploymentPolicyConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_ContinuousDeploymentPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::ContinuousDeploymentPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_ContinuousDeploymentPolicy as ContinuousDeploymentPolicy;
impl crate::template::ToResource for ContinuousDeploymentPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ContinuousDeploymentPolicy",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContinuousDeploymentPolicyConfig".to_string(),
            crate::value::ToValue::to_value(&self.continuous_deployment_policy_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distribution.html
pub struct Distribution_ {
    pub distribution_config: super::cloudfront::distribution::DistributionConfig_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_Distribution {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::Distribution"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_Distribution as Distribution;
impl crate::template::ToResource for Distribution_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Distribution"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DistributionConfig".to_string(),
            crate::value::ToValue::to_value(&self.distribution_config),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-distributiontenant.html
pub struct DistributionTenant_ {
    pub connection_group_id: Option<crate::value::ExpString>,
    pub customizations: Option<super::cloudfront::distributiontenant::Customizations_>,
    pub distribution_id: crate::value::ExpString,
    pub domains: Vec<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub managed_certificate_request:
        Option<super::cloudfront::distributiontenant::ManagedCertificateRequest_>,
    pub name: crate::value::ExpString,
    pub parameters: Option<Vec<super::cloudfront::distributiontenant::Parameter_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_DistributionTenant {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::DistributionTenant"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_DistributionTenant as DistributionTenant;
impl crate::template::ToResource for DistributionTenant_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DistributionTenant"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connection_group_id {
            properties.insert(
                "ConnectionGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customizations {
            properties.insert(
                "Customizations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DistributionId".to_string(),
            crate::value::ToValue::to_value(&self.distribution_id),
        );
        properties.insert(
            "Domains".to_string(),
            crate::value::ToValue::to_value(&self.domains),
        );
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.managed_certificate_request {
            properties.insert(
                "ManagedCertificateRequest".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-function.html
pub struct Function_ {
    pub auto_publish: Option<crate::value::ExpBool>,
    pub function_code: crate::value::ExpString,
    pub function_config: super::cloudfront::function::FunctionConfig_,
    pub function_metadata: Option<super::cloudfront::function::FunctionMetadata_>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_Function {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::Function"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_Function as Function;
impl crate::template::ToResource for Function_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Function"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_publish {
            properties.insert(
                "AutoPublish".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FunctionCode".to_string(),
            crate::value::ToValue::to_value(&self.function_code),
        );
        properties.insert(
            "FunctionConfig".to_string(),
            crate::value::ToValue::to_value(&self.function_config),
        );
        if let Some(ref value) = self.function_metadata {
            properties.insert(
                "FunctionMetadata".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-keygroup.html
pub struct KeyGroup_ {
    pub key_group_config: super::cloudfront::keygroup::KeyGroupConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_KeyGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::KeyGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_KeyGroup as KeyGroup;
impl crate::template::ToResource for KeyGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("KeyGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "KeyGroupConfig".to_string(),
            crate::value::ToValue::to_value(&self.key_group_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-keyvaluestore.html
pub struct KeyValueStore_ {
    pub comment: Option<crate::value::ExpString>,
    pub import_source: Option<super::cloudfront::keyvaluestore::ImportSource_>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_KeyValueStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::KeyValueStore"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_KeyValueStore as KeyValueStore;
impl crate::template::ToResource for KeyValueStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("KeyValueStore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.comment {
            properties.insert(
                "Comment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.import_source {
            properties.insert(
                "ImportSource".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-monitoringsubscription.html
pub struct MonitoringSubscription_ {
    pub distribution_id: crate::value::ExpString,
    pub monitoring_subscription: super::cloudfront::monitoringsubscription::MonitoringSubscription_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_MonitoringSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::MonitoringSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_MonitoringSubscription as MonitoringSubscription;
impl crate::template::ToResource for MonitoringSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MonitoringSubscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DistributionId".to_string(),
            crate::value::ToValue::to_value(&self.distribution_id),
        );
        properties.insert(
            "MonitoringSubscription".to_string(),
            crate::value::ToValue::to_value(&self.monitoring_subscription),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-originaccesscontrol.html
pub struct OriginAccessControl_ {
    pub origin_access_control_config:
        super::cloudfront::originaccesscontrol::OriginAccessControlConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_OriginAccessControl {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::OriginAccessControl"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_OriginAccessControl as OriginAccessControl;
impl crate::template::ToResource for OriginAccessControl_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OriginAccessControl"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "OriginAccessControlConfig".to_string(),
            crate::value::ToValue::to_value(&self.origin_access_control_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-originrequestpolicy.html
pub struct OriginRequestPolicy_ {
    pub origin_request_policy_config:
        super::cloudfront::originrequestpolicy::OriginRequestPolicyConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_OriginRequestPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::OriginRequestPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_OriginRequestPolicy as OriginRequestPolicy;
impl crate::template::ToResource for OriginRequestPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OriginRequestPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "OriginRequestPolicyConfig".to_string(),
            crate::value::ToValue::to_value(&self.origin_request_policy_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-publickey.html
pub struct PublicKey_ {
    pub public_key_config: super::cloudfront::publickey::PublicKeyConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_PublicKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::PublicKey"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_PublicKey as PublicKey;
impl crate::template::ToResource for PublicKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PublicKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PublicKeyConfig".to_string(),
            crate::value::ToValue::to_value(&self.public_key_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-realtimelogconfig.html
pub struct RealtimeLogConfig_ {
    pub end_points: Vec<super::cloudfront::realtimelogconfig::EndPoint_>,
    pub fields: Vec<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub sampling_rate: f64,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_RealtimeLogConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::RealtimeLogConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_RealtimeLogConfig as RealtimeLogConfig;
impl crate::template::ToResource for RealtimeLogConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RealtimeLogConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "EndPoints".to_string(),
            crate::value::ToValue::to_value(&self.end_points),
        );
        properties.insert(
            "Fields".to_string(),
            crate::value::ToValue::to_value(&self.fields),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "SamplingRate".to_string(),
            crate::value::ToValue::to_value(&self.sampling_rate),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-responseheaderspolicy.html
pub struct ResponseHeadersPolicy_ {
    pub response_headers_policy_config:
        super::cloudfront::responseheaderspolicy::ResponseHeadersPolicyConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_ResponseHeadersPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::ResponseHeadersPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_ResponseHeadersPolicy as ResponseHeadersPolicy;
impl crate::template::ToResource for ResponseHeadersPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResponseHeadersPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResponseHeadersPolicyConfig".to_string(),
            crate::value::ToValue::to_value(&self.response_headers_policy_config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-streamingdistribution.html
pub struct StreamingDistribution_ {
    pub streaming_distribution_config:
        super::cloudfront::streamingdistribution::StreamingDistributionConfig_,
    pub tags: Vec<crate::Tag_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_StreamingDistribution {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::StreamingDistribution"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_StreamingDistribution as StreamingDistribution;
impl crate::template::ToResource for StreamingDistribution_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StreamingDistribution"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "StreamingDistributionConfig".to_string(),
            crate::value::ToValue::to_value(&self.streaming_distribution_config),
        );
        properties.insert(
            "Tags".to_string(),
            crate::value::ToValue::to_value(&self.tags),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudfront-vpcorigin.html
pub struct VpcOrigin_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_origin_endpoint_config: super::cloudfront::vpcorigin::VpcOriginEndpointConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudfront_VpcOrigin {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::CloudFront::VpcOrigin"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudfront_VpcOrigin as VpcOrigin;
impl crate::template::ToResource for VpcOrigin_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudFront"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcOrigin"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcOriginEndpointConfig".to_string(),
            crate::value::ToValue::to_value(&self.vpc_origin_endpoint_config),
        );
        properties
    }
}
