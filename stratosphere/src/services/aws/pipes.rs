pub mod pipe {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-awsvpcconfiguration.html
    pub struct AwsVpcConfiguration_ {
        pub assign_public_ip: Option<crate::value::ExpString>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_AwsVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.AwsVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_AwsVpcConfiguration as AwsVpcConfiguration;
    impl crate::value::ToValue for AwsVpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.assign_public_ip {
                properties.insert(
                    "AssignPublicIp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batcharrayproperties.html
    pub struct BatchArrayProperties_ {
        pub size: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_BatchArrayProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.BatchArrayProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_BatchArrayProperties as BatchArrayProperties;
    impl crate::value::ToValue for BatchArrayProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.size {
                properties.insert("Size".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchcontaineroverrides.html
    pub struct BatchContainerOverrides_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub environment: Option<Vec<BatchEnvironmentVariable_>>,
        pub instance_type: Option<crate::value::ExpString>,
        pub resource_requirements: Option<Vec<BatchResourceRequirement_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_BatchContainerOverrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.BatchContainerOverrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_BatchContainerOverrides as BatchContainerOverrides;
    impl crate::value::ToValue for BatchContainerOverrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_type {
                properties.insert(
                    "InstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_requirements {
                properties.insert(
                    "ResourceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchenvironmentvariable.html
    pub struct BatchEnvironmentVariable_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_BatchEnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.BatchEnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_BatchEnvironmentVariable as BatchEnvironmentVariable;
    impl crate::value::ToValue for BatchEnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchjobdependency.html
    pub struct BatchJobDependency_ {
        pub job_id: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_BatchJobDependency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.BatchJobDependency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_BatchJobDependency as BatchJobDependency;
    impl crate::value::ToValue for BatchJobDependency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.job_id {
                properties.insert("JobId".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchresourcerequirement.html
    pub struct BatchResourceRequirement_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_BatchResourceRequirement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.BatchResourceRequirement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_BatchResourceRequirement as BatchResourceRequirement;
    impl crate::value::ToValue for BatchResourceRequirement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-batchretrystrategy.html
    pub struct BatchRetryStrategy_ {
        pub attempts: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_BatchRetryStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.BatchRetryStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_BatchRetryStrategy as BatchRetryStrategy;
    impl crate::value::ToValue for BatchRetryStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attempts {
                properties.insert(
                    "Attempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-capacityproviderstrategyitem.html
    pub struct CapacityProviderStrategyItem_ {
        pub base: Option<i64>,
        pub capacity_provider: crate::value::ExpString,
        pub weight: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_CapacityProviderStrategyItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.CapacityProviderStrategyItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_CapacityProviderStrategyItem as CapacityProviderStrategyItem;
    impl crate::value::ToValue for CapacityProviderStrategyItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base {
                properties.insert("Base".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "CapacityProvider".to_string(),
                crate::value::ToValue::to_value(&self.capacity_provider),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-cloudwatchlogslogdestination.html
    pub struct CloudwatchLogsLogDestination_ {
        pub log_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_CloudwatchLogsLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.CloudwatchLogsLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_CloudwatchLogsLogDestination as CloudwatchLogsLogDestination;
    impl crate::value::ToValue for CloudwatchLogsLogDestination_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-deadletterconfig.html
    pub struct DeadLetterConfig_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_DeadLetterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.DeadLetterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_DeadLetterConfig as DeadLetterConfig;
    impl crate::value::ToValue for DeadLetterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-dimensionmapping.html
    pub struct DimensionMapping_ {
        pub dimension_name: crate::value::ExpString,
        pub dimension_value: crate::value::ExpString,
        pub dimension_value_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_DimensionMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.DimensionMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_DimensionMapping as DimensionMapping;
    impl crate::value::ToValue for DimensionMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionName".to_string(),
                crate::value::ToValue::to_value(&self.dimension_name),
            );
            properties.insert(
                "DimensionValue".to_string(),
                crate::value::ToValue::to_value(&self.dimension_value),
            );
            properties.insert(
                "DimensionValueType".to_string(),
                crate::value::ToValue::to_value(&self.dimension_value_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecscontaineroverride.html
    pub struct EcsContainerOverride_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub cpu: Option<i64>,
        pub environment: Option<Vec<EcsEnvironmentVariable_>>,
        pub environment_files: Option<Vec<EcsEnvironmentFile_>>,
        pub memory: Option<i64>,
        pub memory_reservation: Option<i64>,
        pub name: Option<crate::value::ExpString>,
        pub resource_requirements: Option<Vec<EcsResourceRequirement_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_EcsContainerOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.EcsContainerOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_EcsContainerOverride as EcsContainerOverride;
    impl crate::value::ToValue for EcsContainerOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cpu {
                properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_files {
                properties.insert(
                    "EnvironmentFiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory {
                properties.insert("Memory".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.memory_reservation {
                properties.insert(
                    "MemoryReservation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource_requirements {
                properties.insert(
                    "ResourceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentfile.html
    pub struct EcsEnvironmentFile_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_EcsEnvironmentFile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.EcsEnvironmentFile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_EcsEnvironmentFile as EcsEnvironmentFile;
    impl crate::value::ToValue for EcsEnvironmentFile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsenvironmentvariable.html
    pub struct EcsEnvironmentVariable_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_EcsEnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.EcsEnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_EcsEnvironmentVariable as EcsEnvironmentVariable;
    impl crate::value::ToValue for EcsEnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsephemeralstorage.html
    pub struct EcsEphemeralStorage_ {
        pub size_in_gi_b: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_EcsEphemeralStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.EcsEphemeralStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_EcsEphemeralStorage as EcsEphemeralStorage;
    impl crate::value::ToValue for EcsEphemeralStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SizeInGiB".to_string(),
                crate::value::ToValue::to_value(&self.size_in_gi_b),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsinferenceacceleratoroverride.html
    pub struct EcsInferenceAcceleratorOverride_ {
        pub device_name: Option<crate::value::ExpString>,
        pub device_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_EcsInferenceAcceleratorOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.EcsInferenceAcceleratorOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_EcsInferenceAcceleratorOverride as EcsInferenceAcceleratorOverride;
    impl crate::value::ToValue for EcsInferenceAcceleratorOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.device_name {
                properties.insert(
                    "DeviceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_type {
                properties.insert(
                    "DeviceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecsresourcerequirement.html
    pub struct EcsResourceRequirement_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_EcsResourceRequirement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.EcsResourceRequirement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_EcsResourceRequirement as EcsResourceRequirement;
    impl crate::value::ToValue for EcsResourceRequirement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-ecstaskoverride.html
    pub struct EcsTaskOverride_ {
        pub container_overrides: Option<Vec<EcsContainerOverride_>>,
        pub cpu: Option<crate::value::ExpString>,
        pub ephemeral_storage: Option<Box<EcsEphemeralStorage_>>,
        pub execution_role_arn: Option<crate::value::ExpString>,
        pub inference_accelerator_overrides: Option<Vec<EcsInferenceAcceleratorOverride_>>,
        pub memory: Option<crate::value::ExpString>,
        pub task_role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_EcsTaskOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.EcsTaskOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_EcsTaskOverride as EcsTaskOverride;
    impl crate::value::ToValue for EcsTaskOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_overrides {
                properties.insert(
                    "ContainerOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cpu {
                properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ephemeral_storage {
                properties.insert(
                    "EphemeralStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role_arn {
                properties.insert(
                    "ExecutionRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_accelerator_overrides {
                properties.insert(
                    "InferenceAcceleratorOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory {
                properties.insert("Memory".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.task_role_arn {
                properties.insert(
                    "TaskRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-filter.html
    pub struct Filter_ {
        pub pattern: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pattern {
                properties.insert(
                    "Pattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-filtercriteria.html
    pub struct FilterCriteria_ {
        pub filters: Option<Vec<Filter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_FilterCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.FilterCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_FilterCriteria as FilterCriteria;
    impl crate::value::ToValue for FilterCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-firehoselogdestination.html
    pub struct FirehoseLogDestination_ {
        pub delivery_stream_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_FirehoseLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.FirehoseLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_FirehoseLogDestination as FirehoseLogDestination;
    impl crate::value::ToValue for FirehoseLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delivery_stream_arn {
                properties.insert(
                    "DeliveryStreamArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-mqbrokeraccesscredentials.html
    pub struct MQBrokerAccessCredentials_ {
        pub basic_auth: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_MQBrokerAccessCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.MQBrokerAccessCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_MQBrokerAccessCredentials as MQBrokerAccessCredentials;
    impl crate::value::ToValue for MQBrokerAccessCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BasicAuth".to_string(),
                crate::value::ToValue::to_value(&self.basic_auth),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-mskaccesscredentials.html
    pub struct MSKAccessCredentials_ {
        pub client_certificate_tls_auth: Option<crate::value::ExpString>,
        pub sasl_scram512_auth: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_MSKAccessCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.MSKAccessCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_MSKAccessCredentials as MSKAccessCredentials;
    impl crate::value::ToValue for MSKAccessCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_certificate_tls_auth {
                properties.insert(
                    "ClientCertificateTlsAuth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sasl_scram512_auth {
                properties.insert(
                    "SaslScram512Auth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-multimeasureattributemapping.html
    pub struct MultiMeasureAttributeMapping_ {
        pub measure_value: crate::value::ExpString,
        pub measure_value_type: crate::value::ExpString,
        pub multi_measure_attribute_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_MultiMeasureAttributeMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.MultiMeasureAttributeMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_MultiMeasureAttributeMapping as MultiMeasureAttributeMapping;
    impl crate::value::ToValue for MultiMeasureAttributeMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MeasureValue".to_string(),
                crate::value::ToValue::to_value(&self.measure_value),
            );
            properties.insert(
                "MeasureValueType".to_string(),
                crate::value::ToValue::to_value(&self.measure_value_type),
            );
            properties.insert(
                "MultiMeasureAttributeName".to_string(),
                crate::value::ToValue::to_value(&self.multi_measure_attribute_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-multimeasuremapping.html
    pub struct MultiMeasureMapping_ {
        pub multi_measure_attribute_mappings: Vec<MultiMeasureAttributeMapping_>,
        pub multi_measure_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_MultiMeasureMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.MultiMeasureMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_MultiMeasureMapping as MultiMeasureMapping;
    impl crate::value::ToValue for MultiMeasureMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MultiMeasureAttributeMappings".to_string(),
                crate::value::ToValue::to_value(&self.multi_measure_attribute_mappings),
            );
            properties.insert(
                "MultiMeasureName".to_string(),
                crate::value::ToValue::to_value(&self.multi_measure_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub awsvpc_configuration: Option<Box<AwsVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.awsvpc_configuration {
                properties.insert(
                    "AwsvpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmenthttpparameters.html
    pub struct PipeEnrichmentHttpParameters_ {
        pub header_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub path_parameter_values: Option<Vec<crate::value::ExpString>>,
        pub query_string_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeEnrichmentHttpParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeEnrichmentHttpParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeEnrichmentHttpParameters as PipeEnrichmentHttpParameters;
    impl crate::value::ToValue for PipeEnrichmentHttpParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header_parameters {
                properties.insert(
                    "HeaderParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path_parameter_values {
                properties.insert(
                    "PathParameterValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_string_parameters {
                properties.insert(
                    "QueryStringParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipeenrichmentparameters.html
    pub struct PipeEnrichmentParameters_ {
        pub http_parameters: Option<Box<PipeEnrichmentHttpParameters_>>,
        pub input_template: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeEnrichmentParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeEnrichmentParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeEnrichmentParameters as PipeEnrichmentParameters;
    impl crate::value::ToValue for PipeEnrichmentParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http_parameters {
                properties.insert(
                    "HttpParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_template {
                properties.insert(
                    "InputTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipelogconfiguration.html
    pub struct PipeLogConfiguration_ {
        pub cloudwatch_logs_log_destination: Option<Box<CloudwatchLogsLogDestination_>>,
        pub firehose_log_destination: Option<Box<FirehoseLogDestination_>>,
        pub include_execution_data: Option<Vec<crate::value::ExpString>>,
        pub level: Option<crate::value::ExpString>,
        pub s3_log_destination: Option<Box<S3LogDestination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeLogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeLogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeLogConfiguration as PipeLogConfiguration;
    impl crate::value::ToValue for PipeLogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloudwatch_logs_log_destination {
                properties.insert(
                    "CloudwatchLogsLogDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firehose_log_destination {
                properties.insert(
                    "FirehoseLogDestination".to_string(),
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
            if let Some(ref value) = self.s3_log_destination {
                properties.insert(
                    "S3LogDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceactivemqbrokerparameters.html
    pub struct PipeSourceActiveMQBrokerParameters_ {
        pub batch_size: Option<i64>,
        pub credentials: Box<MQBrokerAccessCredentials_>,
        pub maximum_batching_window_in_seconds: Option<i64>,
        pub queue_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceActiveMQBrokerParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceActiveMQBrokerParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceActiveMQBrokerParameters as PipeSourceActiveMQBrokerParameters;
    impl crate::value::ToValue for PipeSourceActiveMQBrokerParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_size {
                properties.insert(
                    "BatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Credentials".to_string(),
                crate::value::ToValue::to_value(&self.credentials),
            );
            if let Some(ref value) = self.maximum_batching_window_in_seconds {
                properties.insert(
                    "MaximumBatchingWindowInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueueName".to_string(),
                crate::value::ToValue::to_value(&self.queue_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcedynamodbstreamparameters.html
    pub struct PipeSourceDynamoDBStreamParameters_ {
        pub batch_size: Option<i64>,
        pub dead_letter_config: Option<Box<DeadLetterConfig_>>,
        pub maximum_batching_window_in_seconds: Option<i64>,
        pub maximum_record_age_in_seconds: Option<i64>,
        pub maximum_retry_attempts: Option<i64>,
        pub on_partial_batch_item_failure: Option<crate::value::ExpString>,
        pub parallelization_factor: Option<i64>,
        pub starting_position: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceDynamoDBStreamParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceDynamoDBStreamParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceDynamoDBStreamParameters as PipeSourceDynamoDBStreamParameters;
    impl crate::value::ToValue for PipeSourceDynamoDBStreamParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_size {
                properties.insert(
                    "BatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dead_letter_config {
                properties.insert(
                    "DeadLetterConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_batching_window_in_seconds {
                properties.insert(
                    "MaximumBatchingWindowInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_record_age_in_seconds {
                properties.insert(
                    "MaximumRecordAgeInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_retry_attempts {
                properties.insert(
                    "MaximumRetryAttempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_partial_batch_item_failure {
                properties.insert(
                    "OnPartialBatchItemFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallelization_factor {
                properties.insert(
                    "ParallelizationFactor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StartingPosition".to_string(),
                crate::value::ToValue::to_value(&self.starting_position),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcekinesisstreamparameters.html
    pub struct PipeSourceKinesisStreamParameters_ {
        pub batch_size: Option<i64>,
        pub dead_letter_config: Option<Box<DeadLetterConfig_>>,
        pub maximum_batching_window_in_seconds: Option<i64>,
        pub maximum_record_age_in_seconds: Option<i64>,
        pub maximum_retry_attempts: Option<i64>,
        pub on_partial_batch_item_failure: Option<crate::value::ExpString>,
        pub parallelization_factor: Option<i64>,
        pub starting_position: crate::value::ExpString,
        pub starting_position_timestamp: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceKinesisStreamParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceKinesisStreamParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceKinesisStreamParameters as PipeSourceKinesisStreamParameters;
    impl crate::value::ToValue for PipeSourceKinesisStreamParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_size {
                properties.insert(
                    "BatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dead_letter_config {
                properties.insert(
                    "DeadLetterConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_batching_window_in_seconds {
                properties.insert(
                    "MaximumBatchingWindowInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_record_age_in_seconds {
                properties.insert(
                    "MaximumRecordAgeInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_retry_attempts {
                properties.insert(
                    "MaximumRetryAttempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.on_partial_batch_item_failure {
                properties.insert(
                    "OnPartialBatchItemFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parallelization_factor {
                properties.insert(
                    "ParallelizationFactor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StartingPosition".to_string(),
                crate::value::ToValue::to_value(&self.starting_position),
            );
            if let Some(ref value) = self.starting_position_timestamp {
                properties.insert(
                    "StartingPositionTimestamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcemanagedstreamingkafkaparameters.html
    pub struct PipeSourceManagedStreamingKafkaParameters_ {
        pub batch_size: Option<i64>,
        pub consumer_group_id: Option<crate::value::ExpString>,
        pub credentials: Option<Box<MSKAccessCredentials_>>,
        pub maximum_batching_window_in_seconds: Option<i64>,
        pub starting_position: Option<crate::value::ExpString>,
        pub topic_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceManagedStreamingKafkaParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceManagedStreamingKafkaParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceManagedStreamingKafkaParameters as PipeSourceManagedStreamingKafkaParameters;
    impl crate::value::ToValue for PipeSourceManagedStreamingKafkaParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_size {
                properties.insert(
                    "BatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.consumer_group_id {
                properties.insert(
                    "ConsumerGroupID".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.credentials {
                properties.insert(
                    "Credentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_batching_window_in_seconds {
                properties.insert(
                    "MaximumBatchingWindowInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starting_position {
                properties.insert(
                    "StartingPosition".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceparameters.html
    pub struct PipeSourceParameters_ {
        pub active_mq_broker_parameters: Option<Box<PipeSourceActiveMQBrokerParameters_>>,
        pub dynamo_db_stream_parameters: Option<Box<PipeSourceDynamoDBStreamParameters_>>,
        pub filter_criteria: Option<Box<FilterCriteria_>>,
        pub kinesis_stream_parameters: Option<Box<PipeSourceKinesisStreamParameters_>>,
        pub managed_streaming_kafka_parameters:
            Option<Box<PipeSourceManagedStreamingKafkaParameters_>>,
        pub rabbit_mq_broker_parameters: Option<Box<PipeSourceRabbitMQBrokerParameters_>>,
        pub self_managed_kafka_parameters: Option<Box<PipeSourceSelfManagedKafkaParameters_>>,
        pub sqs_queue_parameters: Option<Box<PipeSourceSqsQueueParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceParameters as PipeSourceParameters;
    impl crate::value::ToValue for PipeSourceParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.active_mq_broker_parameters {
                properties.insert(
                    "ActiveMQBrokerParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dynamo_db_stream_parameters {
                properties.insert(
                    "DynamoDBStreamParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_criteria {
                properties.insert(
                    "FilterCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_stream_parameters {
                properties.insert(
                    "KinesisStreamParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_streaming_kafka_parameters {
                properties.insert(
                    "ManagedStreamingKafkaParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rabbit_mq_broker_parameters {
                properties.insert(
                    "RabbitMQBrokerParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_managed_kafka_parameters {
                properties.insert(
                    "SelfManagedKafkaParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sqs_queue_parameters {
                properties.insert(
                    "SqsQueueParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcerabbitmqbrokerparameters.html
    pub struct PipeSourceRabbitMQBrokerParameters_ {
        pub batch_size: Option<i64>,
        pub credentials: Box<MQBrokerAccessCredentials_>,
        pub maximum_batching_window_in_seconds: Option<i64>,
        pub queue_name: crate::value::ExpString,
        pub virtual_host: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceRabbitMQBrokerParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceRabbitMQBrokerParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceRabbitMQBrokerParameters as PipeSourceRabbitMQBrokerParameters;
    impl crate::value::ToValue for PipeSourceRabbitMQBrokerParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_size {
                properties.insert(
                    "BatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Credentials".to_string(),
                crate::value::ToValue::to_value(&self.credentials),
            );
            if let Some(ref value) = self.maximum_batching_window_in_seconds {
                properties.insert(
                    "MaximumBatchingWindowInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueueName".to_string(),
                crate::value::ToValue::to_value(&self.queue_name),
            );
            if let Some(ref value) = self.virtual_host {
                properties.insert(
                    "VirtualHost".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourceselfmanagedkafkaparameters.html
    pub struct PipeSourceSelfManagedKafkaParameters_ {
        pub additional_bootstrap_servers: Option<Vec<crate::value::ExpString>>,
        pub batch_size: Option<i64>,
        pub consumer_group_id: Option<crate::value::ExpString>,
        pub credentials: Option<Box<SelfManagedKafkaAccessConfigurationCredentials_>>,
        pub maximum_batching_window_in_seconds: Option<i64>,
        pub server_root_ca_certificate: Option<crate::value::ExpString>,
        pub starting_position: Option<crate::value::ExpString>,
        pub topic_name: crate::value::ExpString,
        pub vpc: Option<Box<SelfManagedKafkaAccessConfigurationVpc_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceSelfManagedKafkaParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceSelfManagedKafkaParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceSelfManagedKafkaParameters as PipeSourceSelfManagedKafkaParameters;
    impl crate::value::ToValue for PipeSourceSelfManagedKafkaParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_bootstrap_servers {
                properties.insert(
                    "AdditionalBootstrapServers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.batch_size {
                properties.insert(
                    "BatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.consumer_group_id {
                properties.insert(
                    "ConsumerGroupID".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.credentials {
                properties.insert(
                    "Credentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_batching_window_in_seconds {
                properties.insert(
                    "MaximumBatchingWindowInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_root_ca_certificate {
                properties.insert(
                    "ServerRootCaCertificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starting_position {
                properties.insert(
                    "StartingPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TopicName".to_string(),
                crate::value::ToValue::to_value(&self.topic_name),
            );
            if let Some(ref value) = self.vpc {
                properties.insert("Vpc".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipesourcesqsqueueparameters.html
    pub struct PipeSourceSqsQueueParameters_ {
        pub batch_size: Option<i64>,
        pub maximum_batching_window_in_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeSourceSqsQueueParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeSourceSqsQueueParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeSourceSqsQueueParameters as PipeSourceSqsQueueParameters;
    impl crate::value::ToValue for PipeSourceSqsQueueParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_size {
                properties.insert(
                    "BatchSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_batching_window_in_seconds {
                properties.insert(
                    "MaximumBatchingWindowInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetbatchjobparameters.html
    pub struct PipeTargetBatchJobParameters_ {
        pub array_properties: Option<Box<BatchArrayProperties_>>,
        pub container_overrides: Option<Box<BatchContainerOverrides_>>,
        pub depends_on: Option<Vec<BatchJobDependency_>>,
        pub job_definition: crate::value::ExpString,
        pub job_name: crate::value::ExpString,
        pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub retry_strategy: Option<Box<BatchRetryStrategy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetBatchJobParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetBatchJobParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetBatchJobParameters as PipeTargetBatchJobParameters;
    impl crate::value::ToValue for PipeTargetBatchJobParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.array_properties {
                properties.insert(
                    "ArrayProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_overrides {
                properties.insert(
                    "ContainerOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.depends_on {
                properties.insert(
                    "DependsOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "JobDefinition".to_string(),
                crate::value::ToValue::to_value(&self.job_definition),
            );
            properties.insert(
                "JobName".to_string(),
                crate::value::ToValue::to_value(&self.job_name),
            );
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_strategy {
                properties.insert(
                    "RetryStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetcloudwatchlogsparameters.html
    pub struct PipeTargetCloudWatchLogsParameters_ {
        pub log_stream_name: Option<crate::value::ExpString>,
        pub timestamp: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetCloudWatchLogsParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetCloudWatchLogsParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetCloudWatchLogsParameters as PipeTargetCloudWatchLogsParameters;
    impl crate::value::ToValue for PipeTargetCloudWatchLogsParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_stream_name {
                properties.insert(
                    "LogStreamName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestamp {
                properties.insert(
                    "Timestamp".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetecstaskparameters.html
    pub struct PipeTargetEcsTaskParameters_ {
        pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem_>>,
        pub enable_ecs_managed_tags: Option<crate::value::ExpBool>,
        pub enable_execute_command: Option<crate::value::ExpBool>,
        pub group: Option<crate::value::ExpString>,
        pub launch_type: Option<crate::value::ExpString>,
        pub network_configuration: Option<Box<NetworkConfiguration_>>,
        pub overrides: Option<Box<EcsTaskOverride_>>,
        pub placement_constraints: Option<Vec<PlacementConstraint_>>,
        pub placement_strategy: Option<Vec<PlacementStrategy_>>,
        pub platform_version: Option<crate::value::ExpString>,
        pub propagate_tags: Option<crate::value::ExpString>,
        pub reference_id: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
        pub task_count: Option<i64>,
        pub task_definition_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetEcsTaskParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetEcsTaskParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetEcsTaskParameters as PipeTargetEcsTaskParameters;
    impl crate::value::ToValue for PipeTargetEcsTaskParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_provider_strategy {
                properties.insert(
                    "CapacityProviderStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_ecs_managed_tags {
                properties.insert(
                    "EnableECSManagedTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_execute_command {
                properties.insert(
                    "EnableExecuteCommand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group {
                properties.insert("Group".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.launch_type {
                properties.insert(
                    "LaunchType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_configuration {
                properties.insert(
                    "NetworkConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement_constraints {
                properties.insert(
                    "PlacementConstraints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement_strategy {
                properties.insert(
                    "PlacementStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.platform_version {
                properties.insert(
                    "PlatformVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.propagate_tags {
                properties.insert(
                    "PropagateTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reference_id {
                properties.insert(
                    "ReferenceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.task_count {
                properties.insert(
                    "TaskCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TaskDefinitionArn".to_string(),
                crate::value::ToValue::to_value(&self.task_definition_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargeteventbridgeeventbusparameters.html
    pub struct PipeTargetEventBridgeEventBusParameters_ {
        pub detail_type: Option<crate::value::ExpString>,
        pub endpoint_id: Option<crate::value::ExpString>,
        pub resources: Option<Vec<crate::value::ExpString>>,
        pub source: Option<crate::value::ExpString>,
        pub time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetEventBridgeEventBusParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetEventBridgeEventBusParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetEventBridgeEventBusParameters as PipeTargetEventBridgeEventBusParameters;
    impl crate::value::ToValue for PipeTargetEventBridgeEventBusParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.detail_type {
                properties.insert(
                    "DetailType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_id {
                properties.insert(
                    "EndpointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resources {
                properties.insert(
                    "Resources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.time {
                properties.insert("Time".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargethttpparameters.html
    pub struct PipeTargetHttpParameters_ {
        pub header_parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub path_parameter_values: Option<Vec<crate::value::ExpString>>,
        pub query_string_parameters:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetHttpParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetHttpParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetHttpParameters as PipeTargetHttpParameters;
    impl crate::value::ToValue for PipeTargetHttpParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header_parameters {
                properties.insert(
                    "HeaderParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path_parameter_values {
                properties.insert(
                    "PathParameterValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_string_parameters {
                properties.insert(
                    "QueryStringParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetkinesisstreamparameters.html
    pub struct PipeTargetKinesisStreamParameters_ {
        pub partition_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetKinesisStreamParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetKinesisStreamParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetKinesisStreamParameters as PipeTargetKinesisStreamParameters;
    impl crate::value::ToValue for PipeTargetKinesisStreamParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PartitionKey".to_string(),
                crate::value::ToValue::to_value(&self.partition_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetlambdafunctionparameters.html
    pub struct PipeTargetLambdaFunctionParameters_ {
        pub invocation_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetLambdaFunctionParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetLambdaFunctionParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetLambdaFunctionParameters as PipeTargetLambdaFunctionParameters;
    impl crate::value::ToValue for PipeTargetLambdaFunctionParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invocation_type {
                properties.insert(
                    "InvocationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetparameters.html
    pub struct PipeTargetParameters_ {
        pub batch_job_parameters: Option<Box<PipeTargetBatchJobParameters_>>,
        pub cloud_watch_logs_parameters: Option<Box<PipeTargetCloudWatchLogsParameters_>>,
        pub ecs_task_parameters: Option<Box<PipeTargetEcsTaskParameters_>>,
        pub event_bridge_event_bus_parameters:
            Option<Box<PipeTargetEventBridgeEventBusParameters_>>,
        pub http_parameters: Option<Box<PipeTargetHttpParameters_>>,
        pub input_template: Option<crate::value::ExpString>,
        pub kinesis_stream_parameters: Option<Box<PipeTargetKinesisStreamParameters_>>,
        pub lambda_function_parameters: Option<Box<PipeTargetLambdaFunctionParameters_>>,
        pub redshift_data_parameters: Option<Box<PipeTargetRedshiftDataParameters_>>,
        pub sage_maker_pipeline_parameters: Option<Box<PipeTargetSageMakerPipelineParameters_>>,
        pub sqs_queue_parameters: Option<Box<PipeTargetSqsQueueParameters_>>,
        pub step_function_state_machine_parameters: Option<Box<PipeTargetStateMachineParameters_>>,
        pub timestream_parameters: Option<Box<PipeTargetTimestreamParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetParameters as PipeTargetParameters;
    impl crate::value::ToValue for PipeTargetParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.batch_job_parameters {
                properties.insert(
                    "BatchJobParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_logs_parameters {
                properties.insert(
                    "CloudWatchLogsParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecs_task_parameters {
                properties.insert(
                    "EcsTaskParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_bridge_event_bus_parameters {
                properties.insert(
                    "EventBridgeEventBusParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_parameters {
                properties.insert(
                    "HttpParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_template {
                properties.insert(
                    "InputTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_stream_parameters {
                properties.insert(
                    "KinesisStreamParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function_parameters {
                properties.insert(
                    "LambdaFunctionParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_data_parameters {
                properties.insert(
                    "RedshiftDataParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sage_maker_pipeline_parameters {
                properties.insert(
                    "SageMakerPipelineParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sqs_queue_parameters {
                properties.insert(
                    "SqsQueueParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.step_function_state_machine_parameters {
                properties.insert(
                    "StepFunctionStateMachineParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timestream_parameters {
                properties.insert(
                    "TimestreamParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetredshiftdataparameters.html
    pub struct PipeTargetRedshiftDataParameters_ {
        pub database: crate::value::ExpString,
        pub db_user: Option<crate::value::ExpString>,
        pub secret_manager_arn: Option<crate::value::ExpString>,
        pub sqls: Vec<crate::value::ExpString>,
        pub statement_name: Option<crate::value::ExpString>,
        pub with_event: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetRedshiftDataParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetRedshiftDataParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetRedshiftDataParameters as PipeTargetRedshiftDataParameters;
    impl crate::value::ToValue for PipeTargetRedshiftDataParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Database".to_string(),
                crate::value::ToValue::to_value(&self.database),
            );
            if let Some(ref value) = self.db_user {
                properties.insert("DbUser".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.secret_manager_arn {
                properties.insert(
                    "SecretManagerArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Sqls".to_string(),
                crate::value::ToValue::to_value(&self.sqls),
            );
            if let Some(ref value) = self.statement_name {
                properties.insert(
                    "StatementName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.with_event {
                properties.insert(
                    "WithEvent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetsagemakerpipelineparameters.html
    pub struct PipeTargetSageMakerPipelineParameters_ {
        pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetSageMakerPipelineParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetSageMakerPipelineParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetSageMakerPipelineParameters as PipeTargetSageMakerPipelineParameters;
    impl crate::value::ToValue for PipeTargetSageMakerPipelineParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pipeline_parameter_list {
                properties.insert(
                    "PipelineParameterList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetsqsqueueparameters.html
    pub struct PipeTargetSqsQueueParameters_ {
        pub message_deduplication_id: Option<crate::value::ExpString>,
        pub message_group_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetSqsQueueParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetSqsQueueParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetSqsQueueParameters as PipeTargetSqsQueueParameters;
    impl crate::value::ToValue for PipeTargetSqsQueueParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message_deduplication_id {
                properties.insert(
                    "MessageDeduplicationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_group_id {
                properties.insert(
                    "MessageGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargetstatemachineparameters.html
    pub struct PipeTargetStateMachineParameters_ {
        pub invocation_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetStateMachineParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetStateMachineParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetStateMachineParameters as PipeTargetStateMachineParameters;
    impl crate::value::ToValue for PipeTargetStateMachineParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invocation_type {
                properties.insert(
                    "InvocationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-pipetargettimestreamparameters.html
    pub struct PipeTargetTimestreamParameters_ {
        pub dimension_mappings: Vec<DimensionMapping_>,
        pub epoch_time_unit: Option<crate::value::ExpString>,
        pub multi_measure_mappings: Option<Vec<MultiMeasureMapping_>>,
        pub single_measure_mappings: Option<Vec<SingleMeasureMapping_>>,
        pub time_field_type: Option<crate::value::ExpString>,
        pub time_value: crate::value::ExpString,
        pub timestamp_format: Option<crate::value::ExpString>,
        pub version_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PipeTargetTimestreamParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PipeTargetTimestreamParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PipeTargetTimestreamParameters as PipeTargetTimestreamParameters;
    impl crate::value::ToValue for PipeTargetTimestreamParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DimensionMappings".to_string(),
                crate::value::ToValue::to_value(&self.dimension_mappings),
            );
            if let Some(ref value) = self.epoch_time_unit {
                properties.insert(
                    "EpochTimeUnit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multi_measure_mappings {
                properties.insert(
                    "MultiMeasureMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_measure_mappings {
                properties.insert(
                    "SingleMeasureMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_field_type {
                properties.insert(
                    "TimeFieldType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeValue".to_string(),
                crate::value::ToValue::to_value(&self.time_value),
            );
            if let Some(ref value) = self.timestamp_format {
                properties.insert(
                    "TimestampFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VersionValue".to_string(),
                crate::value::ToValue::to_value(&self.version_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementconstraint.html
    pub struct PlacementConstraint_ {
        pub expression: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PlacementConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PlacementConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PlacementConstraint as PlacementConstraint;
    impl crate::value::ToValue for PlacementConstraint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-placementstrategy.html
    pub struct PlacementStrategy_ {
        pub field: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_PlacementStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.PlacementStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_PlacementStrategy as PlacementStrategy;
    impl crate::value::ToValue for PlacementStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-s3logdestination.html
    pub struct S3LogDestination_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub bucket_owner: Option<crate::value::ExpString>,
        pub output_format: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_S3LogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.S3LogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_S3LogDestination as S3LogDestination;
    impl crate::value::ToValue for S3LogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_owner {
                properties.insert(
                    "BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_format {
                properties.insert(
                    "OutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-sagemakerpipelineparameter.html
    pub struct SageMakerPipelineParameter_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_SageMakerPipelineParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.SageMakerPipelineParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_SageMakerPipelineParameter as SageMakerPipelineParameter;
    impl crate::value::ToValue for SageMakerPipelineParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationcredentials.html
    pub struct SelfManagedKafkaAccessConfigurationCredentials_ {
        pub basic_auth: Option<crate::value::ExpString>,
        pub client_certificate_tls_auth: Option<crate::value::ExpString>,
        pub sasl_scram256_auth: Option<crate::value::ExpString>,
        pub sasl_scram512_auth: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_SelfManagedKafkaAccessConfigurationCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.SelfManagedKafkaAccessConfigurationCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_SelfManagedKafkaAccessConfigurationCredentials as SelfManagedKafkaAccessConfigurationCredentials;
    impl crate::value::ToValue for SelfManagedKafkaAccessConfigurationCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.basic_auth {
                properties.insert(
                    "BasicAuth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.client_certificate_tls_auth {
                properties.insert(
                    "ClientCertificateTlsAuth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sasl_scram256_auth {
                properties.insert(
                    "SaslScram256Auth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sasl_scram512_auth {
                properties.insert(
                    "SaslScram512Auth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-selfmanagedkafkaaccessconfigurationvpc.html
    pub struct SelfManagedKafkaAccessConfigurationVpc_ {
        pub security_group: Option<Vec<crate::value::ExpString>>,
        pub subnets: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_SelfManagedKafkaAccessConfigurationVpc {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.SelfManagedKafkaAccessConfigurationVpc"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_SelfManagedKafkaAccessConfigurationVpc as SelfManagedKafkaAccessConfigurationVpc;
    impl crate::value::ToValue for SelfManagedKafkaAccessConfigurationVpc_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group {
                properties.insert(
                    "SecurityGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnets {
                properties.insert(
                    "Subnets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pipes-pipe-singlemeasuremapping.html
    pub struct SingleMeasureMapping_ {
        pub measure_name: crate::value::ExpString,
        pub measure_value: crate::value::ExpString,
        pub measure_value_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pipes_Pipe_SingleMeasureMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Pipes::Pipe.SingleMeasureMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pipes_Pipe_SingleMeasureMapping as SingleMeasureMapping;
    impl crate::value::ToValue for SingleMeasureMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MeasureName".to_string(),
                crate::value::ToValue::to_value(&self.measure_name),
            );
            properties.insert(
                "MeasureValue".to_string(),
                crate::value::ToValue::to_value(&self.measure_value),
            );
            properties.insert(
                "MeasureValueType".to_string(),
                crate::value::ToValue::to_value(&self.measure_value_type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pipes-pipe.html
pub struct Pipe_ {
    pub description: Option<crate::value::ExpString>,
    pub desired_state: Option<crate::value::ExpString>,
    pub enrichment: Option<crate::value::ExpString>,
    pub enrichment_parameters: Option<super::pipes::pipe::PipeEnrichmentParameters_>,
    pub kms_key_identifier: Option<crate::value::ExpString>,
    pub log_configuration: Option<super::pipes::pipe::PipeLogConfiguration_>,
    pub name: Option<crate::value::ExpString>,
    pub role_arn: crate::value::ExpString,
    pub source: crate::value::ExpString,
    pub source_parameters: Option<super::pipes::pipe::PipeSourceParameters_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub target: crate::value::ExpString,
    pub target_parameters: Option<super::pipes::pipe::PipeTargetParameters_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pipes_Pipe {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Pipes::Pipe" $($field
        $value)*)
    };
}
pub use crate::__aws_pipes_Pipe as Pipe;
impl crate::template::ToResource for Pipe_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pipes"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Pipe"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.desired_state {
            properties.insert(
                "DesiredState".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enrichment {
            properties.insert(
                "Enrichment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enrichment_parameters {
            properties.insert(
                "EnrichmentParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_identifier {
            properties.insert(
                "KmsKeyIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_configuration {
            properties.insert(
                "LogConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        if let Some(ref value) = self.source_parameters {
            properties.insert(
                "SourceParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Target".to_string(),
            crate::value::ToValue::to_value(&self.target),
        );
        if let Some(ref value) = self.target_parameters {
            properties.insert(
                "TargetParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
