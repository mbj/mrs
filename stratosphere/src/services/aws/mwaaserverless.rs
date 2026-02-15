pub mod workflow {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaaserverless-workflow-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaaserverless_Workflow_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAAServerless::Workflow.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaaserverless_Workflow_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaaserverless-workflow-loggingconfiguration.html
    pub struct LoggingConfiguration_ {
        pub log_group_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaaserverless_Workflow_LoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAAServerless::Workflow.LoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaaserverless_Workflow_LoggingConfiguration as LoggingConfiguration;
    impl crate::value::ToValue for LoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(&self.log_group_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaaserverless-workflow-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaaserverless_Workflow_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAAServerless::Workflow.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaaserverless_Workflow_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaaserverless-workflow-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub object_key: crate::value::ExpString,
        pub version_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaaserverless_Workflow_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAAServerless::Workflow.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaaserverless_Workflow_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "ObjectKey".to_string(),
                crate::value::ToValue::to_value(&self.object_key),
            );
            if let Some(ref value) = self.version_id {
                properties.insert(
                    "VersionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mwaaserverless-workflow-scheduleconfiguration.html
    pub struct ScheduleConfiguration_ {
        pub cron_expression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mwaaserverless_Workflow_ScheduleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MWAAServerless::Workflow.ScheduleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mwaaserverless_Workflow_ScheduleConfiguration as ScheduleConfiguration;
    impl crate::value::ToValue for ScheduleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cron_expression {
                properties.insert(
                    "CronExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mwaaserverless-workflow.html
pub struct Workflow_ {
    pub definition_s3_location: super::mwaaserverless::workflow::S3Location_,
    pub description: Option<crate::value::ExpString>,
    pub encryption_configuration: Option<super::mwaaserverless::workflow::EncryptionConfiguration_>,
    pub logging_configuration: Option<super::mwaaserverless::workflow::LoggingConfiguration_>,
    pub name: Option<crate::value::ExpString>,
    pub network_configuration: Option<super::mwaaserverless::workflow::NetworkConfiguration_>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub trigger_mode: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mwaaserverless_Workflow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MWAAServerless::Workflow"
        $($field $value)*)
    };
}
pub use crate::__aws_mwaaserverless_Workflow as Workflow;
impl crate::template::ToResource for Workflow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MWAAServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workflow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DefinitionS3Location".to_string(),
            crate::value::ToValue::to_value(&self.definition_s3_location),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_configuration {
            properties.insert(
                "LoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.network_configuration {
            properties.insert(
                "NetworkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trigger_mode {
            properties.insert(
                "TriggerMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
