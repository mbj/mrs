pub mod campaign {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-collectionscheme.html
    pub struct CollectionScheme_ {
        pub condition_based_collection_scheme: Option<Box<ConditionBasedCollectionScheme_>>,
        pub time_based_collection_scheme: Option<Box<TimeBasedCollectionScheme_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_CollectionScheme {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.CollectionScheme"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_CollectionScheme as CollectionScheme;
    impl crate::value::ToValue for CollectionScheme_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition_based_collection_scheme {
                properties.insert(
                    "ConditionBasedCollectionScheme".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_based_collection_scheme {
                properties.insert(
                    "TimeBasedCollectionScheme".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-conditionbasedcollectionscheme.html
    pub struct ConditionBasedCollectionScheme_ {
        pub condition_language_version: Option<i32>,
        pub expression: crate::value::ExpString,
        pub minimum_trigger_interval_ms: Option<f64>,
        pub trigger_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_ConditionBasedCollectionScheme {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.ConditionBasedCollectionScheme"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_ConditionBasedCollectionScheme as ConditionBasedCollectionScheme;
    impl crate::value::ToValue for ConditionBasedCollectionScheme_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition_language_version {
                properties.insert(
                    "ConditionLanguageVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            if let Some(ref value) = self.minimum_trigger_interval_ms {
                properties.insert(
                    "MinimumTriggerIntervalMs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.trigger_mode {
                properties.insert(
                    "TriggerMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-conditionbasedsignalfetchconfig.html
    pub struct ConditionBasedSignalFetchConfig_ {
        pub condition_expression: crate::value::ExpString,
        pub trigger_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_ConditionBasedSignalFetchConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.ConditionBasedSignalFetchConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_ConditionBasedSignalFetchConfig as ConditionBasedSignalFetchConfig;
    impl crate::value::ToValue for ConditionBasedSignalFetchConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConditionExpression".to_string(),
                crate::value::ToValue::to_value(&self.condition_expression),
            );
            properties.insert(
                "TriggerMode".to_string(),
                crate::value::ToValue::to_value(&self.trigger_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-datadestinationconfig.html
    pub struct DataDestinationConfig_ {
        pub mqtt_topic_config: Option<Box<MqttTopicConfig_>>,
        pub s3_config: Option<Box<S3Config_>>,
        pub timestream_config: Option<Box<TimestreamConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_DataDestinationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.DataDestinationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_DataDestinationConfig as DataDestinationConfig;
    impl crate::value::ToValue for DataDestinationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mqtt_topic_config {
                properties.insert(
                    "MqttTopicConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_config {
                properties.insert(
                    "S3Config".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestream_config {
                properties.insert(
                    "TimestreamConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-datapartition.html
    pub struct DataPartition_ {
        pub id: crate::value::ExpString,
        pub storage_options: Box<DataPartitionStorageOptions_>,
        pub upload_options: Option<Box<DataPartitionUploadOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_DataPartition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.DataPartition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_DataPartition as DataPartition;
    impl crate::value::ToValue for DataPartition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "StorageOptions".to_string(),
                crate::value::ToValue::to_value(&self.storage_options),
            );
            if let Some(ref value) = self.upload_options {
                properties.insert(
                    "UploadOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-datapartitionstorageoptions.html
    pub struct DataPartitionStorageOptions_ {
        pub maximum_size: Box<StorageMaximumSize_>,
        pub minimum_time_to_live: Box<StorageMinimumTimeToLive_>,
        pub storage_location: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_DataPartitionStorageOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.DataPartitionStorageOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_DataPartitionStorageOptions as DataPartitionStorageOptions;
    impl crate::value::ToValue for DataPartitionStorageOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaximumSize".to_string(),
                crate::value::ToValue::to_value(&self.maximum_size),
            );
            properties.insert(
                "MinimumTimeToLive".to_string(),
                crate::value::ToValue::to_value(&self.minimum_time_to_live),
            );
            properties.insert(
                "StorageLocation".to_string(),
                crate::value::ToValue::to_value(&self.storage_location),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-datapartitionuploadoptions.html
    pub struct DataPartitionUploadOptions_ {
        pub condition_language_version: Option<i32>,
        pub expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_DataPartitionUploadOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.DataPartitionUploadOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_DataPartitionUploadOptions as DataPartitionUploadOptions;
    impl crate::value::ToValue for DataPartitionUploadOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition_language_version {
                properties.insert(
                    "ConditionLanguageVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-mqtttopicconfig.html
    pub struct MqttTopicConfig_ {
        pub execution_role_arn: crate::value::ExpString,
        pub mqtt_topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_MqttTopicConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.MqttTopicConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_MqttTopicConfig as MqttTopicConfig;
    impl crate::value::ToValue for MqttTopicConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.execution_role_arn),
            );
            properties.insert(
                "MqttTopicArn".to_string(),
                crate::value::ToValue::to_value(&self.mqtt_topic_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-s3config.html
    pub struct S3Config_ {
        pub bucket_arn: crate::value::ExpString,
        pub data_format: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub storage_compression_format: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_S3Config {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.S3Config"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_S3Config as S3Config;
    impl crate::value::ToValue for S3Config_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketArn".to_string(),
                crate::value::ToValue::to_value(&self.bucket_arn),
            );
            if let Some(ref value) = self.data_format {
                properties.insert(
                    "DataFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.storage_compression_format {
                properties.insert(
                    "StorageCompressionFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-signalfetchconfig.html
    pub struct SignalFetchConfig_ {
        pub condition_based: Option<Box<ConditionBasedSignalFetchConfig_>>,
        pub time_based: Option<Box<TimeBasedSignalFetchConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_SignalFetchConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.SignalFetchConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_SignalFetchConfig as SignalFetchConfig;
    impl crate::value::ToValue for SignalFetchConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition_based {
                properties.insert(
                    "ConditionBased".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_based {
                properties.insert(
                    "TimeBased".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-signalfetchinformation.html
    pub struct SignalFetchInformation_ {
        pub actions: Vec<crate::value::ExpString>,
        pub condition_language_version: Option<f64>,
        pub fully_qualified_name: crate::value::ExpString,
        pub signal_fetch_config: Box<SignalFetchConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_SignalFetchInformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.SignalFetchInformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_SignalFetchInformation as SignalFetchInformation;
    impl crate::value::ToValue for SignalFetchInformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            if let Some(ref value) = self.condition_language_version {
                properties.insert(
                    "ConditionLanguageVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FullyQualifiedName".to_string(),
                crate::value::ToValue::to_value(&self.fully_qualified_name),
            );
            properties.insert(
                "SignalFetchConfig".to_string(),
                crate::value::ToValue::to_value(&self.signal_fetch_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-signalinformation.html
    pub struct SignalInformation_ {
        pub data_partition_id: Option<crate::value::ExpString>,
        pub max_sample_count: Option<f64>,
        pub minimum_sampling_interval_ms: Option<f64>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_SignalInformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.SignalInformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_SignalInformation as SignalInformation;
    impl crate::value::ToValue for SignalInformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_partition_id {
                properties.insert(
                    "DataPartitionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_sample_count {
                properties.insert(
                    "MaxSampleCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_sampling_interval_ms {
                properties.insert(
                    "MinimumSamplingIntervalMs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-storagemaximumsize.html
    pub struct StorageMaximumSize_ {
        pub unit: crate::value::ExpString,
        pub value: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_StorageMaximumSize {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.StorageMaximumSize"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_StorageMaximumSize as StorageMaximumSize;
    impl crate::value::ToValue for StorageMaximumSize_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-storageminimumtimetolive.html
    pub struct StorageMinimumTimeToLive_ {
        pub unit: crate::value::ExpString,
        pub value: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_StorageMinimumTimeToLive {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.StorageMinimumTimeToLive"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_StorageMinimumTimeToLive as StorageMinimumTimeToLive;
    impl crate::value::ToValue for StorageMinimumTimeToLive_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timebasedcollectionscheme.html
    pub struct TimeBasedCollectionScheme_ {
        pub period_ms: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_TimeBasedCollectionScheme {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.TimeBasedCollectionScheme"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_TimeBasedCollectionScheme as TimeBasedCollectionScheme;
    impl crate::value::ToValue for TimeBasedCollectionScheme_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PeriodMs".to_string(),
                crate::value::ToValue::to_value(&self.period_ms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timebasedsignalfetchconfig.html
    pub struct TimeBasedSignalFetchConfig_ {
        pub execution_frequency_ms: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_TimeBasedSignalFetchConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.TimeBasedSignalFetchConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_TimeBasedSignalFetchConfig as TimeBasedSignalFetchConfig;
    impl crate::value::ToValue for TimeBasedSignalFetchConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExecutionFrequencyMs".to_string(),
                crate::value::ToValue::to_value(&self.execution_frequency_ms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-campaign-timestreamconfig.html
    pub struct TimestreamConfig_ {
        pub execution_role_arn: crate::value::ExpString,
        pub timestream_table_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Campaign_TimestreamConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Campaign.TimestreamConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Campaign_TimestreamConfig as TimestreamConfig;
    impl crate::value::ToValue for TimestreamConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExecutionRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.execution_role_arn),
            );
            properties.insert(
                "TimestreamTableArn".to_string(),
                crate::value::ToValue::to_value(&self.timestream_table_arn),
            );
            properties.into()
        }
    }
}
pub mod decodermanifest {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-caninterface.html
    pub struct CanInterface_ {
        pub name: crate::value::ExpString,
        pub protocol_name: Option<crate::value::ExpString>,
        pub protocol_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_CanInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.CanInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_CanInterface as CanInterface;
    impl crate::value::ToValue for CanInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.protocol_name {
                properties.insert(
                    "ProtocolName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol_version {
                properties.insert(
                    "ProtocolVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-cansignal.html
    pub struct CanSignal_ {
        pub factor: crate::value::ExpString,
        pub is_big_endian: crate::value::ExpString,
        pub is_signed: crate::value::ExpString,
        pub length: crate::value::ExpString,
        pub message_id: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
        pub offset: crate::value::ExpString,
        pub signal_value_type: Option<crate::value::ExpString>,
        pub start_bit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_CanSignal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.CanSignal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_CanSignal as CanSignal;
    impl crate::value::ToValue for CanSignal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Factor".to_string(),
                crate::value::ToValue::to_value(&self.factor),
            );
            properties.insert(
                "IsBigEndian".to_string(),
                crate::value::ToValue::to_value(&self.is_big_endian),
            );
            properties.insert(
                "IsSigned".to_string(),
                crate::value::ToValue::to_value(&self.is_signed),
            );
            properties.insert(
                "Length".to_string(),
                crate::value::ToValue::to_value(&self.length),
            );
            properties.insert(
                "MessageId".to_string(),
                crate::value::ToValue::to_value(&self.message_id),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Offset".to_string(),
                crate::value::ToValue::to_value(&self.offset),
            );
            if let Some(ref value) = self.signal_value_type {
                properties.insert(
                    "SignalValueType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StartBit".to_string(),
                crate::value::ToValue::to_value(&self.start_bit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-customdecodinginterface.html
    pub struct CustomDecodingInterface_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_CustomDecodingInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.CustomDecodingInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_CustomDecodingInterface as CustomDecodingInterface;
    impl crate::value::ToValue for CustomDecodingInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-customdecodingsignal.html
    pub struct CustomDecodingSignal_ {
        pub id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_CustomDecodingSignal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.CustomDecodingSignal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_CustomDecodingSignal as CustomDecodingSignal;
    impl crate::value::ToValue for CustomDecodingSignal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-networkinterfacesitems.html
    pub struct NetworkInterfacesItems_ {
        pub can_interface: Option<Box<CanInterface_>>,
        pub custom_decoding_interface: Option<Box<CustomDecodingInterface_>>,
        pub interface_id: crate::value::ExpString,
        pub obd_interface: Option<Box<ObdInterface_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_NetworkInterfacesItems {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.NetworkInterfacesItems"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_NetworkInterfacesItems as NetworkInterfacesItems;
    impl crate::value::ToValue for NetworkInterfacesItems_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.can_interface {
                properties.insert(
                    "CanInterface".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_decoding_interface {
                properties.insert(
                    "CustomDecodingInterface".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InterfaceId".to_string(),
                crate::value::ToValue::to_value(&self.interface_id),
            );
            if let Some(ref value) = self.obd_interface {
                properties.insert(
                    "ObdInterface".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdinterface.html
    pub struct ObdInterface_ {
        pub dtc_request_interval_seconds: Option<crate::value::ExpString>,
        pub has_transmission_ecu: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub obd_standard: Option<crate::value::ExpString>,
        pub pid_request_interval_seconds: Option<crate::value::ExpString>,
        pub request_message_id: crate::value::ExpString,
        pub use_extended_ids: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_ObdInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.ObdInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_ObdInterface as ObdInterface;
    impl crate::value::ToValue for ObdInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dtc_request_interval_seconds {
                properties.insert(
                    "DtcRequestIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.has_transmission_ecu {
                properties.insert(
                    "HasTransmissionEcu".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.obd_standard {
                properties.insert(
                    "ObdStandard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pid_request_interval_seconds {
                properties.insert(
                    "PidRequestIntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RequestMessageId".to_string(),
                crate::value::ToValue::to_value(&self.request_message_id),
            );
            if let Some(ref value) = self.use_extended_ids {
                properties.insert(
                    "UseExtendedIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-obdsignal.html
    pub struct ObdSignal_ {
        pub bit_mask_length: Option<crate::value::ExpString>,
        pub bit_right_shift: Option<crate::value::ExpString>,
        pub byte_length: crate::value::ExpString,
        pub is_signed: Option<crate::value::ExpString>,
        pub offset: crate::value::ExpString,
        pub pid: crate::value::ExpString,
        pub pid_response_length: crate::value::ExpString,
        pub scaling: crate::value::ExpString,
        pub service_mode: crate::value::ExpString,
        pub signal_value_type: Option<crate::value::ExpString>,
        pub start_byte: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_ObdSignal {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.ObdSignal"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_ObdSignal as ObdSignal;
    impl crate::value::ToValue for ObdSignal_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bit_mask_length {
                properties.insert(
                    "BitMaskLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bit_right_shift {
                properties.insert(
                    "BitRightShift".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ByteLength".to_string(),
                crate::value::ToValue::to_value(&self.byte_length),
            );
            if let Some(ref value) = self.is_signed {
                properties.insert(
                    "IsSigned".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Offset".to_string(),
                crate::value::ToValue::to_value(&self.offset),
            );
            properties.insert(
                "Pid".to_string(),
                crate::value::ToValue::to_value(&self.pid),
            );
            properties.insert(
                "PidResponseLength".to_string(),
                crate::value::ToValue::to_value(&self.pid_response_length),
            );
            properties.insert(
                "Scaling".to_string(),
                crate::value::ToValue::to_value(&self.scaling),
            );
            properties.insert(
                "ServiceMode".to_string(),
                crate::value::ToValue::to_value(&self.service_mode),
            );
            if let Some(ref value) = self.signal_value_type {
                properties.insert(
                    "SignalValueType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StartByte".to_string(),
                crate::value::ToValue::to_value(&self.start_byte),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-decodermanifest-signaldecodersitems.html
    pub struct SignalDecodersItems_ {
        pub can_signal: Option<Box<CanSignal_>>,
        pub custom_decoding_signal: Option<Box<CustomDecodingSignal_>>,
        pub fully_qualified_name: crate::value::ExpString,
        pub interface_id: crate::value::ExpString,
        pub obd_signal: Option<Box<ObdSignal_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_DecoderManifest_SignalDecodersItems {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::DecoderManifest.SignalDecodersItems"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_DecoderManifest_SignalDecodersItems as SignalDecodersItems;
    impl crate::value::ToValue for SignalDecodersItems_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.can_signal {
                properties.insert(
                    "CanSignal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_decoding_signal {
                properties.insert(
                    "CustomDecodingSignal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FullyQualifiedName".to_string(),
                crate::value::ToValue::to_value(&self.fully_qualified_name),
            );
            properties.insert(
                "InterfaceId".to_string(),
                crate::value::ToValue::to_value(&self.interface_id),
            );
            if let Some(ref value) = self.obd_signal {
                properties.insert(
                    "ObdSignal".to_string(),
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
}
pub mod signalcatalog {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-actuator.html
    pub struct Actuator_ {
        pub allowed_values: Option<Vec<crate::value::ExpString>>,
        pub assigned_value: Option<crate::value::ExpString>,
        pub data_type: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub fully_qualified_name: crate::value::ExpString,
        pub max: Option<f64>,
        pub min: Option<f64>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_SignalCatalog_Actuator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::SignalCatalog.Actuator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_SignalCatalog_Actuator as Actuator;
    impl crate::value::ToValue for Actuator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_values {
                properties.insert(
                    "AllowedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.assigned_value {
                properties.insert(
                    "AssignedValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DataType".to_string(),
                crate::value::ToValue::to_value(&self.data_type),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FullyQualifiedName".to_string(),
                crate::value::ToValue::to_value(&self.fully_qualified_name),
            );
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-attribute.html
    pub struct Attribute_ {
        pub allowed_values: Option<Vec<crate::value::ExpString>>,
        pub assigned_value: Option<crate::value::ExpString>,
        pub data_type: crate::value::ExpString,
        pub default_value: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub fully_qualified_name: crate::value::ExpString,
        pub max: Option<f64>,
        pub min: Option<f64>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_SignalCatalog_Attribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::SignalCatalog.Attribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_SignalCatalog_Attribute as Attribute;
    impl crate::value::ToValue for Attribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_values {
                properties.insert(
                    "AllowedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.assigned_value {
                properties.insert(
                    "AssignedValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DataType".to_string(),
                crate::value::ToValue::to_value(&self.data_type),
            );
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
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
                "FullyQualifiedName".to_string(),
                crate::value::ToValue::to_value(&self.fully_qualified_name),
            );
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-branch.html
    pub struct Branch_ {
        pub description: Option<crate::value::ExpString>,
        pub fully_qualified_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_SignalCatalog_Branch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::SignalCatalog.Branch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_SignalCatalog_Branch as Branch;
    impl crate::value::ToValue for Branch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FullyQualifiedName".to_string(),
                crate::value::ToValue::to_value(&self.fully_qualified_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-node.html
    pub struct Node_ {
        pub actuator: Option<Box<Actuator_>>,
        pub attribute: Option<Box<Attribute_>>,
        pub branch: Option<Box<Branch_>>,
        pub sensor: Option<Box<Sensor_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_SignalCatalog_Node {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::SignalCatalog.Node"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_SignalCatalog_Node as Node;
    impl crate::value::ToValue for Node_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.actuator {
                properties.insert(
                    "Actuator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attribute {
                properties.insert(
                    "Attribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.branch {
                properties.insert("Branch".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sensor {
                properties.insert("Sensor".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-nodecounts.html
    pub struct NodeCounts_ {
        pub total_actuators: Option<f64>,
        pub total_attributes: Option<f64>,
        pub total_branches: Option<f64>,
        pub total_nodes: Option<f64>,
        pub total_sensors: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_SignalCatalog_NodeCounts {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::SignalCatalog.NodeCounts"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_SignalCatalog_NodeCounts as NodeCounts;
    impl crate::value::ToValue for NodeCounts_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.total_actuators {
                properties.insert(
                    "TotalActuators".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_attributes {
                properties.insert(
                    "TotalAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_branches {
                properties.insert(
                    "TotalBranches".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_nodes {
                properties.insert(
                    "TotalNodes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total_sensors {
                properties.insert(
                    "TotalSensors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-signalcatalog-sensor.html
    pub struct Sensor_ {
        pub allowed_values: Option<Vec<crate::value::ExpString>>,
        pub data_type: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub fully_qualified_name: crate::value::ExpString,
        pub max: Option<f64>,
        pub min: Option<f64>,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_SignalCatalog_Sensor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::SignalCatalog.Sensor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_SignalCatalog_Sensor as Sensor;
    impl crate::value::ToValue for Sensor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_values {
                properties.insert(
                    "AllowedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DataType".to_string(),
                crate::value::ToValue::to_value(&self.data_type),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FullyQualifiedName".to_string(),
                crate::value::ToValue::to_value(&self.fully_qualified_name),
            );
            if let Some(ref value) = self.max {
                properties.insert("Max".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.min {
                properties.insert("Min".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod vehicle {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-vehicle-periodicstatetemplateupdatestrategy.html
    pub struct PeriodicStateTemplateUpdateStrategy_ {
        pub state_template_update_rate: Box<TimePeriod_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Vehicle_PeriodicStateTemplateUpdateStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Vehicle.PeriodicStateTemplateUpdateStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Vehicle_PeriodicStateTemplateUpdateStrategy as PeriodicStateTemplateUpdateStrategy;
    impl crate::value::ToValue for PeriodicStateTemplateUpdateStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StateTemplateUpdateRate".to_string(),
                crate::value::ToValue::to_value(&self.state_template_update_rate),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-vehicle-statetemplateassociation.html
    pub struct StateTemplateAssociation_ {
        pub identifier: crate::value::ExpString,
        pub state_template_update_strategy: Box<StateTemplateUpdateStrategy_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Vehicle_StateTemplateAssociation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Vehicle.StateTemplateAssociation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Vehicle_StateTemplateAssociation as StateTemplateAssociation;
    impl crate::value::ToValue for StateTemplateAssociation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Identifier".to_string(),
                crate::value::ToValue::to_value(&self.identifier),
            );
            properties.insert(
                "StateTemplateUpdateStrategy".to_string(),
                crate::value::ToValue::to_value(&self.state_template_update_strategy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-vehicle-statetemplateupdatestrategy.html
    pub struct StateTemplateUpdateStrategy_ {
        pub on_change: Option<serde_json::Value>,
        pub periodic: Option<Box<PeriodicStateTemplateUpdateStrategy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Vehicle_StateTemplateUpdateStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Vehicle.StateTemplateUpdateStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Vehicle_StateTemplateUpdateStrategy as StateTemplateUpdateStrategy;
    impl crate::value::ToValue for StateTemplateUpdateStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_change {
                properties.insert(
                    "OnChange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.periodic {
                properties.insert(
                    "Periodic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotfleetwise-vehicle-timeperiod.html
    pub struct TimePeriod_ {
        pub unit: crate::value::ExpString,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotfleetwise_Vehicle_TimePeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTFleetWise::Vehicle.TimePeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotfleetwise_Vehicle_TimePeriod as TimePeriod;
    impl crate::value::ToValue for TimePeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-campaign.html
pub struct Campaign_ {
    pub action: Option<crate::value::ExpString>,
    pub collection_scheme: super::iotfleetwise::campaign::CollectionScheme_,
    pub compression: Option<crate::value::ExpString>,
    pub data_destination_configs:
        Option<Vec<super::iotfleetwise::campaign::DataDestinationConfig_>>,
    pub data_extra_dimensions: Option<Vec<crate::value::ExpString>>,
    pub data_partitions: Option<Vec<super::iotfleetwise::campaign::DataPartition_>>,
    pub description: Option<crate::value::ExpString>,
    pub diagnostics_mode: Option<crate::value::ExpString>,
    pub expiry_time: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub post_trigger_collection_duration: Option<f64>,
    pub priority: Option<i32>,
    pub signal_catalog_arn: crate::value::ExpString,
    pub signals_to_collect: Option<Vec<super::iotfleetwise::campaign::SignalInformation_>>,
    pub signals_to_fetch: Option<Vec<super::iotfleetwise::campaign::SignalFetchInformation_>>,
    pub spooling_mode: Option<crate::value::ExpString>,
    pub start_time: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleetwise_Campaign {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetWise::Campaign"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleetwise_Campaign as Campaign;
impl crate::template::ToResource for Campaign_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Campaign"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.action {
            properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "CollectionScheme".to_string(),
            crate::value::ToValue::to_value(&self.collection_scheme),
        );
        if let Some(ref value) = self.compression {
            properties.insert(
                "Compression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_destination_configs {
            properties.insert(
                "DataDestinationConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_extra_dimensions {
            properties.insert(
                "DataExtraDimensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_partitions {
            properties.insert(
                "DataPartitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.diagnostics_mode {
            properties.insert(
                "DiagnosticsMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expiry_time {
            properties.insert(
                "ExpiryTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.post_trigger_collection_duration {
            properties.insert(
                "PostTriggerCollectionDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.priority {
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SignalCatalogArn".to_string(),
            crate::value::ToValue::to_value(&self.signal_catalog_arn),
        );
        if let Some(ref value) = self.signals_to_collect {
            properties.insert(
                "SignalsToCollect".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.signals_to_fetch {
            properties.insert(
                "SignalsToFetch".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.spooling_mode {
            properties.insert(
                "SpoolingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.start_time {
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetArn".to_string(),
            crate::value::ToValue::to_value(&self.target_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-decodermanifest.html
pub struct DecoderManifest_ {
    pub default_for_unmapped_signals: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub model_manifest_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub network_interfaces:
        Option<Vec<super::iotfleetwise::decodermanifest::NetworkInterfacesItems_>>,
    pub signal_decoders: Option<Vec<super::iotfleetwise::decodermanifest::SignalDecodersItems_>>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleetwise_DecoderManifest {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetWise::DecoderManifest"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleetwise_DecoderManifest as DecoderManifest;
impl crate::template::ToResource for DecoderManifest_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DecoderManifest"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.default_for_unmapped_signals {
            properties.insert(
                "DefaultForUnmappedSignals".to_string(),
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
            "ModelManifestArn".to_string(),
            crate::value::ToValue::to_value(&self.model_manifest_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.network_interfaces {
            properties.insert(
                "NetworkInterfaces".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.signal_decoders {
            properties.insert(
                "SignalDecoders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-fleet.html
pub struct Fleet_ {
    pub description: Option<crate::value::ExpString>,
    pub id: crate::value::ExpString,
    pub signal_catalog_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleetwise_Fleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetWise::Fleet"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleetwise_Fleet as Fleet;
impl crate::template::ToResource for Fleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Fleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
        properties.insert(
            "SignalCatalogArn".to_string(),
            crate::value::ToValue::to_value(&self.signal_catalog_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-modelmanifest.html
pub struct ModelManifest_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub nodes: Option<Vec<crate::value::ExpString>>,
    pub signal_catalog_arn: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleetwise_ModelManifest {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetWise::ModelManifest"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleetwise_ModelManifest as ModelManifest;
impl crate::template::ToResource for ModelManifest_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ModelManifest"),
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
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.nodes {
            properties.insert("Nodes".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "SignalCatalogArn".to_string(),
            crate::value::ToValue::to_value(&self.signal_catalog_arn),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-signalcatalog.html
pub struct SignalCatalog_ {
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub node_counts: Option<super::iotfleetwise::signalcatalog::NodeCounts_>,
    pub nodes: Option<Vec<super::iotfleetwise::signalcatalog::Node_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleetwise_SignalCatalog {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetWise::SignalCatalog"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleetwise_SignalCatalog as SignalCatalog;
impl crate::template::ToResource for SignalCatalog_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SignalCatalog"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.node_counts {
            properties.insert(
                "NodeCounts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.nodes {
            properties.insert("Nodes".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-statetemplate.html
pub struct StateTemplate_ {
    pub data_extra_dimensions: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub metadata_extra_dimensions: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub signal_catalog_arn: crate::value::ExpString,
    pub state_template_properties: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleetwise_StateTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetWise::StateTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleetwise_StateTemplate as StateTemplate;
impl crate::template::ToResource for StateTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StateTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_extra_dimensions {
            properties.insert(
                "DataExtraDimensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metadata_extra_dimensions {
            properties.insert(
                "MetadataExtraDimensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "SignalCatalogArn".to_string(),
            crate::value::ToValue::to_value(&self.signal_catalog_arn),
        );
        properties.insert(
            "StateTemplateProperties".to_string(),
            crate::value::ToValue::to_value(&self.state_template_properties),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotfleetwise-vehicle.html
pub struct Vehicle_ {
    pub association_behavior: Option<crate::value::ExpString>,
    pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub decoder_manifest_arn: crate::value::ExpString,
    pub model_manifest_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub state_templates: Option<Vec<super::iotfleetwise::vehicle::StateTemplateAssociation_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotfleetwise_Vehicle {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTFleetWise::Vehicle"
        $($field $value)*)
    };
}
pub use crate::__aws_iotfleetwise_Vehicle as Vehicle;
impl crate::template::ToResource for Vehicle_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTFleetWise"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Vehicle"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.association_behavior {
            properties.insert(
                "AssociationBehavior".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.attributes {
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DecoderManifestArn".to_string(),
            crate::value::ToValue::to_value(&self.decoder_manifest_arn),
        );
        properties.insert(
            "ModelManifestArn".to_string(),
            crate::value::ToValue::to_value(&self.model_manifest_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.state_templates {
            properties.insert(
                "StateTemplates".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
