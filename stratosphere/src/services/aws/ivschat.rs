pub mod loggingconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivschat-loggingconfiguration-cloudwatchlogsdestinationconfiguration.html>
    pub struct CloudWatchLogsDestinationConfiguration_ {
        pub log_group_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivschat_LoggingConfiguration_CloudWatchLogsDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVSChat::LoggingConfiguration.CloudWatchLogsDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivschat_LoggingConfiguration_CloudWatchLogsDestinationConfiguration as CloudWatchLogsDestinationConfiguration;
    impl crate::value::ToValue for CloudWatchLogsDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroupName".to_string(),
                crate::value::ToValue::to_value(&self.log_group_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivschat-loggingconfiguration-destinationconfiguration.html>
    pub struct DestinationConfiguration_ {
        pub cloud_watch_logs: Option<Box<CloudWatchLogsDestinationConfiguration_>>,
        pub firehose: Option<Box<FirehoseDestinationConfiguration_>>,
        pub s3: Option<Box<S3DestinationConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivschat_LoggingConfiguration_DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVSChat::LoggingConfiguration.DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivschat_LoggingConfiguration_DestinationConfiguration as DestinationConfiguration;
    impl crate::value::ToValue for DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs {
                properties.insert(
                    "CloudWatchLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firehose {
                properties.insert(
                    "Firehose".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivschat-loggingconfiguration-firehosedestinationconfiguration.html>
    pub struct FirehoseDestinationConfiguration_ {
        pub delivery_stream_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivschat_LoggingConfiguration_FirehoseDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVSChat::LoggingConfiguration.FirehoseDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivschat_LoggingConfiguration_FirehoseDestinationConfiguration as FirehoseDestinationConfiguration;
    impl crate::value::ToValue for FirehoseDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeliveryStreamName".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivschat-loggingconfiguration-s3destinationconfiguration.html>
    pub struct S3DestinationConfiguration_ {
        pub bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivschat_LoggingConfiguration_S3DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVSChat::LoggingConfiguration.S3DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivschat_LoggingConfiguration_S3DestinationConfiguration as S3DestinationConfiguration;
    impl crate::value::ToValue for S3DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.into()
        }
    }
}
pub mod room {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ivschat-room-messagereviewhandler.html>
    pub struct MessageReviewHandler_ {
        pub fallback_result: Option<crate::value::ExpString>,
        pub uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ivschat_Room_MessageReviewHandler {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IVSChat::Room.MessageReviewHandler"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ivschat_Room_MessageReviewHandler as MessageReviewHandler;
    impl crate::value::ToValue for MessageReviewHandler_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fallback_result {
                properties.insert(
                    "FallbackResult".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri {
                properties.insert("Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivschat-loggingconfiguration.html>
pub struct LoggingConfiguration_ {
    pub destination_configuration: super::ivschat::loggingconfiguration::DestinationConfiguration_,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivschat_LoggingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVSChat::LoggingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_ivschat_LoggingConfiguration as LoggingConfiguration;
impl crate::template::ToResource for LoggingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVSChat"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoggingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DestinationConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.destination_configuration),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ivschat-room.html>
pub struct Room_ {
    pub logging_configuration_identifiers: Option<Vec<crate::value::ExpString>>,
    pub maximum_message_length: Option<i32>,
    pub maximum_message_rate_per_second: Option<i32>,
    pub message_review_handler: Option<super::ivschat::room::MessageReviewHandler_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ivschat_Room {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IVSChat::Room" $($field
        $value)*)
    };
}
pub use crate::__aws_ivschat_Room as Room;
impl crate::template::ToResource for Room_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IVSChat"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Room"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.logging_configuration_identifiers {
            properties.insert(
                "LoggingConfigurationIdentifiers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_message_length {
            properties.insert(
                "MaximumMessageLength".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_message_rate_per_second {
            properties.insert(
                "MaximumMessageRatePerSecond".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.message_review_handler {
            properties.insert(
                "MessageReviewHandler".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
