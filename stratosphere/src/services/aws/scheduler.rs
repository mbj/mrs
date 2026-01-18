pub mod schedule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-awsvpcconfiguration.html
    pub struct AwsVpcConfiguration_ {
        pub assign_public_ip: Option<crate::value::ExpString>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_AwsVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.AwsVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_AwsVpcConfiguration as AwsVpcConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-capacityproviderstrategyitem.html
    pub struct CapacityProviderStrategyItem_ {
        pub base: Option<f64>,
        pub capacity_provider: crate::value::ExpString,
        pub weight: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_CapacityProviderStrategyItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.CapacityProviderStrategyItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_CapacityProviderStrategyItem as CapacityProviderStrategyItem;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-deadletterconfig.html
    pub struct DeadLetterConfig_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_DeadLetterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.DeadLetterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_DeadLetterConfig as DeadLetterConfig;
    impl crate::value::ToValue for DeadLetterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-ecsparameters.html
    pub struct EcsParameters_ {
        pub capacity_provider_strategy: Option<Vec<CapacityProviderStrategyItem_>>,
        pub enable_ecs_managed_tags: Option<crate::value::ExpBool>,
        pub enable_execute_command: Option<crate::value::ExpBool>,
        pub group: Option<crate::value::ExpString>,
        pub launch_type: Option<crate::value::ExpString>,
        pub network_configuration: Option<Box<NetworkConfiguration_>>,
        pub placement_constraints: Option<Vec<PlacementConstraint_>>,
        pub placement_strategy: Option<Vec<PlacementStrategy_>>,
        pub platform_version: Option<crate::value::ExpString>,
        pub propagate_tags: Option<crate::value::ExpString>,
        pub reference_id: Option<crate::value::ExpString>,
        pub tags: Option<serde_json::Value>,
        pub task_count: Option<f64>,
        pub task_definition_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_EcsParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.EcsParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_EcsParameters as EcsParameters;
    impl crate::value::ToValue for EcsParameters_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-eventbridgeparameters.html
    pub struct EventBridgeParameters_ {
        pub detail_type: crate::value::ExpString,
        pub source: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_EventBridgeParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.EventBridgeParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_EventBridgeParameters as EventBridgeParameters;
    impl crate::value::ToValue for EventBridgeParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DetailType".to_string(),
                crate::value::ToValue::to_value(&self.detail_type),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-flexibletimewindow.html
    pub struct FlexibleTimeWindow_ {
        pub maximum_window_in_minutes: Option<f64>,
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_FlexibleTimeWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.FlexibleTimeWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_FlexibleTimeWindow as FlexibleTimeWindow;
    impl crate::value::ToValue for FlexibleTimeWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_window_in_minutes {
                properties.insert(
                    "MaximumWindowInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-kinesisparameters.html
    pub struct KinesisParameters_ {
        pub partition_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_KinesisParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.KinesisParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_KinesisParameters as KinesisParameters;
    impl crate::value::ToValue for KinesisParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PartitionKey".to_string(),
                crate::value::ToValue::to_value(&self.partition_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub awsvpc_configuration: Option<Box<AwsVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_NetworkConfiguration as NetworkConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementconstraint.html
    pub struct PlacementConstraint_ {
        pub expression: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_PlacementConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.PlacementConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_PlacementConstraint as PlacementConstraint;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-placementstrategy.html
    pub struct PlacementStrategy_ {
        pub field: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_PlacementStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.PlacementStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_PlacementStrategy as PlacementStrategy;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-retrypolicy.html
    pub struct RetryPolicy_ {
        pub maximum_event_age_in_seconds: Option<f64>,
        pub maximum_retry_attempts: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_RetryPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.RetryPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_RetryPolicy as RetryPolicy;
    impl crate::value::ToValue for RetryPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_event_age_in_seconds {
                properties.insert(
                    "MaximumEventAgeInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_retry_attempts {
                properties.insert(
                    "MaximumRetryAttempts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sagemakerpipelineparameter.html
    pub struct SageMakerPipelineParameter_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_SageMakerPipelineParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.SageMakerPipelineParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_SageMakerPipelineParameter as SageMakerPipelineParameter;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sagemakerpipelineparameters.html
    pub struct SageMakerPipelineParameters_ {
        pub pipeline_parameter_list: Option<Vec<SageMakerPipelineParameter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_SageMakerPipelineParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.SageMakerPipelineParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_SageMakerPipelineParameters as SageMakerPipelineParameters;
    impl crate::value::ToValue for SageMakerPipelineParameters_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-sqsparameters.html
    pub struct SqsParameters_ {
        pub message_group_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_SqsParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.SqsParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_SqsParameters as SqsParameters;
    impl crate::value::ToValue for SqsParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message_group_id {
                properties.insert(
                    "MessageGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-scheduler-schedule-target.html
    pub struct Target_ {
        pub arn: crate::value::ExpString,
        pub dead_letter_config: Option<Box<DeadLetterConfig_>>,
        pub ecs_parameters: Option<Box<EcsParameters_>>,
        pub event_bridge_parameters: Option<Box<EventBridgeParameters_>>,
        pub input: Option<crate::value::ExpString>,
        pub kinesis_parameters: Option<Box<KinesisParameters_>>,
        pub retry_policy: Option<Box<RetryPolicy_>>,
        pub role_arn: crate::value::ExpString,
        pub sage_maker_pipeline_parameters: Option<Box<SageMakerPipelineParameters_>>,
        pub sqs_parameters: Option<Box<SqsParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_scheduler_Schedule_Target {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Scheduler::Schedule.Target"
            $($field $value)*)
        };
    }
    pub use crate::__aws_scheduler_Schedule_Target as Target;
    impl crate::value::ToValue for Target_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.dead_letter_config {
                properties.insert(
                    "DeadLetterConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecs_parameters {
                properties.insert(
                    "EcsParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_bridge_parameters {
                properties.insert(
                    "EventBridgeParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input {
                properties.insert("Input".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.kinesis_parameters {
                properties.insert(
                    "KinesisParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retry_policy {
                properties.insert(
                    "RetryPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.sage_maker_pipeline_parameters {
                properties.insert(
                    "SageMakerPipelineParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sqs_parameters {
                properties.insert(
                    "SqsParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedule.html
pub struct Schedule_ {
    pub description: Option<crate::value::ExpString>,
    pub end_date: Option<crate::value::ExpString>,
    pub flexible_time_window: super::scheduler::schedule::FlexibleTimeWindow_,
    pub group_name: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub schedule_expression: crate::value::ExpString,
    pub schedule_expression_timezone: Option<crate::value::ExpString>,
    pub start_date: Option<crate::value::ExpString>,
    pub state: Option<crate::value::ExpString>,
    pub target: super::scheduler::schedule::Target_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_scheduler_Schedule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Scheduler::Schedule"
        $($field $value)*)
    };
}
pub use crate::__aws_scheduler_Schedule as Schedule;
impl crate::template::ToResource for Schedule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Scheduler"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Schedule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.end_date {
            properties.insert(
                "EndDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FlexibleTimeWindow".to_string(),
            crate::value::ToValue::to_value(&self.flexible_time_window),
        );
        if let Some(ref value) = self.group_name {
            properties.insert(
                "GroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ScheduleExpression".to_string(),
            crate::value::ToValue::to_value(&self.schedule_expression),
        );
        if let Some(ref value) = self.schedule_expression_timezone {
            properties.insert(
                "ScheduleExpressionTimezone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.start_date {
            properties.insert(
                "StartDate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Target".to_string(),
            crate::value::ToValue::to_value(&self.target),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-scheduler-schedulegroup.html
pub struct ScheduleGroup_ {
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_scheduler_ScheduleGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Scheduler::ScheduleGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_scheduler_ScheduleGroup as ScheduleGroup;
impl crate::template::ToResource for ScheduleGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Scheduler"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScheduleGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
