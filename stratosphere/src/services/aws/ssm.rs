pub mod association {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-instanceassociationoutputlocation.html
    pub struct InstanceAssociationOutputLocation_ {
        pub s3_location: Option<Box<S3OutputLocation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_Association_InstanceAssociationOutputLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::Association.InstanceAssociationOutputLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_Association_InstanceAssociationOutputLocation as InstanceAssociationOutputLocation;
    impl crate::value::ToValue for InstanceAssociationOutputLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_location {
                properties.insert(
                    "S3Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-s3outputlocation.html
    pub struct S3OutputLocation_ {
        pub output_s3_bucket_name: Option<crate::value::ExpString>,
        pub output_s3_key_prefix: Option<crate::value::ExpString>,
        pub output_s3_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_Association_S3OutputLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::Association.S3OutputLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_Association_S3OutputLocation as S3OutputLocation;
    impl crate::value::ToValue for S3OutputLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.output_s3_bucket_name {
                properties.insert(
                    "OutputS3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_s3_key_prefix {
                properties.insert(
                    "OutputS3KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_s3_region {
                properties.insert(
                    "OutputS3Region".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-association-target.html
    pub struct Target_ {
        pub key: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_Association_Target {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::Association.Target"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_Association_Target as Target;
    impl crate::value::ToValue for Target_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
}
pub mod document {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-attachmentssource.html
    pub struct AttachmentsSource_ {
        pub key: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_Document_AttachmentsSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::Document.AttachmentsSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_Document_AttachmentsSource as AttachmentsSource;
    impl crate::value::ToValue for AttachmentsSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-document-documentrequires.html
    pub struct DocumentRequires_ {
        pub name: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_Document_DocumentRequires {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::Document.DocumentRequires"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_Document_DocumentRequires as DocumentRequires;
    impl crate::value::ToValue for DocumentRequires_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod maintenancewindowtarget {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtarget-targets.html
    pub struct Targets_ {
        pub key: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTarget_Targets {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTarget.Targets"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTarget_Targets as Targets;
    impl crate::value::ToValue for Targets_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
}
pub mod maintenancewindowtask {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-cloudwatchoutputconfig.html
    pub struct CloudWatchOutputConfig_ {
        pub cloud_watch_log_group_name: Option<crate::value::ExpString>,
        pub cloud_watch_output_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_CloudWatchOutputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.CloudWatchOutputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_CloudWatchOutputConfig as CloudWatchOutputConfig;
    impl crate::value::ToValue for CloudWatchOutputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_log_group_name {
                properties.insert(
                    "CloudWatchLogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_output_enabled {
                properties.insert(
                    "CloudWatchOutputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-logginginfo.html
    pub struct LoggingInfo_ {
        pub region: crate::value::ExpString,
        pub s3_bucket: crate::value::ExpString,
        pub s3_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_LoggingInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.LoggingInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_LoggingInfo as LoggingInfo;
    impl crate::value::ToValue for LoggingInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            if let Some(ref value) = self.s3_prefix {
                properties.insert(
                    "S3Prefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowautomationparameters.html
    pub struct MaintenanceWindowAutomationParameters_ {
        pub document_version: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_MaintenanceWindowAutomationParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.MaintenanceWindowAutomationParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_MaintenanceWindowAutomationParameters as MaintenanceWindowAutomationParameters;
    impl crate::value::ToValue for MaintenanceWindowAutomationParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.document_version {
                properties.insert(
                    "DocumentVersion".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowlambdaparameters.html
    pub struct MaintenanceWindowLambdaParameters_ {
        pub client_context: Option<crate::value::ExpString>,
        pub payload: Option<crate::value::ExpString>,
        pub qualifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_MaintenanceWindowLambdaParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.MaintenanceWindowLambdaParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_MaintenanceWindowLambdaParameters as MaintenanceWindowLambdaParameters;
    impl crate::value::ToValue for MaintenanceWindowLambdaParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_context {
                properties.insert(
                    "ClientContext".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.qualifier {
                properties.insert(
                    "Qualifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowruncommandparameters.html
    pub struct MaintenanceWindowRunCommandParameters_ {
        pub cloud_watch_output_config: Option<Box<CloudWatchOutputConfig_>>,
        pub comment: Option<crate::value::ExpString>,
        pub document_hash: Option<crate::value::ExpString>,
        pub document_hash_type: Option<crate::value::ExpString>,
        pub document_version: Option<crate::value::ExpString>,
        pub notification_config: Option<Box<NotificationConfig_>>,
        pub output_s3_bucket_name: Option<crate::value::ExpString>,
        pub output_s3_key_prefix: Option<crate::value::ExpString>,
        pub parameters: Option<serde_json::Value>,
        pub service_role_arn: Option<crate::value::ExpString>,
        pub timeout_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_MaintenanceWindowRunCommandParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.MaintenanceWindowRunCommandParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_MaintenanceWindowRunCommandParameters as MaintenanceWindowRunCommandParameters;
    impl crate::value::ToValue for MaintenanceWindowRunCommandParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_output_config {
                properties.insert(
                    "CloudWatchOutputConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_hash {
                properties.insert(
                    "DocumentHash".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_hash_type {
                properties.insert(
                    "DocumentHashType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.document_version {
                properties.insert(
                    "DocumentVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_config {
                properties.insert(
                    "NotificationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_s3_bucket_name {
                properties.insert(
                    "OutputS3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_s3_key_prefix {
                properties.insert(
                    "OutputS3KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_role_arn {
                properties.insert(
                    "ServiceRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_seconds {
                properties.insert(
                    "TimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-maintenancewindowstepfunctionsparameters.html
    pub struct MaintenanceWindowStepFunctionsParameters_ {
        pub input: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_MaintenanceWindowStepFunctionsParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.MaintenanceWindowStepFunctionsParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_MaintenanceWindowStepFunctionsParameters as MaintenanceWindowStepFunctionsParameters;
    impl crate::value::ToValue for MaintenanceWindowStepFunctionsParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input {
                properties.insert("Input".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-notificationconfig.html
    pub struct NotificationConfig_ {
        pub notification_arn: crate::value::ExpString,
        pub notification_events: Option<Vec<crate::value::ExpString>>,
        pub notification_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_NotificationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.NotificationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_NotificationConfig as NotificationConfig;
    impl crate::value::ToValue for NotificationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NotificationArn".to_string(),
                crate::value::ToValue::to_value(&self.notification_arn),
            );
            if let Some(ref value) = self.notification_events {
                properties.insert(
                    "NotificationEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.notification_type {
                properties.insert(
                    "NotificationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-target.html
    pub struct Target_ {
        pub key: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_Target {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.Target"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_Target as Target;
    impl crate::value::ToValue for Target_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-maintenancewindowtask-taskinvocationparameters.html
    pub struct TaskInvocationParameters_ {
        pub maintenance_window_automation_parameters:
            Option<Box<MaintenanceWindowAutomationParameters_>>,
        pub maintenance_window_lambda_parameters: Option<Box<MaintenanceWindowLambdaParameters_>>,
        pub maintenance_window_run_command_parameters:
            Option<Box<MaintenanceWindowRunCommandParameters_>>,
        pub maintenance_window_step_functions_parameters:
            Option<Box<MaintenanceWindowStepFunctionsParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_MaintenanceWindowTask_TaskInvocationParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::MaintenanceWindowTask.TaskInvocationParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_MaintenanceWindowTask_TaskInvocationParameters as TaskInvocationParameters;
    impl crate::value::ToValue for TaskInvocationParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maintenance_window_automation_parameters {
                properties.insert(
                    "MaintenanceWindowAutomationParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_window_lambda_parameters {
                properties.insert(
                    "MaintenanceWindowLambdaParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_window_run_command_parameters {
                properties.insert(
                    "MaintenanceWindowRunCommandParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_window_step_functions_parameters {
                properties.insert(
                    "MaintenanceWindowStepFunctionsParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod patchbaseline {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfilter.html
    pub struct PatchFilter_ {
        pub key: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_PatchBaseline_PatchFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::PatchBaseline.PatchFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_PatchBaseline_PatchFilter as PatchFilter;
    impl crate::value::ToValue for PatchFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchfiltergroup.html
    pub struct PatchFilterGroup_ {
        pub patch_filters: Option<Vec<PatchFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_PatchBaseline_PatchFilterGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::PatchBaseline.PatchFilterGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_PatchBaseline_PatchFilterGroup as PatchFilterGroup;
    impl crate::value::ToValue for PatchFilterGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.patch_filters {
                properties.insert(
                    "PatchFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-patchsource.html
    pub struct PatchSource_ {
        pub configuration: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub products: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_PatchBaseline_PatchSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::PatchBaseline.PatchSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_PatchBaseline_PatchSource as PatchSource;
    impl crate::value::ToValue for PatchSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.products {
                properties.insert(
                    "Products".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rule.html
    pub struct Rule_ {
        pub approve_after_days: Option<i64>,
        pub approve_until_date: Option<crate::value::ExpString>,
        pub compliance_level: Option<crate::value::ExpString>,
        pub enable_non_security: Option<crate::value::ExpBool>,
        pub patch_filter_group: Option<Box<PatchFilterGroup_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_PatchBaseline_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::PatchBaseline.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_PatchBaseline_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approve_after_days {
                properties.insert(
                    "ApproveAfterDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.approve_until_date {
                properties.insert(
                    "ApproveUntilDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.compliance_level {
                properties.insert(
                    "ComplianceLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_non_security {
                properties.insert(
                    "EnableNonSecurity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.patch_filter_group {
                properties.insert(
                    "PatchFilterGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-patchbaseline-rulegroup.html
    pub struct RuleGroup_ {
        pub patch_rules: Option<Vec<Rule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_PatchBaseline_RuleGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::PatchBaseline.RuleGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_PatchBaseline_RuleGroup as RuleGroup;
    impl crate::value::ToValue for RuleGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.patch_rules {
                properties.insert(
                    "PatchRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod resourcedatasync {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-awsorganizationssource.html
    pub struct AwsOrganizationsSource_ {
        pub organization_source_type: crate::value::ExpString,
        pub organizational_units: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_ResourceDataSync_AwsOrganizationsSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::ResourceDataSync.AwsOrganizationsSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_ResourceDataSync_AwsOrganizationsSource as AwsOrganizationsSource;
    impl crate::value::ToValue for AwsOrganizationsSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OrganizationSourceType".to_string(),
                crate::value::ToValue::to_value(&self.organization_source_type),
            );
            if let Some(ref value) = self.organizational_units {
                properties.insert(
                    "OrganizationalUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-s3destination.html
    pub struct S3Destination_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub bucket_region: crate::value::ExpString,
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub sync_format: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_ResourceDataSync_S3Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::ResourceDataSync.S3Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_ResourceDataSync_S3Destination as S3Destination;
    impl crate::value::ToValue for S3Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BucketRegion".to_string(),
                crate::value::ToValue::to_value(&self.bucket_region),
            );
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KMSKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SyncFormat".to_string(),
                crate::value::ToValue::to_value(&self.sync_format),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ssm-resourcedatasync-syncsource.html
    pub struct SyncSource_ {
        pub aws_organizations_source: Option<Box<AwsOrganizationsSource_>>,
        pub include_future_regions: Option<crate::value::ExpBool>,
        pub source_regions: Vec<crate::value::ExpString>,
        pub source_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ssm_ResourceDataSync_SyncSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SSM::ResourceDataSync.SyncSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ssm_ResourceDataSync_SyncSource as SyncSource;
    impl crate::value::ToValue for SyncSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_organizations_source {
                properties.insert(
                    "AwsOrganizationsSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_future_regions {
                properties.insert(
                    "IncludeFutureRegions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceRegions".to_string(),
                crate::value::ToValue::to_value(&self.source_regions),
            );
            properties.insert(
                "SourceType".to_string(),
                crate::value::ToValue::to_value(&self.source_type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-association.html
pub struct Association_ {
    pub apply_only_at_cron_interval: Option<crate::value::ExpBool>,
    pub association_name: Option<crate::value::ExpString>,
    pub automation_target_parameter_name: Option<crate::value::ExpString>,
    pub calendar_names: Option<Vec<crate::value::ExpString>>,
    pub compliance_severity: Option<crate::value::ExpString>,
    pub document_version: Option<crate::value::ExpString>,
    pub instance_id: Option<crate::value::ExpString>,
    pub max_concurrency: Option<crate::value::ExpString>,
    pub max_errors: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub output_location: Option<super::ssm::association::InstanceAssociationOutputLocation_>,
    pub parameters: Option<serde_json::Value>,
    pub schedule_expression: Option<crate::value::ExpString>,
    pub schedule_offset: Option<i64>,
    pub sync_compliance: Option<crate::value::ExpString>,
    pub targets: Option<Vec<super::ssm::association::Target_>>,
    pub wait_for_success_timeout_seconds: Option<i64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_Association {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::Association" $($field
        $value)*)
    };
}
pub use crate::__aws_ssm_Association as Association;
impl crate::template::ToResource for Association_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Association"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.apply_only_at_cron_interval {
            properties.insert(
                "ApplyOnlyAtCronInterval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.association_name {
            properties.insert(
                "AssociationName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.automation_target_parameter_name {
            properties.insert(
                "AutomationTargetParameterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.calendar_names {
            properties.insert(
                "CalendarNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compliance_severity {
            properties.insert(
                "ComplianceSeverity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.document_version {
            properties.insert(
                "DocumentVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_id {
            properties.insert(
                "InstanceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_concurrency {
            properties.insert(
                "MaxConcurrency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_errors {
            properties.insert(
                "MaxErrors".to_string(),
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
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule_expression {
            properties.insert(
                "ScheduleExpression".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule_offset {
            properties.insert(
                "ScheduleOffset".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sync_compliance {
            properties.insert(
                "SyncCompliance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.targets {
            properties.insert(
                "Targets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.wait_for_success_timeout_seconds {
            properties.insert(
                "WaitForSuccessTimeoutSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-document.html
pub struct Document_ {
    pub attachments: Option<Vec<super::ssm::document::AttachmentsSource_>>,
    pub content: serde_json::Value,
    pub document_format: Option<crate::value::ExpString>,
    pub document_type: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub requires: Option<Vec<super::ssm::document::DocumentRequires_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_type: Option<crate::value::ExpString>,
    pub update_method: Option<crate::value::ExpString>,
    pub version_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_Document {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::Document" $($field
        $value)*)
    };
}
pub use crate::__aws_ssm_Document as Document;
impl crate::template::ToResource for Document_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Document"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attachments {
            properties.insert(
                "Attachments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.document_format {
            properties.insert(
                "DocumentFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.document_type {
            properties.insert(
                "DocumentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.requires {
            properties.insert(
                "Requires".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_type {
            properties.insert(
                "TargetType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.update_method {
            properties.insert(
                "UpdateMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.version_name {
            properties.insert(
                "VersionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindow.html
pub struct MaintenanceWindow_ {
    pub allow_unassociated_targets: crate::value::ExpBool,
    pub cutoff: i64,
    pub description: Option<crate::value::ExpString>,
    pub duration: i64,
    pub end_date: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub schedule: crate::value::ExpString,
    pub schedule_offset: Option<i64>,
    pub schedule_timezone: Option<crate::value::ExpString>,
    pub start_date: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_MaintenanceWindow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::MaintenanceWindow"
        $($field $value)*)
    };
}
pub use crate::__aws_ssm_MaintenanceWindow as MaintenanceWindow;
impl crate::template::ToResource for MaintenanceWindow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MaintenanceWindow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AllowUnassociatedTargets".to_string(),
            crate::value::ToValue::to_value(&self.allow_unassociated_targets),
        );
        properties.insert(
            "Cutoff".to_string(),
            crate::value::ToValue::to_value(&self.cutoff),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Duration".to_string(),
            crate::value::ToValue::to_value(&self.duration),
        );
        if let Some(ref value) = self.end_date {
            properties.insert(
                "EndDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Schedule".to_string(),
            crate::value::ToValue::to_value(&self.schedule),
        );
        if let Some(ref value) = self.schedule_offset {
            properties.insert(
                "ScheduleOffset".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule_timezone {
            properties.insert(
                "ScheduleTimezone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.start_date {
            properties.insert(
                "StartDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtarget.html
pub struct MaintenanceWindowTarget_ {
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub owner_information: Option<crate::value::ExpString>,
    pub resource_type: crate::value::ExpString,
    pub targets: Vec<super::ssm::maintenancewindowtarget::Targets_>,
    pub window_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_MaintenanceWindowTarget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::MaintenanceWindowTarget"
        $($field $value)*)
    };
}
pub use crate::__aws_ssm_MaintenanceWindowTarget as MaintenanceWindowTarget;
impl crate::template::ToResource for MaintenanceWindowTarget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MaintenanceWindowTarget"),
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
        if let Some(ref value) = self.owner_information {
            properties.insert(
                "OwnerInformation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        properties.insert(
            "Targets".to_string(),
            crate::value::ToValue::to_value(&self.targets),
        );
        properties.insert(
            "WindowId".to_string(),
            crate::value::ToValue::to_value(&self.window_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-maintenancewindowtask.html
pub struct MaintenanceWindowTask_ {
    pub cutoff_behavior: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub logging_info: Option<super::ssm::maintenancewindowtask::LoggingInfo_>,
    pub max_concurrency: Option<crate::value::ExpString>,
    pub max_errors: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub priority: i64,
    pub service_role_arn: Option<crate::value::ExpString>,
    pub targets: Option<Vec<super::ssm::maintenancewindowtask::Target_>>,
    pub task_arn: crate::value::ExpString,
    pub task_invocation_parameters:
        Option<super::ssm::maintenancewindowtask::TaskInvocationParameters_>,
    pub task_parameters: Option<serde_json::Value>,
    pub task_type: crate::value::ExpString,
    pub window_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_MaintenanceWindowTask {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::MaintenanceWindowTask"
        $($field $value)*)
    };
}
pub use crate::__aws_ssm_MaintenanceWindowTask as MaintenanceWindowTask;
impl crate::template::ToResource for MaintenanceWindowTask_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MaintenanceWindowTask"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cutoff_behavior {
            properties.insert(
                "CutoffBehavior".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_info {
            properties.insert(
                "LoggingInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_concurrency {
            properties.insert(
                "MaxConcurrency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_errors {
            properties.insert(
                "MaxErrors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Priority".to_string(),
            crate::value::ToValue::to_value(&self.priority),
        );
        if let Some(ref value) = self.service_role_arn {
            properties.insert(
                "ServiceRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.targets {
            properties.insert(
                "Targets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TaskArn".to_string(),
            crate::value::ToValue::to_value(&self.task_arn),
        );
        if let Some(ref value) = self.task_invocation_parameters {
            properties.insert(
                "TaskInvocationParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.task_parameters {
            properties.insert(
                "TaskParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TaskType".to_string(),
            crate::value::ToValue::to_value(&self.task_type),
        );
        properties.insert(
            "WindowId".to_string(),
            crate::value::ToValue::to_value(&self.window_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-parameter.html
pub struct Parameter_ {
    pub allowed_pattern: Option<crate::value::ExpString>,
    pub data_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub policies: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub tier: Option<crate::value::ExpString>,
    pub r#type: crate::value::ExpString,
    pub value: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_Parameter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::Parameter" $($field
        $value)*)
    };
}
pub use crate::__aws_ssm_Parameter as Parameter;
impl crate::template::ToResource for Parameter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Parameter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_pattern {
            properties.insert(
                "AllowedPattern".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_type {
            properties.insert(
                "DataType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.policies {
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tier {
            properties.insert("Tier".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties.insert(
            "Value".to_string(),
            crate::value::ToValue::to_value(&self.value),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-patchbaseline.html
pub struct PatchBaseline_ {
    pub approval_rules: Option<super::ssm::patchbaseline::RuleGroup_>,
    pub approved_patches: Option<Vec<crate::value::ExpString>>,
    pub approved_patches_compliance_level: Option<crate::value::ExpString>,
    pub approved_patches_enable_non_security: Option<crate::value::ExpBool>,
    pub available_security_updates_compliance_status: Option<crate::value::ExpString>,
    pub default_baseline: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub global_filters: Option<super::ssm::patchbaseline::PatchFilterGroup_>,
    pub name: crate::value::ExpString,
    pub operating_system: Option<crate::value::ExpString>,
    pub patch_groups: Option<Vec<crate::value::ExpString>>,
    pub rejected_patches: Option<Vec<crate::value::ExpString>>,
    pub rejected_patches_action: Option<crate::value::ExpString>,
    pub sources: Option<Vec<super::ssm::patchbaseline::PatchSource_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_PatchBaseline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::PatchBaseline"
        $($field $value)*)
    };
}
pub use crate::__aws_ssm_PatchBaseline as PatchBaseline;
impl crate::template::ToResource for PatchBaseline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PatchBaseline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.approval_rules {
            properties.insert(
                "ApprovalRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.approved_patches {
            properties.insert(
                "ApprovedPatches".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.approved_patches_compliance_level {
            properties.insert(
                "ApprovedPatchesComplianceLevel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.approved_patches_enable_non_security {
            properties.insert(
                "ApprovedPatchesEnableNonSecurity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.available_security_updates_compliance_status {
            properties.insert(
                "AvailableSecurityUpdatesComplianceStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_baseline {
            properties.insert(
                "DefaultBaseline".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_filters {
            properties.insert(
                "GlobalFilters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.operating_system {
            properties.insert(
                "OperatingSystem".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.patch_groups {
            properties.insert(
                "PatchGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rejected_patches {
            properties.insert(
                "RejectedPatches".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rejected_patches_action {
            properties.insert(
                "RejectedPatchesAction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sources {
            properties.insert(
                "Sources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcedatasync.html
pub struct ResourceDataSync_ {
    pub bucket_name: Option<crate::value::ExpString>,
    pub bucket_prefix: Option<crate::value::ExpString>,
    pub bucket_region: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub s3_destination: Option<super::ssm::resourcedatasync::S3Destination_>,
    pub sync_format: Option<crate::value::ExpString>,
    pub sync_name: crate::value::ExpString,
    pub sync_source: Option<super::ssm::resourcedatasync::SyncSource_>,
    pub sync_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_ResourceDataSync {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::ResourceDataSync"
        $($field $value)*)
    };
}
pub use crate::__aws_ssm_ResourceDataSync as ResourceDataSync;
impl crate::template::ToResource for ResourceDataSync_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceDataSync"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bucket_name {
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bucket_prefix {
            properties.insert(
                "BucketPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bucket_region {
            properties.insert(
                "BucketRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KMSKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_destination {
            properties.insert(
                "S3Destination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sync_format {
            properties.insert(
                "SyncFormat".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SyncName".to_string(),
            crate::value::ToValue::to_value(&self.sync_name),
        );
        if let Some(ref value) = self.sync_source {
            properties.insert(
                "SyncSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sync_type {
            properties.insert(
                "SyncType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ssm-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub policy: serde_json::Value,
    pub resource_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ssm_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SSM::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_ssm_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SSM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties
    }
}
