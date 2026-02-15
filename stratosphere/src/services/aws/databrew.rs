pub mod dataset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-csvoptions.html
    pub struct CsvOptions_ {
        pub delimiter: Option<crate::value::ExpString>,
        pub header_row: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_CsvOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.CsvOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_CsvOptions as CsvOptions;
    impl crate::value::ToValue for CsvOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_row {
                properties.insert(
                    "HeaderRow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datacataloginputdefinition.html
    pub struct DataCatalogInputDefinition_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub table_name: Option<crate::value::ExpString>,
        pub temp_directory: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_DataCatalogInputDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.DataCatalogInputDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_DataCatalogInputDefinition as DataCatalogInputDefinition;
    impl crate::value::ToValue for DataCatalogInputDefinition_ {
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
            if let Some(ref value) = self.table_name {
                properties.insert(
                    "TableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temp_directory {
                properties.insert(
                    "TempDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-databaseinputdefinition.html
    pub struct DatabaseInputDefinition_ {
        pub database_table_name: Option<crate::value::ExpString>,
        pub glue_connection_name: crate::value::ExpString,
        pub query_string: Option<crate::value::ExpString>,
        pub temp_directory: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_DatabaseInputDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.DatabaseInputDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_DatabaseInputDefinition as DatabaseInputDefinition;
    impl crate::value::ToValue for DatabaseInputDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.database_table_name {
                properties.insert(
                    "DatabaseTableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GlueConnectionName".to_string(),
                crate::value::ToValue::to_value(&self.glue_connection_name),
            );
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temp_directory {
                properties.insert(
                    "TempDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datasetparameter.html
    pub struct DatasetParameter_ {
        pub create_column: Option<crate::value::ExpBool>,
        pub datetime_options: Option<Box<DatetimeOptions_>>,
        pub filter: Option<Box<FilterExpression_>>,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_DatasetParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.DatasetParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_DatasetParameter as DatasetParameter;
    impl crate::value::ToValue for DatasetParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.create_column {
                properties.insert(
                    "CreateColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.datetime_options {
                properties.insert(
                    "DatetimeOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter {
                properties.insert("Filter".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-datetimeoptions.html
    pub struct DatetimeOptions_ {
        pub format: crate::value::ExpString,
        pub locale_code: Option<crate::value::ExpString>,
        pub timezone_offset: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_DatetimeOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.DatetimeOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_DatetimeOptions as DatetimeOptions;
    impl crate::value::ToValue for DatetimeOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Format".to_string(),
                crate::value::ToValue::to_value(&self.format),
            );
            if let Some(ref value) = self.locale_code {
                properties.insert(
                    "LocaleCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timezone_offset {
                properties.insert(
                    "TimezoneOffset".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-exceloptions.html
    pub struct ExcelOptions_ {
        pub header_row: Option<crate::value::ExpBool>,
        pub sheet_indexes: Option<Vec<i32>>,
        pub sheet_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_ExcelOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.ExcelOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_ExcelOptions as ExcelOptions;
    impl crate::value::ToValue for ExcelOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header_row {
                properties.insert(
                    "HeaderRow".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sheet_indexes {
                properties.insert(
                    "SheetIndexes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sheet_names {
                properties.insert(
                    "SheetNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-fileslimit.html
    pub struct FilesLimit_ {
        pub max_files: i32,
        pub order: Option<crate::value::ExpString>,
        pub ordered_by: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_FilesLimit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.FilesLimit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_FilesLimit as FilesLimit;
    impl crate::value::ToValue for FilesLimit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxFiles".to_string(),
                crate::value::ToValue::to_value(&self.max_files),
            );
            if let Some(ref value) = self.order {
                properties.insert("Order".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ordered_by {
                properties.insert(
                    "OrderedBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filterexpression.html
    pub struct FilterExpression_ {
        pub expression: crate::value::ExpString,
        pub values_map: Vec<FilterValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_FilterExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.FilterExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_FilterExpression as FilterExpression;
    impl crate::value::ToValue for FilterExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.insert(
                "ValuesMap".to_string(),
                crate::value::ToValue::to_value(&self.values_map),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-filtervalue.html
    pub struct FilterValue_ {
        pub value: crate::value::ExpString,
        pub value_reference: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_FilterValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.FilterValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_FilterValue as FilterValue;
    impl crate::value::ToValue for FilterValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.insert(
                "ValueReference".to_string(),
                crate::value::ToValue::to_value(&self.value_reference),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-formatoptions.html
    pub struct FormatOptions_ {
        pub csv: Option<Box<CsvOptions_>>,
        pub excel: Option<Box<ExcelOptions_>>,
        pub json: Option<Box<JsonOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_FormatOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.FormatOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_FormatOptions as FormatOptions;
    impl crate::value::ToValue for FormatOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.excel {
                properties.insert("Excel".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-input.html
    pub struct Input_ {
        pub data_catalog_input_definition: Option<Box<DataCatalogInputDefinition_>>,
        pub database_input_definition: Option<Box<DatabaseInputDefinition_>>,
        pub metadata: Option<Box<Metadata_>>,
        pub s3_input_definition: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_Input as Input;
    impl crate::value::ToValue for Input_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_catalog_input_definition {
                properties.insert(
                    "DataCatalogInputDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_input_definition {
                properties.insert(
                    "DatabaseInputDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_definition {
                properties.insert(
                    "S3InputDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-jsonoptions.html
    pub struct JsonOptions_ {
        pub multi_line: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_JsonOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.JsonOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_JsonOptions as JsonOptions;
    impl crate::value::ToValue for JsonOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.multi_line {
                properties.insert(
                    "MultiLine".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-metadata.html
    pub struct Metadata_ {
        pub source_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_Metadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.Metadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_Metadata as Metadata;
    impl crate::value::ToValue for Metadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_arn {
                properties.insert(
                    "SourceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathoptions.html
    pub struct PathOptions_ {
        pub files_limit: Option<Box<FilesLimit_>>,
        pub last_modified_date_condition: Option<Box<FilterExpression_>>,
        pub parameters: Option<Vec<PathParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_PathOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.PathOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_PathOptions as PathOptions;
    impl crate::value::ToValue for PathOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.files_limit {
                properties.insert(
                    "FilesLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_modified_date_condition {
                properties.insert(
                    "LastModifiedDateCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-pathparameter.html
    pub struct PathParameter_ {
        pub dataset_parameter: Box<DatasetParameter_>,
        pub path_parameter_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_PathParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.PathParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_PathParameter as PathParameter;
    impl crate::value::ToValue for PathParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatasetParameter".to_string(),
                crate::value::ToValue::to_value(&self.dataset_parameter),
            );
            properties.insert(
                "PathParameterName".to_string(),
                crate::value::ToValue::to_value(&self.path_parameter_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-dataset-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub bucket_owner: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Dataset_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Dataset.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Dataset_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod job {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-allowedstatistics.html
    pub struct AllowedStatistics_ {
        pub statistics: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_AllowedStatistics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.AllowedStatistics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_AllowedStatistics as AllowedStatistics;
    impl crate::value::ToValue for AllowedStatistics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Statistics".to_string(),
                crate::value::ToValue::to_value(&self.statistics),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-columnselector.html
    pub struct ColumnSelector_ {
        pub name: Option<crate::value::ExpString>,
        pub regex: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_ColumnSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.ColumnSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_ColumnSelector as ColumnSelector;
    impl crate::value::ToValue for ColumnSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-columnstatisticsconfiguration.html
    pub struct ColumnStatisticsConfiguration_ {
        pub selectors: Option<Vec<ColumnSelector_>>,
        pub statistics: Box<StatisticsConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_ColumnStatisticsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.ColumnStatisticsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_ColumnStatisticsConfiguration as ColumnStatisticsConfiguration;
    impl crate::value::ToValue for ColumnStatisticsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.selectors {
                properties.insert(
                    "Selectors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Statistics".to_string(),
                crate::value::ToValue::to_value(&self.statistics),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-csvoutputoptions.html
    pub struct CsvOutputOptions_ {
        pub delimiter: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_CsvOutputOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.CsvOutputOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_CsvOutputOptions as CsvOutputOptions;
    impl crate::value::ToValue for CsvOutputOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-datacatalogoutput.html
    pub struct DataCatalogOutput_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub database_options: Option<Box<DatabaseTableOutputOptions_>>,
        pub overwrite: Option<crate::value::ExpBool>,
        pub s3_options: Option<Box<S3TableOutputOptions_>>,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_DataCatalogOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.DataCatalogOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_DataCatalogOutput as DataCatalogOutput;
    impl crate::value::ToValue for DataCatalogOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            if let Some(ref value) = self.database_options {
                properties.insert(
                    "DatabaseOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overwrite {
                properties.insert(
                    "Overwrite".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_options {
                properties.insert(
                    "S3Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-databaseoutput.html
    pub struct DatabaseOutput_ {
        pub database_options: Box<DatabaseTableOutputOptions_>,
        pub database_output_mode: Option<crate::value::ExpString>,
        pub glue_connection_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_DatabaseOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.DatabaseOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_DatabaseOutput as DatabaseOutput;
    impl crate::value::ToValue for DatabaseOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseOptions".to_string(),
                crate::value::ToValue::to_value(&self.database_options),
            );
            if let Some(ref value) = self.database_output_mode {
                properties.insert(
                    "DatabaseOutputMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "GlueConnectionName".to_string(),
                crate::value::ToValue::to_value(&self.glue_connection_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-databasetableoutputoptions.html
    pub struct DatabaseTableOutputOptions_ {
        pub table_name: crate::value::ExpString,
        pub temp_directory: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_DatabaseTableOutputOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.DatabaseTableOutputOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_DatabaseTableOutputOptions as DatabaseTableOutputOptions;
    impl crate::value::ToValue for DatabaseTableOutputOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            if let Some(ref value) = self.temp_directory {
                properties.insert(
                    "TempDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-entitydetectorconfiguration.html
    pub struct EntityDetectorConfiguration_ {
        pub allowed_statistics: Option<Box<AllowedStatistics_>>,
        pub entity_types: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_EntityDetectorConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.EntityDetectorConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_EntityDetectorConfiguration as EntityDetectorConfiguration;
    impl crate::value::ToValue for EntityDetectorConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_statistics {
                properties.insert(
                    "AllowedStatistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EntityTypes".to_string(),
                crate::value::ToValue::to_value(&self.entity_types),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-jobsample.html
    pub struct JobSample_ {
        pub mode: Option<crate::value::ExpString>,
        pub size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_JobSample {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.JobSample"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_JobSample as JobSample;
    impl crate::value::ToValue for JobSample_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.size {
                properties.insert("Size".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-output.html
    pub struct Output_ {
        pub compression_format: Option<crate::value::ExpString>,
        pub format: Option<crate::value::ExpString>,
        pub format_options: Option<Box<OutputFormatOptions_>>,
        pub location: Box<S3Location_>,
        pub max_output_files: Option<i32>,
        pub overwrite: Option<crate::value::ExpBool>,
        pub partition_columns: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_Output as Output;
    impl crate::value::ToValue for Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compression_format {
                properties.insert(
                    "CompressionFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.format {
                properties.insert("Format".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.format_options {
                properties.insert(
                    "FormatOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            if let Some(ref value) = self.max_output_files {
                properties.insert(
                    "MaxOutputFiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overwrite {
                properties.insert(
                    "Overwrite".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partition_columns {
                properties.insert(
                    "PartitionColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-outputformatoptions.html
    pub struct OutputFormatOptions_ {
        pub csv: Option<Box<CsvOutputOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_OutputFormatOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.OutputFormatOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_OutputFormatOptions as OutputFormatOptions;
    impl crate::value::ToValue for OutputFormatOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-outputlocation.html
    pub struct OutputLocation_ {
        pub bucket: crate::value::ExpString,
        pub bucket_owner: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_OutputLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.OutputLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_OutputLocation as OutputLocation;
    impl crate::value::ToValue for OutputLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-profileconfiguration.html
    pub struct ProfileConfiguration_ {
        pub column_statistics_configurations: Option<Vec<ColumnStatisticsConfiguration_>>,
        pub dataset_statistics_configuration: Option<Box<StatisticsConfiguration_>>,
        pub entity_detector_configuration: Option<Box<EntityDetectorConfiguration_>>,
        pub profile_columns: Option<Vec<ColumnSelector_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_ProfileConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.ProfileConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_ProfileConfiguration as ProfileConfiguration;
    impl crate::value::ToValue for ProfileConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.column_statistics_configurations {
                properties.insert(
                    "ColumnStatisticsConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dataset_statistics_configuration {
                properties.insert(
                    "DatasetStatisticsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entity_detector_configuration {
                properties.insert(
                    "EntityDetectorConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_columns {
                properties.insert(
                    "ProfileColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-recipe.html
    pub struct Recipe_ {
        pub name: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_Recipe {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.Recipe"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_Recipe as Recipe;
    impl crate::value::ToValue for Recipe_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub bucket_owner: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-s3tableoutputoptions.html
    pub struct S3TableOutputOptions_ {
        pub location: Box<S3Location_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_S3TableOutputOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.S3TableOutputOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_S3TableOutputOptions as S3TableOutputOptions;
    impl crate::value::ToValue for S3TableOutputOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-statisticoverride.html
    pub struct StatisticOverride_ {
        pub parameters: std::collections::BTreeMap<String, crate::value::ExpString>,
        pub statistic: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_StatisticOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.StatisticOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_StatisticOverride as StatisticOverride;
    impl crate::value::ToValue for StatisticOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(&self.parameters),
            );
            properties.insert(
                "Statistic".to_string(),
                crate::value::ToValue::to_value(&self.statistic),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-statisticsconfiguration.html
    pub struct StatisticsConfiguration_ {
        pub included_statistics: Option<Vec<crate::value::ExpString>>,
        pub overrides: Option<Vec<StatisticOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_StatisticsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.StatisticsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_StatisticsConfiguration as StatisticsConfiguration;
    impl crate::value::ToValue for StatisticsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.included_statistics {
                properties.insert(
                    "IncludedStatistics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-job-validationconfiguration.html
    pub struct ValidationConfiguration_ {
        pub ruleset_arn: crate::value::ExpString,
        pub validation_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Job_ValidationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Job.ValidationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Job_ValidationConfiguration as ValidationConfiguration;
    impl crate::value::ToValue for ValidationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RulesetArn".to_string(),
                crate::value::ToValue::to_value(&self.ruleset_arn),
            );
            if let Some(ref value) = self.validation_mode {
                properties.insert(
                    "ValidationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod project {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-project-sample.html
    pub struct Sample_ {
        pub size: Option<i32>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Project_Sample {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Project.Sample"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Project_Sample as Sample;
    impl crate::value::ToValue for Sample_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.size {
                properties.insert("Size".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod recipe {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-action.html
    pub struct Action_ {
        pub operation: crate::value::ExpString,
        pub parameters: Option<Box<RecipeParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Operation".to_string(),
                crate::value::ToValue::to_value(&self.operation),
            );
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-conditionexpression.html
    pub struct ConditionExpression_ {
        pub condition: crate::value::ExpString,
        pub target_column: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_ConditionExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.ConditionExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_ConditionExpression as ConditionExpression;
    impl crate::value::ToValue for ConditionExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.insert(
                "TargetColumn".to_string(),
                crate::value::ToValue::to_value(&self.target_column),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-datacataloginputdefinition.html
    pub struct DataCatalogInputDefinition_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub table_name: Option<crate::value::ExpString>,
        pub temp_directory: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_DataCatalogInputDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.DataCatalogInputDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_DataCatalogInputDefinition as DataCatalogInputDefinition;
    impl crate::value::ToValue for DataCatalogInputDefinition_ {
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
            if let Some(ref value) = self.table_name {
                properties.insert(
                    "TableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temp_directory {
                properties.insert(
                    "TempDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-input.html
    pub struct Input_ {
        pub data_catalog_input_definition: Option<Box<DataCatalogInputDefinition_>>,
        pub s3_input_definition: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_Input as Input;
    impl crate::value::ToValue for Input_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_catalog_input_definition {
                properties.insert(
                    "DataCatalogInputDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_definition {
                properties.insert(
                    "S3InputDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipeparameters.html
    pub struct RecipeParameters_ {
        pub aggregate_function: Option<crate::value::ExpString>,
        pub base: Option<crate::value::ExpString>,
        pub case_statement: Option<crate::value::ExpString>,
        pub category_map: Option<crate::value::ExpString>,
        pub chars_to_remove: Option<crate::value::ExpString>,
        pub collapse_consecutive_whitespace: Option<crate::value::ExpString>,
        pub column_data_type: Option<crate::value::ExpString>,
        pub column_range: Option<crate::value::ExpString>,
        pub count: Option<crate::value::ExpString>,
        pub custom_characters: Option<crate::value::ExpString>,
        pub custom_stop_words: Option<crate::value::ExpString>,
        pub custom_value: Option<crate::value::ExpString>,
        pub datasets_columns: Option<crate::value::ExpString>,
        pub date_add_value: Option<crate::value::ExpString>,
        pub date_time_format: Option<crate::value::ExpString>,
        pub date_time_parameters: Option<crate::value::ExpString>,
        pub delete_other_rows: Option<crate::value::ExpString>,
        pub delimiter: Option<crate::value::ExpString>,
        pub end_pattern: Option<crate::value::ExpString>,
        pub end_position: Option<crate::value::ExpString>,
        pub end_value: Option<crate::value::ExpString>,
        pub expand_contractions: Option<crate::value::ExpString>,
        pub exponent: Option<crate::value::ExpString>,
        pub false_string: Option<crate::value::ExpString>,
        pub group_by_agg_function_options: Option<crate::value::ExpString>,
        pub group_by_columns: Option<crate::value::ExpString>,
        pub hidden_columns: Option<crate::value::ExpString>,
        pub ignore_case: Option<crate::value::ExpString>,
        pub include_in_split: Option<crate::value::ExpString>,
        pub input: Option<Box<Input_>>,
        pub interval: Option<crate::value::ExpString>,
        pub is_text: Option<crate::value::ExpString>,
        pub join_keys: Option<crate::value::ExpString>,
        pub join_type: Option<crate::value::ExpString>,
        pub left_columns: Option<crate::value::ExpString>,
        pub limit: Option<crate::value::ExpString>,
        pub lower_bound: Option<crate::value::ExpString>,
        pub map_type: Option<crate::value::ExpString>,
        pub mode_type: Option<crate::value::ExpString>,
        pub multi_line: Option<crate::value::ExpBool>,
        pub num_rows: Option<crate::value::ExpString>,
        pub num_rows_after: Option<crate::value::ExpString>,
        pub num_rows_before: Option<crate::value::ExpString>,
        pub order_by_column: Option<crate::value::ExpString>,
        pub order_by_columns: Option<crate::value::ExpString>,
        pub other: Option<crate::value::ExpString>,
        pub pattern: Option<crate::value::ExpString>,
        pub pattern_option1: Option<crate::value::ExpString>,
        pub pattern_option2: Option<crate::value::ExpString>,
        pub pattern_options: Option<crate::value::ExpString>,
        pub period: Option<crate::value::ExpString>,
        pub position: Option<crate::value::ExpString>,
        pub remove_all_punctuation: Option<crate::value::ExpString>,
        pub remove_all_quotes: Option<crate::value::ExpString>,
        pub remove_all_whitespace: Option<crate::value::ExpString>,
        pub remove_custom_characters: Option<crate::value::ExpString>,
        pub remove_custom_value: Option<crate::value::ExpString>,
        pub remove_leading_and_trailing_punctuation: Option<crate::value::ExpString>,
        pub remove_leading_and_trailing_quotes: Option<crate::value::ExpString>,
        pub remove_leading_and_trailing_whitespace: Option<crate::value::ExpString>,
        pub remove_letters: Option<crate::value::ExpString>,
        pub remove_numbers: Option<crate::value::ExpString>,
        pub remove_source_column: Option<crate::value::ExpString>,
        pub remove_special_characters: Option<crate::value::ExpString>,
        pub right_columns: Option<crate::value::ExpString>,
        pub sample_size: Option<crate::value::ExpString>,
        pub sample_type: Option<crate::value::ExpString>,
        pub second_input: Option<crate::value::ExpString>,
        pub secondary_inputs: Option<Vec<SecondaryInput_>>,
        pub sheet_indexes: Option<Vec<i32>>,
        pub sheet_names: Option<Vec<crate::value::ExpString>>,
        pub source_column: Option<crate::value::ExpString>,
        pub source_column1: Option<crate::value::ExpString>,
        pub source_column2: Option<crate::value::ExpString>,
        pub source_columns: Option<crate::value::ExpString>,
        pub start_column_index: Option<crate::value::ExpString>,
        pub start_pattern: Option<crate::value::ExpString>,
        pub start_position: Option<crate::value::ExpString>,
        pub start_value: Option<crate::value::ExpString>,
        pub stemming_mode: Option<crate::value::ExpString>,
        pub step_count: Option<crate::value::ExpString>,
        pub step_index: Option<crate::value::ExpString>,
        pub stop_words_mode: Option<crate::value::ExpString>,
        pub strategy: Option<crate::value::ExpString>,
        pub target_column: Option<crate::value::ExpString>,
        pub target_column_names: Option<crate::value::ExpString>,
        pub target_date_format: Option<crate::value::ExpString>,
        pub target_index: Option<crate::value::ExpString>,
        pub time_zone: Option<crate::value::ExpString>,
        pub tokenizer_pattern: Option<crate::value::ExpString>,
        pub true_string: Option<crate::value::ExpString>,
        pub udf_lang: Option<crate::value::ExpString>,
        pub units: Option<crate::value::ExpString>,
        pub unpivot_column: Option<crate::value::ExpString>,
        pub upper_bound: Option<crate::value::ExpString>,
        pub use_new_data_frame: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
        pub value1: Option<crate::value::ExpString>,
        pub value2: Option<crate::value::ExpString>,
        pub value_column: Option<crate::value::ExpString>,
        pub view_frame: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_RecipeParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.RecipeParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_RecipeParameters as RecipeParameters;
    impl crate::value::ToValue for RecipeParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregate_function {
                properties.insert(
                    "AggregateFunction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.base {
                properties.insert("Base".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.case_statement {
                properties.insert(
                    "CaseStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.category_map {
                properties.insert(
                    "CategoryMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.chars_to_remove {
                properties.insert(
                    "CharsToRemove".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.collapse_consecutive_whitespace {
                properties.insert(
                    "CollapseConsecutiveWhitespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.column_data_type {
                properties.insert(
                    "ColumnDataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.column_range {
                properties.insert(
                    "ColumnRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.custom_characters {
                properties.insert(
                    "CustomCharacters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_stop_words {
                properties.insert(
                    "CustomStopWords".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_value {
                properties.insert(
                    "CustomValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.datasets_columns {
                properties.insert(
                    "DatasetsColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_add_value {
                properties.insert(
                    "DateAddValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_time_format {
                properties.insert(
                    "DateTimeFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.date_time_parameters {
                properties.insert(
                    "DateTimeParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delete_other_rows {
                properties.insert(
                    "DeleteOtherRows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_pattern {
                properties.insert(
                    "EndPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_position {
                properties.insert(
                    "EndPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_value {
                properties.insert(
                    "EndValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expand_contractions {
                properties.insert(
                    "ExpandContractions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exponent {
                properties.insert(
                    "Exponent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.false_string {
                properties.insert(
                    "FalseString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_by_agg_function_options {
                properties.insert(
                    "GroupByAggFunctionOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_by_columns {
                properties.insert(
                    "GroupByColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hidden_columns {
                properties.insert(
                    "HiddenColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_case {
                properties.insert(
                    "IgnoreCase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_in_split {
                properties.insert(
                    "IncludeInSplit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input {
                properties.insert("Input".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_text {
                properties.insert("IsText".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.join_keys {
                properties.insert(
                    "JoinKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.join_type {
                properties.insert(
                    "JoinType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.left_columns {
                properties.insert(
                    "LeftColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.limit {
                properties.insert("Limit".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.lower_bound {
                properties.insert(
                    "LowerBound".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.map_type {
                properties.insert(
                    "MapType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mode_type {
                properties.insert(
                    "ModeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multi_line {
                properties.insert(
                    "MultiLine".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_rows {
                properties.insert(
                    "NumRows".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_rows_after {
                properties.insert(
                    "NumRowsAfter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.num_rows_before {
                properties.insert(
                    "NumRowsBefore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.order_by_column {
                properties.insert(
                    "OrderByColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.order_by_columns {
                properties.insert(
                    "OrderByColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.other {
                properties.insert("Other".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.pattern {
                properties.insert(
                    "Pattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pattern_option1 {
                properties.insert(
                    "PatternOption1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pattern_option2 {
                properties.insert(
                    "PatternOption2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pattern_options {
                properties.insert(
                    "PatternOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.period {
                properties.insert("Period".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.position {
                properties.insert(
                    "Position".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_all_punctuation {
                properties.insert(
                    "RemoveAllPunctuation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_all_quotes {
                properties.insert(
                    "RemoveAllQuotes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_all_whitespace {
                properties.insert(
                    "RemoveAllWhitespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_custom_characters {
                properties.insert(
                    "RemoveCustomCharacters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_custom_value {
                properties.insert(
                    "RemoveCustomValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_leading_and_trailing_punctuation {
                properties.insert(
                    "RemoveLeadingAndTrailingPunctuation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_leading_and_trailing_quotes {
                properties.insert(
                    "RemoveLeadingAndTrailingQuotes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_leading_and_trailing_whitespace {
                properties.insert(
                    "RemoveLeadingAndTrailingWhitespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_letters {
                properties.insert(
                    "RemoveLetters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_numbers {
                properties.insert(
                    "RemoveNumbers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_source_column {
                properties.insert(
                    "RemoveSourceColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.remove_special_characters {
                properties.insert(
                    "RemoveSpecialCharacters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.right_columns {
                properties.insert(
                    "RightColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_size {
                properties.insert(
                    "SampleSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_type {
                properties.insert(
                    "SampleType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.second_input {
                properties.insert(
                    "SecondInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secondary_inputs {
                properties.insert(
                    "SecondaryInputs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sheet_indexes {
                properties.insert(
                    "SheetIndexes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sheet_names {
                properties.insert(
                    "SheetNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_column {
                properties.insert(
                    "SourceColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_column1 {
                properties.insert(
                    "SourceColumn1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_column2 {
                properties.insert(
                    "SourceColumn2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_columns {
                properties.insert(
                    "SourceColumns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_column_index {
                properties.insert(
                    "StartColumnIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_pattern {
                properties.insert(
                    "StartPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_position {
                properties.insert(
                    "StartPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_value {
                properties.insert(
                    "StartValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stemming_mode {
                properties.insert(
                    "StemmingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.step_count {
                properties.insert(
                    "StepCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.step_index {
                properties.insert(
                    "StepIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_words_mode {
                properties.insert(
                    "StopWordsMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strategy {
                properties.insert(
                    "Strategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_column {
                properties.insert(
                    "TargetColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_column_names {
                properties.insert(
                    "TargetColumnNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_date_format {
                properties.insert(
                    "TargetDateFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_index {
                properties.insert(
                    "TargetIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_zone {
                properties.insert(
                    "TimeZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tokenizer_pattern {
                properties.insert(
                    "TokenizerPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.true_string {
                properties.insert(
                    "TrueString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.udf_lang {
                properties.insert(
                    "UdfLang".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.units {
                properties.insert("Units".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unpivot_column {
                properties.insert(
                    "UnpivotColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upper_bound {
                properties.insert(
                    "UpperBound".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.use_new_data_frame {
                properties.insert(
                    "UseNewDataFrame".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value1 {
                properties.insert("Value1".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value2 {
                properties.insert("Value2".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value_column {
                properties.insert(
                    "ValueColumn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.view_frame {
                properties.insert(
                    "ViewFrame".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-recipestep.html
    pub struct RecipeStep_ {
        pub action: Box<Action_>,
        pub condition_expressions: Option<Vec<ConditionExpression_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_RecipeStep {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.RecipeStep"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_RecipeStep as RecipeStep;
    impl crate::value::ToValue for RecipeStep_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.condition_expressions {
                properties.insert(
                    "ConditionExpressions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-recipe-secondaryinput.html
    pub struct SecondaryInput_ {
        pub data_catalog_input_definition: Option<Box<DataCatalogInputDefinition_>>,
        pub s3_input_definition: Option<Box<S3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Recipe_SecondaryInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Recipe.SecondaryInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Recipe_SecondaryInput as SecondaryInput;
    impl crate::value::ToValue for SecondaryInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_catalog_input_definition {
                properties.insert(
                    "DataCatalogInputDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_input_definition {
                properties.insert(
                    "S3InputDefinition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod ruleset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-ruleset-columnselector.html
    pub struct ColumnSelector_ {
        pub name: Option<crate::value::ExpString>,
        pub regex: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Ruleset_ColumnSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Ruleset.ColumnSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Ruleset_ColumnSelector as ColumnSelector;
    impl crate::value::ToValue for ColumnSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regex {
                properties.insert("Regex".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-ruleset-rule.html
    pub struct Rule_ {
        pub check_expression: crate::value::ExpString,
        pub column_selectors: Option<Vec<ColumnSelector_>>,
        pub disabled: Option<crate::value::ExpBool>,
        pub name: crate::value::ExpString,
        pub substitution_map: Option<Vec<SubstitutionValue_>>,
        pub threshold: Option<Box<Threshold_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Ruleset_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Ruleset.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Ruleset_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CheckExpression".to_string(),
                crate::value::ToValue::to_value(&self.check_expression),
            );
            if let Some(ref value) = self.column_selectors {
                properties.insert(
                    "ColumnSelectors".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disabled {
                properties.insert(
                    "Disabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.substitution_map {
                properties.insert(
                    "SubstitutionMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threshold {
                properties.insert(
                    "Threshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-ruleset-substitutionvalue.html
    pub struct SubstitutionValue_ {
        pub value: crate::value::ExpString,
        pub value_reference: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Ruleset_SubstitutionValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Ruleset.SubstitutionValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Ruleset_SubstitutionValue as SubstitutionValue;
    impl crate::value::ToValue for SubstitutionValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.insert(
                "ValueReference".to_string(),
                crate::value::ToValue::to_value(&self.value_reference),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-databrew-ruleset-threshold.html
    pub struct Threshold_ {
        pub r#type: Option<crate::value::ExpString>,
        pub unit: Option<crate::value::ExpString>,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_databrew_Ruleset_Threshold {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataBrew::Ruleset.Threshold"
            $($field $value)*)
        };
    }
    pub use crate::__aws_databrew_Ruleset_Threshold as Threshold;
    impl crate::value::ToValue for Threshold_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-dataset.html
pub struct Dataset_ {
    pub format: Option<crate::value::ExpString>,
    pub format_options: Option<super::databrew::dataset::FormatOptions_>,
    pub input: super::databrew::dataset::Input_,
    pub name: crate::value::ExpString,
    pub path_options: Option<super::databrew::dataset::PathOptions_>,
    pub source: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_databrew_Dataset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataBrew::Dataset"
        $($field $value)*)
    };
}
pub use crate::__aws_databrew_Dataset as Dataset;
impl crate::template::ToResource for Dataset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataBrew"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dataset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.format {
            properties.insert("Format".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.format_options {
            properties.insert(
                "FormatOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Input".to_string(),
            crate::value::ToValue::to_value(&self.input),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.path_options {
            properties.insert(
                "PathOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source {
            properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-job.html
pub struct Job_ {
    pub data_catalog_outputs: Option<Vec<super::databrew::job::DataCatalogOutput_>>,
    pub database_outputs: Option<Vec<super::databrew::job::DatabaseOutput_>>,
    pub dataset_name: Option<crate::value::ExpString>,
    pub encryption_key_arn: Option<crate::value::ExpString>,
    pub encryption_mode: Option<crate::value::ExpString>,
    pub job_sample: Option<super::databrew::job::JobSample_>,
    pub log_subscription: Option<crate::value::ExpString>,
    pub max_capacity: Option<i32>,
    pub max_retries: Option<i32>,
    pub name: crate::value::ExpString,
    pub output_location: Option<super::databrew::job::OutputLocation_>,
    pub outputs: Option<Vec<super::databrew::job::Output_>>,
    pub profile_configuration: Option<super::databrew::job::ProfileConfiguration_>,
    pub project_name: Option<crate::value::ExpString>,
    pub recipe: Option<super::databrew::job::Recipe_>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub timeout: Option<i32>,
    pub r#type: crate::value::ExpString,
    pub validation_configurations: Option<Vec<super::databrew::job::ValidationConfiguration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_databrew_Job {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataBrew::Job" $($field
        $value)*)
    };
}
pub use crate::__aws_databrew_Job as Job;
impl crate::template::ToResource for Job_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataBrew"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Job"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_catalog_outputs {
            properties.insert(
                "DataCatalogOutputs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_outputs {
            properties.insert(
                "DatabaseOutputs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dataset_name {
            properties.insert(
                "DatasetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_key_arn {
            properties.insert(
                "EncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_mode {
            properties.insert(
                "EncryptionMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.job_sample {
            properties.insert(
                "JobSample".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_subscription {
            properties.insert(
                "LogSubscription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_capacity {
            properties.insert(
                "MaxCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_retries {
            properties.insert(
                "MaxRetries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.output_location {
            properties.insert(
                "OutputLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.outputs {
            properties.insert(
                "Outputs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.profile_configuration {
            properties.insert(
                "ProfileConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.project_name {
            properties.insert(
                "ProjectName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.recipe {
            properties.insert("Recipe".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout {
            properties.insert(
                "Timeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.validation_configurations {
            properties.insert(
                "ValidationConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-project.html
pub struct Project_ {
    pub dataset_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub recipe_name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub sample: Option<super::databrew::project::Sample_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_databrew_Project {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataBrew::Project"
        $($field $value)*)
    };
}
pub use crate::__aws_databrew_Project as Project;
impl crate::template::ToResource for Project_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataBrew"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Project"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DatasetName".to_string(),
            crate::value::ToValue::to_value(&self.dataset_name),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RecipeName".to_string(),
            crate::value::ToValue::to_value(&self.recipe_name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.sample {
            properties.insert("Sample".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-recipe.html
pub struct Recipe_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub steps: Vec<super::databrew::recipe::RecipeStep_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_databrew_Recipe {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataBrew::Recipe"
        $($field $value)*)
    };
}
pub use crate::__aws_databrew_Recipe as Recipe;
impl crate::template::ToResource for Recipe_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataBrew"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Recipe"),
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
        properties.insert(
            "Steps".to_string(),
            crate::value::ToValue::to_value(&self.steps),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-ruleset.html
pub struct Ruleset_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub rules: Vec<super::databrew::ruleset::Rule_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_databrew_Ruleset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataBrew::Ruleset"
        $($field $value)*)
    };
}
pub use crate::__aws_databrew_Ruleset as Ruleset;
impl crate::template::ToResource for Ruleset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataBrew"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Ruleset"),
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
        properties.insert(
            "Rules".to_string(),
            crate::value::ToValue::to_value(&self.rules),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-databrew-schedule.html
pub struct Schedule_ {
    pub cron_expression: crate::value::ExpString,
    pub job_names: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_databrew_Schedule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataBrew::Schedule"
        $($field $value)*)
    };
}
pub use crate::__aws_databrew_Schedule as Schedule;
impl crate::template::ToResource for Schedule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataBrew"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Schedule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CronExpression".to_string(),
            crate::value::ToValue::to_value(&self.cron_expression),
        );
        if let Some(ref value) = self.job_names {
            properties.insert(
                "JobNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
