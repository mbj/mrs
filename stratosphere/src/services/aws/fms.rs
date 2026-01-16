pub mod policy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-iemap.html
    pub struct IEMap_ {
        pub account: Option<Vec<crate::value::ExpString>>,
        pub orgunit: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_IEMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.IEMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_IEMap as IEMap;
    impl crate::value::ToValue for IEMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account {
                properties.insert(
                    "ACCOUNT".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.orgunit {
                properties.insert(
                    "ORGUNIT".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-icmptypecode.html
    pub struct IcmpTypeCode_ {
        pub code: i64,
        pub r#type: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_IcmpTypeCode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.IcmpTypeCode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_IcmpTypeCode as IcmpTypeCode;
    impl crate::value::ToValue for IcmpTypeCode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Code".to_string(),
                crate::value::ToValue::to_value(&self.code),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-networkaclcommonpolicy.html
    pub struct NetworkAclCommonPolicy_ {
        pub network_acl_entry_set: Box<NetworkAclEntrySet_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_NetworkAclCommonPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.NetworkAclCommonPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_NetworkAclCommonPolicy as NetworkAclCommonPolicy;
    impl crate::value::ToValue for NetworkAclCommonPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NetworkAclEntrySet".to_string(),
                crate::value::ToValue::to_value(&self.network_acl_entry_set),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-networkaclentry.html
    pub struct NetworkAclEntry_ {
        pub cidr_block: Option<crate::value::ExpString>,
        pub egress: crate::value::ExpBool,
        pub icmp_type_code: Option<Box<IcmpTypeCode_>>,
        pub ipv6_cidr_block: Option<crate::value::ExpString>,
        pub port_range: Option<Box<PortRange_>>,
        pub protocol: crate::value::ExpString,
        pub rule_action: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_NetworkAclEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.NetworkAclEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_NetworkAclEntry as NetworkAclEntry;
    impl crate::value::ToValue for NetworkAclEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr_block {
                properties.insert(
                    "CidrBlock".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Egress".to_string(),
                crate::value::ToValue::to_value(&self.egress),
            );
            if let Some(ref value) = self.icmp_type_code {
                properties.insert(
                    "IcmpTypeCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_cidr_block {
                properties.insert(
                    "Ipv6CidrBlock".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port_range {
                properties.insert(
                    "PortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "RuleAction".to_string(),
                crate::value::ToValue::to_value(&self.rule_action),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-networkaclentryset.html
    pub struct NetworkAclEntrySet_ {
        pub first_entries: Option<Vec<NetworkAclEntry_>>,
        pub force_remediate_for_first_entries: crate::value::ExpBool,
        pub force_remediate_for_last_entries: crate::value::ExpBool,
        pub last_entries: Option<Vec<NetworkAclEntry_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_NetworkAclEntrySet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.NetworkAclEntrySet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_NetworkAclEntrySet as NetworkAclEntrySet;
    impl crate::value::ToValue for NetworkAclEntrySet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.first_entries {
                properties.insert(
                    "FirstEntries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ForceRemediateForFirstEntries".to_string(),
                crate::value::ToValue::to_value(&self.force_remediate_for_first_entries),
            );
            properties.insert(
                "ForceRemediateForLastEntries".to_string(),
                crate::value::ToValue::to_value(&self.force_remediate_for_last_entries),
            );
            if let Some(ref value) = self.last_entries {
                properties.insert(
                    "LastEntries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-networkfirewallpolicy.html
    pub struct NetworkFirewallPolicy_ {
        pub firewall_deployment_model: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_NetworkFirewallPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.NetworkFirewallPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_NetworkFirewallPolicy as NetworkFirewallPolicy;
    impl crate::value::ToValue for NetworkFirewallPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FirewallDeploymentModel".to_string(),
                crate::value::ToValue::to_value(&self.firewall_deployment_model),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-policyoption.html
    pub struct PolicyOption_ {
        pub network_acl_common_policy: Option<Box<NetworkAclCommonPolicy_>>,
        pub network_firewall_policy: Option<Box<NetworkFirewallPolicy_>>,
        pub third_party_firewall_policy: Option<Box<ThirdPartyFirewallPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_PolicyOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.PolicyOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_PolicyOption as PolicyOption;
    impl crate::value::ToValue for PolicyOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_acl_common_policy {
                properties.insert(
                    "NetworkAclCommonPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_firewall_policy {
                properties.insert(
                    "NetworkFirewallPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.third_party_firewall_policy {
                properties.insert(
                    "ThirdPartyFirewallPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-policytag.html
    pub struct PolicyTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_PolicyTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.PolicyTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_PolicyTag as PolicyTag;
    impl crate::value::ToValue for PolicyTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-portrange.html
    pub struct PortRange_ {
        pub from: i64,
        pub to: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_PortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.PortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_PortRange as PortRange;
    impl crate::value::ToValue for PortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "From".to_string(),
                crate::value::ToValue::to_value(&self.from),
            );
            properties.insert("To".to_string(), crate::value::ToValue::to_value(&self.to));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-securityservicepolicydata.html
    pub struct SecurityServicePolicyData_ {
        pub managed_service_data: Option<crate::value::ExpString>,
        pub policy_option: Option<Box<PolicyOption_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_SecurityServicePolicyData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.SecurityServicePolicyData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_SecurityServicePolicyData as SecurityServicePolicyData;
    impl crate::value::ToValue for SecurityServicePolicyData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.managed_service_data {
                properties.insert(
                    "ManagedServiceData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_option {
                properties.insert(
                    "PolicyOption".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fms-policy-thirdpartyfirewallpolicy.html
    pub struct ThirdPartyFirewallPolicy_ {
        pub firewall_deployment_model: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fms_Policy_ThirdPartyFirewallPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::FMS::Policy.ThirdPartyFirewallPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fms_Policy_ThirdPartyFirewallPolicy as ThirdPartyFirewallPolicy;
    impl crate::value::ToValue for ThirdPartyFirewallPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FirewallDeploymentModel".to_string(),
                crate::value::ToValue::to_value(&self.firewall_deployment_model),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-notificationchannel.html
pub struct NotificationChannel_ {
    pub sns_role_name: crate::value::ExpString,
    pub sns_topic_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fms_NotificationChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::FMS::NotificationChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_fms_NotificationChannel as NotificationChannel;
impl crate::template::ToResource for NotificationChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NotificationChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SnsRoleName".to_string(),
            crate::value::ToValue::to_value(&self.sns_role_name),
        );
        properties.insert(
            "SnsTopicArn".to_string(),
            crate::value::ToValue::to_value(&self.sns_topic_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-policy.html
pub struct Policy_ {
    pub delete_all_policy_resources: Option<crate::value::ExpBool>,
    pub exclude_map: Option<super::fms::policy::IEMap_>,
    pub exclude_resource_tags: crate::value::ExpBool,
    pub include_map: Option<super::fms::policy::IEMap_>,
    pub policy_description: Option<crate::value::ExpString>,
    pub policy_name: crate::value::ExpString,
    pub remediation_enabled: crate::value::ExpBool,
    pub resource_set_ids: Option<Vec<crate::value::ExpString>>,
    pub resource_tag_logical_operator: Option<crate::value::ExpString>,
    pub resource_tags: Option<Vec<super::fms::policy::ResourceTag_>>,
    pub resource_type: Option<crate::value::ExpString>,
    pub resource_type_list: Option<Vec<crate::value::ExpString>>,
    pub resources_clean_up: Option<crate::value::ExpBool>,
    pub security_service_policy_data: super::fms::policy::SecurityServicePolicyData_,
    pub tags: Option<Vec<super::fms::policy::PolicyTag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fms_Policy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::FMS::Policy" $($field
        $value)*)
    };
}
pub use crate::__aws_fms_Policy as Policy;
impl crate::template::ToResource for Policy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Policy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.delete_all_policy_resources {
            properties.insert(
                "DeleteAllPolicyResources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.exclude_map {
            properties.insert(
                "ExcludeMap".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ExcludeResourceTags".to_string(),
            crate::value::ToValue::to_value(&self.exclude_resource_tags),
        );
        if let Some(ref value) = self.include_map {
            properties.insert(
                "IncludeMap".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_description {
            properties.insert(
                "PolicyDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyName".to_string(),
            crate::value::ToValue::to_value(&self.policy_name),
        );
        properties.insert(
            "RemediationEnabled".to_string(),
            crate::value::ToValue::to_value(&self.remediation_enabled),
        );
        if let Some(ref value) = self.resource_set_ids {
            properties.insert(
                "ResourceSetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_tag_logical_operator {
            properties.insert(
                "ResourceTagLogicalOperator".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_tags {
            properties.insert(
                "ResourceTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_type {
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_type_list {
            properties.insert(
                "ResourceTypeList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resources_clean_up {
            properties.insert(
                "ResourcesCleanUp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecurityServicePolicyData".to_string(),
            crate::value::ToValue::to_value(&self.security_service_policy_data),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fms-resourceset.html
pub struct ResourceSet_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub resource_type_list: Vec<crate::value::ExpString>,
    pub resources: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fms_ResourceSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::FMS::ResourceSet" $($field
        $value)*)
    };
}
pub use crate::__aws_fms_ResourceSet as ResourceSet;
impl crate::template::ToResource for ResourceSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FMS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceSet"),
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
        properties.insert(
            "ResourceTypeList".to_string(),
            crate::value::ToValue::to_value(&self.resource_type_list),
        );
        if let Some(ref value) = self.resources {
            properties.insert(
                "Resources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
