pub mod application {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-csvmappingparameters.html>
    pub struct CSVMappingParameters_ {
        pub record_column_delimiter: crate::value::ExpString,
        pub record_row_delimiter: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_CSVMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.CSVMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_CSVMappingParameters as CSVMappingParameters;
    impl crate::value::ToValue for CSVMappingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RecordColumnDelimiter".to_string(),
                crate::value::ToValue::to_value(&self.record_column_delimiter),
            );
            properties.insert(
                "RecordRowDelimiter".to_string(),
                crate::value::ToValue::to_value(&self.record_row_delimiter),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-input.html>
    pub struct Input_ {
        pub input_parallelism: Option<Box<InputParallelism_>>,
        pub input_processing_configuration: Option<Box<InputProcessingConfiguration_>>,
        pub input_schema: Box<InputSchema_>,
        pub kinesis_firehose_input: Option<Box<KinesisFirehoseInput_>>,
        pub kinesis_streams_input: Option<Box<KinesisStreamsInput_>>,
        pub name_prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_Input as Input;
    impl crate::value::ToValue for Input_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_parallelism {
                properties.insert(
                    "InputParallelism".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_processing_configuration {
                properties.insert(
                    "InputProcessingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputSchema".to_string(),
                crate::value::ToValue::to_value(&self.input_schema),
            );
            if let Some(ref value) = self.kinesis_firehose_input {
                properties.insert(
                    "KinesisFirehoseInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_streams_input {
                properties.insert(
                    "KinesisStreamsInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "NamePrefix".to_string(),
                crate::value::ToValue::to_value(&self.name_prefix),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputlambdaprocessor.html>
    pub struct InputLambdaProcessor_ {
        pub resource_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_InputLambdaProcessor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.InputLambdaProcessor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_InputLambdaProcessor as InputLambdaProcessor;
    impl crate::value::ToValue for InputLambdaProcessor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputparallelism.html>
    pub struct InputParallelism_ {
        pub count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_InputParallelism {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.InputParallelism"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_InputParallelism as InputParallelism;
    impl crate::value::ToValue for InputParallelism_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputprocessingconfiguration.html>
    pub struct InputProcessingConfiguration_ {
        pub input_lambda_processor: Option<Box<InputLambdaProcessor_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_InputProcessingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.InputProcessingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_InputProcessingConfiguration as InputProcessingConfiguration;
    impl crate::value::ToValue for InputProcessingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_lambda_processor {
                properties.insert(
                    "InputLambdaProcessor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-inputschema.html>
    pub struct InputSchema_ {
        pub record_columns: Vec<RecordColumn_>,
        pub record_encoding: Option<crate::value::ExpString>,
        pub record_format: Box<RecordFormat_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_InputSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.InputSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_InputSchema as InputSchema;
    impl crate::value::ToValue for InputSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RecordColumns".to_string(),
                crate::value::ToValue::to_value(&self.record_columns),
            );
            if let Some(ref value) = self.record_encoding {
                properties.insert(
                    "RecordEncoding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RecordFormat".to_string(),
                crate::value::ToValue::to_value(&self.record_format),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-jsonmappingparameters.html>
    pub struct JSONMappingParameters_ {
        pub record_row_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_JSONMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.JSONMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_JSONMappingParameters as JSONMappingParameters;
    impl crate::value::ToValue for JSONMappingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RecordRowPath".to_string(),
                crate::value::ToValue::to_value(&self.record_row_path),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-kinesisfirehoseinput.html>
    pub struct KinesisFirehoseInput_ {
        pub resource_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_KinesisFirehoseInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.KinesisFirehoseInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_KinesisFirehoseInput as KinesisFirehoseInput;
    impl crate::value::ToValue for KinesisFirehoseInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-kinesisstreamsinput.html>
    pub struct KinesisStreamsInput_ {
        pub resource_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_KinesisStreamsInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.KinesisStreamsInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_KinesisStreamsInput as KinesisStreamsInput;
    impl crate::value::ToValue for KinesisStreamsInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-mappingparameters.html>
    pub struct MappingParameters_ {
        pub csv_mapping_parameters: Option<Box<CSVMappingParameters_>>,
        pub json_mapping_parameters: Option<Box<JSONMappingParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_MappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.MappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_MappingParameters as MappingParameters;
    impl crate::value::ToValue for MappingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv_mapping_parameters {
                properties.insert(
                    "CSVMappingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_mapping_parameters {
                properties.insert(
                    "JSONMappingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-recordcolumn.html>
    pub struct RecordColumn_ {
        pub mapping: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub sql_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_RecordColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.RecordColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_RecordColumn as RecordColumn;
    impl crate::value::ToValue for RecordColumn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mapping {
                properties.insert(
                    "Mapping".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "SqlType".to_string(),
                crate::value::ToValue::to_value(&self.sql_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-application-recordformat.html>
    pub struct RecordFormat_ {
        pub mapping_parameters: Option<Box<MappingParameters_>>,
        pub record_format_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_Application_RecordFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::Application.RecordFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_Application_RecordFormat as RecordFormat;
    impl crate::value::ToValue for RecordFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mapping_parameters {
                properties.insert(
                    "MappingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RecordFormatType".to_string(),
                crate::value::ToValue::to_value(&self.record_format_type),
            );
            properties.into()
        }
    }
}
pub mod applicationoutput {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-destinationschema.html>
    pub struct DestinationSchema_ {
        pub record_format_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationOutput_DestinationSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationOutput.DestinationSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationOutput_DestinationSchema as DestinationSchema;
    impl crate::value::ToValue for DestinationSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.record_format_type {
                properties.insert(
                    "RecordFormatType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-kinesisfirehoseoutput.html>
    pub struct KinesisFirehoseOutput_ {
        pub resource_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationOutput_KinesisFirehoseOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationOutput.KinesisFirehoseOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationOutput_KinesisFirehoseOutput as KinesisFirehoseOutput;
    impl crate::value::ToValue for KinesisFirehoseOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-kinesisstreamsoutput.html>
    pub struct KinesisStreamsOutput_ {
        pub resource_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationOutput_KinesisStreamsOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationOutput.KinesisStreamsOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationOutput_KinesisStreamsOutput as KinesisStreamsOutput;
    impl crate::value::ToValue for KinesisStreamsOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-lambdaoutput.html>
    pub struct LambdaOutput_ {
        pub resource_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationOutput_LambdaOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationOutput.LambdaOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationOutput_LambdaOutput as LambdaOutput;
    impl crate::value::ToValue for LambdaOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.insert(
                "RoleARN".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationoutput-output.html>
    pub struct Output_ {
        pub destination_schema: Box<DestinationSchema_>,
        pub kinesis_firehose_output: Option<Box<KinesisFirehoseOutput_>>,
        pub kinesis_streams_output: Option<Box<KinesisStreamsOutput_>>,
        pub lambda_output: Option<Box<LambdaOutput_>>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationOutput_Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationOutput.Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationOutput_Output as Output;
    impl crate::value::ToValue for Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationSchema".to_string(),
                crate::value::ToValue::to_value(&self.destination_schema),
            );
            if let Some(ref value) = self.kinesis_firehose_output {
                properties.insert(
                    "KinesisFirehoseOutput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_streams_output {
                properties.insert(
                    "KinesisStreamsOutput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_output {
                properties.insert(
                    "LambdaOutput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod applicationreferencedatasource {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-csvmappingparameters.html>
    pub struct CSVMappingParameters_ {
        pub record_column_delimiter: crate::value::ExpString,
        pub record_row_delimiter: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_CSVMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.CSVMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_CSVMappingParameters as CSVMappingParameters;
    impl crate::value::ToValue for CSVMappingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RecordColumnDelimiter".to_string(),
                crate::value::ToValue::to_value(&self.record_column_delimiter),
            );
            properties.insert(
                "RecordRowDelimiter".to_string(),
                crate::value::ToValue::to_value(&self.record_row_delimiter),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-jsonmappingparameters.html>
    pub struct JSONMappingParameters_ {
        pub record_row_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_JSONMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.JSONMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_JSONMappingParameters as JSONMappingParameters;
    impl crate::value::ToValue for JSONMappingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RecordRowPath".to_string(),
                crate::value::ToValue::to_value(&self.record_row_path),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-mappingparameters.html>
    pub struct MappingParameters_ {
        pub csv_mapping_parameters: Option<Box<CSVMappingParameters_>>,
        pub json_mapping_parameters: Option<Box<JSONMappingParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_MappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.MappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_MappingParameters as MappingParameters;
    impl crate::value::ToValue for MappingParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv_mapping_parameters {
                properties.insert(
                    "CSVMappingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_mapping_parameters {
                properties.insert(
                    "JSONMappingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-recordcolumn.html>
    pub struct RecordColumn_ {
        pub mapping: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub sql_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_RecordColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.RecordColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_RecordColumn as RecordColumn;
    impl crate::value::ToValue for RecordColumn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mapping {
                properties.insert(
                    "Mapping".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "SqlType".to_string(),
                crate::value::ToValue::to_value(&self.sql_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-recordformat.html>
    pub struct RecordFormat_ {
        pub mapping_parameters: Option<Box<MappingParameters_>>,
        pub record_format_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_RecordFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.RecordFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_RecordFormat as RecordFormat;
    impl crate::value::ToValue for RecordFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mapping_parameters {
                properties.insert(
                    "MappingParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RecordFormatType".to_string(),
                crate::value::ToValue::to_value(&self.record_format_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-referencedatasource.html>
    pub struct ReferenceDataSource_ {
        pub reference_schema: Box<ReferenceSchema_>,
        pub s3_reference_data_source: Option<Box<S3ReferenceDataSource_>>,
        pub table_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_ReferenceDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.ReferenceDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_ReferenceDataSource as ReferenceDataSource;
    impl crate::value::ToValue for ReferenceDataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ReferenceSchema".to_string(),
                crate::value::ToValue::to_value(&self.reference_schema),
            );
            if let Some(ref value) = self.s3_reference_data_source {
                properties.insert(
                    "S3ReferenceDataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_name {
                properties.insert(
                    "TableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-referenceschema.html>
    pub struct ReferenceSchema_ {
        pub record_columns: Vec<RecordColumn_>,
        pub record_encoding: Option<crate::value::ExpString>,
        pub record_format: Box<RecordFormat_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_ReferenceSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.ReferenceSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_ReferenceSchema as ReferenceSchema;
    impl crate::value::ToValue for ReferenceSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RecordColumns".to_string(),
                crate::value::ToValue::to_value(&self.record_columns),
            );
            if let Some(ref value) = self.record_encoding {
                properties.insert(
                    "RecordEncoding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RecordFormat".to_string(),
                crate::value::ToValue::to_value(&self.record_format),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalytics-applicationreferencedatasource-s3referencedatasource.html>
    pub struct S3ReferenceDataSource_ {
        pub bucket_arn: crate::value::ExpString,
        pub file_key: crate::value::ExpString,
        pub reference_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource_S3ReferenceDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource.S3ReferenceDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource_S3ReferenceDataSource as S3ReferenceDataSource;
    impl crate::value::ToValue for S3ReferenceDataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketARN".to_string(),
                crate::value::ToValue::to_value(&self.bucket_arn),
            );
            properties.insert(
                "FileKey".to_string(),
                crate::value::ToValue::to_value(&self.file_key),
            );
            properties.insert(
                "ReferenceRoleARN".to_string(),
                crate::value::ToValue::to_value(&self.reference_role_arn),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-application.html>
pub struct Application_ {
    pub application_code: Option<crate::value::ExpString>,
    pub application_description: Option<crate::value::ExpString>,
    pub application_name: Option<crate::value::ExpString>,
    pub inputs: Vec<super::kinesisanalytics::application::Input_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisanalytics_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisAnalytics::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisanalytics_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisAnalytics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_code {
            properties.insert(
                "ApplicationCode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.application_description {
            properties.insert(
                "ApplicationDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.application_name {
            properties.insert(
                "ApplicationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Inputs".to_string(),
            crate::value::ToValue::to_value(&self.inputs),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationoutput.html>
pub struct ApplicationOutput_ {
    pub application_name: crate::value::ExpString,
    pub output: super::kinesisanalytics::applicationoutput::Output_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisanalytics_ApplicationOutput {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisAnalytics::ApplicationOutput"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisanalytics_ApplicationOutput as ApplicationOutput;
impl crate::template::ToResource for ApplicationOutput_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisAnalytics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApplicationOutput"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        properties.insert(
            "Output".to_string(),
            crate::value::ToValue::to_value(&self.output),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalytics-applicationreferencedatasource.html>
pub struct ApplicationReferenceDataSource_ {
    pub application_name: crate::value::ExpString,
    pub reference_data_source:
        super::kinesisanalytics::applicationreferencedatasource::ReferenceDataSource_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisanalytics_ApplicationReferenceDataSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisAnalytics::ApplicationReferenceDataSource"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisanalytics_ApplicationReferenceDataSource as ApplicationReferenceDataSource;
impl crate::template::ToResource for ApplicationReferenceDataSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisAnalytics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ApplicationReferenceDataSource",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        properties.insert(
            "ReferenceDataSource".to_string(),
            crate::value::ToValue::to_value(&self.reference_data_source),
        );
        properties
    }
}
