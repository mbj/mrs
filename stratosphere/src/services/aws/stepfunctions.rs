pub mod activity {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-activity-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub kms_data_key_reuse_period_seconds: Option<i64>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_Activity_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::Activity.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_Activity_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_data_key_reuse_period_seconds {
                properties.insert(
                    "KmsDataKeyReusePeriodSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-activity-tagsentry.html
    pub struct TagsEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_Activity_TagsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::Activity.TagsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_Activity_TagsEntry as TagsEntry;
    impl crate::value::ToValue for TagsEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
pub mod statemachine {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-cloudwatchlogsloggroup.html
    pub struct CloudWatchLogsLogGroup_ {
        pub log_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachine_CloudWatchLogsLogGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachine.CloudWatchLogsLogGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachine_CloudWatchLogsLogGroup as CloudWatchLogsLogGroup;
    impl crate::value::ToValue for CloudWatchLogsLogGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_group_arn {
                properties.insert(
                    "LogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub kms_data_key_reuse_period_seconds: Option<i64>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachine_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachine.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachine_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_data_key_reuse_period_seconds {
                properties.insert(
                    "KmsDataKeyReusePeriodSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-logdestination.html
    pub struct LogDestination_ {
        pub cloud_watch_logs_log_group: Option<Box<CloudWatchLogsLogGroup_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachine_LogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachine.LogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachine_LogDestination as LogDestination;
    impl crate::value::ToValue for LogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_log_group {
                properties.insert(
                    "CloudWatchLogsLogGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-loggingconfiguration.html
    pub struct LoggingConfiguration_ {
        pub destinations: Option<Vec<LogDestination_>>,
        pub include_execution_data: Option<crate::value::ExpBool>,
        pub level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachine_LoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachine.LoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachine_LoggingConfiguration as LoggingConfiguration;
    impl crate::value::ToValue for LoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destinations {
                properties.insert(
                    "Destinations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_execution_data {
                properties.insert(
                    "IncludeExecutionData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level {
                properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachine_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachine.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachine_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-tagsentry.html
    pub struct TagsEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachine_TagsEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachine.TagsEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachine_TagsEntry as TagsEntry;
    impl crate::value::ToValue for TagsEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachine-tracingconfiguration.html
    pub struct TracingConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachine_TracingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachine.TracingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachine_TracingConfiguration as TracingConfiguration;
    impl crate::value::ToValue for TracingConfiguration_ {
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
}
pub mod statemachinealias {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachinealias-deploymentpreference.html
    pub struct DeploymentPreference_ {
        pub alarms: Option<Vec<crate::value::ExpString>>,
        pub interval: Option<i64>,
        pub percentage: Option<i64>,
        pub state_machine_version_arn: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachineAlias_DeploymentPreference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachineAlias.DeploymentPreference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachineAlias_DeploymentPreference as DeploymentPreference;
    impl crate::value::ToValue for DeploymentPreference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarms {
                properties.insert("Alarms".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.percentage {
                properties.insert(
                    "Percentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StateMachineVersionArn".to_string(),
                crate::value::ToValue::to_value(&self.state_machine_version_arn),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-stepfunctions-statemachinealias-routingconfigurationversion.html
    pub struct RoutingConfigurationVersion_ {
        pub state_machine_version_arn: crate::value::ExpString,
        pub weight: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_stepfunctions_StateMachineAlias_RoutingConfigurationVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::StepFunctions::StateMachineAlias.RoutingConfigurationVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_stepfunctions_StateMachineAlias_RoutingConfigurationVersion as RoutingConfigurationVersion;
    impl crate::value::ToValue for RoutingConfigurationVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StateMachineVersionArn".to_string(),
                crate::value::ToValue::to_value(&self.state_machine_version_arn),
            );
            properties.insert(
                "Weight".to_string(),
                crate::value::ToValue::to_value(&self.weight),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-activity.html
pub struct Activity_ {
    pub encryption_configuration: Option<super::stepfunctions::activity::EncryptionConfiguration_>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<super::stepfunctions::activity::TagsEntry_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_stepfunctions_Activity {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::StepFunctions::Activity"
        $($field $value)*)
    };
}
pub use crate::__aws_stepfunctions_Activity as Activity;
impl crate::template::ToResource for Activity_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("StepFunctions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Activity"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachine.html
pub struct StateMachine_ {
    pub definition: Option<serde_json::Value>,
    pub definition_s3_location: Option<super::stepfunctions::statemachine::S3Location_>,
    pub definition_string: Option<crate::value::ExpString>,
    pub definition_substitutions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    pub encryption_configuration:
        Option<super::stepfunctions::statemachine::EncryptionConfiguration_>,
    pub logging_configuration: Option<super::stepfunctions::statemachine::LoggingConfiguration_>,
    pub role_arn: crate::value::ExpString,
    pub state_machine_name: Option<crate::value::ExpString>,
    pub state_machine_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::stepfunctions::statemachine::TagsEntry_>>,
    pub tracing_configuration: Option<super::stepfunctions::statemachine::TracingConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_stepfunctions_StateMachine {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::StepFunctions::StateMachine"
        $($field $value)*)
    };
}
pub use crate::__aws_stepfunctions_StateMachine as StateMachine;
impl crate::template::ToResource for StateMachine_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("StepFunctions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StateMachine"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.definition {
            properties.insert(
                "Definition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_s3_location {
            properties.insert(
                "DefinitionS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_string {
            properties.insert(
                "DefinitionString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_substitutions {
            properties.insert(
                "DefinitionSubstitutions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_configuration {
            properties.insert(
                "LoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.state_machine_name {
            properties.insert(
                "StateMachineName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state_machine_type {
            properties.insert(
                "StateMachineType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tracing_configuration {
            properties.insert(
                "TracingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachinealias.html
pub struct StateMachineAlias_ {
    pub deployment_preference:
        Option<super::stepfunctions::statemachinealias::DeploymentPreference_>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub routing_configuration:
        Option<Vec<super::stepfunctions::statemachinealias::RoutingConfigurationVersion_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_stepfunctions_StateMachineAlias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::StepFunctions::StateMachineAlias"
        $($field $value)*)
    };
}
pub use crate::__aws_stepfunctions_StateMachineAlias as StateMachineAlias;
impl crate::template::ToResource for StateMachineAlias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("StepFunctions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StateMachineAlias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deployment_preference {
            properties.insert(
                "DeploymentPreference".to_string(),
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
        if let Some(ref value) = self.routing_configuration {
            properties.insert(
                "RoutingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-stepfunctions-statemachineversion.html
pub struct StateMachineVersion_ {
    pub description: Option<crate::value::ExpString>,
    pub state_machine_arn: crate::value::ExpString,
    pub state_machine_revision_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_stepfunctions_StateMachineVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::StepFunctions::StateMachineVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_stepfunctions_StateMachineVersion as StateMachineVersion;
impl crate::template::ToResource for StateMachineVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("StepFunctions"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StateMachineVersion"),
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
            "StateMachineArn".to_string(),
            crate::value::ToValue::to_value(&self.state_machine_arn),
        );
        if let Some(ref value) = self.state_machine_revision_id {
            properties.insert(
                "StateMachineRevisionId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
