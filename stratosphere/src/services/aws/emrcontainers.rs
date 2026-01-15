pub mod virtualcluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-virtualcluster-containerinfo.html
    pub struct ContainerInfo_ {
        pub eks_info: Box<EksInfo_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_VirtualCluster_ContainerInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::EMRContainers::VirtualCluster.ContainerInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_VirtualCluster_ContainerInfo as ContainerInfo;
    impl crate::value::ToValue for ContainerInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EksInfo".to_string(),
                crate::value::ToValue::to_value(&self.eks_info),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-virtualcluster-containerprovider.html
    pub struct ContainerProvider_ {
        pub id: crate::value::ExpString,
        pub info: Box<ContainerInfo_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_VirtualCluster_ContainerProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::EMRContainers::VirtualCluster.ContainerProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_VirtualCluster_ContainerProvider as ContainerProvider;
    impl crate::value::ToValue for ContainerProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Info".to_string(),
                crate::value::ToValue::to_value(&self.info),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrcontainers-virtualcluster-eksinfo.html
    pub struct EksInfo_ {
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrcontainers_VirtualCluster_EksInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::EMRContainers::VirtualCluster.EksInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrcontainers_VirtualCluster_EksInfo as EksInfo;
    impl crate::value::ToValue for EksInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrcontainers-virtualcluster.html
pub struct VirtualCluster_ {
    pub container_provider: super::emrcontainers::virtualcluster::ContainerProvider_,
    pub name: crate::value::ExpString,
    pub security_configuration_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emrcontainers_VirtualCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::EMRContainers::VirtualCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_emrcontainers_VirtualCluster as VirtualCluster;
impl crate::template::ToResource for VirtualCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMRContainers"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VirtualCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContainerProvider".to_string(),
            crate::value::ToValue::to_value(&self.container_provider),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.security_configuration_id {
            properties.insert(
                "SecurityConfigurationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
