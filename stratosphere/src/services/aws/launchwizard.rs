pub mod deployment {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-launchwizard-deployment-tags.html>
    pub struct Tags_ {
        pub key: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_launchwizard_Deployment_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LaunchWizard::Deployment.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_launchwizard_Deployment_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
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
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-launchwizard-deployment.html>
pub struct Deployment_ {
    pub deployment_pattern_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub specifications: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub tags: Option<Vec<super::launchwizard::deployment::Tags_>>,
    pub workload_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_launchwizard_Deployment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LaunchWizard::Deployment"
        $($field $value)*)
    };
}
pub use crate::__aws_launchwizard_Deployment as Deployment;
impl crate::template::ToResource for Deployment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LaunchWizard"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Deployment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DeploymentPatternName".to_string(),
            crate::value::ToValue::to_value(&self.deployment_pattern_name),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.specifications {
            properties.insert(
                "Specifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkloadName".to_string(),
            crate::value::ToValue::to_value(&self.workload_name),
        );
        properties
    }
}
