pub mod privatednsnamespace {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-privatednsnamespace-privatednspropertiesmutable.html>
    pub struct PrivateDnsPropertiesMutable_ {
        pub soa: Option<Box<SOA_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_PrivateDnsNamespace_PrivateDnsPropertiesMutable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::PrivateDnsNamespace.PrivateDnsPropertiesMutable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_PrivateDnsNamespace_PrivateDnsPropertiesMutable as PrivateDnsPropertiesMutable;
    impl crate::value::ToValue for PrivateDnsPropertiesMutable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.soa {
                properties.insert("SOA".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-privatednsnamespace-properties.html>
    pub struct Properties_ {
        pub dns_properties: Option<Box<PrivateDnsPropertiesMutable_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_PrivateDnsNamespace_Properties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::PrivateDnsNamespace.Properties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_PrivateDnsNamespace_Properties as Properties;
    impl crate::value::ToValue for Properties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dns_properties {
                properties.insert(
                    "DnsProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-privatednsnamespace-soa.html>
    pub struct SOA_ {
        pub ttl: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_PrivateDnsNamespace_SOA {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::PrivateDnsNamespace.SOA"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_PrivateDnsNamespace_SOA as SOA;
    impl crate::value::ToValue for SOA_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ttl {
                properties.insert("TTL".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod publicdnsnamespace {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-publicdnsnamespace-properties.html>
    pub struct Properties_ {
        pub dns_properties: Option<Box<PublicDnsPropertiesMutable_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_PublicDnsNamespace_Properties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::PublicDnsNamespace.Properties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_PublicDnsNamespace_Properties as Properties;
    impl crate::value::ToValue for Properties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dns_properties {
                properties.insert(
                    "DnsProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-publicdnsnamespace-publicdnspropertiesmutable.html>
    pub struct PublicDnsPropertiesMutable_ {
        pub soa: Option<Box<SOA_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_PublicDnsNamespace_PublicDnsPropertiesMutable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::PublicDnsNamespace.PublicDnsPropertiesMutable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_PublicDnsNamespace_PublicDnsPropertiesMutable as PublicDnsPropertiesMutable;
    impl crate::value::ToValue for PublicDnsPropertiesMutable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.soa {
                properties.insert("SOA".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-publicdnsnamespace-soa.html>
    pub struct SOA_ {
        pub ttl: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_PublicDnsNamespace_SOA {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::PublicDnsNamespace.SOA"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_PublicDnsNamespace_SOA as SOA;
    impl crate::value::ToValue for SOA_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ttl {
                properties.insert("TTL".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod service {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-dnsconfig.html>
    pub struct DnsConfig_ {
        pub dns_records: Vec<DnsRecord_>,
        pub namespace_id: Option<crate::value::ExpString>,
        pub routing_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_Service_DnsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::Service.DnsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_Service_DnsConfig as DnsConfig;
    impl crate::value::ToValue for DnsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DnsRecords".to_string(),
                crate::value::ToValue::to_value(&self.dns_records),
            );
            if let Some(ref value) = self.namespace_id {
                properties.insert(
                    "NamespaceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.routing_policy {
                properties.insert(
                    "RoutingPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-dnsrecord.html>
    pub struct DnsRecord_ {
        pub ttl: f64,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_Service_DnsRecord {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::Service.DnsRecord"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_Service_DnsRecord as DnsRecord;
    impl crate::value::ToValue for DnsRecord_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TTL".to_string(),
                crate::value::ToValue::to_value(&self.ttl),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-healthcheckconfig.html>
    pub struct HealthCheckConfig_ {
        pub failure_threshold: Option<f64>,
        pub resource_path: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_Service_HealthCheckConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::Service.HealthCheckConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_Service_HealthCheckConfig as HealthCheckConfig;
    impl crate::value::ToValue for HealthCheckConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failure_threshold {
                properties.insert(
                    "FailureThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_path {
                properties.insert(
                    "ResourcePath".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-servicediscovery-service-healthcheckcustomconfig.html>
    pub struct HealthCheckCustomConfig_ {
        pub failure_threshold: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_servicediscovery_Service_HealthCheckCustomConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ServiceDiscovery::Service.HealthCheckCustomConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_servicediscovery_Service_HealthCheckCustomConfig as HealthCheckCustomConfig;
    impl crate::value::ToValue for HealthCheckCustomConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failure_threshold {
                properties.insert(
                    "FailureThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-httpnamespace.html>
pub struct HttpNamespace_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicediscovery_HttpNamespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceDiscovery::HttpNamespace"
        $($field $value)*)
    };
}
pub use crate::__aws_servicediscovery_HttpNamespace as HttpNamespace;
impl crate::template::ToResource for HttpNamespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceDiscovery"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HttpNamespace"),
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
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-instance.html>
pub struct Instance_ {
    pub instance_attributes: serde_json::Value,
    pub instance_id: Option<crate::value::ExpString>,
    pub service_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicediscovery_Instance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceDiscovery::Instance"
        $($field $value)*)
    };
}
pub use crate::__aws_servicediscovery_Instance as Instance;
impl crate::template::ToResource for Instance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceDiscovery"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Instance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceAttributes".to_string(),
            crate::value::ToValue::to_value(&self.instance_attributes),
        );
        if let Some(ref value) = self.instance_id {
            properties.insert(
                "InstanceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceId".to_string(),
            crate::value::ToValue::to_value(&self.service_id),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-privatednsnamespace.html>
pub struct PrivateDnsNamespace_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub properties: Option<super::servicediscovery::privatednsnamespace::Properties_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicediscovery_PrivateDnsNamespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceDiscovery::PrivateDnsNamespace"
        $($field $value)*)
    };
}
pub use crate::__aws_servicediscovery_PrivateDnsNamespace as PrivateDnsNamespace;
impl crate::template::ToResource for PrivateDnsNamespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceDiscovery"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PrivateDnsNamespace"),
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
        if let Some(ref value) = self.properties {
            properties.insert(
                "Properties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Vpc".to_string(),
            crate::value::ToValue::to_value(&self.vpc),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-publicdnsnamespace.html>
pub struct PublicDnsNamespace_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub properties: Option<super::servicediscovery::publicdnsnamespace::Properties_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicediscovery_PublicDnsNamespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceDiscovery::PublicDnsNamespace"
        $($field $value)*)
    };
}
pub use crate::__aws_servicediscovery_PublicDnsNamespace as PublicDnsNamespace;
impl crate::template::ToResource for PublicDnsNamespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceDiscovery"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PublicDnsNamespace"),
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
        if let Some(ref value) = self.properties {
            properties.insert(
                "Properties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-servicediscovery-service.html>
pub struct Service_ {
    pub description: Option<crate::value::ExpString>,
    pub dns_config: Option<super::servicediscovery::service::DnsConfig_>,
    pub health_check_config: Option<super::servicediscovery::service::HealthCheckConfig_>,
    pub health_check_custom_config:
        Option<super::servicediscovery::service::HealthCheckCustomConfig_>,
    pub name: Option<crate::value::ExpString>,
    pub namespace_id: Option<crate::value::ExpString>,
    pub service_attributes: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_servicediscovery_Service {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ServiceDiscovery::Service"
        $($field $value)*)
    };
}
pub use crate::__aws_servicediscovery_Service as Service;
impl crate::template::ToResource for Service_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ServiceDiscovery"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Service"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dns_config {
            properties.insert(
                "DnsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_config {
            properties.insert(
                "HealthCheckConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_custom_config {
            properties.insert(
                "HealthCheckCustomConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.namespace_id {
            properties.insert(
                "NamespaceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_attributes {
            properties.insert(
                "ServiceAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
