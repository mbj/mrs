pub mod inferencescheduler {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-datainputconfiguration.html
    pub struct DataInputConfiguration_ {
        pub inference_input_name_configuration: Option<Box<InputNameConfiguration_>>,
        pub input_time_zone_offset: Option<crate::value::ExpString>,
        pub s3_input_configuration: Box<S3InputConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutequipment_InferenceScheduler_DataInputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LookoutEquipment::InferenceScheduler.DataInputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutequipment_InferenceScheduler_DataInputConfiguration as DataInputConfiguration;
    impl crate::value::ToValue for DataInputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inference_input_name_configuration {
                properties.insert(
                    "InferenceInputNameConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_time_zone_offset {
                properties.insert(
                    "InputTimeZoneOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3InputConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.s3_input_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-dataoutputconfiguration.html
    pub struct DataOutputConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub s3_output_configuration: Box<S3OutputConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutequipment_InferenceScheduler_DataOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LookoutEquipment::InferenceScheduler.DataOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutequipment_InferenceScheduler_DataOutputConfiguration as DataOutputConfiguration;
    impl crate::value::ToValue for DataOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3OutputConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.s3_output_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-inputnameconfiguration.html
    pub struct InputNameConfiguration_ {
        pub component_timestamp_delimiter: Option<crate::value::ExpString>,
        pub timestamp_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutequipment_InferenceScheduler_InputNameConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LookoutEquipment::InferenceScheduler.InputNameConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutequipment_InferenceScheduler_InputNameConfiguration as InputNameConfiguration;
    impl crate::value::ToValue for InputNameConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_timestamp_delimiter {
                properties.insert(
                    "ComponentTimestampDelimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp_format {
                properties.insert(
                    "TimestampFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3inputconfiguration.html
    pub struct S3InputConfiguration_ {
        pub bucket: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutequipment_InferenceScheduler_S3InputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LookoutEquipment::InferenceScheduler.S3InputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutequipment_InferenceScheduler_S3InputConfiguration as S3InputConfiguration;
    impl crate::value::ToValue for S3InputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutequipment-inferencescheduler-s3outputconfiguration.html
    pub struct S3OutputConfiguration_ {
        pub bucket: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutequipment_InferenceScheduler_S3OutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::LookoutEquipment::InferenceScheduler.S3OutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutequipment_InferenceScheduler_S3OutputConfiguration as S3OutputConfiguration;
    impl crate::value::ToValue for S3OutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutequipment-inferencescheduler.html
pub struct InferenceScheduler_ {
    pub data_delay_offset_in_minutes: Option<i32>,
    pub data_input_configuration:
        super::lookoutequipment::inferencescheduler::DataInputConfiguration_,
    pub data_output_configuration:
        super::lookoutequipment::inferencescheduler::DataOutputConfiguration_,
    pub data_upload_frequency: crate::value::ExpString,
    pub inference_scheduler_name: Option<crate::value::ExpString>,
    pub model_name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub server_side_kms_key_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lookoutequipment_InferenceScheduler {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::LookoutEquipment::InferenceScheduler"
        $($field $value)*)
    };
}
pub use crate::__aws_lookoutequipment_InferenceScheduler as InferenceScheduler;
impl crate::template::ToResource for InferenceScheduler_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LookoutEquipment"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InferenceScheduler"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_delay_offset_in_minutes {
            properties.insert(
                "DataDelayOffsetInMinutes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataInputConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.data_input_configuration),
        );
        properties.insert(
            "DataOutputConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.data_output_configuration),
        );
        properties.insert(
            "DataUploadFrequency".to_string(),
            crate::value::ToValue::to_value(&self.data_upload_frequency),
        );
        if let Some(ref value) = self.inference_scheduler_name {
            properties.insert(
                "InferenceSchedulerName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ModelName".to_string(),
            crate::value::ToValue::to_value(&self.model_name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.server_side_kms_key_id {
            properties.insert(
                "ServerSideKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
