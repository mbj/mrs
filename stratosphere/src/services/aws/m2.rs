pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-m2-application-definition.html
    pub struct Definition_ {
        pub content: Option<crate::value::ExpString>,
        pub s3_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_m2_Application_Definition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::M2::Application.Definition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_m2_Application_Definition as Definition;
    impl crate::value::ToValue for Definition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_location {
                properties.insert(
                    "S3Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod environment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-m2-environment-efsstorageconfiguration.html
    pub struct EfsStorageConfiguration_ {
        pub file_system_id: crate::value::ExpString,
        pub mount_point: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_m2_Environment_EfsStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::M2::Environment.EfsStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_m2_Environment_EfsStorageConfiguration as EfsStorageConfiguration;
    impl crate::value::ToValue for EfsStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            properties.insert(
                "MountPoint".to_string(),
                crate::value::ToValue::to_value(&self.mount_point),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-m2-environment-fsxstorageconfiguration.html
    pub struct FsxStorageConfiguration_ {
        pub file_system_id: crate::value::ExpString,
        pub mount_point: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_m2_Environment_FsxStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::M2::Environment.FsxStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_m2_Environment_FsxStorageConfiguration as FsxStorageConfiguration;
    impl crate::value::ToValue for FsxStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            properties.insert(
                "MountPoint".to_string(),
                crate::value::ToValue::to_value(&self.mount_point),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-m2-environment-highavailabilityconfig.html
    pub struct HighAvailabilityConfig_ {
        pub desired_capacity: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_m2_Environment_HighAvailabilityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::M2::Environment.HighAvailabilityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_m2_Environment_HighAvailabilityConfig as HighAvailabilityConfig;
    impl crate::value::ToValue for HighAvailabilityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DesiredCapacity".to_string(),
                crate::value::ToValue::to_value(&self.desired_capacity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-m2-environment-storageconfiguration.html
    pub struct StorageConfiguration_ {
        pub efs: Option<Box<EfsStorageConfiguration_>>,
        pub fsx: Option<Box<FsxStorageConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_m2_Environment_StorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::M2::Environment.StorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_m2_Environment_StorageConfiguration as StorageConfiguration;
    impl crate::value::ToValue for StorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.efs {
                properties.insert("Efs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.fsx {
                properties.insert("Fsx".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-m2-application.html
pub struct Application_ {
    pub definition: Option<super::m2::application::Definition_>,
    pub description: Option<crate::value::ExpString>,
    pub engine_type: crate::value::ExpString,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_m2_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::M2::Application" $($field
        $value)*)
    };
}
pub use crate::__aws_m2_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("M2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.definition {
            properties.insert(
                "Definition".to_string(),
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
            "EngineType".to_string(),
            crate::value::ToValue::to_value(&self.engine_type),
        );
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-m2-deployment.html
pub struct Deployment_ {
    pub application_id: crate::value::ExpString,
    pub application_version: i64,
    pub environment_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_m2_Deployment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::M2::Deployment" $($field
        $value)*)
    };
}
pub use crate::__aws_m2_Deployment as Deployment;
impl crate::template::ToResource for Deployment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("M2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Deployment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "ApplicationVersion".to_string(),
            crate::value::ToValue::to_value(&self.application_version),
        );
        properties.insert(
            "EnvironmentId".to_string(),
            crate::value::ToValue::to_value(&self.environment_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-m2-environment.html
pub struct Environment_ {
    pub description: Option<crate::value::ExpString>,
    pub engine_type: crate::value::ExpString,
    pub engine_version: Option<crate::value::ExpString>,
    pub high_availability_config: Option<super::m2::environment::HighAvailabilityConfig_>,
    pub instance_type: crate::value::ExpString,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub network_type: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub storage_configurations: Option<Vec<super::m2::environment::StorageConfiguration_>>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_m2_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::M2::Environment" $($field
        $value)*)
    };
}
pub use crate::__aws_m2_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("M2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
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
            "EngineType".to_string(),
            crate::value::ToValue::to_value(&self.engine_type),
        );
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.high_availability_config {
            properties.insert(
                "HighAvailabilityConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_configurations {
            properties.insert(
                "StorageConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
