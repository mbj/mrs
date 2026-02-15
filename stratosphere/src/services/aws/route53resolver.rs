pub mod firewallrulegroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-firewallrulegroup-firewallrule.html
    pub struct FirewallRule_ {
        pub action: crate::value::ExpString,
        pub block_override_dns_type: Option<crate::value::ExpString>,
        pub block_override_domain: Option<crate::value::ExpString>,
        pub block_override_ttl: Option<i32>,
        pub block_response: Option<crate::value::ExpString>,
        pub confidence_threshold: Option<crate::value::ExpString>,
        pub dns_threat_protection: Option<crate::value::ExpString>,
        pub firewall_domain_list_id: Option<crate::value::ExpString>,
        pub firewall_domain_redirection_action: Option<crate::value::ExpString>,
        pub firewall_threat_protection_id: Option<crate::value::ExpString>,
        pub priority: i32,
        pub qtype: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53resolver_FirewallRuleGroup_FirewallRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53Resolver::FirewallRuleGroup.FirewallRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53resolver_FirewallRuleGroup_FirewallRule as FirewallRule;
    impl crate::value::ToValue for FirewallRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.block_override_dns_type {
                properties.insert(
                    "BlockOverrideDnsType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.block_override_domain {
                properties.insert(
                    "BlockOverrideDomain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.block_override_ttl {
                properties.insert(
                    "BlockOverrideTtl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.block_response {
                properties.insert(
                    "BlockResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.confidence_threshold {
                properties.insert(
                    "ConfidenceThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_threat_protection {
                properties.insert(
                    "DnsThreatProtection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firewall_domain_list_id {
                properties.insert(
                    "FirewallDomainListId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firewall_domain_redirection_action {
                properties.insert(
                    "FirewallDomainRedirectionAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firewall_threat_protection_id {
                properties.insert(
                    "FirewallThreatProtectionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            if let Some(ref value) = self.qtype {
                properties.insert("Qtype".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod resolverendpoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverendpoint-ipaddressrequest.html
    pub struct IpAddressRequest_ {
        pub ip: Option<crate::value::ExpString>,
        pub ipv6: Option<crate::value::ExpString>,
        pub subnet_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53resolver_ResolverEndpoint_IpAddressRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53Resolver::ResolverEndpoint.IpAddressRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53resolver_ResolverEndpoint_IpAddressRequest as IpAddressRequest;
    impl crate::value::ToValue for IpAddressRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip {
                properties.insert("Ip".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ipv6 {
                properties.insert("Ipv6".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(&self.subnet_id),
            );
            properties.into()
        }
    }
}
pub mod resolverrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53resolver-resolverrule-targetaddress.html
    pub struct TargetAddress_ {
        pub ip: Option<crate::value::ExpString>,
        pub ipv6: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
        pub server_name_indication: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53resolver_ResolverRule_TargetAddress {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53Resolver::ResolverRule.TargetAddress"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53resolver_ResolverRule_TargetAddress as TargetAddress;
    impl crate::value::ToValue for TargetAddress_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ip {
                properties.insert("Ip".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ipv6 {
                properties.insert("Ipv6".to_string(), crate::value::ToValue::to_value(value));
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
            if let Some(ref value) = self.server_name_indication {
                properties.insert(
                    "ServerNameIndication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewalldomainlist.html
pub struct FirewallDomainList_ {
    pub domain_file_url: Option<crate::value::ExpString>,
    pub domains: Option<Vec<crate::value::ExpString>>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_FirewallDomainList {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::FirewallDomainList"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_FirewallDomainList as FirewallDomainList;
impl crate::template::ToResource for FirewallDomainList_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FirewallDomainList"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.domain_file_url {
            properties.insert(
                "DomainFileUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domains {
            properties.insert(
                "Domains".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroup.html
pub struct FirewallRuleGroup_ {
    pub firewall_rules: Option<Vec<super::route53resolver::firewallrulegroup::FirewallRule_>>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_FirewallRuleGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::FirewallRuleGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_FirewallRuleGroup as FirewallRuleGroup;
impl crate::template::ToResource for FirewallRuleGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FirewallRuleGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.firewall_rules {
            properties.insert(
                "FirewallRules".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-firewallrulegroupassociation.html
pub struct FirewallRuleGroupAssociation_ {
    pub firewall_rule_group_id: crate::value::ExpString,
    pub mutation_protection: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub priority: i32,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_FirewallRuleGroupAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::FirewallRuleGroupAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_FirewallRuleGroupAssociation as FirewallRuleGroupAssociation;
impl crate::template::ToResource for FirewallRuleGroupAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "FirewallRuleGroupAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FirewallRuleGroupId".to_string(),
            crate::value::ToValue::to_value(&self.firewall_rule_group_id),
        );
        if let Some(ref value) = self.mutation_protection {
            properties.insert(
                "MutationProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Priority".to_string(),
            crate::value::ToValue::to_value(&self.priority),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-outpostresolver.html
pub struct OutpostResolver_ {
    pub instance_count: Option<i32>,
    pub name: crate::value::ExpString,
    pub outpost_arn: crate::value::ExpString,
    pub preferred_instance_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_OutpostResolver {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::OutpostResolver"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_OutpostResolver as OutpostResolver;
impl crate::template::ToResource for OutpostResolver_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OutpostResolver"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.instance_count {
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "OutpostArn".to_string(),
            crate::value::ToValue::to_value(&self.outpost_arn),
        );
        properties.insert(
            "PreferredInstanceType".to_string(),
            crate::value::ToValue::to_value(&self.preferred_instance_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverconfig.html
pub struct ResolverConfig_ {
    pub autodefined_reverse_flag: crate::value::ExpString,
    pub resource_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_ResolverConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::ResolverConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_ResolverConfig as ResolverConfig;
impl crate::template::ToResource for ResolverConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResolverConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AutodefinedReverseFlag".to_string(),
            crate::value::ToValue::to_value(&self.autodefined_reverse_flag),
        );
        properties.insert(
            "ResourceId".to_string(),
            crate::value::ToValue::to_value(&self.resource_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverdnssecconfig.html
pub struct ResolverDNSSECConfig_ {
    pub resource_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_ResolverDNSSECConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::ResolverDNSSECConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_ResolverDNSSECConfig as ResolverDNSSECConfig;
impl crate::template::ToResource for ResolverDNSSECConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResolverDNSSECConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.resource_id {
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverendpoint.html
pub struct ResolverEndpoint_ {
    pub direction: crate::value::ExpString,
    pub ip_addresses: Vec<super::route53resolver::resolverendpoint::IpAddressRequest_>,
    pub name: Option<crate::value::ExpString>,
    pub outpost_arn: Option<crate::value::ExpString>,
    pub preferred_instance_type: Option<crate::value::ExpString>,
    pub protocols: Option<Vec<crate::value::ExpString>>,
    pub resolver_endpoint_type: Option<crate::value::ExpString>,
    pub security_group_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_ResolverEndpoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::ResolverEndpoint"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_ResolverEndpoint as ResolverEndpoint;
impl crate::template::ToResource for ResolverEndpoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResolverEndpoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Direction".to_string(),
            crate::value::ToValue::to_value(&self.direction),
        );
        properties.insert(
            "IpAddresses".to_string(),
            crate::value::ToValue::to_value(&self.ip_addresses),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.outpost_arn {
            properties.insert(
                "OutpostArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_instance_type {
            properties.insert(
                "PreferredInstanceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protocols {
            properties.insert(
                "Protocols".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resolver_endpoint_type {
            properties.insert(
                "ResolverEndpointType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecurityGroupIds".to_string(),
            crate::value::ToValue::to_value(&self.security_group_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfig.html
pub struct ResolverQueryLoggingConfig_ {
    pub destination_arn: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_ResolverQueryLoggingConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::ResolverQueryLoggingConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_ResolverQueryLoggingConfig as ResolverQueryLoggingConfig;
impl crate::template::ToResource for ResolverQueryLoggingConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ResolverQueryLoggingConfig",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.destination_arn {
            properties.insert(
                "DestinationArn".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverqueryloggingconfigassociation.html
pub struct ResolverQueryLoggingConfigAssociation_ {
    pub resolver_query_log_config_id: Option<crate::value::ExpString>,
    pub resource_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_ResolverQueryLoggingConfigAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::ResolverQueryLoggingConfigAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_ResolverQueryLoggingConfigAssociation as ResolverQueryLoggingConfigAssociation;
impl crate::template::ToResource for ResolverQueryLoggingConfigAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ResolverQueryLoggingConfigAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.resolver_query_log_config_id {
            properties.insert(
                "ResolverQueryLogConfigId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_id {
            properties.insert(
                "ResourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverrule.html
pub struct ResolverRule_ {
    pub delegation_record: Option<crate::value::ExpString>,
    pub domain_name: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub resolver_endpoint_id: Option<crate::value::ExpString>,
    pub rule_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_ips: Option<Vec<super::route53resolver::resolverrule::TargetAddress_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_ResolverRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::ResolverRule"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_ResolverRule as ResolverRule;
impl crate::template::ToResource for ResolverRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResolverRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.delegation_record {
            properties.insert(
                "DelegationRecord".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_name {
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.resolver_endpoint_id {
            properties.insert(
                "ResolverEndpointId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleType".to_string(),
            crate::value::ToValue::to_value(&self.rule_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_ips {
            properties.insert(
                "TargetIps".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53resolver-resolverruleassociation.html
pub struct ResolverRuleAssociation_ {
    pub name: Option<crate::value::ExpString>,
    pub resolver_rule_id: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53resolver_ResolverRuleAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53Resolver::ResolverRuleAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_route53resolver_ResolverRuleAssociation as ResolverRuleAssociation;
impl crate::template::ToResource for ResolverRuleAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53Resolver"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResolverRuleAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ResolverRuleId".to_string(),
            crate::value::ToValue::to_value(&self.resolver_rule_id),
        );
        properties.insert(
            "VPCId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
