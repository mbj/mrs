pub mod resourceset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-dnstargetresource.html>
    pub struct DNSTargetResource_ {
        pub domain_name: Option<crate::value::ExpString>,
        pub hosted_zone_arn: Option<crate::value::ExpString>,
        pub record_set_id: Option<crate::value::ExpString>,
        pub record_type: Option<crate::value::ExpString>,
        pub target_resource: Option<Box<TargetResource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoveryreadiness_ResourceSet_DNSTargetResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryReadiness::ResourceSet.DNSTargetResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoveryreadiness_ResourceSet_DNSTargetResource as DNSTargetResource;
    impl crate::value::ToValue for DNSTargetResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_name {
                properties.insert(
                    "DomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hosted_zone_arn {
                properties.insert(
                    "HostedZoneArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_set_id {
                properties.insert(
                    "RecordSetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_type {
                properties.insert(
                    "RecordType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_resource {
                properties.insert(
                    "TargetResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-nlbresource.html>
    pub struct NLBResource_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoveryreadiness_ResourceSet_NLBResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryReadiness::ResourceSet.NLBResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoveryreadiness_ResourceSet_NLBResource as NLBResource;
    impl crate::value::ToValue for NLBResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-r53resourcerecord.html>
    pub struct R53ResourceRecord_ {
        pub domain_name: Option<crate::value::ExpString>,
        pub record_set_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoveryreadiness_ResourceSet_R53ResourceRecord {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryReadiness::ResourceSet.R53ResourceRecord"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoveryreadiness_ResourceSet_R53ResourceRecord as R53ResourceRecord;
    impl crate::value::ToValue for R53ResourceRecord_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_name {
                properties.insert(
                    "DomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_set_id {
                properties.insert(
                    "RecordSetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-resource.html>
    pub struct Resource_ {
        pub component_id: Option<crate::value::ExpString>,
        pub dns_target_resource: Option<Box<DNSTargetResource_>>,
        pub readiness_scopes: Option<Vec<crate::value::ExpString>>,
        pub resource_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoveryreadiness_ResourceSet_Resource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryReadiness::ResourceSet.Resource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoveryreadiness_ResourceSet_Resource as Resource;
    impl crate::value::ToValue for Resource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_id {
                properties.insert(
                    "ComponentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_target_resource {
                properties.insert(
                    "DnsTargetResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.readiness_scopes {
                properties.insert(
                    "ReadinessScopes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_arn {
                properties.insert(
                    "ResourceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53recoveryreadiness-resourceset-targetresource.html>
    pub struct TargetResource_ {
        pub nlb_resource: Option<Box<NLBResource_>>,
        pub r53_resource: Option<Box<R53ResourceRecord_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53recoveryreadiness_ResourceSet_TargetResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53RecoveryReadiness::ResourceSet.TargetResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53recoveryreadiness_ResourceSet_TargetResource as TargetResource;
    impl crate::value::ToValue for TargetResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.nlb_resource {
                properties.insert(
                    "NLBResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r53_resource {
                properties.insert(
                    "R53Resource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-cell.html>
pub struct Cell_ {
    pub cell_name: Option<crate::value::ExpString>,
    pub cells: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoveryreadiness_Cell {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryReadiness::Cell"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoveryreadiness_Cell as Cell;
impl crate::template::ToResource for Cell_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "Route53RecoveryReadiness",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cell"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cell_name {
            properties.insert(
                "CellName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cells {
            properties.insert("Cells".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-readinesscheck.html>
pub struct ReadinessCheck_ {
    pub readiness_check_name: Option<crate::value::ExpString>,
    pub resource_set_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoveryreadiness_ReadinessCheck {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryReadiness::ReadinessCheck"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoveryreadiness_ReadinessCheck as ReadinessCheck;
impl crate::template::ToResource for ReadinessCheck_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "Route53RecoveryReadiness",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReadinessCheck"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.readiness_check_name {
            properties.insert(
                "ReadinessCheckName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_set_name {
            properties.insert(
                "ResourceSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-recoverygroup.html>
pub struct RecoveryGroup_ {
    pub cells: Option<Vec<crate::value::ExpString>>,
    pub recovery_group_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoveryreadiness_RecoveryGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryReadiness::RecoveryGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoveryreadiness_RecoveryGroup as RecoveryGroup;
impl crate::template::ToResource for RecoveryGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "Route53RecoveryReadiness",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RecoveryGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cells {
            properties.insert("Cells".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.recovery_group_name {
            properties.insert(
                "RecoveryGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53recoveryreadiness-resourceset.html>
pub struct ResourceSet_ {
    pub resource_set_name: Option<crate::value::ExpString>,
    pub resource_set_type: crate::value::ExpString,
    pub resources: Vec<super::route53recoveryreadiness::resourceset::Resource_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53recoveryreadiness_ResourceSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53RecoveryReadiness::ResourceSet"
        $($field $value)*)
    };
}
pub use crate::__aws_route53recoveryreadiness_ResourceSet as ResourceSet;
impl crate::template::ToResource for ResourceSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName(
                    "Route53RecoveryReadiness",
                ),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.resource_set_name {
            properties.insert(
                "ResourceSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceSetType".to_string(),
            crate::value::ToValue::to_value(&self.resource_set_type),
        );
        properties.insert(
            "Resources".to_string(),
            crate::value::ToValue::to_value(&self.resources),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
