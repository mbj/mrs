pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationcodeconfiguration.html
    pub struct ApplicationCodeConfiguration_ {
        pub code_content: Box<CodeContent_>,
        pub code_content_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ApplicationCodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ApplicationCodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ApplicationCodeConfiguration as ApplicationCodeConfiguration;
    impl crate::value::ToValue for ApplicationCodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CodeContent".to_string(),
                crate::value::ToValue::to_value(&self.code_content),
            );
            properties.insert(
                "CodeContentType".to_string(),
                crate::value::ToValue::to_value(&self.code_content_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationconfiguration.html
    pub struct ApplicationConfiguration_ {
        pub application_code_configuration: Option<Box<ApplicationCodeConfiguration_>>,
        pub application_encryption_configuration: Option<Box<ApplicationEncryptionConfiguration_>>,
        pub application_snapshot_configuration: Option<Box<ApplicationSnapshotConfiguration_>>,
        pub application_system_rollback_configuration:
            Option<Box<ApplicationSystemRollbackConfiguration_>>,
        pub environment_properties: Option<Box<EnvironmentProperties_>>,
        pub flink_application_configuration: Option<Box<FlinkApplicationConfiguration_>>,
        pub sql_application_configuration: Option<Box<SqlApplicationConfiguration_>>,
        pub vpc_configurations: Option<Vec<VpcConfiguration_>>,
        pub zeppelin_application_configuration: Option<Box<ZeppelinApplicationConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ApplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ApplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ApplicationConfiguration as ApplicationConfiguration;
    impl crate::value::ToValue for ApplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_code_configuration {
                properties.insert(
                    "ApplicationCodeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_encryption_configuration {
                properties.insert(
                    "ApplicationEncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_snapshot_configuration {
                properties.insert(
                    "ApplicationSnapshotConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.application_system_rollback_configuration {
                properties.insert(
                    "ApplicationSystemRollbackConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_properties {
                properties.insert(
                    "EnvironmentProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flink_application_configuration {
                properties.insert(
                    "FlinkApplicationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sql_application_configuration {
                properties.insert(
                    "SqlApplicationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_configurations {
                properties.insert(
                    "VpcConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zeppelin_application_configuration {
                properties.insert(
                    "ZeppelinApplicationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationencryptionconfiguration.html
    pub struct ApplicationEncryptionConfiguration_ {
        pub key_id: Option<crate::value::ExpString>,
        pub key_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ApplicationEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ApplicationEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ApplicationEncryptionConfiguration as ApplicationEncryptionConfiguration;
    impl crate::value::ToValue for ApplicationEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_id {
                properties.insert("KeyId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "KeyType".to_string(),
                crate::value::ToValue::to_value(&self.key_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationmaintenanceconfiguration.html
    pub struct ApplicationMaintenanceConfiguration_ {
        pub application_maintenance_window_start_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ApplicationMaintenanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ApplicationMaintenanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ApplicationMaintenanceConfiguration as ApplicationMaintenanceConfiguration;
    impl crate::value::ToValue for ApplicationMaintenanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationMaintenanceWindowStartTime".to_string(),
                crate::value::ToValue::to_value(&self.application_maintenance_window_start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationrestoreconfiguration.html
    pub struct ApplicationRestoreConfiguration_ {
        pub application_restore_type: crate::value::ExpString,
        pub snapshot_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ApplicationRestoreConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ApplicationRestoreConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ApplicationRestoreConfiguration as ApplicationRestoreConfiguration;
    impl crate::value::ToValue for ApplicationRestoreConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationRestoreType".to_string(),
                crate::value::ToValue::to_value(&self.application_restore_type),
            );
            if let Some(ref value) = self.snapshot_name {
                properties.insert(
                    "SnapshotName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationsnapshotconfiguration.html
    pub struct ApplicationSnapshotConfiguration_ {
        pub snapshots_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ApplicationSnapshotConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ApplicationSnapshotConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ApplicationSnapshotConfiguration as ApplicationSnapshotConfiguration;
    impl crate::value::ToValue for ApplicationSnapshotConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SnapshotsEnabled".to_string(),
                crate::value::ToValue::to_value(&self.snapshots_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-applicationsystemrollbackconfiguration.html
    pub struct ApplicationSystemRollbackConfiguration_ {
        pub rollback_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ApplicationSystemRollbackConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ApplicationSystemRollbackConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ApplicationSystemRollbackConfiguration as ApplicationSystemRollbackConfiguration;
    impl crate::value::ToValue for ApplicationSystemRollbackConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RollbackEnabled".to_string(),
                crate::value::ToValue::to_value(&self.rollback_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-csvmappingparameters.html
    pub struct CSVMappingParameters_ {
        pub record_column_delimiter: crate::value::ExpString,
        pub record_row_delimiter: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_CSVMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.CSVMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_CSVMappingParameters as CSVMappingParameters;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-catalogconfiguration.html
    pub struct CatalogConfiguration_ {
        pub glue_data_catalog_configuration: Option<Box<GlueDataCatalogConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_CatalogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.CatalogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_CatalogConfiguration as CatalogConfiguration;
    impl crate::value::ToValue for CatalogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.glue_data_catalog_configuration {
                properties.insert(
                    "GlueDataCatalogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-checkpointconfiguration.html
    pub struct CheckpointConfiguration_ {
        pub checkpoint_interval: Option<i64>,
        pub checkpointing_enabled: Option<crate::value::ExpBool>,
        pub configuration_type: crate::value::ExpString,
        pub min_pause_between_checkpoints: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_CheckpointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.CheckpointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_CheckpointConfiguration as CheckpointConfiguration;
    impl crate::value::ToValue for CheckpointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.checkpoint_interval {
                properties.insert(
                    "CheckpointInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.checkpointing_enabled {
                properties.insert(
                    "CheckpointingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConfigurationType".to_string(),
                crate::value::ToValue::to_value(&self.configuration_type),
            );
            if let Some(ref value) = self.min_pause_between_checkpoints {
                properties.insert(
                    "MinPauseBetweenCheckpoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-codecontent.html
    pub struct CodeContent_ {
        pub s3_content_location: Option<Box<S3ContentLocation_>>,
        pub text_content: Option<crate::value::ExpString>,
        pub zip_file_content: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_CodeContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.CodeContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_CodeContent as CodeContent;
    impl crate::value::ToValue for CodeContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_content_location {
                properties.insert(
                    "S3ContentLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text_content {
                properties.insert(
                    "TextContent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zip_file_content {
                properties.insert(
                    "ZipFileContent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-customartifactconfiguration.html
    pub struct CustomArtifactConfiguration_ {
        pub artifact_type: crate::value::ExpString,
        pub maven_reference: Option<Box<MavenReference_>>,
        pub s3_content_location: Option<Box<S3ContentLocation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_CustomArtifactConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.CustomArtifactConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_CustomArtifactConfiguration as CustomArtifactConfiguration;
    impl crate::value::ToValue for CustomArtifactConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ArtifactType".to_string(),
                crate::value::ToValue::to_value(&self.artifact_type),
            );
            if let Some(ref value) = self.maven_reference {
                properties.insert(
                    "MavenReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_content_location {
                properties.insert(
                    "S3ContentLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-deployasapplicationconfiguration.html
    pub struct DeployAsApplicationConfiguration_ {
        pub s3_content_location: Box<S3ContentBaseLocation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_DeployAsApplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.DeployAsApplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_DeployAsApplicationConfiguration as DeployAsApplicationConfiguration;
    impl crate::value::ToValue for DeployAsApplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3ContentLocation".to_string(),
                crate::value::ToValue::to_value(&self.s3_content_location),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-environmentproperties.html
    pub struct EnvironmentProperties_ {
        pub property_groups: Option<Vec<PropertyGroup_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_EnvironmentProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.EnvironmentProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_EnvironmentProperties as EnvironmentProperties;
    impl crate::value::ToValue for EnvironmentProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.property_groups {
                properties.insert(
                    "PropertyGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkapplicationconfiguration.html
    pub struct FlinkApplicationConfiguration_ {
        pub checkpoint_configuration: Option<Box<CheckpointConfiguration_>>,
        pub monitoring_configuration: Option<Box<MonitoringConfiguration_>>,
        pub parallelism_configuration: Option<Box<ParallelismConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_FlinkApplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.FlinkApplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_FlinkApplicationConfiguration as FlinkApplicationConfiguration;
    impl crate::value::ToValue for FlinkApplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.checkpoint_configuration {
                properties.insert(
                    "CheckpointConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitoring_configuration {
                properties.insert(
                    "MonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallelism_configuration {
                properties.insert(
                    "ParallelismConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-flinkrunconfiguration.html
    pub struct FlinkRunConfiguration_ {
        pub allow_non_restored_state: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_FlinkRunConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.FlinkRunConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_FlinkRunConfiguration as FlinkRunConfiguration;
    impl crate::value::ToValue for FlinkRunConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_non_restored_state {
                properties.insert(
                    "AllowNonRestoredState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-gluedatacatalogconfiguration.html
    pub struct GlueDataCatalogConfiguration_ {
        pub database_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_GlueDataCatalogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.GlueDataCatalogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_GlueDataCatalogConfiguration as GlueDataCatalogConfiguration;
    impl crate::value::ToValue for GlueDataCatalogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.database_arn {
                properties.insert(
                    "DatabaseARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-input.html
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
    macro_rules! __aws_kinesisanalyticsv2_Application_Input {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.Input"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_Input as Input;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputlambdaprocessor.html
    pub struct InputLambdaProcessor_ {
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_InputLambdaProcessor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.InputLambdaProcessor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_InputLambdaProcessor as InputLambdaProcessor;
    impl crate::value::ToValue for InputLambdaProcessor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputparallelism.html
    pub struct InputParallelism_ {
        pub count: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_InputParallelism {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.InputParallelism"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_InputParallelism as InputParallelism;
    impl crate::value::ToValue for InputParallelism_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputprocessingconfiguration.html
    pub struct InputProcessingConfiguration_ {
        pub input_lambda_processor: Option<Box<InputLambdaProcessor_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_InputProcessingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.InputProcessingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_InputProcessingConfiguration as InputProcessingConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-inputschema.html
    pub struct InputSchema_ {
        pub record_columns: Vec<RecordColumn_>,
        pub record_encoding: Option<crate::value::ExpString>,
        pub record_format: Box<RecordFormat_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_InputSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.InputSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_InputSchema as InputSchema;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-jsonmappingparameters.html
    pub struct JSONMappingParameters_ {
        pub record_row_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_JSONMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.JSONMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_JSONMappingParameters as JSONMappingParameters;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-kinesisfirehoseinput.html
    pub struct KinesisFirehoseInput_ {
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_KinesisFirehoseInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.KinesisFirehoseInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_KinesisFirehoseInput as KinesisFirehoseInput;
    impl crate::value::ToValue for KinesisFirehoseInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-kinesisstreamsinput.html
    pub struct KinesisStreamsInput_ {
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_KinesisStreamsInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.KinesisStreamsInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_KinesisStreamsInput as KinesisStreamsInput;
    impl crate::value::ToValue for KinesisStreamsInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mappingparameters.html
    pub struct MappingParameters_ {
        pub csv_mapping_parameters: Option<Box<CSVMappingParameters_>>,
        pub json_mapping_parameters: Option<Box<JSONMappingParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_MappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.MappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_MappingParameters as MappingParameters;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-mavenreference.html
    pub struct MavenReference_ {
        pub artifact_id: crate::value::ExpString,
        pub group_id: crate::value::ExpString,
        pub version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_MavenReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.MavenReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_MavenReference as MavenReference;
    impl crate::value::ToValue for MavenReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ArtifactId".to_string(),
                crate::value::ToValue::to_value(&self.artifact_id),
            );
            properties.insert(
                "GroupId".to_string(),
                crate::value::ToValue::to_value(&self.group_id),
            );
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(&self.version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-monitoringconfiguration.html
    pub struct MonitoringConfiguration_ {
        pub configuration_type: crate::value::ExpString,
        pub log_level: Option<crate::value::ExpString>,
        pub metrics_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_MonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.MonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_MonitoringConfiguration as MonitoringConfiguration;
    impl crate::value::ToValue for MonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConfigurationType".to_string(),
                crate::value::ToValue::to_value(&self.configuration_type),
            );
            if let Some(ref value) = self.log_level {
                properties.insert(
                    "LogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics_level {
                properties.insert(
                    "MetricsLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-parallelismconfiguration.html
    pub struct ParallelismConfiguration_ {
        pub auto_scaling_enabled: Option<crate::value::ExpBool>,
        pub configuration_type: crate::value::ExpString,
        pub parallelism: Option<i32>,
        pub parallelism_per_kpu: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ParallelismConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ParallelismConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ParallelismConfiguration as ParallelismConfiguration;
    impl crate::value::ToValue for ParallelismConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_scaling_enabled {
                properties.insert(
                    "AutoScalingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConfigurationType".to_string(),
                crate::value::ToValue::to_value(&self.configuration_type),
            );
            if let Some(ref value) = self.parallelism {
                properties.insert(
                    "Parallelism".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallelism_per_kpu {
                properties.insert(
                    "ParallelismPerKPU".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-propertygroup.html
    pub struct PropertyGroup_ {
        pub property_group_id: Option<crate::value::ExpString>,
        pub property_map: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_PropertyGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.PropertyGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_PropertyGroup as PropertyGroup;
    impl crate::value::ToValue for PropertyGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.property_group_id {
                properties.insert(
                    "PropertyGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_map {
                properties.insert(
                    "PropertyMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordcolumn.html
    pub struct RecordColumn_ {
        pub mapping: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub sql_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_RecordColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.RecordColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_RecordColumn as RecordColumn;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-recordformat.html
    pub struct RecordFormat_ {
        pub mapping_parameters: Option<Box<MappingParameters_>>,
        pub record_format_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_RecordFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.RecordFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_RecordFormat as RecordFormat;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-runconfiguration.html
    pub struct RunConfiguration_ {
        pub application_restore_configuration: Option<Box<ApplicationRestoreConfiguration_>>,
        pub flink_run_configuration: Option<Box<FlinkRunConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_RunConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.RunConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_RunConfiguration as RunConfiguration;
    impl crate::value::ToValue for RunConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_restore_configuration {
                properties.insert(
                    "ApplicationRestoreConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.flink_run_configuration {
                properties.insert(
                    "FlinkRunConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentbaselocation.html
    pub struct S3ContentBaseLocation_ {
        pub base_path: Option<crate::value::ExpString>,
        pub bucket_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_S3ContentBaseLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.S3ContentBaseLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_S3ContentBaseLocation as S3ContentBaseLocation;
    impl crate::value::ToValue for S3ContentBaseLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base_path {
                properties.insert(
                    "BasePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BucketARN".to_string(),
                crate::value::ToValue::to_value(&self.bucket_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-s3contentlocation.html
    pub struct S3ContentLocation_ {
        pub bucket_arn: crate::value::ExpString,
        pub file_key: crate::value::ExpString,
        pub object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_S3ContentLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.S3ContentLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_S3ContentLocation as S3ContentLocation;
    impl crate::value::ToValue for S3ContentLocation_ {
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
            if let Some(ref value) = self.object_version {
                properties.insert(
                    "ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-sqlapplicationconfiguration.html
    pub struct SqlApplicationConfiguration_ {
        pub inputs: Option<Vec<Input_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_SqlApplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.SqlApplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_SqlApplicationConfiguration as SqlApplicationConfiguration;
    impl crate::value::ToValue for SqlApplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inputs {
                properties.insert("Inputs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub security_group_ids: Vec<crate::value::ExpString>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinapplicationconfiguration.html
    pub struct ZeppelinApplicationConfiguration_ {
        pub catalog_configuration: Option<Box<CatalogConfiguration_>>,
        pub custom_artifacts_configuration: Option<Vec<CustomArtifactConfiguration_>>,
        pub deploy_as_application_configuration: Option<Box<DeployAsApplicationConfiguration_>>,
        pub monitoring_configuration: Option<Box<ZeppelinMonitoringConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ZeppelinApplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ZeppelinApplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ZeppelinApplicationConfiguration as ZeppelinApplicationConfiguration;
    impl crate::value::ToValue for ZeppelinApplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_configuration {
                properties.insert(
                    "CatalogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_artifacts_configuration {
                properties.insert(
                    "CustomArtifactsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deploy_as_application_configuration {
                properties.insert(
                    "DeployAsApplicationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitoring_configuration {
                properties.insert(
                    "MonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-application-zeppelinmonitoringconfiguration.html
    pub struct ZeppelinMonitoringConfiguration_ {
        pub log_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_Application_ZeppelinMonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::Application.ZeppelinMonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_Application_ZeppelinMonitoringConfiguration as ZeppelinMonitoringConfiguration;
    impl crate::value::ToValue for ZeppelinMonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_level {
                properties.insert(
                    "LogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod applicationcloudwatchloggingoption {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationcloudwatchloggingoption-cloudwatchloggingoption.html
    pub struct CloudWatchLoggingOption_ {
        pub log_stream_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationCloudWatchLoggingOption_CloudWatchLoggingOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption.CloudWatchLoggingOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationCloudWatchLoggingOption_CloudWatchLoggingOption as CloudWatchLoggingOption;
    impl crate::value::ToValue for CloudWatchLoggingOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogStreamARN".to_string(),
                crate::value::ToValue::to_value(&self.log_stream_arn),
            );
            properties.into()
        }
    }
}
pub mod applicationoutput {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-destinationschema.html
    pub struct DestinationSchema_ {
        pub record_format_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationOutput_DestinationSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationOutput.DestinationSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationOutput_DestinationSchema as DestinationSchema;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-kinesisfirehoseoutput.html
    pub struct KinesisFirehoseOutput_ {
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationOutput_KinesisFirehoseOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationOutput.KinesisFirehoseOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationOutput_KinesisFirehoseOutput as KinesisFirehoseOutput;
    impl crate::value::ToValue for KinesisFirehoseOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-kinesisstreamsoutput.html
    pub struct KinesisStreamsOutput_ {
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationOutput_KinesisStreamsOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationOutput.KinesisStreamsOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationOutput_KinesisStreamsOutput as KinesisStreamsOutput;
    impl crate::value::ToValue for KinesisStreamsOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-lambdaoutput.html
    pub struct LambdaOutput_ {
        pub resource_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationOutput_LambdaOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationOutput.LambdaOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationOutput_LambdaOutput as LambdaOutput;
    impl crate::value::ToValue for LambdaOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourceARN".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationoutput-output.html
    pub struct Output_ {
        pub destination_schema: Box<DestinationSchema_>,
        pub kinesis_firehose_output: Option<Box<KinesisFirehoseOutput_>>,
        pub kinesis_streams_output: Option<Box<KinesisStreamsOutput_>>,
        pub lambda_output: Option<Box<LambdaOutput_>>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationOutput_Output {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationOutput.Output"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationOutput_Output as Output;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-csvmappingparameters.html
    pub struct CSVMappingParameters_ {
        pub record_column_delimiter: crate::value::ExpString,
        pub record_row_delimiter: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_CSVMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.CSVMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_CSVMappingParameters as CSVMappingParameters;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-jsonmappingparameters.html
    pub struct JSONMappingParameters_ {
        pub record_row_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_JSONMappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.JSONMappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_JSONMappingParameters as JSONMappingParameters;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-mappingparameters.html
    pub struct MappingParameters_ {
        pub csv_mapping_parameters: Option<Box<CSVMappingParameters_>>,
        pub json_mapping_parameters: Option<Box<JSONMappingParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_MappingParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.MappingParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_MappingParameters as MappingParameters;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordcolumn.html
    pub struct RecordColumn_ {
        pub mapping: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub sql_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_RecordColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.RecordColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_RecordColumn as RecordColumn;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-recordformat.html
    pub struct RecordFormat_ {
        pub mapping_parameters: Option<Box<MappingParameters_>>,
        pub record_format_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_RecordFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.RecordFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_RecordFormat as RecordFormat;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referencedatasource.html
    pub struct ReferenceDataSource_ {
        pub reference_schema: Box<ReferenceSchema_>,
        pub s3_reference_data_source: Option<Box<S3ReferenceDataSource_>>,
        pub table_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_ReferenceDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.ReferenceDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_ReferenceDataSource as ReferenceDataSource;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-referenceschema.html
    pub struct ReferenceSchema_ {
        pub record_columns: Vec<RecordColumn_>,
        pub record_encoding: Option<crate::value::ExpString>,
        pub record_format: Box<RecordFormat_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_ReferenceSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.ReferenceSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_ReferenceSchema as ReferenceSchema;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kinesisanalyticsv2-applicationreferencedatasource-s3referencedatasource.html
    pub struct S3ReferenceDataSource_ {
        pub bucket_arn: crate::value::ExpString,
        pub file_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource_S3ReferenceDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource.S3ReferenceDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource_S3ReferenceDataSource as S3ReferenceDataSource;
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
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-application.html
pub struct Application_ {
    pub application_configuration:
        Option<super::kinesisanalyticsv2::application::ApplicationConfiguration_>,
    pub application_description: Option<crate::value::ExpString>,
    pub application_maintenance_configuration:
        Option<super::kinesisanalyticsv2::application::ApplicationMaintenanceConfiguration_>,
    pub application_mode: Option<crate::value::ExpString>,
    pub application_name: Option<crate::value::ExpString>,
    pub run_configuration: Option<super::kinesisanalyticsv2::application::RunConfiguration_>,
    pub runtime_environment: crate::value::ExpString,
    pub service_execution_role: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisanalyticsv2_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisAnalyticsV2::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisanalyticsv2_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisAnalyticsV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_configuration {
            properties.insert(
                "ApplicationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.application_description {
            properties.insert(
                "ApplicationDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.application_maintenance_configuration {
            properties.insert(
                "ApplicationMaintenanceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.application_mode {
            properties.insert(
                "ApplicationMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.application_name {
            properties.insert(
                "ApplicationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.run_configuration {
            properties.insert(
                "RunConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuntimeEnvironment".to_string(),
            crate::value::ToValue::to_value(&self.runtime_environment),
        );
        properties.insert(
            "ServiceExecutionRole".to_string(),
            crate::value::ToValue::to_value(&self.service_execution_role),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationcloudwatchloggingoption.html
pub struct ApplicationCloudWatchLoggingOption_ {
    pub application_name: crate::value::ExpString,
    pub cloud_watch_logging_option:
        super::kinesisanalyticsv2::applicationcloudwatchloggingoption::CloudWatchLoggingOption_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisanalyticsv2_ApplicationCloudWatchLoggingOption {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisAnalyticsV2::ApplicationCloudWatchLoggingOption"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisanalyticsv2_ApplicationCloudWatchLoggingOption as ApplicationCloudWatchLoggingOption;
impl crate::template::ToResource for ApplicationCloudWatchLoggingOption_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisAnalyticsV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ApplicationCloudWatchLoggingOption",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationName".to_string(),
            crate::value::ToValue::to_value(&self.application_name),
        );
        properties.insert(
            "CloudWatchLoggingOption".to_string(),
            crate::value::ToValue::to_value(&self.cloud_watch_logging_option),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationoutput.html
pub struct ApplicationOutput_ {
    pub application_name: crate::value::ExpString,
    pub output: super::kinesisanalyticsv2::applicationoutput::Output_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisanalyticsv2_ApplicationOutput {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisAnalyticsV2::ApplicationOutput"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisanalyticsv2_ApplicationOutput as ApplicationOutput;
impl crate::template::ToResource for ApplicationOutput_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisAnalyticsV2"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kinesisanalyticsv2-applicationreferencedatasource.html
pub struct ApplicationReferenceDataSource_ {
    pub application_name: crate::value::ExpString,
    pub reference_data_source:
        super::kinesisanalyticsv2::applicationreferencedatasource::ReferenceDataSource_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kinesisanalyticsv2_ApplicationReferenceDataSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KinesisAnalyticsV2::ApplicationReferenceDataSource"
        $($field $value)*)
    };
}
pub use crate::__aws_kinesisanalyticsv2_ApplicationReferenceDataSource as ApplicationReferenceDataSource;
impl crate::template::ToResource for ApplicationReferenceDataSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KinesisAnalyticsV2"),
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
