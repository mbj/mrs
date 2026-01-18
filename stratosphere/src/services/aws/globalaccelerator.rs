pub mod crossaccountattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-crossaccountattachment-resource.html
    pub struct Resource_ {
        pub cidr: Option<crate::value::ExpString>,
        pub endpoint_id: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_globalaccelerator_CrossAccountAttachment_Resource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GlobalAccelerator::CrossAccountAttachment.Resource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_globalaccelerator_CrossAccountAttachment_Resource as Resource;
    impl crate::value::ToValue for Resource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.endpoint_id {
                properties.insert(
                    "EndpointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod endpointgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-endpointconfiguration.html
    pub struct EndpointConfiguration_ {
        pub attachment_arn: Option<crate::value::ExpString>,
        pub client_ip_preservation_enabled: Option<crate::value::ExpBool>,
        pub endpoint_id: crate::value::ExpString,
        pub weight: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_globalaccelerator_EndpointGroup_EndpointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GlobalAccelerator::EndpointGroup.EndpointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_globalaccelerator_EndpointGroup_EndpointConfiguration as EndpointConfiguration;
    impl crate::value::ToValue for EndpointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_arn {
                properties.insert(
                    "AttachmentArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_ip_preservation_enabled {
                properties.insert(
                    "ClientIPPreservationEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EndpointId".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_id),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-endpointgroup-portoverride.html
    pub struct PortOverride_ {
        pub endpoint_port: i64,
        pub listener_port: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_globalaccelerator_EndpointGroup_PortOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GlobalAccelerator::EndpointGroup.PortOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_globalaccelerator_EndpointGroup_PortOverride as PortOverride;
    impl crate::value::ToValue for PortOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndpointPort".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_port),
            );
            properties.insert(
                "ListenerPort".to_string(),
                crate::value::ToValue::to_value(&self.listener_port),
            );
            properties.into()
        }
    }
}
pub mod listener {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-globalaccelerator-listener-portrange.html
    pub struct PortRange_ {
        pub from_port: i64,
        pub to_port: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_globalaccelerator_Listener_PortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GlobalAccelerator::Listener.PortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_globalaccelerator_Listener_PortRange as PortRange;
    impl crate::value::ToValue for PortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-accelerator.html
pub struct Accelerator_ {
    pub enabled: Option<crate::value::ExpBool>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub ip_addresses: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_globalaccelerator_Accelerator {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GlobalAccelerator::Accelerator"
        $($field $value)*)
    };
}
pub use crate::__aws_globalaccelerator_Accelerator as Accelerator;
impl crate::template::ToResource for Accelerator_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GlobalAccelerator"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Accelerator"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_addresses {
            properties.insert(
                "IpAddresses".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-crossaccountattachment.html
pub struct CrossAccountAttachment_ {
    pub name: crate::value::ExpString,
    pub principals: Option<Vec<crate::value::ExpString>>,
    pub resources: Option<Vec<super::globalaccelerator::crossaccountattachment::Resource_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_globalaccelerator_CrossAccountAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GlobalAccelerator::CrossAccountAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_globalaccelerator_CrossAccountAttachment as CrossAccountAttachment;
impl crate::template::ToResource for CrossAccountAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GlobalAccelerator"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CrossAccountAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.principals {
            properties.insert(
                "Principals".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-endpointgroup.html
pub struct EndpointGroup_ {
    pub endpoint_configurations:
        Option<Vec<super::globalaccelerator::endpointgroup::EndpointConfiguration_>>,
    pub endpoint_group_region: crate::value::ExpString,
    pub health_check_interval_seconds: Option<i64>,
    pub health_check_path: Option<crate::value::ExpString>,
    pub health_check_port: Option<i64>,
    pub health_check_protocol: Option<crate::value::ExpString>,
    pub listener_arn: crate::value::ExpString,
    pub port_overrides: Option<Vec<super::globalaccelerator::endpointgroup::PortOverride_>>,
    pub threshold_count: Option<i64>,
    pub traffic_dial_percentage: Option<f64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_globalaccelerator_EndpointGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GlobalAccelerator::EndpointGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_globalaccelerator_EndpointGroup as EndpointGroup;
impl crate::template::ToResource for EndpointGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GlobalAccelerator"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EndpointGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.endpoint_configurations {
            properties.insert(
                "EndpointConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EndpointGroupRegion".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_group_region),
        );
        if let Some(ref value) = self.health_check_interval_seconds {
            properties.insert(
                "HealthCheckIntervalSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_path {
            properties.insert(
                "HealthCheckPath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_port {
            properties.insert(
                "HealthCheckPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_protocol {
            properties.insert(
                "HealthCheckProtocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ListenerArn".to_string(),
            crate::value::ToValue::to_value(&self.listener_arn),
        );
        if let Some(ref value) = self.port_overrides {
            properties.insert(
                "PortOverrides".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.threshold_count {
            properties.insert(
                "ThresholdCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.traffic_dial_percentage {
            properties.insert(
                "TrafficDialPercentage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-globalaccelerator-listener.html
pub struct Listener_ {
    pub accelerator_arn: crate::value::ExpString,
    pub client_affinity: Option<crate::value::ExpString>,
    pub port_ranges: Vec<super::globalaccelerator::listener::PortRange_>,
    pub protocol: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_globalaccelerator_Listener {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GlobalAccelerator::Listener"
        $($field $value)*)
    };
}
pub use crate::__aws_globalaccelerator_Listener as Listener;
impl crate::template::ToResource for Listener_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GlobalAccelerator"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Listener"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AcceleratorArn".to_string(),
            crate::value::ToValue::to_value(&self.accelerator_arn),
        );
        if let Some(ref value) = self.client_affinity {
            properties.insert(
                "ClientAffinity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PortRanges".to_string(),
            crate::value::ToValue::to_value(&self.port_ranges),
        );
        properties.insert(
            "Protocol".to_string(),
            crate::value::ToValue::to_value(&self.protocol),
        );
        properties
    }
}
