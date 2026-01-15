pub mod alert {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-alert-action.html
    pub struct Action_ {
        pub lambda_configuration: Option<Box<LambdaConfiguration_>>,
        pub sns_configuration: Option<Box<SNSConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_Alert_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::Alert.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_Alert_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_configuration {
                properties.insert(
                    "LambdaConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sns_configuration {
                properties.insert(
                    "SNSConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-alert-lambdaconfiguration.html
    pub struct LambdaConfiguration_ {
        pub lambda_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_Alert_LambdaConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::Alert.LambdaConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_Alert_LambdaConfiguration as LambdaConfiguration;
    impl crate::value::ToValue for LambdaConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-alert-snsconfiguration.html
    pub struct SNSConfiguration_ {
        pub role_arn: crate::value::ExpString,
        pub sns_topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_Alert_SNSConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::Alert.SNSConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_Alert_SNSConfiguration as SNSConfiguration;
    impl crate::value::ToValue for SNSConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SnsTopicArn".to_string(),
                crate::value::ToValue::to_value(&self.sns_topic_arn),
            );
            properties.into()
        }
    }
}
pub mod anomalydetector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-anomalydetectorconfig.html
    pub struct AnomalyDetectorConfig_ {
        pub anomaly_detector_frequency: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_AnomalyDetectorConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.AnomalyDetectorConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_AnomalyDetectorConfig as AnomalyDetectorConfig;
    impl crate::value::ToValue for AnomalyDetectorConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AnomalyDetectorFrequency".to_string(),
                crate::value::ToValue::to_value(&self.anomaly_detector_frequency),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-appflowconfig.html
    pub struct AppFlowConfig_ {
        pub flow_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_AppFlowConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.AppFlowConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_AppFlowConfig as AppFlowConfig;
    impl crate::value::ToValue for AppFlowConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FlowName".to_string(),
                crate::value::ToValue::to_value(&self.flow_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-cloudwatchconfig.html
    pub struct CloudwatchConfig_ {
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_CloudwatchConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.CloudwatchConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_CloudwatchConfig as CloudwatchConfig;
    impl crate::value::ToValue for CloudwatchConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-csvformatdescriptor.html
    pub struct CsvFormatDescriptor_ {
        pub charset: Option<crate::value::ExpString>,
        pub contains_header: Option<crate::value::ExpBool>,
        pub delimiter: Option<crate::value::ExpString>,
        pub file_compression: Option<crate::value::ExpString>,
        pub header_list: Option<Vec<crate::value::ExpString>>,
        pub quote_symbol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_CsvFormatDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.CsvFormatDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_CsvFormatDescriptor as CsvFormatDescriptor;
    impl crate::value::ToValue for CsvFormatDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.charset {
                properties.insert(
                    "Charset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contains_header {
                properties.insert(
                    "ContainsHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_compression {
                properties.insert(
                    "FileCompression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_list {
                properties.insert(
                    "HeaderList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.quote_symbol {
                properties.insert(
                    "QuoteSymbol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-fileformatdescriptor.html
    pub struct FileFormatDescriptor_ {
        pub csv_format_descriptor: Option<Box<CsvFormatDescriptor_>>,
        pub json_format_descriptor: Option<Box<JsonFormatDescriptor_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_FileFormatDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.FileFormatDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_FileFormatDescriptor as FileFormatDescriptor;
    impl crate::value::ToValue for FileFormatDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv_format_descriptor {
                properties.insert(
                    "CsvFormatDescriptor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_format_descriptor {
                properties.insert(
                    "JsonFormatDescriptor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-jsonformatdescriptor.html
    pub struct JsonFormatDescriptor_ {
        pub charset: Option<crate::value::ExpString>,
        pub file_compression: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_JsonFormatDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.JsonFormatDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_JsonFormatDescriptor as JsonFormatDescriptor;
    impl crate::value::ToValue for JsonFormatDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.charset {
                properties.insert(
                    "Charset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_compression {
                properties.insert(
                    "FileCompression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metric.html
    pub struct Metric_ {
        pub aggregation_function: crate::value::ExpString,
        pub metric_name: crate::value::ExpString,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_Metric {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.Metric"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_Metric as Metric;
    impl crate::value::ToValue for Metric_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AggregationFunction".to_string(),
                crate::value::ToValue::to_value(&self.aggregation_function),
            );
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricset.html
    pub struct MetricSet_ {
        pub dimension_list: Option<Vec<crate::value::ExpString>>,
        pub metric_list: Vec<Metric_>,
        pub metric_set_description: Option<crate::value::ExpString>,
        pub metric_set_frequency: Option<crate::value::ExpString>,
        pub metric_set_name: crate::value::ExpString,
        pub metric_source: Box<MetricSource_>,
        pub offset: Option<i64>,
        pub timestamp_column: Option<Box<TimestampColumn_>>,
        pub timezone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_MetricSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.MetricSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_MetricSet as MetricSet;
    impl crate::value::ToValue for MetricSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimension_list {
                properties.insert(
                    "DimensionList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricList".to_string(),
                crate::value::ToValue::to_value(&self.metric_list),
            );
            if let Some(ref value) = self.metric_set_description {
                properties.insert(
                    "MetricSetDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_set_frequency {
                properties.insert(
                    "MetricSetFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricSetName".to_string(),
                crate::value::ToValue::to_value(&self.metric_set_name),
            );
            properties.insert(
                "MetricSource".to_string(),
                crate::value::ToValue::to_value(&self.metric_source),
            );
            if let Some(ref value) = self.offset {
                properties.insert("Offset".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.timestamp_column {
                properties.insert(
                    "TimestampColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timezone {
                properties.insert(
                    "Timezone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-metricsource.html
    pub struct MetricSource_ {
        pub app_flow_config: Option<Box<AppFlowConfig_>>,
        pub cloudwatch_config: Option<Box<CloudwatchConfig_>>,
        pub rds_source_config: Option<Box<RDSSourceConfig_>>,
        pub redshift_source_config: Option<Box<RedshiftSourceConfig_>>,
        pub s3_source_config: Option<Box<S3SourceConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_MetricSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.MetricSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_MetricSource as MetricSource;
    impl crate::value::ToValue for MetricSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_flow_config {
                properties.insert(
                    "AppFlowConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloudwatch_config {
                properties.insert(
                    "CloudwatchConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rds_source_config {
                properties.insert(
                    "RDSSourceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_source_config {
                properties.insert(
                    "RedshiftSourceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_source_config {
                properties.insert(
                    "S3SourceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-rdssourceconfig.html
    pub struct RDSSourceConfig_ {
        pub db_instance_identifier: crate::value::ExpString,
        pub database_host: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub database_port: i64,
        pub role_arn: crate::value::ExpString,
        pub secret_manager_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
        pub vpc_configuration: Box<VpcConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_RDSSourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.RDSSourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_RDSSourceConfig as RDSSourceConfig;
    impl crate::value::ToValue for RDSSourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DBInstanceIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.db_instance_identifier),
            );
            properties.insert(
                "DatabaseHost".to_string(),
                crate::value::ToValue::to_value(&self.database_host),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "DatabasePort".to_string(),
                crate::value::ToValue::to_value(&self.database_port),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretManagerArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_manager_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.insert(
                "VpcConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.vpc_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-redshiftsourceconfig.html
    pub struct RedshiftSourceConfig_ {
        pub cluster_identifier: crate::value::ExpString,
        pub database_host: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub database_port: i64,
        pub role_arn: crate::value::ExpString,
        pub secret_manager_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
        pub vpc_configuration: Box<VpcConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_RedshiftSourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.RedshiftSourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_RedshiftSourceConfig as RedshiftSourceConfig;
    impl crate::value::ToValue for RedshiftSourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.cluster_identifier),
            );
            properties.insert(
                "DatabaseHost".to_string(),
                crate::value::ToValue::to_value(&self.database_host),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "DatabasePort".to_string(),
                crate::value::ToValue::to_value(&self.database_port),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "SecretManagerArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_manager_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.insert(
                "VpcConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.vpc_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-s3sourceconfig.html
    pub struct S3SourceConfig_ {
        pub file_format_descriptor: Box<FileFormatDescriptor_>,
        pub historical_data_path_list: Option<Vec<crate::value::ExpString>>,
        pub role_arn: crate::value::ExpString,
        pub templated_path_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_S3SourceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.S3SourceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_S3SourceConfig as S3SourceConfig;
    impl crate::value::ToValue for S3SourceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileFormatDescriptor".to_string(),
                crate::value::ToValue::to_value(&self.file_format_descriptor),
            );
            if let Some(ref value) = self.historical_data_path_list {
                properties.insert(
                    "HistoricalDataPathList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.templated_path_list {
                properties.insert(
                    "TemplatedPathList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-timestampcolumn.html
    pub struct TimestampColumn_ {
        pub column_format: Option<crate::value::ExpString>,
        pub column_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_TimestampColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.TimestampColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_TimestampColumn as TimestampColumn;
    impl crate::value::ToValue for TimestampColumn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.column_format {
                properties.insert(
                    "ColumnFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.column_name {
                properties.insert(
                    "ColumnName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lookoutmetrics-anomalydetector-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub security_group_id_list: Vec<crate::value::ExpString>,
        pub subnet_id_list: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lookoutmetrics_AnomalyDetector_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::LookoutMetrics::AnomalyDetector.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lookoutmetrics_AnomalyDetector_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupIdList".to_string(),
                crate::value::ToValue::to_value(&self.security_group_id_list),
            );
            properties.insert(
                "SubnetIdList".to_string(),
                crate::value::ToValue::to_value(&self.subnet_id_list),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-alert.html
pub struct Alert_ {
    pub action: super::lookoutmetrics::alert::Action_,
    pub alert_description: Option<crate::value::ExpString>,
    pub alert_name: Option<crate::value::ExpString>,
    pub alert_sensitivity_threshold: i64,
    pub anomaly_detector_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lookoutmetrics_Alert {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::LookoutMetrics::Alert"
        $($field $value)*)
    };
}
pub use crate::__aws_lookoutmetrics_Alert as Alert;
impl crate::template::ToResource for Alert_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LookoutMetrics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Alert"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Action".to_string(),
            crate::value::ToValue::to_value(&self.action),
        );
        if let Some(ref value) = self.alert_description {
            properties.insert(
                "AlertDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.alert_name {
            properties.insert(
                "AlertName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AlertSensitivityThreshold".to_string(),
            crate::value::ToValue::to_value(&self.alert_sensitivity_threshold),
        );
        properties.insert(
            "AnomalyDetectorArn".to_string(),
            crate::value::ToValue::to_value(&self.anomaly_detector_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lookoutmetrics-anomalydetector.html
pub struct AnomalyDetector_ {
    pub anomaly_detector_config: super::lookoutmetrics::anomalydetector::AnomalyDetectorConfig_,
    pub anomaly_detector_description: Option<crate::value::ExpString>,
    pub anomaly_detector_name: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub metric_set_list: Vec<super::lookoutmetrics::anomalydetector::MetricSet_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lookoutmetrics_AnomalyDetector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::LookoutMetrics::AnomalyDetector"
        $($field $value)*)
    };
}
pub use crate::__aws_lookoutmetrics_AnomalyDetector as AnomalyDetector;
impl crate::template::ToResource for AnomalyDetector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("LookoutMetrics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnomalyDetector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AnomalyDetectorConfig".to_string(),
            crate::value::ToValue::to_value(&self.anomaly_detector_config),
        );
        if let Some(ref value) = self.anomaly_detector_description {
            properties.insert(
                "AnomalyDetectorDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.anomaly_detector_name {
            properties.insert(
                "AnomalyDetectorName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MetricSetList".to_string(),
            crate::value::ToValue::to_value(&self.metric_set_list),
        );
        properties
    }
}
