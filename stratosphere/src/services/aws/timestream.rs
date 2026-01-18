pub mod influxdbinstance {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-influxdbinstance-logdeliveryconfiguration.html
    pub struct LogDeliveryConfiguration_ {
        pub s3_configuration: Box<S3Configuration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_InfluxDBInstance_LogDeliveryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::InfluxDBInstance.LogDeliveryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_InfluxDBInstance_LogDeliveryConfiguration as LogDeliveryConfiguration;
    impl crate::value::ToValue for LogDeliveryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-influxdbinstance-s3configuration.html
    pub struct S3Configuration_ {
        pub bucket_name: crate::value::ExpString,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_InfluxDBInstance_S3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::InfluxDBInstance.S3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_InfluxDBInstance_S3Configuration as S3Configuration;
    impl crate::value::ToValue for S3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
}
pub mod scheduledquery {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-dimensionmapping.html
    pub struct DimensionMapping_ {
        pub dimension_value_type: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_DimensionMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.DimensionMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_DimensionMapping as DimensionMapping;
    impl crate::value::ToValue for DimensionMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionValueType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_value_type),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-errorreportconfiguration.html
    pub struct ErrorReportConfiguration_ {
        pub s3_configuration: Box<S3Configuration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_ErrorReportConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.ErrorReportConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_ErrorReportConfiguration as ErrorReportConfiguration;
    impl crate::value::ToValue for ErrorReportConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-mixedmeasuremapping.html
    pub struct MixedMeasureMapping_ {
        pub measure_name: Option<crate::value::ExpString>,
        pub measure_value_type: crate::value::ExpString,
        pub multi_measure_attribute_mappings: Option<Vec<MultiMeasureAttributeMapping_>>,
        pub source_column: Option<crate::value::ExpString>,
        pub target_measure_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_MixedMeasureMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.MixedMeasureMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_MixedMeasureMapping as MixedMeasureMapping;
    impl crate::value::ToValue for MixedMeasureMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.measure_name {
                properties.insert(
                    "MeasureName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MeasureValueType".to_string(),
                crate::value::ToValue::to_value(&self.measure_value_type),
            );
            if let Some(ref value) = self.multi_measure_attribute_mappings {
                properties.insert(
                    "MultiMeasureAttributeMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_column {
                properties.insert(
                    "SourceColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_measure_name {
                properties.insert(
                    "TargetMeasureName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-multimeasureattributemapping.html
    pub struct MultiMeasureAttributeMapping_ {
        pub measure_value_type: crate::value::ExpString,
        pub source_column: crate::value::ExpString,
        pub target_multi_measure_attribute_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_MultiMeasureAttributeMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.MultiMeasureAttributeMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_MultiMeasureAttributeMapping as MultiMeasureAttributeMapping;
    impl crate::value::ToValue for MultiMeasureAttributeMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MeasureValueType".to_string(),
                crate::value::ToValue::to_value(&self.measure_value_type),
            );
            properties.insert(
                "SourceColumn".to_string(),
                crate::value::ToValue::to_value(&self.source_column),
            );
            if let Some(ref value) = self.target_multi_measure_attribute_name {
                properties.insert(
                    "TargetMultiMeasureAttributeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-multimeasuremappings.html
    pub struct MultiMeasureMappings_ {
        pub multi_measure_attribute_mappings: Vec<MultiMeasureAttributeMapping_>,
        pub target_multi_measure_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_MultiMeasureMappings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.MultiMeasureMappings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_MultiMeasureMappings as MultiMeasureMappings;
    impl crate::value::ToValue for MultiMeasureMappings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MultiMeasureAttributeMappings".to_string(),
                crate::value::ToValue::to_value(&self.multi_measure_attribute_mappings),
            );
            if let Some(ref value) = self.target_multi_measure_name {
                properties.insert(
                    "TargetMultiMeasureName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-notificationconfiguration.html
    pub struct NotificationConfiguration_ {
        pub sns_configuration: Box<SnsConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_NotificationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.NotificationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_NotificationConfiguration as NotificationConfiguration;
    impl crate::value::ToValue for NotificationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SnsConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.sns_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-s3configuration.html
    pub struct S3Configuration_ {
        pub bucket_name: crate::value::ExpString,
        pub encryption_option: Option<crate::value::ExpString>,
        pub object_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_S3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.S3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_S3Configuration as S3Configuration;
    impl crate::value::ToValue for S3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.encryption_option {
                properties.insert(
                    "EncryptionOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.object_key_prefix {
                properties.insert(
                    "ObjectKeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-scheduleconfiguration.html
    pub struct ScheduleConfiguration_ {
        pub schedule_expression: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_ScheduleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.ScheduleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_ScheduleConfiguration as ScheduleConfiguration;
    impl crate::value::ToValue for ScheduleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(&self.schedule_expression),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-snsconfiguration.html
    pub struct SnsConfiguration_ {
        pub topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_SnsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.SnsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_SnsConfiguration as SnsConfiguration;
    impl crate::value::ToValue for SnsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TopicArn".to_string(),
                crate::value::ToValue::to_value(&self.topic_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-targetconfiguration.html
    pub struct TargetConfiguration_ {
        pub timestream_configuration: Box<TimestreamConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_TargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.TargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_TargetConfiguration as TargetConfiguration;
    impl crate::value::ToValue for TargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TimestreamConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.timestream_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-scheduledquery-timestreamconfiguration.html
    pub struct TimestreamConfiguration_ {
        pub database_name: crate::value::ExpString,
        pub dimension_mappings: Vec<DimensionMapping_>,
        pub measure_name_column: Option<crate::value::ExpString>,
        pub mixed_measure_mappings: Option<Vec<MixedMeasureMapping_>>,
        pub multi_measure_mappings: Option<Box<MultiMeasureMappings_>>,
        pub table_name: crate::value::ExpString,
        pub time_column: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_ScheduledQuery_TimestreamConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::ScheduledQuery.TimestreamConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_ScheduledQuery_TimestreamConfiguration as TimestreamConfiguration;
    impl crate::value::ToValue for TimestreamConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "DimensionMappings".to_string(),
                crate::value::ToValue::to_value(&self.dimension_mappings),
            );
            if let Some(ref value) = self.measure_name_column {
                properties.insert(
                    "MeasureNameColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mixed_measure_mappings {
                properties.insert(
                    "MixedMeasureMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multi_measure_mappings {
                properties.insert(
                    "MultiMeasureMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.insert(
                "TimeColumn".to_string(),
                crate::value::ToValue::to_value(&self.time_column),
            );
            properties.into()
        }
    }
}
pub mod table {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-table-magneticstorerejecteddatalocation.html
    pub struct MagneticStoreRejectedDataLocation_ {
        pub s3_configuration: Option<Box<S3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_Table_MagneticStoreRejectedDataLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::Table.MagneticStoreRejectedDataLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_Table_MagneticStoreRejectedDataLocation as MagneticStoreRejectedDataLocation;
    impl crate::value::ToValue for MagneticStoreRejectedDataLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_configuration {
                properties.insert(
                    "S3Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-table-magneticstorewriteproperties.html
    pub struct MagneticStoreWriteProperties_ {
        pub enable_magnetic_store_writes: crate::value::ExpBool,
        pub magnetic_store_rejected_data_location: Option<Box<MagneticStoreRejectedDataLocation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_Table_MagneticStoreWriteProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::Table.MagneticStoreWriteProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_Table_MagneticStoreWriteProperties as MagneticStoreWriteProperties;
    impl crate::value::ToValue for MagneticStoreWriteProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnableMagneticStoreWrites".to_string(),
                crate::value::ToValue::to_value(&self.enable_magnetic_store_writes),
            );
            if let Some(ref value) = self.magnetic_store_rejected_data_location {
                properties.insert(
                    "MagneticStoreRejectedDataLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-table-partitionkey.html
    pub struct PartitionKey_ {
        pub enforcement_in_record: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_Table_PartitionKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::Table.PartitionKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_Table_PartitionKey as PartitionKey;
    impl crate::value::ToValue for PartitionKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enforcement_in_record {
                properties.insert(
                    "EnforcementInRecord".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-table-retentionproperties.html
    pub struct RetentionProperties_ {
        pub magnetic_store_retention_period_in_days: Option<crate::value::ExpString>,
        pub memory_store_retention_period_in_hours: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_Table_RetentionProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::Table.RetentionProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_Table_RetentionProperties as RetentionProperties;
    impl crate::value::ToValue for RetentionProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.magnetic_store_retention_period_in_days {
                properties.insert(
                    "MagneticStoreRetentionPeriodInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_store_retention_period_in_hours {
                properties.insert(
                    "MemoryStoreRetentionPeriodInHours".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-table-s3configuration.html
    pub struct S3Configuration_ {
        pub bucket_name: crate::value::ExpString,
        pub encryption_option: crate::value::ExpString,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub object_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_Table_S3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::Table.S3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_Table_S3Configuration as S3Configuration;
    impl crate::value::ToValue for S3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "EncryptionOption".to_string(),
                crate::value::ToValue::to_value(&self.encryption_option),
            );
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.object_key_prefix {
                properties.insert(
                    "ObjectKeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-timestream-table-schema.html
    pub struct Schema_ {
        pub composite_partition_key: Option<Vec<PartitionKey_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_timestream_Table_Schema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Timestream::Table.Schema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_timestream_Table_Schema as Schema;
    impl crate::value::ToValue for Schema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.composite_partition_key {
                properties.insert(
                    "CompositePartitionKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-timestream-database.html
pub struct Database_ {
    pub database_name: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_timestream_Database {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Timestream::Database"
        $($field $value)*)
    };
}
pub use crate::__aws_timestream_Database as Database;
impl crate::template::ToResource for Database_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Timestream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Database"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.database_name {
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-timestream-influxdbinstance.html
pub struct InfluxDBInstance_ {
    pub allocated_storage: Option<i64>,
    pub bucket: Option<crate::value::ExpString>,
    pub db_instance_type: Option<crate::value::ExpString>,
    pub db_parameter_group_identifier: Option<crate::value::ExpString>,
    pub db_storage_type: Option<crate::value::ExpString>,
    pub deployment_type: Option<crate::value::ExpString>,
    pub log_delivery_configuration:
        Option<super::timestream::influxdbinstance::LogDeliveryConfiguration_>,
    pub name: Option<crate::value::ExpString>,
    pub network_type: Option<crate::value::ExpString>,
    pub organization: Option<crate::value::ExpString>,
    pub password: Option<crate::value::ExpString>,
    pub port: Option<i64>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub username: Option<crate::value::ExpString>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub vpc_subnet_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_timestream_InfluxDBInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Timestream::InfluxDBInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_timestream_InfluxDBInstance as InfluxDBInstance;
impl crate::template::ToResource for InfluxDBInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Timestream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InfluxDBInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allocated_storage {
            properties.insert(
                "AllocatedStorage".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bucket {
            properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.db_instance_type {
            properties.insert(
                "DbInstanceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_parameter_group_identifier {
            properties.insert(
                "DbParameterGroupIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_storage_type {
            properties.insert(
                "DbStorageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_type {
            properties.insert(
                "DeploymentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_delivery_configuration {
            properties.insert(
                "LogDeliveryConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.organization {
            properties.insert(
                "Organization".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.password {
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.username {
            properties.insert(
                "Username".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_security_group_ids {
            properties.insert(
                "VpcSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_subnet_ids {
            properties.insert(
                "VpcSubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-timestream-scheduledquery.html
pub struct ScheduledQuery_ {
    pub client_token: Option<crate::value::ExpString>,
    pub error_report_configuration: super::timestream::scheduledquery::ErrorReportConfiguration_,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub notification_configuration: super::timestream::scheduledquery::NotificationConfiguration_,
    pub query_string: crate::value::ExpString,
    pub schedule_configuration: super::timestream::scheduledquery::ScheduleConfiguration_,
    pub scheduled_query_execution_role_arn: crate::value::ExpString,
    pub scheduled_query_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_configuration: Option<super::timestream::scheduledquery::TargetConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_timestream_ScheduledQuery {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Timestream::ScheduledQuery"
        $($field $value)*)
    };
}
pub use crate::__aws_timestream_ScheduledQuery as ScheduledQuery;
impl crate::template::ToResource for ScheduledQuery_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Timestream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScheduledQuery"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.client_token {
            properties.insert(
                "ClientToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ErrorReportConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.error_report_configuration),
        );
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NotificationConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.notification_configuration),
        );
        properties.insert(
            "QueryString".to_string(),
            crate::value::ToValue::to_value(&self.query_string),
        );
        properties.insert(
            "ScheduleConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.schedule_configuration),
        );
        properties.insert(
            "ScheduledQueryExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.scheduled_query_execution_role_arn),
        );
        if let Some(ref value) = self.scheduled_query_name {
            properties.insert(
                "ScheduledQueryName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_configuration {
            properties.insert(
                "TargetConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-timestream-table.html
pub struct Table_ {
    pub database_name: crate::value::ExpString,
    pub magnetic_store_write_properties:
        Option<super::timestream::table::MagneticStoreWriteProperties_>,
    pub retention_properties: Option<super::timestream::table::RetentionProperties_>,
    pub schema: Option<super::timestream::table::Schema_>,
    pub table_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_timestream_Table {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Timestream::Table"
        $($field $value)*)
    };
}
pub use crate::__aws_timestream_Table as Table;
impl crate::template::ToResource for Table_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Timestream"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Table"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.database_name),
        );
        if let Some(ref value) = self.magnetic_store_write_properties {
            properties.insert(
                "MagneticStoreWriteProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_properties {
            properties.insert(
                "RetentionProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schema {
            properties.insert("Schema".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.table_name {
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
