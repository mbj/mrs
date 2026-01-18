pub mod stream {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streamencryption.html
    pub struct StreamEncryption_ {
        pub encryption_type: crate::value::ExpString,
        pub key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesis_Stream_StreamEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Kinesis::Stream.StreamEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesis_Stream_StreamEncryption as StreamEncryption;
    impl crate::value::ToValue for StreamEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionType".to_string(),
                crate::value::ToValue::to_value(&self.encryption_type),
            );
            properties.insert(
                "KeyId".to_string(),
                crate::value::ToValue::to_value(&self.key_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesis-stream-streammodedetails.html
    pub struct StreamModeDetails_ {
        pub stream_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesis_Stream_StreamModeDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Kinesis::Stream.StreamModeDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesis_Stream_StreamModeDetails as StreamModeDetails;
    impl crate::value::ToValue for StreamModeDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StreamMode".to_string(),
                crate::value::ToValue::to_value(&self.stream_mode),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub resource_arn: crate::value::ExpString,
    pub resource_policy: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesis_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Kinesis::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesis_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Kinesis"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties.insert(
            "ResourcePolicy".to_string(),
            crate::value::ToValue::to_value(&self.resource_policy),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-stream.html
pub struct Stream_ {
    pub desired_shard_level_metrics: Option<Vec<crate::value::ExpString>>,
    pub name: Option<crate::value::ExpString>,
    pub retention_period_hours: Option<i64>,
    pub shard_count: Option<i64>,
    pub stream_encryption: Option<super::kinesis::stream::StreamEncryption_>,
    pub stream_mode_details: Option<super::kinesis::stream::StreamModeDetails_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesis_Stream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Kinesis::Stream" $($field
        $value)*)
    };
}
pub use crate::__aws_kinesis_Stream as Stream;
impl crate::template::ToResource for Stream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Kinesis"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.desired_shard_level_metrics {
            properties.insert(
                "DesiredShardLevelMetrics".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.retention_period_hours {
            properties.insert(
                "RetentionPeriodHours".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.shard_count {
            properties.insert(
                "ShardCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stream_encryption {
            properties.insert(
                "StreamEncryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stream_mode_details {
            properties.insert(
                "StreamModeDetails".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesis-streamconsumer.html
pub struct StreamConsumer_ {
    pub consumer_name: crate::value::ExpString,
    pub stream_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesis_StreamConsumer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Kinesis::StreamConsumer"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesis_StreamConsumer as StreamConsumer;
impl crate::template::ToResource for StreamConsumer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Kinesis"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StreamConsumer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConsumerName".to_string(),
            crate::value::ToValue::to_value(&self.consumer_name),
        );
        properties.insert(
            "StreamARN".to_string(),
            crate::value::ToValue::to_value(&self.stream_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
