pub mod environmentec2 {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloud9-environmentec2-repository.html
    pub struct Repository_ {
        pub path_component: crate::value::ExpString,
        pub repository_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloud9_EnvironmentEC2_Repository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Cloud9::EnvironmentEC2.Repository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloud9_EnvironmentEC2_Repository as Repository;
    impl crate::value::ToValue for Repository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PathComponent".to_string(),
                crate::value::ToValue::to_value(&self.path_component),
            );
            properties.insert(
                "RepositoryUrl".to_string(),
                crate::value::ToValue::to_value(&self.repository_url),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloud9-environmentec2.html
pub struct EnvironmentEC2_ {
    pub automatic_stop_time_minutes: Option<i64>,
    pub connection_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub image_id: crate::value::ExpString,
    pub instance_type: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub owner_arn: Option<crate::value::ExpString>,
    pub repositories: Option<Vec<super::cloud9::environmentec2::Repository_>>,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloud9_EnvironmentEC2 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Cloud9::EnvironmentEC2"
        $($field $value)*)
    };
}
pub use crate::__aws_cloud9_EnvironmentEC2 as EnvironmentEC2;
impl crate::template::ToResource for EnvironmentEC2_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cloud9"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EnvironmentEC2"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.automatic_stop_time_minutes {
            properties.insert(
                "AutomaticStopTimeMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connection_type {
            properties.insert(
                "ConnectionType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ImageId".to_string(),
            crate::value::ToValue::to_value(&self.image_id),
        );
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.owner_arn {
            properties.insert(
                "OwnerArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.repositories {
            properties.insert(
                "Repositories".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_id {
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
