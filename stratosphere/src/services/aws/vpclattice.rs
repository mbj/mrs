pub mod domainverification {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-domainverification-txtmethodconfig.html
    pub struct TxtMethodConfig_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_DomainVerification_TxtMethodConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::DomainVerification.TxtMethodConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_DomainVerification_TxtMethodConfig as TxtMethodConfig;
    impl crate::value::ToValue for TxtMethodConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod listener {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-listener-defaultaction.html
    pub struct DefaultAction_ {
        pub fixed_response: Option<Box<FixedResponse_>>,
        pub forward: Option<Box<Forward_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Listener_DefaultAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Listener.DefaultAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Listener_DefaultAction as DefaultAction;
    impl crate::value::ToValue for DefaultAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fixed_response {
                properties.insert(
                    "FixedResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forward {
                properties.insert(
                    "Forward".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-listener-fixedresponse.html
    pub struct FixedResponse_ {
        pub status_code: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Listener_FixedResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Listener.FixedResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Listener_FixedResponse as FixedResponse;
    impl crate::value::ToValue for FixedResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-listener-forward.html
    pub struct Forward_ {
        pub target_groups: Vec<WeightedTargetGroup_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Listener_Forward {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Listener.Forward"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Listener_Forward as Forward;
    impl crate::value::ToValue for Forward_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetGroups".to_string(),
                crate::value::ToValue::to_value(&self.target_groups),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-listener-weightedtargetgroup.html
    pub struct WeightedTargetGroup_ {
        pub target_group_identifier: crate::value::ExpString,
        pub weight: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Listener_WeightedTargetGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Listener.WeightedTargetGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Listener_WeightedTargetGroup as WeightedTargetGroup;
    impl crate::value::ToValue for WeightedTargetGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetGroupIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.target_group_identifier),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod resourceconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-resourceconfiguration-dnsresource.html
    pub struct DnsResource_ {
        pub domain_name: crate::value::ExpString,
        pub ip_address_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_ResourceConfiguration_DnsResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::ResourceConfiguration.DnsResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_ResourceConfiguration_DnsResource as DnsResource;
    impl crate::value::ToValue for DnsResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(&self.domain_name),
            );
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(&self.ip_address_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-resourceconfiguration-resourceconfigurationdefinition.html
    pub struct ResourceConfigurationDefinition_ {
        pub arn_resource: Option<crate::value::ExpString>,
        pub dns_resource: Option<Box<DnsResource_>>,
        pub ip_resource: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_ResourceConfiguration_ResourceConfigurationDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::ResourceConfiguration.ResourceConfigurationDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_ResourceConfiguration_ResourceConfigurationDefinition as ResourceConfigurationDefinition;
    impl crate::value::ToValue for ResourceConfigurationDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn_resource {
                properties.insert(
                    "ArnResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_resource {
                properties.insert(
                    "DnsResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_resource {
                properties.insert(
                    "IpResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod rule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-action.html
    pub struct Action_ {
        pub fixed_response: Option<Box<FixedResponse_>>,
        pub forward: Option<Box<Forward_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fixed_response {
                properties.insert(
                    "FixedResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forward {
                properties.insert(
                    "Forward".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-fixedresponse.html
    pub struct FixedResponse_ {
        pub status_code: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_FixedResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.FixedResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_FixedResponse as FixedResponse;
    impl crate::value::ToValue for FixedResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StatusCode".to_string(),
                crate::value::ToValue::to_value(&self.status_code),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-forward.html
    pub struct Forward_ {
        pub target_groups: Vec<WeightedTargetGroup_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_Forward {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.Forward"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_Forward as Forward;
    impl crate::value::ToValue for Forward_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetGroups".to_string(),
                crate::value::ToValue::to_value(&self.target_groups),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-headermatch.html
    pub struct HeaderMatch_ {
        pub case_sensitive: Option<crate::value::ExpBool>,
        pub r#match: Box<HeaderMatchType_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_HeaderMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.HeaderMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_HeaderMatch as HeaderMatch;
    impl crate::value::ToValue for HeaderMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.case_sensitive {
                properties.insert(
                    "CaseSensitive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-headermatchtype.html
    pub struct HeaderMatchType_ {
        pub contains: Option<crate::value::ExpString>,
        pub exact: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_HeaderMatchType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.HeaderMatchType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_HeaderMatchType as HeaderMatchType;
    impl crate::value::ToValue for HeaderMatchType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.contains {
                properties.insert(
                    "Contains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-httpmatch.html
    pub struct HttpMatch_ {
        pub header_matches: Option<Vec<HeaderMatch_>>,
        pub method: Option<crate::value::ExpString>,
        pub path_match: Option<Box<PathMatch_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_HttpMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.HttpMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_HttpMatch as HttpMatch;
    impl crate::value::ToValue for HttpMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header_matches {
                properties.insert(
                    "HeaderMatches".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.path_match {
                properties.insert(
                    "PathMatch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-match.html
    pub struct Match_ {
        pub http_match: Box<HttpMatch_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_Match {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.Match"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_Match as Match;
    impl crate::value::ToValue for Match_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HttpMatch".to_string(),
                crate::value::ToValue::to_value(&self.http_match),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-pathmatch.html
    pub struct PathMatch_ {
        pub case_sensitive: Option<crate::value::ExpBool>,
        pub r#match: Box<PathMatchType_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_PathMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.PathMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_PathMatch as PathMatch;
    impl crate::value::ToValue for PathMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.case_sensitive {
                properties.insert(
                    "CaseSensitive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Match".to_string(),
                crate::value::ToValue::to_value(&self.r#match),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-pathmatchtype.html
    pub struct PathMatchType_ {
        pub exact: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_PathMatchType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.PathMatchType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_PathMatchType as PathMatchType;
    impl crate::value::ToValue for PathMatchType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exact {
                properties.insert("Exact".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-rule-weightedtargetgroup.html
    pub struct WeightedTargetGroup_ {
        pub target_group_identifier: crate::value::ExpString,
        pub weight: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Rule_WeightedTargetGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Rule.WeightedTargetGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Rule_WeightedTargetGroup as WeightedTargetGroup;
    impl crate::value::ToValue for WeightedTargetGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetGroupIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.target_group_identifier),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod service {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-service-dnsentry.html
    pub struct DnsEntry_ {
        pub domain_name: Option<crate::value::ExpString>,
        pub hosted_zone_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_Service_DnsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::Service.DnsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_Service_DnsEntry as DnsEntry;
    impl crate::value::ToValue for DnsEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_name {
                properties.insert(
                    "DomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hosted_zone_id {
                properties.insert(
                    "HostedZoneId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod servicenetwork {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-servicenetwork-sharingconfig.html
    pub struct SharingConfig_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_ServiceNetwork_SharingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::ServiceNetwork.SharingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_ServiceNetwork_SharingConfig as SharingConfig;
    impl crate::value::ToValue for SharingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
}
pub mod servicenetworkserviceassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-servicenetworkserviceassociation-dnsentry.html
    pub struct DnsEntry_ {
        pub domain_name: Option<crate::value::ExpString>,
        pub hosted_zone_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_ServiceNetworkServiceAssociation_DnsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::ServiceNetworkServiceAssociation.DnsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_ServiceNetworkServiceAssociation_DnsEntry as DnsEntry;
    impl crate::value::ToValue for DnsEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_name {
                properties.insert(
                    "DomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hosted_zone_id {
                properties.insert(
                    "HostedZoneId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod servicenetworkvpcassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-servicenetworkvpcassociation-dnsoptions.html
    pub struct DnsOptions_ {
        pub private_dns_preference: Option<crate::value::ExpString>,
        pub private_dns_specified_domains: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_ServiceNetworkVpcAssociation_DnsOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::ServiceNetworkVpcAssociation.DnsOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_ServiceNetworkVpcAssociation_DnsOptions as DnsOptions;
    impl crate::value::ToValue for DnsOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.private_dns_preference {
                properties.insert(
                    "PrivateDnsPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_dns_specified_domains {
                properties.insert(
                    "PrivateDnsSpecifiedDomains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod targetgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-targetgroup-healthcheckconfig.html
    pub struct HealthCheckConfig_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub health_check_interval_seconds: Option<i32>,
        pub health_check_timeout_seconds: Option<i32>,
        pub healthy_threshold_count: Option<i32>,
        pub matcher: Option<Box<Matcher_>>,
        pub path: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub protocol: Option<crate::value::ExpString>,
        pub protocol_version: Option<crate::value::ExpString>,
        pub unhealthy_threshold_count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_TargetGroup_HealthCheckConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::TargetGroup.HealthCheckConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_TargetGroup_HealthCheckConfig as HealthCheckConfig;
    impl crate::value::ToValue for HealthCheckConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check_interval_seconds {
                properties.insert(
                    "HealthCheckIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check_timeout_seconds {
                properties.insert(
                    "HealthCheckTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.healthy_threshold_count {
                properties.insert(
                    "HealthyThresholdCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.matcher {
                properties.insert(
                    "Matcher".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol_version {
                properties.insert(
                    "ProtocolVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unhealthy_threshold_count {
                properties.insert(
                    "UnhealthyThresholdCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-targetgroup-matcher.html
    pub struct Matcher_ {
        pub http_code: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_TargetGroup_Matcher {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::TargetGroup.Matcher"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_TargetGroup_Matcher as Matcher;
    impl crate::value::ToValue for Matcher_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HttpCode".to_string(),
                crate::value::ToValue::to_value(&self.http_code),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-targetgroup-target.html
    pub struct Target_ {
        pub id: crate::value::ExpString,
        pub port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_TargetGroup_Target {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::TargetGroup.Target"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_TargetGroup_Target as Target;
    impl crate::value::ToValue for Target_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-vpclattice-targetgroup-targetgroupconfig.html
    pub struct TargetGroupConfig_ {
        pub health_check: Option<Box<HealthCheckConfig_>>,
        pub ip_address_type: Option<crate::value::ExpString>,
        pub lambda_event_structure_version: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub protocol: Option<crate::value::ExpString>,
        pub protocol_version: Option<crate::value::ExpString>,
        pub vpc_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_vpclattice_TargetGroup_TargetGroupConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::VpcLattice::TargetGroup.TargetGroupConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_vpclattice_TargetGroup_TargetGroupConfig as TargetGroupConfig;
    impl crate::value::ToValue for TargetGroupConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.health_check {
                properties.insert(
                    "HealthCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address_type {
                properties.insert(
                    "IpAddressType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_event_structure_version {
                properties.insert(
                    "LambdaEventStructureVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol_version {
                properties.insert(
                    "ProtocolVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_identifier {
                properties.insert(
                    "VpcIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-accesslogsubscription.html
pub struct AccessLogSubscription_ {
    pub destination_arn: crate::value::ExpString,
    pub resource_identifier: Option<crate::value::ExpString>,
    pub service_network_log_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_AccessLogSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::AccessLogSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_AccessLogSubscription as AccessLogSubscription;
impl crate::template::ToResource for AccessLogSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessLogSubscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationArn".to_string(),
            crate::value::ToValue::to_value(&self.destination_arn),
        );
        if let Some(ref value) = self.resource_identifier {
            properties.insert(
                "ResourceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_network_log_type {
            properties.insert(
                "ServiceNetworkLogType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-authpolicy.html
pub struct AuthPolicy_ {
    pub policy: serde_json::Value,
    pub resource_identifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_AuthPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::AuthPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_AuthPolicy as AuthPolicy;
impl crate::template::ToResource for AuthPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AuthPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "ResourceIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.resource_identifier),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-domainverification.html
pub struct DomainVerification_ {
    pub domain_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_DomainVerification {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::DomainVerification"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_DomainVerification as DomainVerification;
impl crate::template::ToResource for DomainVerification_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DomainVerification"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-listener.html
pub struct Listener_ {
    pub default_action: super::vpclattice::listener::DefaultAction_,
    pub name: Option<crate::value::ExpString>,
    pub port: Option<i32>,
    pub protocol: crate::value::ExpString,
    pub service_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_Listener {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::Listener"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_Listener as Listener;
impl crate::template::ToResource for Listener_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Listener"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DefaultAction".to_string(),
            crate::value::ToValue::to_value(&self.default_action),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Protocol".to_string(),
            crate::value::ToValue::to_value(&self.protocol),
        );
        if let Some(ref value) = self.service_identifier {
            properties.insert(
                "ServiceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-resourceconfiguration.html
pub struct ResourceConfiguration_ {
    pub allow_association_to_sharable_service_network: Option<crate::value::ExpBool>,
    pub custom_domain_name: Option<crate::value::ExpString>,
    pub domain_verification_id: Option<crate::value::ExpString>,
    pub group_domain: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub port_ranges: Option<Vec<crate::value::ExpString>>,
    pub protocol_type: Option<crate::value::ExpString>,
    pub resource_configuration_auth_type: Option<crate::value::ExpString>,
    pub resource_configuration_definition:
        Option<super::vpclattice::resourceconfiguration::ResourceConfigurationDefinition_>,
    pub resource_configuration_group_id: Option<crate::value::ExpString>,
    pub resource_configuration_type: crate::value::ExpString,
    pub resource_gateway_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_ResourceConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::ResourceConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_ResourceConfiguration as ResourceConfiguration;
impl crate::template::ToResource for ResourceConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allow_association_to_sharable_service_network {
            properties.insert(
                "AllowAssociationToSharableServiceNetwork".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_domain_name {
            properties.insert(
                "CustomDomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_verification_id {
            properties.insert(
                "DomainVerificationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.group_domain {
            properties.insert(
                "GroupDomain".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.port_ranges {
            properties.insert(
                "PortRanges".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protocol_type {
            properties.insert(
                "ProtocolType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_configuration_auth_type {
            properties.insert(
                "ResourceConfigurationAuthType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_configuration_definition {
            properties.insert(
                "ResourceConfigurationDefinition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_configuration_group_id {
            properties.insert(
                "ResourceConfigurationGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceConfigurationType".to_string(),
            crate::value::ToValue::to_value(&self.resource_configuration_type),
        );
        if let Some(ref value) = self.resource_gateway_id {
            properties.insert(
                "ResourceGatewayId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-resourcegateway.html
pub struct ResourceGateway_ {
    pub ip_address_type: Option<crate::value::ExpString>,
    pub ipv4_addresses_per_eni: Option<i32>,
    pub name: crate::value::ExpString,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_identifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_ResourceGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::ResourceGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_ResourceGateway as ResourceGateway;
impl crate::template::ToResource for ResourceGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv4_addresses_per_eni {
            properties.insert(
                "Ipv4AddressesPerEni".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        properties.insert(
            "VpcIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.vpc_identifier),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub policy: serde_json::Value,
    pub resource_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-rule.html
pub struct Rule_ {
    pub action: super::vpclattice::rule::Action_,
    pub listener_identifier: Option<crate::value::ExpString>,
    pub r#match: super::vpclattice::rule::Match_,
    pub name: Option<crate::value::ExpString>,
    pub priority: i32,
    pub service_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_Rule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::Rule"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_Rule as Rule;
impl crate::template::ToResource for Rule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Rule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Action".to_string(),
            crate::value::ToValue::to_value(&self.action),
        );
        if let Some(ref value) = self.listener_identifier {
            properties.insert(
                "ListenerIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Match".to_string(),
            crate::value::ToValue::to_value(&self.r#match),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Priority".to_string(),
            crate::value::ToValue::to_value(&self.priority),
        );
        if let Some(ref value) = self.service_identifier {
            properties.insert(
                "ServiceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-service.html
pub struct Service_ {
    pub auth_type: Option<crate::value::ExpString>,
    pub certificate_arn: Option<crate::value::ExpString>,
    pub custom_domain_name: Option<crate::value::ExpString>,
    pub dns_entry: Option<super::vpclattice::service::DnsEntry_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_Service {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::Service"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_Service as Service;
impl crate::template::ToResource for Service_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Service"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auth_type {
            properties.insert(
                "AuthType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_arn {
            properties.insert(
                "CertificateArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_domain_name {
            properties.insert(
                "CustomDomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dns_entry {
            properties.insert(
                "DnsEntry".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-servicenetwork.html
pub struct ServiceNetwork_ {
    pub auth_type: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub sharing_config: Option<super::vpclattice::servicenetwork::SharingConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_ServiceNetwork {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::ServiceNetwork"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_ServiceNetwork as ServiceNetwork;
impl crate::template::ToResource for ServiceNetwork_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServiceNetwork"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auth_type {
            properties.insert(
                "AuthType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.sharing_config {
            properties.insert(
                "SharingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-servicenetworkresourceassociation.html
pub struct ServiceNetworkResourceAssociation_ {
    pub private_dns_enabled: Option<crate::value::ExpBool>,
    pub resource_configuration_id: Option<crate::value::ExpString>,
    pub service_network_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_ServiceNetworkResourceAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::ServiceNetworkResourceAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_ServiceNetworkResourceAssociation as ServiceNetworkResourceAssociation;
impl crate::template::ToResource for ServiceNetworkResourceAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ServiceNetworkResourceAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.private_dns_enabled {
            properties.insert(
                "PrivateDnsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_configuration_id {
            properties.insert(
                "ResourceConfigurationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_network_id {
            properties.insert(
                "ServiceNetworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-servicenetworkserviceassociation.html
pub struct ServiceNetworkServiceAssociation_ {
    pub dns_entry: Option<super::vpclattice::servicenetworkserviceassociation::DnsEntry_>,
    pub service_identifier: Option<crate::value::ExpString>,
    pub service_network_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_ServiceNetworkServiceAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::ServiceNetworkServiceAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_ServiceNetworkServiceAssociation as ServiceNetworkServiceAssociation;
impl crate::template::ToResource for ServiceNetworkServiceAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ServiceNetworkServiceAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dns_entry {
            properties.insert(
                "DnsEntry".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_identifier {
            properties.insert(
                "ServiceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_network_identifier {
            properties.insert(
                "ServiceNetworkIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-servicenetworkvpcassociation.html
pub struct ServiceNetworkVpcAssociation_ {
    pub dns_options: Option<super::vpclattice::servicenetworkvpcassociation::DnsOptions_>,
    pub private_dns_enabled: Option<crate::value::ExpBool>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub service_network_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_identifier: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_ServiceNetworkVpcAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::ServiceNetworkVpcAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_ServiceNetworkVpcAssociation as ServiceNetworkVpcAssociation;
impl crate::template::ToResource for ServiceNetworkVpcAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ServiceNetworkVpcAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dns_options {
            properties.insert(
                "DnsOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_dns_enabled {
            properties.insert(
                "PrivateDnsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_network_identifier {
            properties.insert(
                "ServiceNetworkIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_identifier {
            properties.insert(
                "VpcIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-vpclattice-targetgroup.html
pub struct TargetGroup_ {
    pub config: Option<super::vpclattice::targetgroup::TargetGroupConfig_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub targets: Option<Vec<super::vpclattice::targetgroup::Target_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_vpclattice_TargetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::VpcLattice::TargetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_vpclattice_TargetGroup as TargetGroup;
impl crate::template::ToResource for TargetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("VpcLattice"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TargetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.config {
            properties.insert("Config".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
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
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
