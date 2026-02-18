pub mod stream {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisvideo-stream-streamstorageconfiguration.html>
    pub struct StreamStorageConfiguration_ {
        pub default_storage_tier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisvideo_Stream_StreamStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisVideo::Stream.StreamStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisvideo_Stream_StreamStorageConfiguration as StreamStorageConfiguration;
    impl crate::value::ToValue for StreamStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_storage_tier {
                properties.insert(
                    "DefaultStorageTier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisvideo-signalingchannel.html>
pub struct SignalingChannel_ {
    pub message_ttl_seconds: Option<i32>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisvideo_SignalingChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisVideo::SignalingChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisvideo_SignalingChannel as SignalingChannel;
impl crate::template::ToResource for SignalingChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisVideo"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SignalingChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.message_ttl_seconds {
            properties.insert(
                "MessageTtlSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisvideo-stream.html>
pub struct Stream_ {
    pub data_retention_in_hours: Option<i32>,
    pub device_name: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub media_type: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub stream_storage_configuration:
        Option<super::kinesisvideo::stream::StreamStorageConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisvideo_Stream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisVideo::Stream"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisvideo_Stream as Stream;
impl crate::template::ToResource for Stream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisVideo"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_retention_in_hours {
            properties.insert(
                "DataRetentionInHours".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.device_name {
            properties.insert(
                "DeviceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.media_type {
            properties.insert(
                "MediaType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.stream_storage_configuration {
            properties.insert(
                "StreamStorageConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
