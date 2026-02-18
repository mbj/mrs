pub mod testcase {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-batch.html>
    pub struct Batch_ {
        pub batch_job_name: crate::value::ExpString,
        pub batch_job_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub export_data_set_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_Batch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.Batch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_Batch as Batch;
    impl crate::value::ToValue for Batch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BatchJobName".to_string(),
                crate::value::ToValue::to_value(&self.batch_job_name),
            );
            if let Some(ref value) = self.batch_job_parameters {
                properties.insert(
                    "BatchJobParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.export_data_set_names {
                properties.insert(
                    "ExportDataSetNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-cloudformationaction.html>
    pub struct CloudFormationAction_ {
        pub action_type: Option<crate::value::ExpString>,
        pub resource: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_CloudFormationAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.CloudFormationAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_CloudFormationAction as CloudFormationAction;
    impl crate::value::ToValue for CloudFormationAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_type {
                properties.insert(
                    "ActionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Resource".to_string(),
                crate::value::ToValue::to_value(&self.resource),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-compareaction.html>
    pub struct CompareAction_ {
        pub input: Box<Input_>,
        pub output: Option<Box<Output_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_CompareAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.CompareAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_CompareAction as CompareAction;
    impl crate::value::ToValue for CompareAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Input".to_string(),
                crate::value::ToValue::to_value(&self.input),
            );
            if let Some(ref value) = self.output {
                properties.insert("Output".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-dataset.html>
    pub struct DataSet_ {
        pub ccsid: crate::value::ExpString,
        pub format: crate::value::ExpString,
        pub length: f64,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_DataSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.DataSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_DataSet as DataSet;
    impl crate::value::ToValue for DataSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Ccsid".to_string(),
                crate::value::ToValue::to_value(&self.ccsid),
            );
            properties.insert(
                "Format".to_string(),
                crate::value::ToValue::to_value(&self.format),
            );
            properties.insert(
                "Length".to_string(),
                crate::value::ToValue::to_value(&self.length),
            );
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-databasecdc.html>
    pub struct DatabaseCDC_ {
        pub source_metadata: Box<SourceDatabaseMetadata_>,
        pub target_metadata: Box<TargetDatabaseMetadata_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_DatabaseCDC {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.DatabaseCDC"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_DatabaseCDC as DatabaseCDC;
    impl crate::value::ToValue for DatabaseCDC_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceMetadata".to_string(),
                crate::value::ToValue::to_value(&self.source_metadata),
            );
            properties.insert(
                "TargetMetadata".to_string(),
                crate::value::ToValue::to_value(&self.target_metadata),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-filemetadata.html>
    pub struct FileMetadata_ {
        pub data_sets: Option<Vec<DataSet_>>,
        pub database_cdc: Option<Box<DatabaseCDC_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_FileMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.FileMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_FileMetadata as FileMetadata;
    impl crate::value::ToValue for FileMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_sets {
                properties.insert(
                    "DataSets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.database_cdc {
                properties.insert(
                    "DatabaseCDC".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-input.html>
    pub struct Input_ {
        pub file: Box<InputFile_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_Input as Input;
    impl crate::value::ToValue for Input_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "File".to_string(),
                crate::value::ToValue::to_value(&self.file),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-inputfile.html>
    pub struct InputFile_ {
        pub file_metadata: Box<FileMetadata_>,
        pub source_location: crate::value::ExpString,
        pub target_location: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_InputFile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.InputFile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_InputFile as InputFile;
    impl crate::value::ToValue for InputFile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileMetadata".to_string(),
                crate::value::ToValue::to_value(&self.file_metadata),
            );
            properties.insert(
                "SourceLocation".to_string(),
                crate::value::ToValue::to_value(&self.source_location),
            );
            properties.insert(
                "TargetLocation".to_string(),
                crate::value::ToValue::to_value(&self.target_location),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-m2managedactionproperties.html>
    pub struct M2ManagedActionProperties_ {
        pub force_stop: Option<crate::value::ExpBool>,
        pub import_data_set_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_M2ManagedActionProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.M2ManagedActionProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_M2ManagedActionProperties as M2ManagedActionProperties;
    impl crate::value::ToValue for M2ManagedActionProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.force_stop {
                properties.insert(
                    "ForceStop".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.import_data_set_location {
                properties.insert(
                    "ImportDataSetLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-m2managedapplicationaction.html>
    pub struct M2ManagedApplicationAction_ {
        pub action_type: crate::value::ExpString,
        pub properties: Option<Box<M2ManagedActionProperties_>>,
        pub resource: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_M2ManagedApplicationAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.M2ManagedApplicationAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_M2ManagedApplicationAction as M2ManagedApplicationAction;
    impl crate::value::ToValue for M2ManagedApplicationAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionType".to_string(),
                crate::value::ToValue::to_value(&self.action_type),
            );
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Resource".to_string(),
                crate::value::ToValue::to_value(&self.resource),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-m2nonmanagedapplicationaction.html>
    pub struct M2NonManagedApplicationAction_ {
        pub action_type: crate::value::ExpString,
        pub resource: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_M2NonManagedApplicationAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.M2NonManagedApplicationAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_M2NonManagedApplicationAction as M2NonManagedApplicationAction;
    impl crate::value::ToValue for M2NonManagedApplicationAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionType".to_string(),
                crate::value::ToValue::to_value(&self.action_type),
            );
            properties.insert(
                "Resource".to_string(),
                crate::value::ToValue::to_value(&self.resource),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-mainframeaction.html>
    pub struct MainframeAction_ {
        pub action_type: Box<MainframeActionType_>,
        pub properties: Option<Box<MainframeActionProperties_>>,
        pub resource: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_MainframeAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.MainframeAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_MainframeAction as MainframeAction;
    impl crate::value::ToValue for MainframeAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionType".to_string(),
                crate::value::ToValue::to_value(&self.action_type),
            );
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Resource".to_string(),
                crate::value::ToValue::to_value(&self.resource),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-mainframeactionproperties.html>
    pub struct MainframeActionProperties_ {
        pub dms_task_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_MainframeActionProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.MainframeActionProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_MainframeActionProperties as MainframeActionProperties;
    impl crate::value::ToValue for MainframeActionProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dms_task_arn {
                properties.insert(
                    "DmsTaskArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-mainframeactiontype.html>
    pub struct MainframeActionType_ {
        pub batch: Option<Box<Batch_>>,
        pub tn3270: Option<Box<TN3270_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_MainframeActionType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.MainframeActionType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_MainframeActionType as MainframeActionType;
    impl crate::value::ToValue for MainframeActionType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch {
                properties.insert("Batch".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tn3270 {
                properties.insert("Tn3270".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-output.html>
    pub struct Output_ {
        pub file: Box<OutputFile_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_Output as Output;
    impl crate::value::ToValue for Output_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "File".to_string(),
                crate::value::ToValue::to_value(&self.file),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-outputfile.html>
    pub struct OutputFile_ {
        pub file_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_OutputFile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.OutputFile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_OutputFile as OutputFile;
    impl crate::value::ToValue for OutputFile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file_location {
                properties.insert(
                    "FileLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-resourceaction.html>
    pub struct ResourceAction_ {
        pub cloud_formation_action: Option<Box<CloudFormationAction_>>,
        pub m2_managed_application_action: Option<Box<M2ManagedApplicationAction_>>,
        pub m2_non_managed_application_action: Option<Box<M2NonManagedApplicationAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_ResourceAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.ResourceAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_ResourceAction as ResourceAction;
    impl crate::value::ToValue for ResourceAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_formation_action {
                properties.insert(
                    "CloudFormationAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.m2_managed_application_action {
                properties.insert(
                    "M2ManagedApplicationAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.m2_non_managed_application_action {
                properties.insert(
                    "M2NonManagedApplicationAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-script.html>
    pub struct Script_ {
        pub script_location: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_Script {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.Script"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_Script as Script;
    impl crate::value::ToValue for Script_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ScriptLocation".to_string(),
                crate::value::ToValue::to_value(&self.script_location),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-sourcedatabasemetadata.html>
    pub struct SourceDatabaseMetadata_ {
        pub capture_tool: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_SourceDatabaseMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.SourceDatabaseMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_SourceDatabaseMetadata as SourceDatabaseMetadata;
    impl crate::value::ToValue for SourceDatabaseMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CaptureTool".to_string(),
                crate::value::ToValue::to_value(&self.capture_tool),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-step.html>
    pub struct Step_ {
        pub action: Box<StepAction_>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_Step {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.Step"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_Step as Step;
    impl crate::value::ToValue for Step_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
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
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-stepaction.html>
    pub struct StepAction_ {
        pub compare_action: Option<Box<CompareAction_>>,
        pub mainframe_action: Option<Box<MainframeAction_>>,
        pub resource_action: Option<Box<ResourceAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_StepAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.StepAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_StepAction as StepAction;
    impl crate::value::ToValue for StepAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compare_action {
                properties.insert(
                    "CompareAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mainframe_action {
                properties.insert(
                    "MainframeAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_action {
                properties.insert(
                    "ResourceAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-tn3270.html>
    pub struct TN3270_ {
        pub export_data_set_names: Option<Vec<crate::value::ExpString>>,
        pub script: Box<Script_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_TN3270 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.TN3270"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_TN3270 as TN3270;
    impl crate::value::ToValue for TN3270_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.export_data_set_names {
                properties.insert(
                    "ExportDataSetNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Script".to_string(),
                crate::value::ToValue::to_value(&self.script),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-targetdatabasemetadata.html>
    pub struct TargetDatabaseMetadata_ {
        pub capture_tool: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_TargetDatabaseMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.TargetDatabaseMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_TargetDatabaseMetadata as TargetDatabaseMetadata;
    impl crate::value::ToValue for TargetDatabaseMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CaptureTool".to_string(),
                crate::value::ToValue::to_value(&self.capture_tool),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-apptest-testcase-testcaselatestversion.html>
    pub struct TestCaseLatestVersion_ {
        pub status: crate::value::ExpString,
        pub version: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_apptest_TestCase_TestCaseLatestVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::AppTest::TestCase.TestCaseLatestVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_apptest_TestCase_TestCaseLatestVersion as TestCaseLatestVersion;
    impl crate::value::ToValue for TestCaseLatestVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-apptest-testcase.html>
pub struct TestCase_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub steps: Vec<super::apptest::testcase::Step_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_apptest_TestCase {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::AppTest::TestCase"
        $($field $value)*)
    };
}
pub use crate::__aws_apptest_TestCase as TestCase;
impl crate::template::ToResource for TestCase_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AppTest"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TestCase"),
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
