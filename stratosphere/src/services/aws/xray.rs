pub mod group {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-group-insightsconfiguration.html
    pub struct InsightsConfiguration_ {
        pub insights_enabled: Option<crate::value::ExpBool>,
        pub notifications_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_xray_Group_InsightsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::XRay::Group.InsightsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_xray_Group_InsightsConfiguration as InsightsConfiguration;
    impl crate::value::ToValue for InsightsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.insights_enabled {
                properties.insert(
                    "InsightsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notifications_enabled {
                properties.insert(
                    "NotificationsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod samplingrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-xray-samplingrule-samplingrule.html
    pub struct SamplingRule_ {
        pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub fixed_rate: f64,
        pub http_method: crate::value::ExpString,
        pub host: crate::value::ExpString,
        pub priority: i64,
        pub reservoir_size: i64,
        pub resource_arn: crate::value::ExpString,
        pub rule_arn: Option<crate::value::ExpString>,
        pub rule_name: Option<crate::value::ExpString>,
        pub service_name: crate::value::ExpString,
        pub service_type: crate::value::ExpString,
        pub url_path: crate::value::ExpString,
        pub version: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_xray_SamplingRule_SamplingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::XRay::SamplingRule.SamplingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_xray_SamplingRule_SamplingRule as SamplingRule;
    impl crate::value::ToValue for SamplingRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FixedRate".to_string(),
                crate::value::ToValue::to_value(&self.fixed_rate),
            );
            properties.insert(
                "HTTPMethod".to_string(),
                crate::value::ToValue::to_value(&self.http_method),
            );
            properties.insert(
                "Host".to_string(),
                crate::value::ToValue::to_value(&self.host),
            );
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "ReservoirSize".to_string(),
                crate::value::ToValue::to_value(&self.reservoir_size),
            );
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            if let Some(ref value) = self.rule_arn {
                properties.insert(
                    "RuleARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_name {
                properties.insert(
                    "RuleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(&self.service_name),
            );
            properties.insert(
                "ServiceType".to_string(),
                crate::value::ToValue::to_value(&self.service_type),
            );
            properties.insert(
                "URLPath".to_string(),
                crate::value::ToValue::to_value(&self.url_path),
            );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-group.html
pub struct Group_ {
    pub filter_expression: Option<crate::value::ExpString>,
    pub group_name: crate::value::ExpString,
    pub insights_configuration: Option<super::xray::group::InsightsConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_xray_Group {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::XRay::Group" $($field
        $value)*)
    };
}
pub use crate::__aws_xray_Group as Group;
impl crate::template::ToResource for Group_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("XRay"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Group"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.filter_expression {
            properties.insert(
                "FilterExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GroupName".to_string(),
            crate::value::ToValue::to_value(&self.group_name),
        );
        if let Some(ref value) = self.insights_configuration {
            properties.insert(
                "InsightsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub bypass_policy_lockout_check: Option<crate::value::ExpBool>,
    pub policy_document: crate::value::ExpString,
    pub policy_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_xray_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::XRay::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_xray_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("XRay"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bypass_policy_lockout_check {
            properties.insert(
                "BypassPolicyLockoutCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-samplingrule.html
pub struct SamplingRule_ {
    pub sampling_rule: Option<super::xray::samplingrule::SamplingRule_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_xray_SamplingRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::XRay::SamplingRule"
        $($field $value)*)
    };
}
pub use crate::__aws_xray_SamplingRule as SamplingRule;
impl crate::template::ToResource for SamplingRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("XRay"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SamplingRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.sampling_rule {
            properties.insert(
                "SamplingRule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-xray-transactionsearchconfig.html
pub struct TransactionSearchConfig_ {
    pub indexing_percentage: Option<f64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_xray_TransactionSearchConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::XRay::TransactionSearchConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_xray_TransactionSearchConfig as TransactionSearchConfig;
impl crate::template::ToResource for TransactionSearchConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("XRay"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TransactionSearchConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.indexing_percentage {
            properties.insert(
                "IndexingPercentage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
