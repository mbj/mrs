pub mod deliverystream {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessbufferinghints.html>
    pub struct AmazonOpenSearchServerlessBufferingHints_ {
        pub interval_in_seconds: Option<i32>,
        pub size_in_m_bs: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_AmazonOpenSearchServerlessBufferingHints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.AmazonOpenSearchServerlessBufferingHints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_AmazonOpenSearchServerlessBufferingHints as AmazonOpenSearchServerlessBufferingHints;
    impl crate::value::ToValue for AmazonOpenSearchServerlessBufferingHints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interval_in_seconds {
                properties.insert(
                    "IntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_m_bs {
                properties.insert(
                    "SizeInMBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessdestinationconfiguration.html>
    pub struct AmazonOpenSearchServerlessDestinationConfiguration_ {
        pub buffering_hints: Option<Box<AmazonOpenSearchServerlessBufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub collection_endpoint: Option<crate::value::ExpString>,
        pub index_name: crate::value::ExpString,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub retry_options: Option<Box<AmazonOpenSearchServerlessRetryOptions_>>,
        pub role_arn: crate::value::ExpString,
        pub s3_backup_mode: Option<crate::value::ExpString>,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub vpc_configuration: Option<Box<VpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_AmazonOpenSearchServerlessDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.AmazonOpenSearchServerlessDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_AmazonOpenSearchServerlessDestinationConfiguration as AmazonOpenSearchServerlessDestinationConfiguration;
    impl crate::value::ToValue for AmazonOpenSearchServerlessDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.collection_endpoint {
                properties.insert(
                    "CollectionEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserverlessretryoptions.html>
    pub struct AmazonOpenSearchServerlessRetryOptions_ {
        pub duration_in_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_AmazonOpenSearchServerlessRetryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.AmazonOpenSearchServerlessRetryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_AmazonOpenSearchServerlessRetryOptions as AmazonOpenSearchServerlessRetryOptions;
    impl crate::value::ToValue for AmazonOpenSearchServerlessRetryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicebufferinghints.html>
    pub struct AmazonopensearchserviceBufferingHints_ {
        pub interval_in_seconds: Option<i32>,
        pub size_in_m_bs: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_AmazonopensearchserviceBufferingHints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.AmazonopensearchserviceBufferingHints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_AmazonopensearchserviceBufferingHints as AmazonopensearchserviceBufferingHints;
    impl crate::value::ToValue for AmazonopensearchserviceBufferingHints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interval_in_seconds {
                properties.insert(
                    "IntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_m_bs {
                properties.insert(
                    "SizeInMBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchservicedestinationconfiguration.html>
    pub struct AmazonopensearchserviceDestinationConfiguration_ {
        pub buffering_hints: Option<Box<AmazonopensearchserviceBufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub cluster_endpoint: Option<crate::value::ExpString>,
        pub document_id_options: Option<Box<DocumentIdOptions_>>,
        pub domain_arn: Option<crate::value::ExpString>,
        pub index_name: crate::value::ExpString,
        pub index_rotation_period: Option<crate::value::ExpString>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub retry_options: Option<Box<AmazonopensearchserviceRetryOptions_>>,
        pub role_arn: crate::value::ExpString,
        pub s3_backup_mode: Option<crate::value::ExpString>,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub type_name: Option<crate::value::ExpString>,
        pub vpc_configuration: Option<Box<VpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_AmazonopensearchserviceDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.AmazonopensearchserviceDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_AmazonopensearchserviceDestinationConfiguration as AmazonopensearchserviceDestinationConfiguration;
    impl crate::value::ToValue for AmazonopensearchserviceDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cluster_endpoint {
                properties.insert(
                    "ClusterEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_id_options {
                properties.insert(
                    "DocumentIdOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_arn {
                properties.insert(
                    "DomainARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            if let Some(ref value) = self.index_rotation_period {
                properties.insert(
                    "IndexRotationPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            if let Some(ref value) = self.type_name {
                properties.insert(
                    "TypeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-amazonopensearchserviceretryoptions.html>
    pub struct AmazonopensearchserviceRetryOptions_ {
        pub duration_in_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_AmazonopensearchserviceRetryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.AmazonopensearchserviceRetryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_AmazonopensearchserviceRetryOptions as AmazonopensearchserviceRetryOptions;
    impl crate::value::ToValue for AmazonopensearchserviceRetryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-authenticationconfiguration.html>
    pub struct AuthenticationConfiguration_ {
        pub connectivity: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_AuthenticationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.AuthenticationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_AuthenticationConfiguration as AuthenticationConfiguration;
    impl crate::value::ToValue for AuthenticationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Connectivity".to_string(),
                crate::value::ToValue::to_value(&self.connectivity),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-bufferinghints.html>
    pub struct BufferingHints_ {
        pub interval_in_seconds: Option<i32>,
        pub size_in_m_bs: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_BufferingHints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.BufferingHints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_BufferingHints as BufferingHints;
    impl crate::value::ToValue for BufferingHints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interval_in_seconds {
                properties.insert(
                    "IntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_m_bs {
                properties.insert(
                    "SizeInMBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-catalogconfiguration.html>
    pub struct CatalogConfiguration_ {
        pub catalog_arn: Option<crate::value::ExpString>,
        pub warehouse_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_CatalogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.CatalogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_CatalogConfiguration as CatalogConfiguration;
    impl crate::value::ToValue for CatalogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_arn {
                properties.insert(
                    "CatalogArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.warehouse_location {
                properties.insert(
                    "WarehouseLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-cloudwatchloggingoptions.html>
    pub struct CloudWatchLoggingOptions_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub log_group_name: Option<crate::value::ExpString>,
        pub log_stream_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_CloudWatchLoggingOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.CloudWatchLoggingOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_CloudWatchLoggingOptions as CloudWatchLoggingOptions;
    impl crate::value::ToValue for CloudWatchLoggingOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_stream_name {
                properties.insert(
                    "LogStreamName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-copycommand.html>
    pub struct CopyCommand_ {
        pub copy_options: Option<crate::value::ExpString>,
        pub data_table_columns: Option<crate::value::ExpString>,
        pub data_table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_CopyCommand {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.CopyCommand"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_CopyCommand as CopyCommand;
    impl crate::value::ToValue for CopyCommand_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copy_options {
                properties.insert(
                    "CopyOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_table_columns {
                properties.insert(
                    "DataTableColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DataTableName".to_string(),
                crate::value::ToValue::to_value(&self.data_table_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dataformatconversionconfiguration.html>
    pub struct DataFormatConversionConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub input_format_configuration: Option<Box<InputFormatConfiguration_>>,
        pub output_format_configuration: Option<Box<OutputFormatConfiguration_>>,
        pub schema_configuration: Option<Box<SchemaConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DataFormatConversionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DataFormatConversionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DataFormatConversionConfiguration as DataFormatConversionConfiguration;
    impl crate::value::ToValue for DataFormatConversionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_format_configuration {
                properties.insert(
                    "InputFormatConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_format_configuration {
                properties.insert(
                    "OutputFormatConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema_configuration {
                properties.insert(
                    "SchemaConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-databasecolumns.html>
    pub struct DatabaseColumns_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DatabaseColumns {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DatabaseColumns"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DatabaseColumns as DatabaseColumns;
    impl crate::value::ToValue for DatabaseColumns_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-databasesourceauthenticationconfiguration.html>
    pub struct DatabaseSourceAuthenticationConfiguration_ {
        pub secrets_manager_configuration: Box<SecretsManagerConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DatabaseSourceAuthenticationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DatabaseSourceAuthenticationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DatabaseSourceAuthenticationConfiguration as DatabaseSourceAuthenticationConfiguration;
    impl crate::value::ToValue for DatabaseSourceAuthenticationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretsManagerConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.secrets_manager_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-databasesourceconfiguration.html>
    pub struct DatabaseSourceConfiguration_ {
        pub columns: Option<Box<DatabaseColumns_>>,
        pub database_source_authentication_configuration:
            Box<DatabaseSourceAuthenticationConfiguration_>,
        pub database_source_vpc_configuration: Box<DatabaseSourceVPCConfiguration_>,
        pub databases: Box<Databases_>,
        pub digest: Option<crate::value::ExpString>,
        pub endpoint: crate::value::ExpString,
        pub port: i32,
        pub public_certificate: Option<crate::value::ExpString>,
        pub ssl_mode: Option<crate::value::ExpString>,
        pub snapshot_watermark_table: crate::value::ExpString,
        pub surrogate_keys: Option<Vec<crate::value::ExpString>>,
        pub tables: Box<DatabaseTables_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DatabaseSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DatabaseSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DatabaseSourceConfiguration as DatabaseSourceConfiguration;
    impl crate::value::ToValue for DatabaseSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.columns {
                properties.insert(
                    "Columns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseSourceAuthenticationConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.database_source_authentication_configuration),
            );
            properties.insert(
                "DatabaseSourceVPCConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.database_source_vpc_configuration),
            );
            properties.insert(
                "Databases".to_string(),
                crate::value::ToValue::to_value(&self.databases),
            );
            if let Some(ref value) = self.digest {
                properties.insert("Digest".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            if let Some(ref value) = self.public_certificate {
                properties.insert(
                    "PublicCertificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssl_mode {
                properties.insert(
                    "SSLMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SnapshotWatermarkTable".to_string(),
                crate::value::ToValue::to_value(&self.snapshot_watermark_table),
            );
            if let Some(ref value) = self.surrogate_keys {
                properties.insert(
                    "SurrogateKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Tables".to_string(),
                crate::value::ToValue::to_value(&self.tables),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-databasesourcevpcconfiguration.html>
    pub struct DatabaseSourceVPCConfiguration_ {
        pub vpc_endpoint_service_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DatabaseSourceVPCConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DatabaseSourceVPCConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DatabaseSourceVPCConfiguration as DatabaseSourceVPCConfiguration;
    impl crate::value::ToValue for DatabaseSourceVPCConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VpcEndpointServiceName".to_string(),
                crate::value::ToValue::to_value(&self.vpc_endpoint_service_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-databasetables.html>
    pub struct DatabaseTables_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DatabaseTables {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DatabaseTables"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DatabaseTables as DatabaseTables;
    impl crate::value::ToValue for DatabaseTables_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-databases.html>
    pub struct Databases_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_Databases {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.Databases"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_Databases as Databases;
    impl crate::value::ToValue for Databases_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deliverystreamencryptionconfigurationinput.html>
    pub struct DeliveryStreamEncryptionConfigurationInput_ {
        pub key_arn: Option<crate::value::ExpString>,
        pub key_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DeliveryStreamEncryptionConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DeliveryStreamEncryptionConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DeliveryStreamEncryptionConfigurationInput as DeliveryStreamEncryptionConfigurationInput;
    impl crate::value::ToValue for DeliveryStreamEncryptionConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_arn {
                properties.insert("KeyARN".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "KeyType".to_string(),
                crate::value::ToValue::to_value(&self.key_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-deserializer.html>
    pub struct Deserializer_ {
        pub hive_json_ser_de: Option<Box<HiveJsonSerDe_>>,
        pub open_x_json_ser_de: Option<Box<OpenXJsonSerDe_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_Deserializer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.Deserializer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_Deserializer as Deserializer;
    impl crate::value::ToValue for Deserializer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hive_json_ser_de {
                properties.insert(
                    "HiveJsonSerDe".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.open_x_json_ser_de {
                properties.insert(
                    "OpenXJsonSerDe".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-destinationtableconfiguration.html>
    pub struct DestinationTableConfiguration_ {
        pub destination_database_name: crate::value::ExpString,
        pub destination_table_name: crate::value::ExpString,
        pub partition_spec: Option<Box<PartitionSpec_>>,
        pub s3_error_output_prefix: Option<crate::value::ExpString>,
        pub unique_keys: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DestinationTableConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DestinationTableConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DestinationTableConfiguration as DestinationTableConfiguration;
    impl crate::value::ToValue for DestinationTableConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationDatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.destination_database_name),
            );
            properties.insert(
                "DestinationTableName".to_string(),
                crate::value::ToValue::to_value(&self.destination_table_name),
            );
            if let Some(ref value) = self.partition_spec {
                properties.insert(
                    "PartitionSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_error_output_prefix {
                properties.insert(
                    "S3ErrorOutputPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unique_keys {
                properties.insert(
                    "UniqueKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-directputsourceconfiguration.html>
    pub struct DirectPutSourceConfiguration_ {
        pub throughput_hint_in_m_bs: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DirectPutSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DirectPutSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DirectPutSourceConfiguration as DirectPutSourceConfiguration;
    impl crate::value::ToValue for DirectPutSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.throughput_hint_in_m_bs {
                properties.insert(
                    "ThroughputHintInMBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-documentidoptions.html>
    pub struct DocumentIdOptions_ {
        pub default_document_id_format: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DocumentIdOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DocumentIdOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DocumentIdOptions as DocumentIdOptions;
    impl crate::value::ToValue for DocumentIdOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultDocumentIdFormat".to_string(),
                crate::value::ToValue::to_value(&self.default_document_id_format),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-dynamicpartitioningconfiguration.html>
    pub struct DynamicPartitioningConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub retry_options: Option<Box<RetryOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_DynamicPartitioningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.DynamicPartitioningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_DynamicPartitioningConfiguration as DynamicPartitioningConfiguration;
    impl crate::value::ToValue for DynamicPartitioningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchbufferinghints.html>
    pub struct ElasticsearchBufferingHints_ {
        pub interval_in_seconds: Option<i32>,
        pub size_in_m_bs: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_ElasticsearchBufferingHints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.ElasticsearchBufferingHints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_ElasticsearchBufferingHints as ElasticsearchBufferingHints;
    impl crate::value::ToValue for ElasticsearchBufferingHints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interval_in_seconds {
                properties.insert(
                    "IntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_m_bs {
                properties.insert(
                    "SizeInMBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchdestinationconfiguration.html>
    pub struct ElasticsearchDestinationConfiguration_ {
        pub buffering_hints: Option<Box<ElasticsearchBufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub cluster_endpoint: Option<crate::value::ExpString>,
        pub document_id_options: Option<Box<DocumentIdOptions_>>,
        pub domain_arn: Option<crate::value::ExpString>,
        pub index_name: crate::value::ExpString,
        pub index_rotation_period: Option<crate::value::ExpString>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub retry_options: Option<Box<ElasticsearchRetryOptions_>>,
        pub role_arn: crate::value::ExpString,
        pub s3_backup_mode: Option<crate::value::ExpString>,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub type_name: Option<crate::value::ExpString>,
        pub vpc_configuration: Option<Box<VpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_ElasticsearchDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.ElasticsearchDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_ElasticsearchDestinationConfiguration as ElasticsearchDestinationConfiguration;
    impl crate::value::ToValue for ElasticsearchDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cluster_endpoint {
                properties.insert(
                    "ClusterEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_id_options {
                properties.insert(
                    "DocumentIdOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_arn {
                properties.insert(
                    "DomainARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            if let Some(ref value) = self.index_rotation_period {
                properties.insert(
                    "IndexRotationPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            if let Some(ref value) = self.type_name {
                properties.insert(
                    "TypeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-elasticsearchretryoptions.html>
    pub struct ElasticsearchRetryOptions_ {
        pub duration_in_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_ElasticsearchRetryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.ElasticsearchRetryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_ElasticsearchRetryOptions as ElasticsearchRetryOptions;
    impl crate::value::ToValue for ElasticsearchRetryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-encryptionconfiguration.html>
    pub struct EncryptionConfiguration_ {
        pub kms_encryption_config: Option<Box<KMSEncryptionConfig_>>,
        pub no_encryption_config: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_encryption_config {
                properties.insert(
                    "KMSEncryptionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_encryption_config {
                properties.insert(
                    "NoEncryptionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-extendeds3destinationconfiguration.html>
    pub struct ExtendedS3DestinationConfiguration_ {
        pub bucket_arn: crate::value::ExpString,
        pub buffering_hints: Option<Box<BufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub compression_format: Option<crate::value::ExpString>,
        pub custom_time_zone: Option<crate::value::ExpString>,
        pub data_format_conversion_configuration: Option<Box<DataFormatConversionConfiguration_>>,
        pub dynamic_partitioning_configuration: Option<Box<DynamicPartitioningConfiguration_>>,
        pub encryption_configuration: Option<Box<EncryptionConfiguration_>>,
        pub error_output_prefix: Option<crate::value::ExpString>,
        pub file_extension: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub role_arn: crate::value::ExpString,
        pub s3_backup_configuration: Option<Box<S3DestinationConfiguration_>>,
        pub s3_backup_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_ExtendedS3DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.ExtendedS3DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_ExtendedS3DestinationConfiguration as ExtendedS3DestinationConfiguration;
    impl crate::value::ToValue for ExtendedS3DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketARN".to_string(),
                crate::value::ToValue::to_value(&self.bucket_arn),
            );
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compression_format {
                properties.insert(
                    "CompressionFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_time_zone {
                properties.insert(
                    "CustomTimeZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_format_conversion_configuration {
                properties.insert(
                    "DataFormatConversionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamic_partitioning_configuration {
                properties.insert(
                    "DynamicPartitioningConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_output_prefix {
                properties.insert(
                    "ErrorOutputPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_extension {
                properties.insert(
                    "FileExtension".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.s3_backup_configuration {
                properties.insert(
                    "S3BackupConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-hivejsonserde.html>
    pub struct HiveJsonSerDe_ {
        pub timestamp_formats: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_HiveJsonSerDe {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.HiveJsonSerDe"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_HiveJsonSerDe as HiveJsonSerDe;
    impl crate::value::ToValue for HiveJsonSerDe_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.timestamp_formats {
                properties.insert(
                    "TimestampFormats".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointcommonattribute.html>
    pub struct HttpEndpointCommonAttribute_ {
        pub attribute_name: crate::value::ExpString,
        pub attribute_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_HttpEndpointCommonAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.HttpEndpointCommonAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_HttpEndpointCommonAttribute as HttpEndpointCommonAttribute;
    impl crate::value::ToValue for HttpEndpointCommonAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            properties.insert(
                "AttributeValue".to_string(),
                crate::value::ToValue::to_value(&self.attribute_value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointconfiguration.html>
    pub struct HttpEndpointConfiguration_ {
        pub access_key: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_HttpEndpointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.HttpEndpointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_HttpEndpointConfiguration as HttpEndpointConfiguration;
    impl crate::value::ToValue for HttpEndpointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_key {
                properties.insert(
                    "AccessKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointdestinationconfiguration.html>
    pub struct HttpEndpointDestinationConfiguration_ {
        pub buffering_hints: Option<Box<BufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub endpoint_configuration: Box<HttpEndpointConfiguration_>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub request_configuration: Option<Box<HttpEndpointRequestConfiguration_>>,
        pub retry_options: Option<Box<RetryOptions_>>,
        pub role_arn: Option<crate::value::ExpString>,
        pub s3_backup_mode: Option<crate::value::ExpString>,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub secrets_manager_configuration: Option<Box<SecretsManagerConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_HttpEndpointDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.HttpEndpointDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_HttpEndpointDestinationConfiguration as HttpEndpointDestinationConfiguration;
    impl crate::value::ToValue for HttpEndpointDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EndpointConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.endpoint_configuration),
            );
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.request_configuration {
                properties.insert(
                    "RequestConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            if let Some(ref value) = self.secrets_manager_configuration {
                properties.insert(
                    "SecretsManagerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-httpendpointrequestconfiguration.html>
    pub struct HttpEndpointRequestConfiguration_ {
        pub common_attributes: Option<Vec<HttpEndpointCommonAttribute_>>,
        pub content_encoding: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_HttpEndpointRequestConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.HttpEndpointRequestConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_HttpEndpointRequestConfiguration as HttpEndpointRequestConfiguration;
    impl crate::value::ToValue for HttpEndpointRequestConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.common_attributes {
                properties.insert(
                    "CommonAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_encoding {
                properties.insert(
                    "ContentEncoding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-icebergdestinationconfiguration.html>
    pub struct IcebergDestinationConfiguration_ {
        pub append_only: Option<crate::value::ExpBool>,
        pub buffering_hints: Option<Box<BufferingHints_>>,
        pub catalog_configuration: Box<CatalogConfiguration_>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub destination_table_configuration_list: Option<Vec<DestinationTableConfiguration_>>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub retry_options: Option<Box<RetryOptions_>>,
        pub role_arn: crate::value::ExpString,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub schema_evolution_configuration: Option<Box<SchemaEvolutionConfiguration_>>,
        pub table_creation_configuration: Option<Box<TableCreationConfiguration_>>,
        pub s3_backup_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_IcebergDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.IcebergDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_IcebergDestinationConfiguration as IcebergDestinationConfiguration;
    impl crate::value::ToValue for IcebergDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.append_only {
                properties.insert(
                    "AppendOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "CatalogConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.catalog_configuration),
            );
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.destination_table_configuration_list {
                properties.insert(
                    "DestinationTableConfigurationList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            if let Some(ref value) = self.schema_evolution_configuration {
                properties.insert(
                    "SchemaEvolutionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_creation_configuration {
                properties.insert(
                    "TableCreationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "s3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-inputformatconfiguration.html>
    pub struct InputFormatConfiguration_ {
        pub deserializer: Option<Box<Deserializer_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_InputFormatConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.InputFormatConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_InputFormatConfiguration as InputFormatConfiguration;
    impl crate::value::ToValue for InputFormatConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deserializer {
                properties.insert(
                    "Deserializer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kmsencryptionconfig.html>
    pub struct KMSEncryptionConfig_ {
        pub awskms_key_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_KMSEncryptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.KMSEncryptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_KMSEncryptionConfig as KMSEncryptionConfig;
    impl crate::value::ToValue for KMSEncryptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AWSKMSKeyARN".to_string(),
                crate::value::ToValue::to_value(&self.awskms_key_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-kinesisstreamsourceconfiguration.html>
    pub struct KinesisStreamSourceConfiguration_ {
        pub kinesis_stream_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_KinesisStreamSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.KinesisStreamSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_KinesisStreamSourceConfiguration as KinesisStreamSourceConfiguration;
    impl crate::value::ToValue for KinesisStreamSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KinesisStreamARN".to_string(),
                crate::value::ToValue::to_value(&self.kinesis_stream_arn),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-msksourceconfiguration.html>
    pub struct MSKSourceConfiguration_ {
        pub authentication_configuration: Box<AuthenticationConfiguration_>,
        pub msk_cluster_arn: crate::value::ExpString,
        pub read_from_timestamp: Option<crate::value::ExpString>,
        pub topic_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_MSKSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.MSKSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_MSKSourceConfiguration as MSKSourceConfiguration;
    impl crate::value::ToValue for MSKSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthenticationConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.authentication_configuration),
            );
            properties.insert(
                "MSKClusterARN".to_string(),
                crate::value::ToValue::to_value(&self.msk_cluster_arn),
            );
            if let Some(ref value) = self.read_from_timestamp {
                properties.insert(
                    "ReadFromTimestamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TopicName".to_string(),
                crate::value::ToValue::to_value(&self.topic_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-openxjsonserde.html>
    pub struct OpenXJsonSerDe_ {
        pub case_insensitive: Option<crate::value::ExpBool>,
        pub column_to_json_key_mappings:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub convert_dots_in_json_keys_to_underscores: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_OpenXJsonSerDe {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.OpenXJsonSerDe"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_OpenXJsonSerDe as OpenXJsonSerDe;
    impl crate::value::ToValue for OpenXJsonSerDe_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.case_insensitive {
                properties.insert(
                    "CaseInsensitive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.column_to_json_key_mappings {
                properties.insert(
                    "ColumnToJsonKeyMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.convert_dots_in_json_keys_to_underscores {
                properties.insert(
                    "ConvertDotsInJsonKeysToUnderscores".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-orcserde.html>
    pub struct OrcSerDe_ {
        pub block_size_bytes: Option<i32>,
        pub bloom_filter_columns: Option<Vec<crate::value::ExpString>>,
        pub bloom_filter_false_positive_probability: Option<f64>,
        pub compression: Option<crate::value::ExpString>,
        pub dictionary_key_threshold: Option<f64>,
        pub enable_padding: Option<crate::value::ExpBool>,
        pub format_version: Option<crate::value::ExpString>,
        pub padding_tolerance: Option<f64>,
        pub row_index_stride: Option<i32>,
        pub stripe_size_bytes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_OrcSerDe {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.OrcSerDe"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_OrcSerDe as OrcSerDe;
    impl crate::value::ToValue for OrcSerDe_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_size_bytes {
                properties.insert(
                    "BlockSizeBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bloom_filter_columns {
                properties.insert(
                    "BloomFilterColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bloom_filter_false_positive_probability {
                properties.insert(
                    "BloomFilterFalsePositiveProbability".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compression {
                properties.insert(
                    "Compression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dictionary_key_threshold {
                properties.insert(
                    "DictionaryKeyThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_padding {
                properties.insert(
                    "EnablePadding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.format_version {
                properties.insert(
                    "FormatVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.padding_tolerance {
                properties.insert(
                    "PaddingTolerance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.row_index_stride {
                properties.insert(
                    "RowIndexStride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stripe_size_bytes {
                properties.insert(
                    "StripeSizeBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-outputformatconfiguration.html>
    pub struct OutputFormatConfiguration_ {
        pub serializer: Option<Box<Serializer_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_OutputFormatConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.OutputFormatConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_OutputFormatConfiguration as OutputFormatConfiguration;
    impl crate::value::ToValue for OutputFormatConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.serializer {
                properties.insert(
                    "Serializer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-parquetserde.html>
    pub struct ParquetSerDe_ {
        pub block_size_bytes: Option<i32>,
        pub compression: Option<crate::value::ExpString>,
        pub enable_dictionary_compression: Option<crate::value::ExpBool>,
        pub max_padding_bytes: Option<i32>,
        pub page_size_bytes: Option<i32>,
        pub writer_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_ParquetSerDe {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.ParquetSerDe"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_ParquetSerDe as ParquetSerDe;
    impl crate::value::ToValue for ParquetSerDe_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_size_bytes {
                properties.insert(
                    "BlockSizeBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compression {
                properties.insert(
                    "Compression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_dictionary_compression {
                properties.insert(
                    "EnableDictionaryCompression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_padding_bytes {
                properties.insert(
                    "MaxPaddingBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.page_size_bytes {
                properties.insert(
                    "PageSizeBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.writer_version {
                properties.insert(
                    "WriterVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-partitionfield.html>
    pub struct PartitionField_ {
        pub source_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_PartitionField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.PartitionField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_PartitionField as PartitionField;
    impl crate::value::ToValue for PartitionField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceName".to_string(),
                crate::value::ToValue::to_value(&self.source_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-partitionspec.html>
    pub struct PartitionSpec_ {
        pub identity: Option<Vec<PartitionField_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_PartitionSpec {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.PartitionSpec"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_PartitionSpec as PartitionSpec;
    impl crate::value::ToValue for PartitionSpec_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.identity {
                properties.insert(
                    "Identity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processingconfiguration.html>
    pub struct ProcessingConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub processors: Option<Vec<Processor_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_ProcessingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.ProcessingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_ProcessingConfiguration as ProcessingConfiguration;
    impl crate::value::ToValue for ProcessingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processors {
                properties.insert(
                    "Processors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processor.html>
    pub struct Processor_ {
        pub parameters: Option<Vec<ProcessorParameter_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_Processor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.Processor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_Processor as Processor;
    impl crate::value::ToValue for Processor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-processorparameter.html>
    pub struct ProcessorParameter_ {
        pub parameter_name: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_ProcessorParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.ProcessorParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_ProcessorParameter as ProcessorParameter;
    impl crate::value::ToValue for ProcessorParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParameterName".to_string(),
                crate::value::ToValue::to_value(&self.parameter_name),
            );
            properties.insert(
                "ParameterValue".to_string(),
                crate::value::ToValue::to_value(&self.parameter_value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftdestinationconfiguration.html>
    pub struct RedshiftDestinationConfiguration_ {
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub cluster_jdbcurl: crate::value::ExpString,
        pub copy_command: Box<CopyCommand_>,
        pub password: Option<crate::value::ExpString>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub retry_options: Option<Box<RedshiftRetryOptions_>>,
        pub role_arn: crate::value::ExpString,
        pub s3_backup_configuration: Option<Box<S3DestinationConfiguration_>>,
        pub s3_backup_mode: Option<crate::value::ExpString>,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub secrets_manager_configuration: Option<Box<SecretsManagerConfiguration_>>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_RedshiftDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.RedshiftDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_RedshiftDestinationConfiguration as RedshiftDestinationConfiguration;
    impl crate::value::ToValue for RedshiftDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ClusterJDBCURL".to_string(),
                crate::value::ToValue::to_value(&self.cluster_jdbcurl),
            );
            properties.insert(
                "CopyCommand".to_string(),
                crate::value::ToValue::to_value(&self.copy_command),
            );
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.s3_backup_configuration {
                properties.insert(
                    "S3BackupConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            if let Some(ref value) = self.secrets_manager_configuration {
                properties.insert(
                    "SecretsManagerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-redshiftretryoptions.html>
    pub struct RedshiftRetryOptions_ {
        pub duration_in_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_RedshiftRetryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.RedshiftRetryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_RedshiftRetryOptions as RedshiftRetryOptions;
    impl crate::value::ToValue for RedshiftRetryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-retryoptions.html>
    pub struct RetryOptions_ {
        pub duration_in_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_RetryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.RetryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_RetryOptions as RetryOptions;
    impl crate::value::ToValue for RetryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-s3destinationconfiguration.html>
    pub struct S3DestinationConfiguration_ {
        pub bucket_arn: crate::value::ExpString,
        pub buffering_hints: Option<Box<BufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub compression_format: Option<crate::value::ExpString>,
        pub encryption_configuration: Option<Box<EncryptionConfiguration_>>,
        pub error_output_prefix: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_S3DestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.S3DestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_S3DestinationConfiguration as S3DestinationConfiguration;
    impl crate::value::ToValue for S3DestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketARN".to_string(),
                crate::value::ToValue::to_value(&self.bucket_arn),
            );
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compression_format {
                properties.insert(
                    "CompressionFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.error_output_prefix {
                properties.insert(
                    "ErrorOutputPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaconfiguration.html>
    pub struct SchemaConfiguration_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub table_name: Option<crate::value::ExpString>,
        pub version_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SchemaConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SchemaConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SchemaConfiguration as SchemaConfiguration;
    impl crate::value::ToValue for SchemaConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_name {
                properties.insert(
                    "TableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version_id {
                properties.insert(
                    "VersionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-schemaevolutionconfiguration.html>
    pub struct SchemaEvolutionConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SchemaEvolutionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SchemaEvolutionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SchemaEvolutionConfiguration as SchemaEvolutionConfiguration;
    impl crate::value::ToValue for SchemaEvolutionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-secretsmanagerconfiguration.html>
    pub struct SecretsManagerConfiguration_ {
        pub enabled: crate::value::ExpBool,
        pub role_arn: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SecretsManagerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SecretsManagerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SecretsManagerConfiguration as SecretsManagerConfiguration;
    impl crate::value::ToValue for SecretsManagerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-serializer.html>
    pub struct Serializer_ {
        pub orc_ser_de: Option<Box<OrcSerDe_>>,
        pub parquet_ser_de: Option<Box<ParquetSerDe_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_Serializer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.Serializer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_Serializer as Serializer;
    impl crate::value::ToValue for Serializer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.orc_ser_de {
                properties.insert(
                    "OrcSerDe".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parquet_ser_de {
                properties.insert(
                    "ParquetSerDe".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakebufferinghints.html>
    pub struct SnowflakeBufferingHints_ {
        pub interval_in_seconds: Option<i32>,
        pub size_in_m_bs: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SnowflakeBufferingHints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SnowflakeBufferingHints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SnowflakeBufferingHints as SnowflakeBufferingHints;
    impl crate::value::ToValue for SnowflakeBufferingHints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interval_in_seconds {
                properties.insert(
                    "IntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_m_bs {
                properties.insert(
                    "SizeInMBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakedestinationconfiguration.html>
    pub struct SnowflakeDestinationConfiguration_ {
        pub account_url: crate::value::ExpString,
        pub buffering_hints: Option<Box<SnowflakeBufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub content_column_name: Option<crate::value::ExpString>,
        pub data_loading_option: Option<crate::value::ExpString>,
        pub database: crate::value::ExpString,
        pub key_passphrase: Option<crate::value::ExpString>,
        pub meta_data_column_name: Option<crate::value::ExpString>,
        pub private_key: Option<crate::value::ExpString>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub retry_options: Option<Box<SnowflakeRetryOptions_>>,
        pub role_arn: crate::value::ExpString,
        pub s3_backup_mode: Option<crate::value::ExpString>,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub schema: crate::value::ExpString,
        pub secrets_manager_configuration: Option<Box<SecretsManagerConfiguration_>>,
        pub snowflake_role_configuration: Option<Box<SnowflakeRoleConfiguration_>>,
        pub snowflake_vpc_configuration: Option<Box<SnowflakeVpcConfiguration_>>,
        pub table: crate::value::ExpString,
        pub user: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SnowflakeDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SnowflakeDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SnowflakeDestinationConfiguration as SnowflakeDestinationConfiguration;
    impl crate::value::ToValue for SnowflakeDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountUrl".to_string(),
                crate::value::ToValue::to_value(&self.account_url),
            );
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.content_column_name {
                properties.insert(
                    "ContentColumnName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_loading_option {
                properties.insert(
                    "DataLoadingOption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Database".to_string(),
                crate::value::ToValue::to_value(&self.database),
            );
            if let Some(ref value) = self.key_passphrase {
                properties.insert(
                    "KeyPassphrase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.meta_data_column_name {
                properties.insert(
                    "MetaDataColumnName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_key {
                properties.insert(
                    "PrivateKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            properties.insert(
                "Schema".to_string(),
                crate::value::ToValue::to_value(&self.schema),
            );
            if let Some(ref value) = self.secrets_manager_configuration {
                properties.insert(
                    "SecretsManagerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snowflake_role_configuration {
                properties.insert(
                    "SnowflakeRoleConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snowflake_vpc_configuration {
                properties.insert(
                    "SnowflakeVpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Table".to_string(),
                crate::value::ToValue::to_value(&self.table),
            );
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakeretryoptions.html>
    pub struct SnowflakeRetryOptions_ {
        pub duration_in_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SnowflakeRetryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SnowflakeRetryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SnowflakeRetryOptions as SnowflakeRetryOptions;
    impl crate::value::ToValue for SnowflakeRetryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakeroleconfiguration.html>
    pub struct SnowflakeRoleConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub snowflake_role: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SnowflakeRoleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SnowflakeRoleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SnowflakeRoleConfiguration as SnowflakeRoleConfiguration;
    impl crate::value::ToValue for SnowflakeRoleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snowflake_role {
                properties.insert(
                    "SnowflakeRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-snowflakevpcconfiguration.html>
    pub struct SnowflakeVpcConfiguration_ {
        pub private_link_vpce_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SnowflakeVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SnowflakeVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SnowflakeVpcConfiguration as SnowflakeVpcConfiguration;
    impl crate::value::ToValue for SnowflakeVpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PrivateLinkVpceId".to_string(),
                crate::value::ToValue::to_value(&self.private_link_vpce_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkbufferinghints.html>
    pub struct SplunkBufferingHints_ {
        pub interval_in_seconds: Option<i32>,
        pub size_in_m_bs: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SplunkBufferingHints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SplunkBufferingHints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SplunkBufferingHints as SplunkBufferingHints;
    impl crate::value::ToValue for SplunkBufferingHints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interval_in_seconds {
                properties.insert(
                    "IntervalInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_m_bs {
                properties.insert(
                    "SizeInMBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkdestinationconfiguration.html>
    pub struct SplunkDestinationConfiguration_ {
        pub buffering_hints: Option<Box<SplunkBufferingHints_>>,
        pub cloud_watch_logging_options: Option<Box<CloudWatchLoggingOptions_>>,
        pub hec_acknowledgment_timeout_in_seconds: Option<i32>,
        pub hec_endpoint: crate::value::ExpString,
        pub hec_endpoint_type: crate::value::ExpString,
        pub hec_token: Option<crate::value::ExpString>,
        pub processing_configuration: Option<Box<ProcessingConfiguration_>>,
        pub retry_options: Option<Box<SplunkRetryOptions_>>,
        pub s3_backup_mode: Option<crate::value::ExpString>,
        pub s3_configuration: Box<S3DestinationConfiguration_>,
        pub secrets_manager_configuration: Option<Box<SecretsManagerConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SplunkDestinationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SplunkDestinationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SplunkDestinationConfiguration as SplunkDestinationConfiguration;
    impl crate::value::ToValue for SplunkDestinationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buffering_hints {
                properties.insert(
                    "BufferingHints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logging_options {
                properties.insert(
                    "CloudWatchLoggingOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hec_acknowledgment_timeout_in_seconds {
                properties.insert(
                    "HECAcknowledgmentTimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HECEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.hec_endpoint),
            );
            properties.insert(
                "HECEndpointType".to_string(),
                crate::value::ToValue::to_value(&self.hec_endpoint_type),
            );
            if let Some(ref value) = self.hec_token {
                properties.insert(
                    "HECToken".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.processing_configuration {
                properties.insert(
                    "ProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_options {
                properties.insert(
                    "RetryOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_backup_mode {
                properties.insert(
                    "S3BackupMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3Configuration".to_string(),
                crate::value::ToValue::to_value(&self.s3_configuration),
            );
            if let Some(ref value) = self.secrets_manager_configuration {
                properties.insert(
                    "SecretsManagerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-splunkretryoptions.html>
    pub struct SplunkRetryOptions_ {
        pub duration_in_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_SplunkRetryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.SplunkRetryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_SplunkRetryOptions as SplunkRetryOptions;
    impl crate::value::ToValue for SplunkRetryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.duration_in_seconds {
                properties.insert(
                    "DurationInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-tablecreationconfiguration.html>
    pub struct TableCreationConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_TableCreationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.TableCreationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_TableCreationConfiguration as TableCreationConfiguration;
    impl crate::value::ToValue for TableCreationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisfirehose-deliverystream-vpcconfiguration.html>
    pub struct VpcConfiguration_ {
        pub role_arn: crate::value::ExpString,
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisfirehose_DeliveryStream_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisFirehose::DeliveryStream.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisfirehose_DeliveryStream_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(&self.security_group_ids),
            );
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisfirehose-deliverystream.html>
pub struct DeliveryStream_ {
    pub amazon_open_search_serverless_destination_configuration: Option<
        super::kinesisfirehose::deliverystream::AmazonOpenSearchServerlessDestinationConfiguration_,
    >,
    pub amazonopensearchservice_destination_configuration: Option<
        super::kinesisfirehose::deliverystream::AmazonopensearchserviceDestinationConfiguration_,
    >,
    pub database_source_configuration:
        Option<super::kinesisfirehose::deliverystream::DatabaseSourceConfiguration_>,
    pub delivery_stream_encryption_configuration_input:
        Option<super::kinesisfirehose::deliverystream::DeliveryStreamEncryptionConfigurationInput_>,
    pub delivery_stream_name: Option<crate::value::ExpString>,
    pub delivery_stream_type: Option<crate::value::ExpString>,
    pub direct_put_source_configuration:
        Option<super::kinesisfirehose::deliverystream::DirectPutSourceConfiguration_>,
    pub elasticsearch_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::ElasticsearchDestinationConfiguration_>,
    pub extended_s3_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::ExtendedS3DestinationConfiguration_>,
    pub http_endpoint_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::HttpEndpointDestinationConfiguration_>,
    pub iceberg_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::IcebergDestinationConfiguration_>,
    pub kinesis_stream_source_configuration:
        Option<super::kinesisfirehose::deliverystream::KinesisStreamSourceConfiguration_>,
    pub msk_source_configuration:
        Option<super::kinesisfirehose::deliverystream::MSKSourceConfiguration_>,
    pub redshift_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::RedshiftDestinationConfiguration_>,
    pub s3_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::S3DestinationConfiguration_>,
    pub snowflake_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::SnowflakeDestinationConfiguration_>,
    pub splunk_destination_configuration:
        Option<super::kinesisfirehose::deliverystream::SplunkDestinationConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisfirehose_DeliveryStream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisFirehose::DeliveryStream"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisfirehose_DeliveryStream as DeliveryStream;
impl crate::template::ToResource for DeliveryStream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisFirehose"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DeliveryStream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.amazon_open_search_serverless_destination_configuration {
            properties.insert(
                "AmazonOpenSearchServerlessDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.amazonopensearchservice_destination_configuration {
            properties.insert(
                "AmazonopensearchserviceDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_source_configuration {
            properties.insert(
                "DatabaseSourceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_stream_encryption_configuration_input {
            properties.insert(
                "DeliveryStreamEncryptionConfigurationInput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_stream_name {
            properties.insert(
                "DeliveryStreamName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_stream_type {
            properties.insert(
                "DeliveryStreamType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.direct_put_source_configuration {
            properties.insert(
                "DirectPutSourceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elasticsearch_destination_configuration {
            properties.insert(
                "ElasticsearchDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.extended_s3_destination_configuration {
            properties.insert(
                "ExtendedS3DestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.http_endpoint_destination_configuration {
            properties.insert(
                "HttpEndpointDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iceberg_destination_configuration {
            properties.insert(
                "IcebergDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kinesis_stream_source_configuration {
            properties.insert(
                "KinesisStreamSourceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.msk_source_configuration {
            properties.insert(
                "MSKSourceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redshift_destination_configuration {
            properties.insert(
                "RedshiftDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_destination_configuration {
            properties.insert(
                "S3DestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snowflake_destination_configuration {
            properties.insert(
                "SnowflakeDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.splunk_destination_configuration {
            properties.insert(
                "SplunkDestinationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
