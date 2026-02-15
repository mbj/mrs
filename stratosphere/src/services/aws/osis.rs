pub mod pipeline {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-bufferoptions.html
    pub struct BufferOptions_ {
        pub persistent_buffer_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_osis_Pipeline_BufferOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OSIS::Pipeline.BufferOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_osis_Pipeline_BufferOptions as BufferOptions;
    impl crate::value::ToValue for BufferOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PersistentBufferEnabled".to_string(),
                crate::value::ToValue::to_value(&self.persistent_buffer_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-cloudwatchlogdestination.html
    pub struct CloudWatchLogDestination_ {
        pub log_group: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_osis_Pipeline_CloudWatchLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OSIS::Pipeline.CloudWatchLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_osis_Pipeline_CloudWatchLogDestination as CloudWatchLogDestination;
    impl crate::value::ToValue for CloudWatchLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroup".to_string(),
                crate::value::ToValue::to_value(&self.log_group),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-encryptionatrestoptions.html
    pub struct EncryptionAtRestOptions_ {
        pub kms_key_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_osis_Pipeline_EncryptionAtRestOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OSIS::Pipeline.EncryptionAtRestOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_osis_Pipeline_EncryptionAtRestOptions as EncryptionAtRestOptions;
    impl crate::value::ToValue for EncryptionAtRestOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(&self.kms_key_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-logpublishingoptions.html
    pub struct LogPublishingOptions_ {
        pub cloud_watch_log_destination: Option<Box<CloudWatchLogDestination_>>,
        pub is_logging_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_osis_Pipeline_LogPublishingOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OSIS::Pipeline.LogPublishingOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_osis_Pipeline_LogPublishingOptions as LogPublishingOptions;
    impl crate::value::ToValue for LogPublishingOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_log_destination {
                properties.insert(
                    "CloudWatchLogDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_logging_enabled {
                properties.insert(
                    "IsLoggingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcattachmentoptions.html
    pub struct VpcAttachmentOptions_ {
        pub attach_to_vpc: crate::value::ExpBool,
        pub cidr_block: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_osis_Pipeline_VpcAttachmentOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OSIS::Pipeline.VpcAttachmentOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_osis_Pipeline_VpcAttachmentOptions as VpcAttachmentOptions;
    impl crate::value::ToValue for VpcAttachmentOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttachToVpc".to_string(),
                crate::value::ToValue::to_value(&self.attach_to_vpc),
            );
            properties.insert(
                "CidrBlock".to_string(),
                crate::value::ToValue::to_value(&self.cidr_block),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcendpoint.html
    pub struct VpcEndpoint_ {
        pub vpc_endpoint_id: Option<crate::value::ExpString>,
        pub vpc_id: Option<crate::value::ExpString>,
        pub vpc_options: Option<Box<VpcOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_osis_Pipeline_VpcEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OSIS::Pipeline.VpcEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_osis_Pipeline_VpcEndpoint as VpcEndpoint;
    impl crate::value::ToValue for VpcEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_endpoint_id {
                properties.insert(
                    "VpcEndpointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vpc_options {
                properties.insert(
                    "VpcOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-osis-pipeline-vpcoptions.html
    pub struct VpcOptions_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Vec<crate::value::ExpString>,
        pub vpc_attachment_options: Option<Box<VpcAttachmentOptions_>>,
        pub vpc_endpoint_management: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_osis_Pipeline_VpcOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OSIS::Pipeline.VpcOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_osis_Pipeline_VpcOptions as VpcOptions;
    impl crate::value::ToValue for VpcOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            if let Some(ref value) = self.vpc_attachment_options {
                properties.insert(
                    "VpcAttachmentOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_endpoint_management {
                properties.insert(
                    "VpcEndpointManagement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-osis-pipeline.html
pub struct Pipeline_ {
    pub buffer_options: Option<super::osis::pipeline::BufferOptions_>,
    pub encryption_at_rest_options: Option<super::osis::pipeline::EncryptionAtRestOptions_>,
    pub log_publishing_options: Option<super::osis::pipeline::LogPublishingOptions_>,
    pub max_units: i32,
    pub min_units: i32,
    pub pipeline_configuration_body: crate::value::ExpString,
    pub pipeline_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_options: Option<super::osis::pipeline::VpcOptions_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_osis_Pipeline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OSIS::Pipeline" $($field
        $value)*)
    };
}
pub use crate::__aws_osis_Pipeline as Pipeline;
impl crate::template::ToResource for Pipeline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OSIS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Pipeline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.buffer_options {
            properties.insert(
                "BufferOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_at_rest_options {
            properties.insert(
                "EncryptionAtRestOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_publishing_options {
            properties.insert(
                "LogPublishingOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MaxUnits".to_string(),
            crate::value::ToValue::to_value(&self.max_units),
        );
        properties.insert(
            "MinUnits".to_string(),
            crate::value::ToValue::to_value(&self.min_units),
        );
        properties.insert(
            "PipelineConfigurationBody".to_string(),
            crate::value::ToValue::to_value(&self.pipeline_configuration_body),
        );
        properties.insert(
            "PipelineName".to_string(),
            crate::value::ToValue::to_value(&self.pipeline_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_options {
            properties.insert(
                "VpcOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
