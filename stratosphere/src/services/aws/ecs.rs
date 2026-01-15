pub mod capacityprovider {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-autoscalinggroupprovider.html
    pub struct AutoScalingGroupProvider_ {
        pub auto_scaling_group_arn: crate::value::ExpString,
        pub managed_draining: Option<crate::value::ExpString>,
        pub managed_scaling: Option<Box<ManagedScaling_>>,
        pub managed_termination_protection: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_CapacityProvider_AutoScalingGroupProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::CapacityProvider.AutoScalingGroupProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_CapacityProvider_AutoScalingGroupProvider as AutoScalingGroupProvider;
    impl crate::value::ToValue for AutoScalingGroupProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AutoScalingGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.auto_scaling_group_arn),
            );
            if let Some(ref value) = self.managed_draining {
                properties.insert(
                    "ManagedDraining".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_scaling {
                properties.insert(
                    "ManagedScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_termination_protection {
                properties.insert(
                    "ManagedTerminationProtection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-capacityprovider-managedscaling.html
    pub struct ManagedScaling_ {
        pub instance_warmup_period: Option<i64>,
        pub maximum_scaling_step_size: Option<i64>,
        pub minimum_scaling_step_size: Option<i64>,
        pub status: Option<crate::value::ExpString>,
        pub target_capacity: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_CapacityProvider_ManagedScaling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::CapacityProvider.ManagedScaling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_CapacityProvider_ManagedScaling as ManagedScaling;
    impl crate::value::ToValue for ManagedScaling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_warmup_period {
                properties.insert(
                    "InstanceWarmupPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_scaling_step_size {
                properties.insert(
                    "MaximumScalingStepSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_scaling_step_size {
                properties.insert(
                    "MinimumScalingStepSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.target_capacity {
                properties.insert(
                    "TargetCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-capacityproviderstrategyitem.html
    pub struct CapacityProviderStrategyItem_ {
        pub base: Option<i64>,
        pub capacity_provider: Option<crate::value::ExpString>,
        pub weight: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Cluster_CapacityProviderStrategyItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Cluster.CapacityProviderStrategyItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Cluster_CapacityProviderStrategyItem as CapacityProviderStrategyItem;
    impl crate::value::ToValue for CapacityProviderStrategyItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base {
                properties.insert("Base".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.capacity_provider {
                properties.insert(
                    "CapacityProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-clusterconfiguration.html
    pub struct ClusterConfiguration_ {
        pub execute_command_configuration: Option<Box<ExecuteCommandConfiguration_>>,
        pub managed_storage_configuration: Option<Box<ManagedStorageConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Cluster_ClusterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Cluster.ClusterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Cluster_ClusterConfiguration as ClusterConfiguration;
    impl crate::value::ToValue for ClusterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.execute_command_configuration {
                properties.insert(
                    "ExecuteCommandConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_storage_configuration {
                properties.insert(
                    "ManagedStorageConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-clustersettings.html
    pub struct ClusterSettings_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Cluster_ClusterSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Cluster.ClusterSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Cluster_ClusterSettings as ClusterSettings;
    impl crate::value::ToValue for ClusterSettings_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandconfiguration.html
    pub struct ExecuteCommandConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
        pub log_configuration: Option<Box<ExecuteCommandLogConfiguration_>>,
        pub logging: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Cluster_ExecuteCommandConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Cluster.ExecuteCommandConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Cluster_ExecuteCommandConfiguration as ExecuteCommandConfiguration;
    impl crate::value::ToValue for ExecuteCommandConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_configuration {
                properties.insert(
                    "LogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.logging {
                properties.insert(
                    "Logging".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-executecommandlogconfiguration.html
    pub struct ExecuteCommandLogConfiguration_ {
        pub cloud_watch_encryption_enabled: Option<crate::value::ExpBool>,
        pub cloud_watch_log_group_name: Option<crate::value::ExpString>,
        pub s3_bucket_name: Option<crate::value::ExpString>,
        pub s3_encryption_enabled: Option<crate::value::ExpBool>,
        pub s3_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Cluster_ExecuteCommandLogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Cluster.ExecuteCommandLogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Cluster_ExecuteCommandLogConfiguration as ExecuteCommandLogConfiguration;
    impl crate::value::ToValue for ExecuteCommandLogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_encryption_enabled {
                properties.insert(
                    "CloudWatchEncryptionEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cloud_watch_log_group_name {
                properties.insert(
                    "CloudWatchLogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_name {
                properties.insert(
                    "S3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_encryption_enabled {
                properties.insert(
                    "S3EncryptionEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_key_prefix {
                properties.insert(
                    "S3KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-managedstorageconfiguration.html
    pub struct ManagedStorageConfiguration_ {
        pub fargate_ephemeral_storage_kms_key_id: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Cluster_ManagedStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Cluster.ManagedStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Cluster_ManagedStorageConfiguration as ManagedStorageConfiguration;
    impl crate::value::ToValue for ManagedStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fargate_ephemeral_storage_kms_key_id {
                properties.insert(
                    "FargateEphemeralStorageKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-cluster-serviceconnectdefaults.html
    pub struct ServiceConnectDefaults_ {
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Cluster_ServiceConnectDefaults {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Cluster.ServiceConnectDefaults"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Cluster_ServiceConnectDefaults as ServiceConnectDefaults;
    impl crate::value::ToValue for ServiceConnectDefaults_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod clustercapacityproviderassociations {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-clustercapacityproviderassociations-capacityproviderstrategy.html
    pub struct CapacityProviderStrategy_ {
        pub base: Option<i64>,
        pub capacity_provider: crate::value::ExpString,
        pub weight: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_ClusterCapacityProviderAssociations_CapacityProviderStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::ClusterCapacityProviderAssociations.CapacityProviderStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_ClusterCapacityProviderAssociations_CapacityProviderStrategy as CapacityProviderStrategy;
    impl crate::value::ToValue for CapacityProviderStrategy_ {
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
}
pub mod service {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-advancedconfiguration.html
    pub struct AdvancedConfiguration_ {
        pub alternate_target_group_arn: crate::value::ExpString,
        pub production_listener_rule: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub test_listener_rule: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_AdvancedConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.AdvancedConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_AdvancedConfiguration as AdvancedConfiguration;
    impl crate::value::ToValue for AdvancedConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlternateTargetGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.alternate_target_group_arn),
            );
            if let Some(ref value) = self.production_listener_rule {
                properties.insert(
                    "ProductionListenerRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.test_listener_rule {
                properties.insert(
                    "TestListenerRule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-awsvpcconfiguration.html
    pub struct AwsVpcConfiguration_ {
        pub assign_public_ip: Option<crate::value::ExpString>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub subnets: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_AwsVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.AwsVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_AwsVpcConfiguration as AwsVpcConfiguration;
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
            if let Some(ref value) = self.subnets {
                properties.insert(
                    "Subnets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-capacityproviderstrategyitem.html
    pub struct CapacityProviderStrategyItem_ {
        pub base: Option<i64>,
        pub capacity_provider: Option<crate::value::ExpString>,
        pub weight: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_CapacityProviderStrategyItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.CapacityProviderStrategyItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_CapacityProviderStrategyItem as CapacityProviderStrategyItem;
    impl crate::value::ToValue for CapacityProviderStrategyItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base {
                properties.insert("Base".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.capacity_provider {
                properties.insert(
                    "CapacityProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentalarms.html
    pub struct DeploymentAlarms_ {
        pub alarm_names: Vec<crate::value::ExpString>,
        pub enable: crate::value::ExpBool,
        pub rollback: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_DeploymentAlarms {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.DeploymentAlarms"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_DeploymentAlarms as DeploymentAlarms;
    impl crate::value::ToValue for DeploymentAlarms_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AlarmNames".to_string(),
                crate::value::ToValue::to_value(&self.alarm_names),
            );
            properties.insert(
                "Enable".to_string(),
                crate::value::ToValue::to_value(&self.enable),
            );
            properties.insert(
                "Rollback".to_string(),
                crate::value::ToValue::to_value(&self.rollback),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentcircuitbreaker.html
    pub struct DeploymentCircuitBreaker_ {
        pub enable: crate::value::ExpBool,
        pub rollback: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_DeploymentCircuitBreaker {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.DeploymentCircuitBreaker"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_DeploymentCircuitBreaker as DeploymentCircuitBreaker;
    impl crate::value::ToValue for DeploymentCircuitBreaker_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enable".to_string(),
                crate::value::ToValue::to_value(&self.enable),
            );
            properties.insert(
                "Rollback".to_string(),
                crate::value::ToValue::to_value(&self.rollback),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentconfiguration.html
    pub struct DeploymentConfiguration_ {
        pub alarms: Option<Box<DeploymentAlarms_>>,
        pub bake_time_in_minutes: Option<i64>,
        pub deployment_circuit_breaker: Option<Box<DeploymentCircuitBreaker_>>,
        pub lifecycle_hooks: Option<Vec<DeploymentLifecycleHook_>>,
        pub maximum_percent: Option<i64>,
        pub minimum_healthy_percent: Option<i64>,
        pub strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_DeploymentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.DeploymentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_DeploymentConfiguration as DeploymentConfiguration;
    impl crate::value::ToValue for DeploymentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarms {
                properties.insert("Alarms".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.bake_time_in_minutes {
                properties.insert(
                    "BakeTimeInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deployment_circuit_breaker {
                properties.insert(
                    "DeploymentCircuitBreaker".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lifecycle_hooks {
                properties.insert(
                    "LifecycleHooks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_percent {
                properties.insert(
                    "MaximumPercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_healthy_percent {
                properties.insert(
                    "MinimumHealthyPercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.strategy {
                properties.insert(
                    "Strategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentcontroller.html
    pub struct DeploymentController_ {
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_DeploymentController {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.DeploymentController"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_DeploymentController as DeploymentController;
    impl crate::value::ToValue for DeploymentController_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-deploymentlifecyclehook.html
    pub struct DeploymentLifecycleHook_ {
        pub hook_details: Option<serde_json::Value>,
        pub hook_target_arn: crate::value::ExpString,
        pub lifecycle_stages: Vec<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_DeploymentLifecycleHook {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.DeploymentLifecycleHook"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_DeploymentLifecycleHook as DeploymentLifecycleHook;
    impl crate::value::ToValue for DeploymentLifecycleHook_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hook_details {
                properties.insert(
                    "HookDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HookTargetArn".to_string(),
                crate::value::ToValue::to_value(&self.hook_target_arn),
            );
            properties.insert(
                "LifecycleStages".to_string(),
                crate::value::ToValue::to_value(&self.lifecycle_stages),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-ebstagspecification.html
    pub struct EBSTagSpecification_ {
        pub propagate_tags: Option<crate::value::ExpString>,
        pub resource_type: crate::value::ExpString,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_EBSTagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.EBSTagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_EBSTagSpecification as EBSTagSpecification;
    impl crate::value::ToValue for EBSTagSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.propagate_tags {
                properties.insert(
                    "PropagateTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResourceType".to_string(),
                crate::value::ToValue::to_value(&self.resource_type),
            );
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-forcenewdeployment.html
    pub struct ForceNewDeployment_ {
        pub enable_force_new_deployment: crate::value::ExpBool,
        pub force_new_deployment_nonce: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ForceNewDeployment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ForceNewDeployment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ForceNewDeployment as ForceNewDeployment;
    impl crate::value::ToValue for ForceNewDeployment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnableForceNewDeployment".to_string(),
                crate::value::ToValue::to_value(&self.enable_force_new_deployment),
            );
            if let Some(ref value) = self.force_new_deployment_nonce {
                properties.insert(
                    "ForceNewDeploymentNonce".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-loadbalancer.html
    pub struct LoadBalancer_ {
        pub advanced_configuration: Option<Box<AdvancedConfiguration_>>,
        pub container_name: Option<crate::value::ExpString>,
        pub container_port: Option<i64>,
        pub load_balancer_name: Option<crate::value::ExpString>,
        pub target_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_LoadBalancer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.LoadBalancer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_LoadBalancer as LoadBalancer;
    impl crate::value::ToValue for LoadBalancer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.advanced_configuration {
                properties.insert(
                    "AdvancedConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_name {
                properties.insert(
                    "ContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_port {
                properties.insert(
                    "ContainerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_balancer_name {
                properties.insert(
                    "LoadBalancerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_group_arn {
                properties.insert(
                    "TargetGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-logconfiguration.html
    pub struct LogConfiguration_ {
        pub log_driver: Option<crate::value::ExpString>,
        pub options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub secret_options: Option<Vec<Secret_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_driver {
                properties.insert(
                    "LogDriver".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_options {
                properties.insert(
                    "SecretOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub awsvpc_configuration: Option<Box<AwsVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_NetworkConfiguration as NetworkConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementconstraint.html
    pub struct PlacementConstraint_ {
        pub expression: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_PlacementConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.PlacementConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_PlacementConstraint as PlacementConstraint;
    impl crate::value::ToValue for PlacementConstraint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-placementstrategy.html
    pub struct PlacementStrategy_ {
        pub field: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_PlacementStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.PlacementStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_PlacementStrategy as PlacementStrategy;
    impl crate::value::ToValue for PlacementStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-secret.html
    pub struct Secret_ {
        pub name: crate::value::ExpString,
        pub value_from: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_Secret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.Secret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_Secret as Secret;
    impl crate::value::ToValue for Secret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "ValueFrom".to_string(),
                crate::value::ToValue::to_value(&self.value_from),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectclientalias.html
    pub struct ServiceConnectClientAlias_ {
        pub dns_name: Option<crate::value::ExpString>,
        pub port: i64,
        pub test_traffic_rules: Option<Box<ServiceConnectTestTrafficRules_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectClientAlias {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectClientAlias"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectClientAlias as ServiceConnectClientAlias;
    impl crate::value::ToValue for ServiceConnectClientAlias_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dns_name {
                properties.insert(
                    "DnsName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            if let Some(ref value) = self.test_traffic_rules {
                properties.insert(
                    "TestTrafficRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectconfiguration.html
    pub struct ServiceConnectConfiguration_ {
        pub enabled: crate::value::ExpBool,
        pub log_configuration: Option<Box<LogConfiguration_>>,
        pub namespace: Option<crate::value::ExpString>,
        pub services: Option<Vec<ServiceConnectService_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectConfiguration as ServiceConnectConfiguration;
    impl crate::value::ToValue for ServiceConnectConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.log_configuration {
                properties.insert(
                    "LogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.services {
                properties.insert(
                    "Services".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnectservice.html
    pub struct ServiceConnectService_ {
        pub client_aliases: Option<Vec<ServiceConnectClientAlias_>>,
        pub discovery_name: Option<crate::value::ExpString>,
        pub ingress_port_override: Option<i64>,
        pub port_name: crate::value::ExpString,
        pub timeout: Option<Box<TimeoutConfiguration_>>,
        pub tls: Option<Box<ServiceConnectTlsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectService {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectService"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectService as ServiceConnectService;
    impl crate::value::ToValue for ServiceConnectService_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_aliases {
                properties.insert(
                    "ClientAliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.discovery_name {
                properties.insert(
                    "DiscoveryName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ingress_port_override {
                properties.insert(
                    "IngressPortOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PortName".to_string(),
                crate::value::ToValue::to_value(&self.port_name),
            );
            if let Some(ref value) = self.timeout {
                properties.insert(
                    "Timeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tls {
                properties.insert("Tls".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttesttrafficrules.html
    pub struct ServiceConnectTestTrafficRules_ {
        pub header: Box<ServiceConnectTestTrafficRulesHeader_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectTestTrafficRules {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectTestTrafficRules"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectTestTrafficRules as ServiceConnectTestTrafficRules;
    impl crate::value::ToValue for ServiceConnectTestTrafficRules_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Header".to_string(),
                crate::value::ToValue::to_value(&self.header),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttesttrafficrulesheader.html
    pub struct ServiceConnectTestTrafficRulesHeader_ {
        pub name: crate::value::ExpString,
        pub value: Option<Box<ServiceConnectTestTrafficRulesHeaderValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectTestTrafficRulesHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectTestTrafficRulesHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectTestTrafficRulesHeader as ServiceConnectTestTrafficRulesHeader;
    impl crate::value::ToValue for ServiceConnectTestTrafficRulesHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttesttrafficrulesheadervalue.html
    pub struct ServiceConnectTestTrafficRulesHeaderValue_ {
        pub exact: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectTestTrafficRulesHeaderValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectTestTrafficRulesHeaderValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectTestTrafficRulesHeaderValue as ServiceConnectTestTrafficRulesHeaderValue;
    impl crate::value::ToValue for ServiceConnectTestTrafficRulesHeaderValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Exact".to_string(),
                crate::value::ToValue::to_value(&self.exact),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlscertificateauthority.html
    pub struct ServiceConnectTlsCertificateAuthority_ {
        pub aws_pca_authority_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectTlsCertificateAuthority {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectTlsCertificateAuthority"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectTlsCertificateAuthority as ServiceConnectTlsCertificateAuthority;
    impl crate::value::ToValue for ServiceConnectTlsCertificateAuthority_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_pca_authority_arn {
                properties.insert(
                    "AwsPcaAuthorityArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceconnecttlsconfiguration.html
    pub struct ServiceConnectTlsConfiguration_ {
        pub issuer_certificate_authority: Box<ServiceConnectTlsCertificateAuthority_>,
        pub kms_key: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceConnectTlsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceConnectTlsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceConnectTlsConfiguration as ServiceConnectTlsConfiguration;
    impl crate::value::ToValue for ServiceConnectTlsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IssuerCertificateAuthority".to_string(),
                crate::value::ToValue::to_value(&self.issuer_certificate_authority),
            );
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicemanagedebsvolumeconfiguration.html
    pub struct ServiceManagedEBSVolumeConfiguration_ {
        pub encrypted: Option<crate::value::ExpBool>,
        pub filesystem_type: Option<crate::value::ExpString>,
        pub iops: Option<i64>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub size_in_gi_b: Option<i64>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub tag_specifications: Option<Vec<EBSTagSpecification_>>,
        pub throughput: Option<i64>,
        pub volume_initialization_rate: Option<i64>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceManagedEBSVolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceManagedEBSVolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceManagedEBSVolumeConfiguration as ServiceManagedEBSVolumeConfiguration;
    impl crate::value::ToValue for ServiceManagedEBSVolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encrypted {
                properties.insert(
                    "Encrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filesystem_type {
                properties.insert(
                    "FilesystemType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            if let Some(ref value) = self.size_in_gi_b {
                properties.insert(
                    "SizeInGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshot_id {
                properties.insert(
                    "SnapshotId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_specifications {
                properties.insert(
                    "TagSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput {
                properties.insert(
                    "Throughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_initialization_rate {
                properties.insert(
                    "VolumeInitializationRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_type {
                properties.insert(
                    "VolumeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-serviceregistry.html
    pub struct ServiceRegistry_ {
        pub container_name: Option<crate::value::ExpString>,
        pub container_port: Option<i64>,
        pub port: Option<i64>,
        pub registry_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceRegistry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceRegistry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceRegistry as ServiceRegistry;
    impl crate::value::ToValue for ServiceRegistry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_name {
                properties.insert(
                    "ContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_port {
                properties.insert(
                    "ContainerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.registry_arn {
                properties.insert(
                    "RegistryArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-servicevolumeconfiguration.html
    pub struct ServiceVolumeConfiguration_ {
        pub managed_ebs_volume: Option<Box<ServiceManagedEBSVolumeConfiguration_>>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_ServiceVolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.ServiceVolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_ServiceVolumeConfiguration as ServiceVolumeConfiguration;
    impl crate::value::ToValue for ServiceVolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.managed_ebs_volume {
                properties.insert(
                    "ManagedEBSVolume".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-timeoutconfiguration.html
    pub struct TimeoutConfiguration_ {
        pub idle_timeout_seconds: Option<i64>,
        pub per_request_timeout_seconds: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_TimeoutConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.TimeoutConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_TimeoutConfiguration as TimeoutConfiguration;
    impl crate::value::ToValue for TimeoutConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_timeout_seconds {
                properties.insert(
                    "IdleTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.per_request_timeout_seconds {
                properties.insert(
                    "PerRequestTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-service-vpclatticeconfiguration.html
    pub struct VpcLatticeConfiguration_ {
        pub port_name: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
        pub target_group_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_Service_VpcLatticeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::Service.VpcLatticeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_Service_VpcLatticeConfiguration as VpcLatticeConfiguration;
    impl crate::value::ToValue for VpcLatticeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PortName".to_string(),
                crate::value::ToValue::to_value(&self.port_name),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "TargetGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.target_group_arn),
            );
            properties.into()
        }
    }
}
pub mod taskdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-authorizationconfig.html
    pub struct AuthorizationConfig_ {
        pub access_point_id: Option<crate::value::ExpString>,
        pub iam: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_AuthorizationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.AuthorizationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_AuthorizationConfig as AuthorizationConfig;
    impl crate::value::ToValue for AuthorizationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_point_id {
                properties.insert(
                    "AccessPointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam {
                properties.insert("IAM".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdefinition.html
    pub struct ContainerDefinition_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub cpu: Option<i64>,
        pub credential_specs: Option<Vec<crate::value::ExpString>>,
        pub depends_on: Option<Vec<ContainerDependency_>>,
        pub disable_networking: Option<crate::value::ExpBool>,
        pub dns_search_domains: Option<Vec<crate::value::ExpString>>,
        pub dns_servers: Option<Vec<crate::value::ExpString>>,
        pub docker_labels: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub docker_security_options: Option<Vec<crate::value::ExpString>>,
        pub entry_point: Option<Vec<crate::value::ExpString>>,
        pub environment: Option<Vec<KeyValuePair_>>,
        pub environment_files: Option<Vec<EnvironmentFile_>>,
        pub essential: Option<crate::value::ExpBool>,
        pub extra_hosts: Option<Vec<HostEntry_>>,
        pub firelens_configuration: Option<Box<FirelensConfiguration_>>,
        pub health_check: Option<Box<HealthCheck_>>,
        pub hostname: Option<crate::value::ExpString>,
        pub image: crate::value::ExpString,
        pub interactive: Option<crate::value::ExpBool>,
        pub links: Option<Vec<crate::value::ExpString>>,
        pub linux_parameters: Option<Box<LinuxParameters_>>,
        pub log_configuration: Option<Box<LogConfiguration_>>,
        pub memory: Option<i64>,
        pub memory_reservation: Option<i64>,
        pub mount_points: Option<Vec<MountPoint_>>,
        pub name: crate::value::ExpString,
        pub port_mappings: Option<Vec<PortMapping_>>,
        pub privileged: Option<crate::value::ExpBool>,
        pub pseudo_terminal: Option<crate::value::ExpBool>,
        pub readonly_root_filesystem: Option<crate::value::ExpBool>,
        pub repository_credentials: Option<Box<RepositoryCredentials_>>,
        pub resource_requirements: Option<Vec<ResourceRequirement_>>,
        pub restart_policy: Option<Box<RestartPolicy_>>,
        pub secrets: Option<Vec<Secret_>>,
        pub start_timeout: Option<i64>,
        pub stop_timeout: Option<i64>,
        pub system_controls: Option<Vec<SystemControl_>>,
        pub ulimits: Option<Vec<Ulimit_>>,
        pub user: Option<crate::value::ExpString>,
        pub version_consistency: Option<crate::value::ExpString>,
        pub volumes_from: Option<Vec<VolumeFrom_>>,
        pub working_directory: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_ContainerDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.ContainerDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_ContainerDefinition as ContainerDefinition;
    impl crate::value::ToValue for ContainerDefinition_ {
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
            if let Some(ref value) = self.credential_specs {
                properties.insert(
                    "CredentialSpecs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.depends_on {
                properties.insert(
                    "DependsOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_networking {
                properties.insert(
                    "DisableNetworking".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_search_domains {
                properties.insert(
                    "DnsSearchDomains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dns_servers {
                properties.insert(
                    "DnsServers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.docker_labels {
                properties.insert(
                    "DockerLabels".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.docker_security_options {
                properties.insert(
                    "DockerSecurityOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entry_point {
                properties.insert(
                    "EntryPoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
            if let Some(ref value) = self.essential {
                properties.insert(
                    "Essential".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.extra_hosts {
                properties.insert(
                    "ExtraHosts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firelens_configuration {
                properties.insert(
                    "FirelensConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check {
                properties.insert(
                    "HealthCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hostname {
                properties.insert(
                    "Hostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Image".to_string(),
                crate::value::ToValue::to_value(&self.image),
            );
            if let Some(ref value) = self.interactive {
                properties.insert(
                    "Interactive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.links {
                properties.insert("Links".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.linux_parameters {
                properties.insert(
                    "LinuxParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_configuration {
                properties.insert(
                    "LogConfiguration".to_string(),
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
            if let Some(ref value) = self.mount_points {
                properties.insert(
                    "MountPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.port_mappings {
                properties.insert(
                    "PortMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.privileged {
                properties.insert(
                    "Privileged".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pseudo_terminal {
                properties.insert(
                    "PseudoTerminal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.readonly_root_filesystem {
                properties.insert(
                    "ReadonlyRootFilesystem".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.repository_credentials {
                properties.insert(
                    "RepositoryCredentials".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_requirements {
                properties.insert(
                    "ResourceRequirements".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_policy {
                properties.insert(
                    "RestartPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secrets {
                properties.insert(
                    "Secrets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_timeout {
                properties.insert(
                    "StartTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_timeout {
                properties.insert(
                    "StopTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.system_controls {
                properties.insert(
                    "SystemControls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ulimits {
                properties.insert(
                    "Ulimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user {
                properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.version_consistency {
                properties.insert(
                    "VersionConsistency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volumes_from {
                properties.insert(
                    "VolumesFrom".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.working_directory {
                properties.insert(
                    "WorkingDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-containerdependency.html
    pub struct ContainerDependency_ {
        pub condition: Option<crate::value::ExpString>,
        pub container_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_ContainerDependency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.ContainerDependency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_ContainerDependency as ContainerDependency;
    impl crate::value::ToValue for ContainerDependency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_name {
                properties.insert(
                    "ContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-device.html
    pub struct Device_ {
        pub container_path: Option<crate::value::ExpString>,
        pub host_path: Option<crate::value::ExpString>,
        pub permissions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_Device {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.Device"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_Device as Device;
    impl crate::value::ToValue for Device_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_path {
                properties.insert(
                    "ContainerPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host_path {
                properties.insert(
                    "HostPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.permissions {
                properties.insert(
                    "Permissions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-dockervolumeconfiguration.html
    pub struct DockerVolumeConfiguration_ {
        pub autoprovision: Option<crate::value::ExpBool>,
        pub driver: Option<crate::value::ExpString>,
        pub driver_opts: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub labels: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_DockerVolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.DockerVolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_DockerVolumeConfiguration as DockerVolumeConfiguration;
    impl crate::value::ToValue for DockerVolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.autoprovision {
                properties.insert(
                    "Autoprovision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.driver {
                properties.insert("Driver".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.driver_opts {
                properties.insert(
                    "DriverOpts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.labels {
                properties.insert("Labels".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-efsvolumeconfiguration.html
    pub struct EFSVolumeConfiguration_ {
        pub authorization_config: Option<Box<AuthorizationConfig_>>,
        pub filesystem_id: crate::value::ExpString,
        pub root_directory: Option<crate::value::ExpString>,
        pub transit_encryption: Option<crate::value::ExpString>,
        pub transit_encryption_port: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_EFSVolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.EFSVolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_EFSVolumeConfiguration as EFSVolumeConfiguration;
    impl crate::value::ToValue for EFSVolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_config {
                properties.insert(
                    "AuthorizationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FilesystemId".to_string(),
                crate::value::ToValue::to_value(&self.filesystem_id),
            );
            if let Some(ref value) = self.root_directory {
                properties.insert(
                    "RootDirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_encryption {
                properties.insert(
                    "TransitEncryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transit_encryption_port {
                properties.insert(
                    "TransitEncryptionPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-environmentfile.html
    pub struct EnvironmentFile_ {
        pub r#type: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_EnvironmentFile {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.EnvironmentFile"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_EnvironmentFile as EnvironmentFile;
    impl crate::value::ToValue for EnvironmentFile_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ephemeralstorage.html
    pub struct EphemeralStorage_ {
        pub size_in_gi_b: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_EphemeralStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.EphemeralStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_EphemeralStorage as EphemeralStorage;
    impl crate::value::ToValue for EphemeralStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.size_in_gi_b {
                properties.insert(
                    "SizeInGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-fsxauthorizationconfig.html
    pub struct FSxAuthorizationConfig_ {
        pub credentials_parameter: crate::value::ExpString,
        pub domain: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_FSxAuthorizationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.FSxAuthorizationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_FSxAuthorizationConfig as FSxAuthorizationConfig;
    impl crate::value::ToValue for FSxAuthorizationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CredentialsParameter".to_string(),
                crate::value::ToValue::to_value(&self.credentials_parameter),
            );
            properties.insert(
                "Domain".to_string(),
                crate::value::ToValue::to_value(&self.domain),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-fsxwindowsfileservervolumeconfiguration.html
    pub struct FSxWindowsFileServerVolumeConfiguration_ {
        pub authorization_config: Option<Box<FSxAuthorizationConfig_>>,
        pub file_system_id: crate::value::ExpString,
        pub root_directory: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_FSxWindowsFileServerVolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.FSxWindowsFileServerVolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_FSxWindowsFileServerVolumeConfiguration as FSxWindowsFileServerVolumeConfiguration;
    impl crate::value::ToValue for FSxWindowsFileServerVolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.authorization_config {
                properties.insert(
                    "AuthorizationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FileSystemId".to_string(),
                crate::value::ToValue::to_value(&self.file_system_id),
            );
            properties.insert(
                "RootDirectory".to_string(),
                crate::value::ToValue::to_value(&self.root_directory),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-firelensconfiguration.html
    pub struct FirelensConfiguration_ {
        pub options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_FirelensConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.FirelensConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_FirelensConfiguration as FirelensConfiguration;
    impl crate::value::ToValue for FirelensConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-healthcheck.html
    pub struct HealthCheck_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub interval: Option<i64>,
        pub retries: Option<i64>,
        pub start_period: Option<i64>,
        pub timeout: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_HealthCheck {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.HealthCheck"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_HealthCheck as HealthCheck;
    impl crate::value::ToValue for HealthCheck_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval {
                properties.insert(
                    "Interval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retries {
                properties.insert(
                    "Retries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_period {
                properties.insert(
                    "StartPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout {
                properties.insert(
                    "Timeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-hostentry.html
    pub struct HostEntry_ {
        pub hostname: Option<crate::value::ExpString>,
        pub ip_address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_HostEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.HostEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_HostEntry as HostEntry;
    impl crate::value::ToValue for HostEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hostname {
                properties.insert(
                    "Hostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address {
                properties.insert(
                    "IpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-hostvolumeproperties.html
    pub struct HostVolumeProperties_ {
        pub source_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_HostVolumeProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.HostVolumeProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_HostVolumeProperties as HostVolumeProperties;
    impl crate::value::ToValue for HostVolumeProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source_path {
                properties.insert(
                    "SourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-kernelcapabilities.html
    pub struct KernelCapabilities_ {
        pub add: Option<Vec<crate::value::ExpString>>,
        pub drop: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_KernelCapabilities {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.KernelCapabilities"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_KernelCapabilities as KernelCapabilities;
    impl crate::value::ToValue for KernelCapabilities_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add {
                properties.insert("Add".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.drop {
                properties.insert("Drop".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-keyvaluepair.html
    pub struct KeyValuePair_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_KeyValuePair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.KeyValuePair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_KeyValuePair as KeyValuePair;
    impl crate::value::ToValue for KeyValuePair_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-linuxparameters.html
    pub struct LinuxParameters_ {
        pub capabilities: Option<Box<KernelCapabilities_>>,
        pub devices: Option<Vec<Device_>>,
        pub init_process_enabled: Option<crate::value::ExpBool>,
        pub max_swap: Option<i64>,
        pub shared_memory_size: Option<i64>,
        pub swappiness: Option<i64>,
        pub tmpfs: Option<Vec<Tmpfs_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_LinuxParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.LinuxParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_LinuxParameters as LinuxParameters;
    impl crate::value::ToValue for LinuxParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capabilities {
                properties.insert(
                    "Capabilities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.devices {
                properties.insert(
                    "Devices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.init_process_enabled {
                properties.insert(
                    "InitProcessEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_swap {
                properties.insert(
                    "MaxSwap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shared_memory_size {
                properties.insert(
                    "SharedMemorySize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.swappiness {
                properties.insert(
                    "Swappiness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tmpfs {
                properties.insert("Tmpfs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-logconfiguration.html
    pub struct LogConfiguration_ {
        pub log_driver: crate::value::ExpString,
        pub options: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub secret_options: Option<Vec<Secret_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogDriver".to_string(),
                crate::value::ToValue::to_value(&self.log_driver),
            );
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_options {
                properties.insert(
                    "SecretOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-mountpoint.html
    pub struct MountPoint_ {
        pub container_path: Option<crate::value::ExpString>,
        pub read_only: Option<crate::value::ExpBool>,
        pub source_volume: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_MountPoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.MountPoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_MountPoint as MountPoint;
    impl crate::value::ToValue for MountPoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_path {
                properties.insert(
                    "ContainerPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_volume {
                properties.insert(
                    "SourceVolume".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-portmapping.html
    pub struct PortMapping_ {
        pub app_protocol: Option<crate::value::ExpString>,
        pub container_port: Option<i64>,
        pub container_port_range: Option<crate::value::ExpString>,
        pub host_port: Option<i64>,
        pub name: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_PortMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.PortMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_PortMapping as PortMapping;
    impl crate::value::ToValue for PortMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_protocol {
                properties.insert(
                    "AppProtocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_port {
                properties.insert(
                    "ContainerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_port_range {
                properties.insert(
                    "ContainerPortRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host_port {
                properties.insert(
                    "HostPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-proxyconfiguration.html
    pub struct ProxyConfiguration_ {
        pub container_name: crate::value::ExpString,
        pub proxy_configuration_properties: Option<Vec<KeyValuePair_>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_ProxyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.ProxyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_ProxyConfiguration as ProxyConfiguration;
    impl crate::value::ToValue for ProxyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContainerName".to_string(),
                crate::value::ToValue::to_value(&self.container_name),
            );
            if let Some(ref value) = self.proxy_configuration_properties {
                properties.insert(
                    "ProxyConfigurationProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-repositorycredentials.html
    pub struct RepositoryCredentials_ {
        pub credentials_parameter: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_RepositoryCredentials {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.RepositoryCredentials"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_RepositoryCredentials as RepositoryCredentials;
    impl crate::value::ToValue for RepositoryCredentials_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.credentials_parameter {
                properties.insert(
                    "CredentialsParameter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-resourcerequirement.html
    pub struct ResourceRequirement_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_ResourceRequirement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.ResourceRequirement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_ResourceRequirement as ResourceRequirement;
    impl crate::value::ToValue for ResourceRequirement_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-restartpolicy.html
    pub struct RestartPolicy_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub ignored_exit_codes: Option<Vec<i64>>,
        pub restart_attempt_period: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_RestartPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.RestartPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_RestartPolicy as RestartPolicy;
    impl crate::value::ToValue for RestartPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignored_exit_codes {
                properties.insert(
                    "IgnoredExitCodes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restart_attempt_period {
                properties.insert(
                    "RestartAttemptPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-runtimeplatform.html
    pub struct RuntimePlatform_ {
        pub cpu_architecture: Option<crate::value::ExpString>,
        pub operating_system_family: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_RuntimePlatform {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.RuntimePlatform"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_RuntimePlatform as RuntimePlatform;
    impl crate::value::ToValue for RuntimePlatform_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu_architecture {
                properties.insert(
                    "CpuArchitecture".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operating_system_family {
                properties.insert(
                    "OperatingSystemFamily".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-secret.html
    pub struct Secret_ {
        pub name: crate::value::ExpString,
        pub value_from: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_Secret {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.Secret"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_Secret as Secret;
    impl crate::value::ToValue for Secret_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "ValueFrom".to_string(),
                crate::value::ToValue::to_value(&self.value_from),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-systemcontrol.html
    pub struct SystemControl_ {
        pub namespace: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_SystemControl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.SystemControl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_SystemControl as SystemControl;
    impl crate::value::ToValue for SystemControl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-taskdefinitionplacementconstraint.html
    pub struct TaskDefinitionPlacementConstraint_ {
        pub expression: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_TaskDefinitionPlacementConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.TaskDefinitionPlacementConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_TaskDefinitionPlacementConstraint as TaskDefinitionPlacementConstraint;
    impl crate::value::ToValue for TaskDefinitionPlacementConstraint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-tmpfs.html
    pub struct Tmpfs_ {
        pub container_path: Option<crate::value::ExpString>,
        pub mount_options: Option<Vec<crate::value::ExpString>>,
        pub size: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_Tmpfs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.Tmpfs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_Tmpfs as Tmpfs;
    impl crate::value::ToValue for Tmpfs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_path {
                properties.insert(
                    "ContainerPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mount_options {
                properties.insert(
                    "MountOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Size".to_string(),
                crate::value::ToValue::to_value(&self.size),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-ulimit.html
    pub struct Ulimit_ {
        pub hard_limit: i64,
        pub name: crate::value::ExpString,
        pub soft_limit: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_Ulimit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.Ulimit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_Ulimit as Ulimit;
    impl crate::value::ToValue for Ulimit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HardLimit".to_string(),
                crate::value::ToValue::to_value(&self.hard_limit),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "SoftLimit".to_string(),
                crate::value::ToValue::to_value(&self.soft_limit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volume.html
    pub struct Volume_ {
        pub configured_at_launch: Option<crate::value::ExpBool>,
        pub docker_volume_configuration: Option<Box<DockerVolumeConfiguration_>>,
        pub efs_volume_configuration: Option<Box<EFSVolumeConfiguration_>>,
        pub f_sx_windows_file_server_volume_configuration:
            Option<Box<FSxWindowsFileServerVolumeConfiguration_>>,
        pub host: Option<Box<HostVolumeProperties_>>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_Volume {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.Volume"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_Volume as Volume;
    impl crate::value::ToValue for Volume_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configured_at_launch {
                properties.insert(
                    "ConfiguredAtLaunch".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.docker_volume_configuration {
                properties.insert(
                    "DockerVolumeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.efs_volume_configuration {
                properties.insert(
                    "EFSVolumeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.f_sx_windows_file_server_volume_configuration {
                properties.insert(
                    "FSxWindowsFileServerVolumeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host {
                properties.insert("Host".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskdefinition-volumefrom.html
    pub struct VolumeFrom_ {
        pub read_only: Option<crate::value::ExpBool>,
        pub source_container: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskDefinition_VolumeFrom {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskDefinition.VolumeFrom"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskDefinition_VolumeFrom as VolumeFrom;
    impl crate::value::ToValue for VolumeFrom_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_container {
                properties.insert(
                    "SourceContainer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod taskset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-awsvpcconfiguration.html
    pub struct AwsVpcConfiguration_ {
        pub assign_public_ip: Option<crate::value::ExpString>,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskSet_AwsVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskSet.AwsVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskSet_AwsVpcConfiguration as AwsVpcConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-capacityproviderstrategyitem.html
    pub struct CapacityProviderStrategyItem_ {
        pub base: Option<i64>,
        pub capacity_provider: Option<crate::value::ExpString>,
        pub weight: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskSet_CapacityProviderStrategyItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskSet.CapacityProviderStrategyItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskSet_CapacityProviderStrategyItem as CapacityProviderStrategyItem;
    impl crate::value::ToValue for CapacityProviderStrategyItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base {
                properties.insert("Base".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.capacity_provider {
                properties.insert(
                    "CapacityProvider".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-loadbalancer.html
    pub struct LoadBalancer_ {
        pub container_name: Option<crate::value::ExpString>,
        pub container_port: Option<i64>,
        pub target_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskSet_LoadBalancer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskSet.LoadBalancer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskSet_LoadBalancer as LoadBalancer;
    impl crate::value::ToValue for LoadBalancer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_name {
                properties.insert(
                    "ContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_port {
                properties.insert(
                    "ContainerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_group_arn {
                properties.insert(
                    "TargetGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub aws_vpc_configuration: Option<Box<AwsVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskSet_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskSet.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskSet_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_vpc_configuration {
                properties.insert(
                    "AwsVpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-scale.html
    pub struct Scale_ {
        pub unit: Option<crate::value::ExpString>,
        pub value: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskSet_Scale {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskSet.Scale"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskSet_Scale as Scale;
    impl crate::value::ToValue for Scale_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ecs-taskset-serviceregistry.html
    pub struct ServiceRegistry_ {
        pub container_name: Option<crate::value::ExpString>,
        pub container_port: Option<i64>,
        pub port: Option<i64>,
        pub registry_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ecs_TaskSet_ServiceRegistry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::ECS::TaskSet.ServiceRegistry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ecs_TaskSet_ServiceRegistry as ServiceRegistry;
    impl crate::value::ToValue for ServiceRegistry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_name {
                properties.insert(
                    "ContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_port {
                properties.insert(
                    "ContainerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.registry_arn {
                properties.insert(
                    "RegistryArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-capacityprovider.html
pub struct CapacityProvider_ {
    pub auto_scaling_group_provider:
        Option<super::ecs::capacityprovider::AutoScalingGroupProvider_>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecs_CapacityProvider {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ECS::CapacityProvider"
        $($field $value)*)
    };
}
pub use crate::__aws_ecs_CapacityProvider as CapacityProvider;
impl crate::template::ToResource for CapacityProvider_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CapacityProvider"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling_group_provider {
            properties.insert(
                "AutoScalingGroupProvider".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-cluster.html
pub struct Cluster_ {
    pub capacity_providers: Option<Vec<crate::value::ExpString>>,
    pub cluster_name: Option<crate::value::ExpString>,
    pub cluster_settings: Option<Vec<super::ecs::cluster::ClusterSettings_>>,
    pub configuration: Option<super::ecs::cluster::ClusterConfiguration_>,
    pub default_capacity_provider_strategy:
        Option<Vec<super::ecs::cluster::CapacityProviderStrategyItem_>>,
    pub service_connect_defaults: Option<super::ecs::cluster::ServiceConnectDefaults_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecs_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ECS::Cluster" $($field
        $value)*)
    };
}
pub use crate::__aws_ecs_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.capacity_providers {
            properties.insert(
                "CapacityProviders".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_name {
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_settings {
            properties.insert(
                "ClusterSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configuration {
            properties.insert(
                "Configuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_capacity_provider_strategy {
            properties.insert(
                "DefaultCapacityProviderStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_connect_defaults {
            properties.insert(
                "ServiceConnectDefaults".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-clustercapacityproviderassociations.html
pub struct ClusterCapacityProviderAssociations_ {
    pub capacity_providers: Vec<crate::value::ExpString>,
    pub cluster: crate::value::ExpString,
    pub default_capacity_provider_strategy:
        Vec<super::ecs::clustercapacityproviderassociations::CapacityProviderStrategy_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecs_ClusterCapacityProviderAssociations {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ECS::ClusterCapacityProviderAssociations"
        $($field $value)*)
    };
}
pub use crate::__aws_ecs_ClusterCapacityProviderAssociations as ClusterCapacityProviderAssociations;
impl crate::template::ToResource for ClusterCapacityProviderAssociations_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ClusterCapacityProviderAssociations",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CapacityProviders".to_string(),
            crate::value::ToValue::to_value(&self.capacity_providers),
        );
        properties.insert(
            "Cluster".to_string(),
            crate::value::ToValue::to_value(&self.cluster),
        );
        properties.insert(
            "DefaultCapacityProviderStrategy".to_string(),
            crate::value::ToValue::to_value(&self.default_capacity_provider_strategy),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-primarytaskset.html
pub struct PrimaryTaskSet_ {
    pub cluster: crate::value::ExpString,
    pub service: crate::value::ExpString,
    pub task_set_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecs_PrimaryTaskSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ECS::PrimaryTaskSet"
        $($field $value)*)
    };
}
pub use crate::__aws_ecs_PrimaryTaskSet as PrimaryTaskSet;
impl crate::template::ToResource for PrimaryTaskSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PrimaryTaskSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Cluster".to_string(),
            crate::value::ToValue::to_value(&self.cluster),
        );
        properties.insert(
            "Service".to_string(),
            crate::value::ToValue::to_value(&self.service),
        );
        properties.insert(
            "TaskSetId".to_string(),
            crate::value::ToValue::to_value(&self.task_set_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-service.html
pub struct Service_ {
    pub availability_zone_rebalancing: Option<crate::value::ExpString>,
    pub capacity_provider_strategy: Option<Vec<super::ecs::service::CapacityProviderStrategyItem_>>,
    pub cluster: Option<crate::value::ExpString>,
    pub deployment_configuration: Option<super::ecs::service::DeploymentConfiguration_>,
    pub deployment_controller: Option<super::ecs::service::DeploymentController_>,
    pub desired_count: Option<i64>,
    pub enable_ecs_managed_tags: Option<crate::value::ExpBool>,
    pub enable_execute_command: Option<crate::value::ExpBool>,
    pub force_new_deployment: Option<super::ecs::service::ForceNewDeployment_>,
    pub health_check_grace_period_seconds: Option<i64>,
    pub launch_type: Option<crate::value::ExpString>,
    pub load_balancers: Option<Vec<super::ecs::service::LoadBalancer_>>,
    pub network_configuration: Option<super::ecs::service::NetworkConfiguration_>,
    pub placement_constraints: Option<Vec<super::ecs::service::PlacementConstraint_>>,
    pub placement_strategies: Option<Vec<super::ecs::service::PlacementStrategy_>>,
    pub platform_version: Option<crate::value::ExpString>,
    pub propagate_tags: Option<crate::value::ExpString>,
    pub role: Option<crate::value::ExpString>,
    pub scheduling_strategy: Option<crate::value::ExpString>,
    pub service_connect_configuration: Option<super::ecs::service::ServiceConnectConfiguration_>,
    pub service_name: Option<crate::value::ExpString>,
    pub service_registries: Option<Vec<super::ecs::service::ServiceRegistry_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub task_definition: Option<crate::value::ExpString>,
    pub volume_configurations: Option<Vec<super::ecs::service::ServiceVolumeConfiguration_>>,
    pub vpc_lattice_configurations: Option<Vec<super::ecs::service::VpcLatticeConfiguration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecs_Service {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ECS::Service" $($field
        $value)*)
    };
}
pub use crate::__aws_ecs_Service as Service;
impl crate::template::ToResource for Service_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Service"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone_rebalancing {
            properties.insert(
                "AvailabilityZoneRebalancing".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.capacity_provider_strategy {
            properties.insert(
                "CapacityProviderStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster {
            properties.insert(
                "Cluster".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_configuration {
            properties.insert(
                "DeploymentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_controller {
            properties.insert(
                "DeploymentController".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.desired_count {
            properties.insert(
                "DesiredCount".to_string(),
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
        if let Some(ref value) = self.force_new_deployment {
            properties.insert(
                "ForceNewDeployment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_grace_period_seconds {
            properties.insert(
                "HealthCheckGracePeriodSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.launch_type {
            properties.insert(
                "LaunchType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.load_balancers {
            properties.insert(
                "LoadBalancers".to_string(),
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
        if let Some(ref value) = self.placement_strategies {
            properties.insert(
                "PlacementStrategies".to_string(),
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
        if let Some(ref value) = self.role {
            properties.insert("Role".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.scheduling_strategy {
            properties.insert(
                "SchedulingStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_connect_configuration {
            properties.insert(
                "ServiceConnectConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_name {
            properties.insert(
                "ServiceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_registries {
            properties.insert(
                "ServiceRegistries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.task_definition {
            properties.insert(
                "TaskDefinition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volume_configurations {
            properties.insert(
                "VolumeConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_lattice_configurations {
            properties.insert(
                "VpcLatticeConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskdefinition.html
pub struct TaskDefinition_ {
    pub container_definitions: Option<Vec<super::ecs::taskdefinition::ContainerDefinition_>>,
    pub cpu: Option<crate::value::ExpString>,
    pub enable_fault_injection: Option<crate::value::ExpBool>,
    pub ephemeral_storage: Option<super::ecs::taskdefinition::EphemeralStorage_>,
    pub execution_role_arn: Option<crate::value::ExpString>,
    pub family: Option<crate::value::ExpString>,
    pub ipc_mode: Option<crate::value::ExpString>,
    pub memory: Option<crate::value::ExpString>,
    pub network_mode: Option<crate::value::ExpString>,
    pub pid_mode: Option<crate::value::ExpString>,
    pub placement_constraints:
        Option<Vec<super::ecs::taskdefinition::TaskDefinitionPlacementConstraint_>>,
    pub proxy_configuration: Option<super::ecs::taskdefinition::ProxyConfiguration_>,
    pub requires_compatibilities: Option<Vec<crate::value::ExpString>>,
    pub runtime_platform: Option<super::ecs::taskdefinition::RuntimePlatform_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub task_role_arn: Option<crate::value::ExpString>,
    pub volumes: Option<Vec<super::ecs::taskdefinition::Volume_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecs_TaskDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ECS::TaskDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_ecs_TaskDefinition as TaskDefinition;
impl crate::template::ToResource for TaskDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TaskDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.container_definitions {
            properties.insert(
                "ContainerDefinitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cpu {
            properties.insert("Cpu".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.enable_fault_injection {
            properties.insert(
                "EnableFaultInjection".to_string(),
                crate::value::ToValue::to_value(value),
            );
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
        if let Some(ref value) = self.family {
            properties.insert("Family".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.ipc_mode {
            properties.insert(
                "IpcMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.memory {
            properties.insert("Memory".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.network_mode {
            properties.insert(
                "NetworkMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pid_mode {
            properties.insert(
                "PidMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.placement_constraints {
            properties.insert(
                "PlacementConstraints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.proxy_configuration {
            properties.insert(
                "ProxyConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.requires_compatibilities {
            properties.insert(
                "RequiresCompatibilities".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.runtime_platform {
            properties.insert(
                "RuntimePlatform".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.task_role_arn {
            properties.insert(
                "TaskRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volumes {
            properties.insert(
                "Volumes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ecs-taskset.html
pub struct TaskSet_ {
    pub capacity_provider_strategy: Option<Vec<super::ecs::taskset::CapacityProviderStrategyItem_>>,
    pub cluster: crate::value::ExpString,
    pub external_id: Option<crate::value::ExpString>,
    pub launch_type: Option<crate::value::ExpString>,
    pub load_balancers: Option<Vec<super::ecs::taskset::LoadBalancer_>>,
    pub network_configuration: Option<super::ecs::taskset::NetworkConfiguration_>,
    pub platform_version: Option<crate::value::ExpString>,
    pub scale: Option<super::ecs::taskset::Scale_>,
    pub service: crate::value::ExpString,
    pub service_registries: Option<Vec<super::ecs::taskset::ServiceRegistry_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub task_definition: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ecs_TaskSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::ECS::TaskSet" $($field
        $value)*)
    };
}
pub use crate::__aws_ecs_TaskSet as TaskSet;
impl crate::template::ToResource for TaskSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ECS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TaskSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.capacity_provider_strategy {
            properties.insert(
                "CapacityProviderStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Cluster".to_string(),
            crate::value::ToValue::to_value(&self.cluster),
        );
        if let Some(ref value) = self.external_id {
            properties.insert(
                "ExternalId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.launch_type {
            properties.insert(
                "LaunchType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.load_balancers {
            properties.insert(
                "LoadBalancers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_configuration {
            properties.insert(
                "NetworkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.platform_version {
            properties.insert(
                "PlatformVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scale {
            properties.insert("Scale".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Service".to_string(),
            crate::value::ToValue::to_value(&self.service),
        );
        if let Some(ref value) = self.service_registries {
            properties.insert(
                "ServiceRegistries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TaskDefinition".to_string(),
            crate::value::ToValue::to_value(&self.task_definition),
        );
        properties
    }
}
