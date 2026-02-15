pub mod alias {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-alias-routingstrategy.html
    pub struct RoutingStrategy_ {
        pub fleet_id: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Alias_RoutingStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Alias.RoutingStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Alias_RoutingStrategy as RoutingStrategy;
    impl crate::value::ToValue for RoutingStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fleet_id {
                properties.insert(
                    "FleetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
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
}
pub mod build {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-build-storagelocation.html
    pub struct StorageLocation_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub object_version: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Build_StorageLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Build.StorageLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Build_StorageLocation as StorageLocation;
    impl crate::value::ToValue for StorageLocation_ {
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
            if let Some(ref value) = self.object_version {
                properties.insert(
                    "ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
}
pub mod containerfleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-connectionportrange.html
    pub struct ConnectionPortRange_ {
        pub from_port: i32,
        pub to_port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_ConnectionPortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.ConnectionPortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_ConnectionPortRange as ConnectionPortRange;
    impl crate::value::ToValue for ConnectionPortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-deploymentconfiguration.html
    pub struct DeploymentConfiguration_ {
        pub impairment_strategy: Option<crate::value::ExpString>,
        pub minimum_healthy_percentage: Option<i32>,
        pub protection_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_DeploymentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.DeploymentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_DeploymentConfiguration as DeploymentConfiguration;
    impl crate::value::ToValue for DeploymentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.impairment_strategy {
                properties.insert(
                    "ImpairmentStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_healthy_percentage {
                properties.insert(
                    "MinimumHealthyPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protection_strategy {
                properties.insert(
                    "ProtectionStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-deploymentdetails.html
    pub struct DeploymentDetails_ {
        pub latest_deployment_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_DeploymentDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.DeploymentDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_DeploymentDetails as DeploymentDetails;
    impl crate::value::ToValue for DeploymentDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.latest_deployment_id {
                properties.insert(
                    "LatestDeploymentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-gamesessioncreationlimitpolicy.html
    pub struct GameSessionCreationLimitPolicy_ {
        pub new_game_sessions_per_creator: Option<i32>,
        pub policy_period_in_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_GameSessionCreationLimitPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.GameSessionCreationLimitPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_GameSessionCreationLimitPolicy as GameSessionCreationLimitPolicy;
    impl crate::value::ToValue for GameSessionCreationLimitPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.new_game_sessions_per_creator {
                properties.insert(
                    "NewGameSessionsPerCreator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_period_in_minutes {
                properties.insert(
                    "PolicyPeriodInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-ippermission.html
    pub struct IpPermission_ {
        pub from_port: i32,
        pub ip_range: crate::value::ExpString,
        pub protocol: crate::value::ExpString,
        pub to_port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_IpPermission {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.IpPermission"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_IpPermission as IpPermission;
    impl crate::value::ToValue for IpPermission_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "IpRange".to_string(),
                crate::value::ToValue::to_value(&self.ip_range),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-locationcapacity.html
    pub struct LocationCapacity_ {
        pub desired_ec2_instances: Option<i32>,
        pub managed_capacity_configuration: Option<Box<ManagedCapacityConfiguration_>>,
        pub max_size: i32,
        pub min_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_LocationCapacity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.LocationCapacity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_LocationCapacity as LocationCapacity;
    impl crate::value::ToValue for LocationCapacity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.desired_ec2_instances {
                properties.insert(
                    "DesiredEC2Instances".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_capacity_configuration {
                properties.insert(
                    "ManagedCapacityConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxSize".to_string(),
                crate::value::ToValue::to_value(&self.max_size),
            );
            if let Some(ref value) = self.min_size {
                properties.insert(
                    "MinSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-locationconfiguration.html
    pub struct LocationConfiguration_ {
        pub location: crate::value::ExpString,
        pub location_capacity: Option<Box<LocationCapacity_>>,
        pub stopped_actions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_LocationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.LocationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_LocationConfiguration as LocationConfiguration;
    impl crate::value::ToValue for LocationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            if let Some(ref value) = self.location_capacity {
                properties.insert(
                    "LocationCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stopped_actions {
                properties.insert(
                    "StoppedActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-logconfiguration.html
    pub struct LogConfiguration_ {
        pub log_destination: Option<crate::value::ExpString>,
        pub log_group_arn: Option<crate::value::ExpString>,
        pub s3_bucket_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_LogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.LogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_LogConfiguration as LogConfiguration;
    impl crate::value::ToValue for LogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.log_destination {
                properties.insert(
                    "LogDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group_arn {
                properties.insert(
                    "LogGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_name {
                properties.insert(
                    "S3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-managedcapacityconfiguration.html
    pub struct ManagedCapacityConfiguration_ {
        pub scale_in_after_inactivity_minutes: Option<i32>,
        pub zero_capacity_strategy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_ManagedCapacityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.ManagedCapacityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_ManagedCapacityConfiguration as ManagedCapacityConfiguration;
    impl crate::value::ToValue for ManagedCapacityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.scale_in_after_inactivity_minutes {
                properties.insert(
                    "ScaleInAfterInactivityMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ZeroCapacityStrategy".to_string(),
                crate::value::ToValue::to_value(&self.zero_capacity_strategy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-scalingpolicy.html
    pub struct ScalingPolicy_ {
        pub comparison_operator: Option<crate::value::ExpString>,
        pub evaluation_periods: Option<i32>,
        pub metric_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub policy_type: Option<crate::value::ExpString>,
        pub scaling_adjustment: Option<i32>,
        pub scaling_adjustment_type: Option<crate::value::ExpString>,
        pub target_configuration: Option<Box<TargetConfiguration_>>,
        pub threshold: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_ScalingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.ScalingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_ScalingPolicy as ScalingPolicy;
    impl crate::value::ToValue for ScalingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comparison_operator {
                properties.insert(
                    "ComparisonOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluation_periods {
                properties.insert(
                    "EvaluationPeriods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.policy_type {
                properties.insert(
                    "PolicyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_adjustment {
                properties.insert(
                    "ScalingAdjustment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_adjustment_type {
                properties.insert(
                    "ScalingAdjustmentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_configuration {
                properties.insert(
                    "TargetConfiguration".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containerfleet-targetconfiguration.html
    pub struct TargetConfiguration_ {
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerFleet_TargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerFleet.TargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerFleet_TargetConfiguration as TargetConfiguration;
    impl crate::value::ToValue for TargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
}
pub mod containergroupdefinition {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-containerdependency.html
    pub struct ContainerDependency_ {
        pub condition: crate::value::ExpString,
        pub container_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_ContainerDependency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.ContainerDependency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_ContainerDependency as ContainerDependency;
    impl crate::value::ToValue for ContainerDependency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.insert(
                "ContainerName".to_string(),
                crate::value::ToValue::to_value(&self.container_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-containerenvironment.html
    pub struct ContainerEnvironment_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_ContainerEnvironment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.ContainerEnvironment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_ContainerEnvironment as ContainerEnvironment;
    impl crate::value::ToValue for ContainerEnvironment_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-containerhealthcheck.html
    pub struct ContainerHealthCheck_ {
        pub command: Vec<crate::value::ExpString>,
        pub interval: Option<i32>,
        pub retries: Option<i32>,
        pub start_period: Option<i32>,
        pub timeout: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_ContainerHealthCheck {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.ContainerHealthCheck"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_ContainerHealthCheck as ContainerHealthCheck;
    impl crate::value::ToValue for ContainerHealthCheck_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Command".to_string(),
                crate::value::ToValue::to_value(&self.command),
            );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-containermountpoint.html
    pub struct ContainerMountPoint_ {
        pub access_level: Option<crate::value::ExpString>,
        pub container_path: Option<crate::value::ExpString>,
        pub instance_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_ContainerMountPoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.ContainerMountPoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_ContainerMountPoint as ContainerMountPoint;
    impl crate::value::ToValue for ContainerMountPoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_level {
                properties.insert(
                    "AccessLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_path {
                properties.insert(
                    "ContainerPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstancePath".to_string(),
                crate::value::ToValue::to_value(&self.instance_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-containerportrange.html
    pub struct ContainerPortRange_ {
        pub from_port: i32,
        pub protocol: crate::value::ExpString,
        pub to_port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_ContainerPortRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.ContainerPortRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_ContainerPortRange as ContainerPortRange;
    impl crate::value::ToValue for ContainerPortRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-gameservercontainerdefinition.html
    pub struct GameServerContainerDefinition_ {
        pub container_name: crate::value::ExpString,
        pub depends_on: Option<Vec<ContainerDependency_>>,
        pub environment_override: Option<Vec<ContainerEnvironment_>>,
        pub image_uri: crate::value::ExpString,
        pub mount_points: Option<Vec<ContainerMountPoint_>>,
        pub port_configuration: Option<Box<PortConfiguration_>>,
        pub resolved_image_digest: Option<crate::value::ExpString>,
        pub server_sdk_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_GameServerContainerDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.GameServerContainerDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_GameServerContainerDefinition as GameServerContainerDefinition;
    impl crate::value::ToValue for GameServerContainerDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContainerName".to_string(),
                crate::value::ToValue::to_value(&self.container_name),
            );
            if let Some(ref value) = self.depends_on {
                properties.insert(
                    "DependsOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_override {
                properties.insert(
                    "EnvironmentOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            if let Some(ref value) = self.mount_points {
                properties.insert(
                    "MountPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port_configuration {
                properties.insert(
                    "PortConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resolved_image_digest {
                properties.insert(
                    "ResolvedImageDigest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ServerSdkVersion".to_string(),
                crate::value::ToValue::to_value(&self.server_sdk_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-portconfiguration.html
    pub struct PortConfiguration_ {
        pub container_port_ranges: Vec<ContainerPortRange_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_PortConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.PortConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_PortConfiguration as PortConfiguration;
    impl crate::value::ToValue for PortConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContainerPortRanges".to_string(),
                crate::value::ToValue::to_value(&self.container_port_ranges),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-containergroupdefinition-supportcontainerdefinition.html
    pub struct SupportContainerDefinition_ {
        pub container_name: crate::value::ExpString,
        pub depends_on: Option<Vec<ContainerDependency_>>,
        pub environment_override: Option<Vec<ContainerEnvironment_>>,
        pub essential: Option<crate::value::ExpBool>,
        pub health_check: Option<Box<ContainerHealthCheck_>>,
        pub image_uri: crate::value::ExpString,
        pub memory_hard_limit_mebibytes: Option<i32>,
        pub mount_points: Option<Vec<ContainerMountPoint_>>,
        pub port_configuration: Option<Box<PortConfiguration_>>,
        pub resolved_image_digest: Option<crate::value::ExpString>,
        pub vcpu: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_ContainerGroupDefinition_SupportContainerDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::ContainerGroupDefinition.SupportContainerDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_ContainerGroupDefinition_SupportContainerDefinition as SupportContainerDefinition;
    impl crate::value::ToValue for SupportContainerDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContainerName".to_string(),
                crate::value::ToValue::to_value(&self.container_name),
            );
            if let Some(ref value) = self.depends_on {
                properties.insert(
                    "DependsOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment_override {
                properties.insert(
                    "EnvironmentOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.essential {
                properties.insert(
                    "Essential".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check {
                properties.insert(
                    "HealthCheck".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageUri".to_string(),
                crate::value::ToValue::to_value(&self.image_uri),
            );
            if let Some(ref value) = self.memory_hard_limit_mebibytes {
                properties.insert(
                    "MemoryHardLimitMebibytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mount_points {
                properties.insert(
                    "MountPoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port_configuration {
                properties.insert(
                    "PortConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resolved_image_digest {
                properties.insert(
                    "ResolvedImageDigest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vcpu {
                properties.insert("Vcpu".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod fleet {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-anywhereconfiguration.html
    pub struct AnywhereConfiguration_ {
        pub cost: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_AnywhereConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.AnywhereConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_AnywhereConfiguration as AnywhereConfiguration;
    impl crate::value::ToValue for AnywhereConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cost".to_string(),
                crate::value::ToValue::to_value(&self.cost),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-certificateconfiguration.html
    pub struct CertificateConfiguration_ {
        pub certificate_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_CertificateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.CertificateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_CertificateConfiguration as CertificateConfiguration;
    impl crate::value::ToValue for CertificateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateType".to_string(),
                crate::value::ToValue::to_value(&self.certificate_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-ippermission.html
    pub struct IpPermission_ {
        pub from_port: i32,
        pub ip_range: crate::value::ExpString,
        pub protocol: crate::value::ExpString,
        pub to_port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_IpPermission {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.IpPermission"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_IpPermission as IpPermission;
    impl crate::value::ToValue for IpPermission_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FromPort".to_string(),
                crate::value::ToValue::to_value(&self.from_port),
            );
            properties.insert(
                "IpRange".to_string(),
                crate::value::ToValue::to_value(&self.ip_range),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.insert(
                "ToPort".to_string(),
                crate::value::ToValue::to_value(&self.to_port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationcapacity.html
    pub struct LocationCapacity_ {
        pub desired_ec2_instances: Option<i32>,
        pub managed_capacity_configuration: Option<Box<ManagedCapacityConfiguration_>>,
        pub max_size: i32,
        pub min_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_LocationCapacity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.LocationCapacity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_LocationCapacity as LocationCapacity;
    impl crate::value::ToValue for LocationCapacity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.desired_ec2_instances {
                properties.insert(
                    "DesiredEC2Instances".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_capacity_configuration {
                properties.insert(
                    "ManagedCapacityConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxSize".to_string(),
                crate::value::ToValue::to_value(&self.max_size),
            );
            if let Some(ref value) = self.min_size {
                properties.insert(
                    "MinSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-locationconfiguration.html
    pub struct LocationConfiguration_ {
        pub location: crate::value::ExpString,
        pub location_capacity: Option<Box<LocationCapacity_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_LocationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.LocationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_LocationConfiguration as LocationConfiguration;
    impl crate::value::ToValue for LocationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            if let Some(ref value) = self.location_capacity {
                properties.insert(
                    "LocationCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-managedcapacityconfiguration.html
    pub struct ManagedCapacityConfiguration_ {
        pub scale_in_after_inactivity_minutes: Option<i32>,
        pub zero_capacity_strategy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_ManagedCapacityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.ManagedCapacityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_ManagedCapacityConfiguration as ManagedCapacityConfiguration;
    impl crate::value::ToValue for ManagedCapacityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.scale_in_after_inactivity_minutes {
                properties.insert(
                    "ScaleInAfterInactivityMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ZeroCapacityStrategy".to_string(),
                crate::value::ToValue::to_value(&self.zero_capacity_strategy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-resourcecreationlimitpolicy.html
    pub struct ResourceCreationLimitPolicy_ {
        pub new_game_sessions_per_creator: Option<i32>,
        pub policy_period_in_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_ResourceCreationLimitPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.ResourceCreationLimitPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_ResourceCreationLimitPolicy as ResourceCreationLimitPolicy;
    impl crate::value::ToValue for ResourceCreationLimitPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.new_game_sessions_per_creator {
                properties.insert(
                    "NewGameSessionsPerCreator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_period_in_minutes {
                properties.insert(
                    "PolicyPeriodInMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-runtimeconfiguration.html
    pub struct RuntimeConfiguration_ {
        pub game_session_activation_timeout_seconds: Option<i32>,
        pub max_concurrent_game_session_activations: Option<i32>,
        pub server_processes: Option<Vec<ServerProcess_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_RuntimeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.RuntimeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_RuntimeConfiguration as RuntimeConfiguration;
    impl crate::value::ToValue for RuntimeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.game_session_activation_timeout_seconds {
                properties.insert(
                    "GameSessionActivationTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_concurrent_game_session_activations {
                properties.insert(
                    "MaxConcurrentGameSessionActivations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_processes {
                properties.insert(
                    "ServerProcesses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-scalingpolicy.html
    pub struct ScalingPolicy_ {
        pub comparison_operator: Option<crate::value::ExpString>,
        pub evaluation_periods: Option<i32>,
        pub location: Option<crate::value::ExpString>,
        pub metric_name: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub policy_type: Option<crate::value::ExpString>,
        pub scaling_adjustment: Option<i32>,
        pub scaling_adjustment_type: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
        pub target_configuration: Option<Box<TargetConfiguration_>>,
        pub threshold: Option<f64>,
        pub update_status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_ScalingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.ScalingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_ScalingPolicy as ScalingPolicy;
    impl crate::value::ToValue for ScalingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comparison_operator {
                properties.insert(
                    "ComparisonOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluation_periods {
                properties.insert(
                    "EvaluationPeriods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.policy_type {
                properties.insert(
                    "PolicyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_adjustment {
                properties.insert(
                    "ScalingAdjustment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_adjustment_type {
                properties.insert(
                    "ScalingAdjustmentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.target_configuration {
                properties.insert(
                    "TargetConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threshold {
                properties.insert(
                    "Threshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_status {
                properties.insert(
                    "UpdateStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-serverprocess.html
    pub struct ServerProcess_ {
        pub concurrent_executions: i32,
        pub launch_path: crate::value::ExpString,
        pub parameters: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_ServerProcess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.ServerProcess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_ServerProcess as ServerProcess;
    impl crate::value::ToValue for ServerProcess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConcurrentExecutions".to_string(),
                crate::value::ToValue::to_value(&self.concurrent_executions),
            );
            properties.insert(
                "LaunchPath".to_string(),
                crate::value::ToValue::to_value(&self.launch_path),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-fleet-targetconfiguration.html
    pub struct TargetConfiguration_ {
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Fleet_TargetConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Fleet.TargetConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Fleet_TargetConfiguration as TargetConfiguration;
    impl crate::value::ToValue for TargetConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
}
pub mod gameservergroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-autoscalingpolicy.html
    pub struct AutoScalingPolicy_ {
        pub estimated_instance_warmup: Option<f64>,
        pub target_tracking_configuration: Box<TargetTrackingConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameServerGroup_AutoScalingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameServerGroup.AutoScalingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameServerGroup_AutoScalingPolicy as AutoScalingPolicy;
    impl crate::value::ToValue for AutoScalingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.estimated_instance_warmup {
                properties.insert(
                    "EstimatedInstanceWarmup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetTrackingConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.target_tracking_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-instancedefinition.html
    pub struct InstanceDefinition_ {
        pub instance_type: crate::value::ExpString,
        pub weighted_capacity: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameServerGroup_InstanceDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameServerGroup.InstanceDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameServerGroup_InstanceDefinition as InstanceDefinition;
    impl crate::value::ToValue for InstanceDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.weighted_capacity {
                properties.insert(
                    "WeightedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-launchtemplate.html
    pub struct LaunchTemplate_ {
        pub launch_template_id: Option<crate::value::ExpString>,
        pub launch_template_name: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameServerGroup_LaunchTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameServerGroup.LaunchTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameServerGroup_LaunchTemplate as LaunchTemplate;
    impl crate::value::ToValue for LaunchTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.launch_template_id {
                properties.insert(
                    "LaunchTemplateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_template_name {
                properties.insert(
                    "LaunchTemplateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gameservergroup-targettrackingconfiguration.html
    pub struct TargetTrackingConfiguration_ {
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameServerGroup_TargetTrackingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameServerGroup.TargetTrackingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameServerGroup_TargetTrackingConfiguration as TargetTrackingConfiguration;
    impl crate::value::ToValue for TargetTrackingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
}
pub mod gamesessionqueue {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-filterconfiguration.html
    pub struct FilterConfiguration_ {
        pub allowed_locations: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameSessionQueue_FilterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameSessionQueue.FilterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameSessionQueue_FilterConfiguration as FilterConfiguration;
    impl crate::value::ToValue for FilterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_locations {
                properties.insert(
                    "AllowedLocations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-gamesessionqueuedestination.html
    pub struct GameSessionQueueDestination_ {
        pub destination_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameSessionQueue_GameSessionQueueDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameSessionQueue.GameSessionQueueDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameSessionQueue_GameSessionQueueDestination as GameSessionQueueDestination;
    impl crate::value::ToValue for GameSessionQueueDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_arn {
                properties.insert(
                    "DestinationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-playerlatencypolicy.html
    pub struct PlayerLatencyPolicy_ {
        pub maximum_individual_player_latency_milliseconds: Option<i32>,
        pub policy_duration_seconds: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameSessionQueue_PlayerLatencyPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameSessionQueue.PlayerLatencyPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameSessionQueue_PlayerLatencyPolicy as PlayerLatencyPolicy;
    impl crate::value::ToValue for PlayerLatencyPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_individual_player_latency_milliseconds {
                properties.insert(
                    "MaximumIndividualPlayerLatencyMilliseconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.policy_duration_seconds {
                properties.insert(
                    "PolicyDurationSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-gamesessionqueue-priorityconfiguration.html
    pub struct PriorityConfiguration_ {
        pub location_order: Option<Vec<crate::value::ExpString>>,
        pub priority_order: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_GameSessionQueue_PriorityConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::GameSessionQueue.PriorityConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_GameSessionQueue_PriorityConfiguration as PriorityConfiguration;
    impl crate::value::ToValue for PriorityConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.location_order {
                properties.insert(
                    "LocationOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.priority_order {
                properties.insert(
                    "PriorityOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod matchmakingconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-matchmakingconfiguration-gameproperty.html
    pub struct GameProperty_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_MatchmakingConfiguration_GameProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::MatchmakingConfiguration.GameProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_MatchmakingConfiguration_GameProperty as GameProperty;
    impl crate::value::ToValue for GameProperty_ {
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
pub mod script {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-gamelift-script-s3location.html
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub object_version: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_gamelift_Script_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::GameLift::Script.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_gamelift_Script_S3Location as S3Location;
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
            if let Some(ref value) = self.object_version {
                properties.insert(
                    "ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-alias.html
pub struct Alias_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub routing_strategy: super::gamelift::alias::RoutingStrategy_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_Alias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::Alias" $($field
        $value)*)
    };
}
pub use crate::__aws_gamelift_Alias as Alias;
impl crate::template::ToResource for Alias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Alias"),
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
            "RoutingStrategy".to_string(),
            crate::value::ToValue::to_value(&self.routing_strategy),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-build.html
pub struct Build_ {
    pub name: Option<crate::value::ExpString>,
    pub operating_system: Option<crate::value::ExpString>,
    pub server_sdk_version: Option<crate::value::ExpString>,
    pub storage_location: Option<super::gamelift::build::StorageLocation_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub version: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_Build {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::Build" $($field
        $value)*)
    };
}
pub use crate::__aws_gamelift_Build as Build;
impl crate::template::ToResource for Build_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Build"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.operating_system {
            properties.insert(
                "OperatingSystem".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_sdk_version {
            properties.insert(
                "ServerSdkVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_location {
            properties.insert(
                "StorageLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.version {
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-containerfleet.html
pub struct ContainerFleet_ {
    pub billing_type: Option<crate::value::ExpString>,
    pub deployment_configuration: Option<super::gamelift::containerfleet::DeploymentConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub fleet_role_arn: crate::value::ExpString,
    pub game_server_container_group_definition_name: Option<crate::value::ExpString>,
    pub game_server_container_groups_per_instance: Option<i32>,
    pub game_session_creation_limit_policy:
        Option<super::gamelift::containerfleet::GameSessionCreationLimitPolicy_>,
    pub instance_connection_port_range:
        Option<super::gamelift::containerfleet::ConnectionPortRange_>,
    pub instance_inbound_permissions: Option<Vec<super::gamelift::containerfleet::IpPermission_>>,
    pub instance_type: Option<crate::value::ExpString>,
    pub locations: Option<Vec<super::gamelift::containerfleet::LocationConfiguration_>>,
    pub log_configuration: Option<super::gamelift::containerfleet::LogConfiguration_>,
    pub metric_groups: Option<Vec<crate::value::ExpString>>,
    pub new_game_session_protection_policy: Option<crate::value::ExpString>,
    pub per_instance_container_group_definition_name: Option<crate::value::ExpString>,
    pub scaling_policies: Option<Vec<super::gamelift::containerfleet::ScalingPolicy_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_ContainerFleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::ContainerFleet"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_ContainerFleet as ContainerFleet;
impl crate::template::ToResource for ContainerFleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContainerFleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.billing_type {
            properties.insert(
                "BillingType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deployment_configuration {
            properties.insert(
                "DeploymentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FleetRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.fleet_role_arn),
        );
        if let Some(ref value) = self.game_server_container_group_definition_name {
            properties.insert(
                "GameServerContainerGroupDefinitionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.game_server_container_groups_per_instance {
            properties.insert(
                "GameServerContainerGroupsPerInstance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.game_session_creation_limit_policy {
            properties.insert(
                "GameSessionCreationLimitPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_connection_port_range {
            properties.insert(
                "InstanceConnectionPortRange".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_inbound_permissions {
            properties.insert(
                "InstanceInboundPermissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_type {
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.locations {
            properties.insert(
                "Locations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_configuration {
            properties.insert(
                "LogConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metric_groups {
            properties.insert(
                "MetricGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.new_game_session_protection_policy {
            properties.insert(
                "NewGameSessionProtectionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.per_instance_container_group_definition_name {
            properties.insert(
                "PerInstanceContainerGroupDefinitionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_policies {
            properties.insert(
                "ScalingPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-containergroupdefinition.html
pub struct ContainerGroupDefinition_ {
    pub container_group_type: Option<crate::value::ExpString>,
    pub game_server_container_definition:
        Option<super::gamelift::containergroupdefinition::GameServerContainerDefinition_>,
    pub name: crate::value::ExpString,
    pub operating_system: crate::value::ExpString,
    pub source_version_number: Option<i32>,
    pub support_container_definitions:
        Option<Vec<super::gamelift::containergroupdefinition::SupportContainerDefinition_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub total_memory_limit_mebibytes: i32,
    pub total_vcpu_limit: f64,
    pub version_description: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_ContainerGroupDefinition {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::ContainerGroupDefinition"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_ContainerGroupDefinition as ContainerGroupDefinition;
impl crate::template::ToResource for ContainerGroupDefinition_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContainerGroupDefinition"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.container_group_type {
            properties.insert(
                "ContainerGroupType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.game_server_container_definition {
            properties.insert(
                "GameServerContainerDefinition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "OperatingSystem".to_string(),
            crate::value::ToValue::to_value(&self.operating_system),
        );
        if let Some(ref value) = self.source_version_number {
            properties.insert(
                "SourceVersionNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.support_container_definitions {
            properties.insert(
                "SupportContainerDefinitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TotalMemoryLimitMebibytes".to_string(),
            crate::value::ToValue::to_value(&self.total_memory_limit_mebibytes),
        );
        properties.insert(
            "TotalVcpuLimit".to_string(),
            crate::value::ToValue::to_value(&self.total_vcpu_limit),
        );
        if let Some(ref value) = self.version_description {
            properties.insert(
                "VersionDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-fleet.html
pub struct Fleet_ {
    pub anywhere_configuration: Option<super::gamelift::fleet::AnywhereConfiguration_>,
    pub apply_capacity: Option<crate::value::ExpString>,
    pub build_id: Option<crate::value::ExpString>,
    pub certificate_configuration: Option<super::gamelift::fleet::CertificateConfiguration_>,
    pub compute_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub ec2_inbound_permissions: Option<Vec<super::gamelift::fleet::IpPermission_>>,
    pub ec2_instance_type: Option<crate::value::ExpString>,
    pub fleet_type: Option<crate::value::ExpString>,
    pub instance_role_arn: Option<crate::value::ExpString>,
    pub instance_role_credentials_provider: Option<crate::value::ExpString>,
    pub locations: Option<Vec<super::gamelift::fleet::LocationConfiguration_>>,
    pub metric_groups: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub new_game_session_protection_policy: Option<crate::value::ExpString>,
    pub peer_vpc_aws_account_id: Option<crate::value::ExpString>,
    pub peer_vpc_id: Option<crate::value::ExpString>,
    pub resource_creation_limit_policy:
        Option<super::gamelift::fleet::ResourceCreationLimitPolicy_>,
    pub runtime_configuration: Option<super::gamelift::fleet::RuntimeConfiguration_>,
    pub scaling_policies: Option<Vec<super::gamelift::fleet::ScalingPolicy_>>,
    pub script_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_Fleet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::Fleet" $($field
        $value)*)
    };
}
pub use crate::__aws_gamelift_Fleet as Fleet;
impl crate::template::ToResource for Fleet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Fleet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.anywhere_configuration {
            properties.insert(
                "AnywhereConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.apply_capacity {
            properties.insert(
                "ApplyCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.build_id {
            properties.insert(
                "BuildId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_configuration {
            properties.insert(
                "CertificateConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_type {
            properties.insert(
                "ComputeType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ec2_inbound_permissions {
            properties.insert(
                "EC2InboundPermissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ec2_instance_type {
            properties.insert(
                "EC2InstanceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fleet_type {
            properties.insert(
                "FleetType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_role_arn {
            properties.insert(
                "InstanceRoleARN".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instance_role_credentials_provider {
            properties.insert(
                "InstanceRoleCredentialsProvider".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.locations {
            properties.insert(
                "Locations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metric_groups {
            properties.insert(
                "MetricGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.new_game_session_protection_policy {
            properties.insert(
                "NewGameSessionProtectionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.peer_vpc_aws_account_id {
            properties.insert(
                "PeerVpcAwsAccountId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.peer_vpc_id {
            properties.insert(
                "PeerVpcId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_creation_limit_policy {
            properties.insert(
                "ResourceCreationLimitPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.runtime_configuration {
            properties.insert(
                "RuntimeConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_policies {
            properties.insert(
                "ScalingPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.script_id {
            properties.insert(
                "ScriptId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gameservergroup.html
pub struct GameServerGroup_ {
    pub auto_scaling_policy: Option<super::gamelift::gameservergroup::AutoScalingPolicy_>,
    pub balancing_strategy: Option<crate::value::ExpString>,
    pub delete_option: Option<crate::value::ExpString>,
    pub game_server_group_name: crate::value::ExpString,
    pub game_server_protection_policy: Option<crate::value::ExpString>,
    pub instance_definitions: Vec<super::gamelift::gameservergroup::InstanceDefinition_>,
    pub launch_template: Option<super::gamelift::gameservergroup::LaunchTemplate_>,
    pub max_size: Option<f64>,
    pub min_size: Option<f64>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_subnets: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_GameServerGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::GameServerGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_GameServerGroup as GameServerGroup;
impl crate::template::ToResource for GameServerGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GameServerGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling_policy {
            properties.insert(
                "AutoScalingPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.balancing_strategy {
            properties.insert(
                "BalancingStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delete_option {
            properties.insert(
                "DeleteOption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "GameServerGroupName".to_string(),
            crate::value::ToValue::to_value(&self.game_server_group_name),
        );
        if let Some(ref value) = self.game_server_protection_policy {
            properties.insert(
                "GameServerProtectionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceDefinitions".to_string(),
            crate::value::ToValue::to_value(&self.instance_definitions),
        );
        if let Some(ref value) = self.launch_template {
            properties.insert(
                "LaunchTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_size {
            properties.insert(
                "MaxSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.min_size {
            properties.insert(
                "MinSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_subnets {
            properties.insert(
                "VpcSubnets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-gamesessionqueue.html
pub struct GameSessionQueue_ {
    pub custom_event_data: Option<crate::value::ExpString>,
    pub destinations: Option<Vec<super::gamelift::gamesessionqueue::GameSessionQueueDestination_>>,
    pub filter_configuration: Option<super::gamelift::gamesessionqueue::FilterConfiguration_>,
    pub name: crate::value::ExpString,
    pub notification_target: Option<crate::value::ExpString>,
    pub player_latency_policies:
        Option<Vec<super::gamelift::gamesessionqueue::PlayerLatencyPolicy_>>,
    pub priority_configuration: Option<super::gamelift::gamesessionqueue::PriorityConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub timeout_in_seconds: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_GameSessionQueue {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::GameSessionQueue"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_GameSessionQueue as GameSessionQueue;
impl crate::template::ToResource for GameSessionQueue_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GameSessionQueue"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.custom_event_data {
            properties.insert(
                "CustomEventData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destinations {
            properties.insert(
                "Destinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_configuration {
            properties.insert(
                "FilterConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.notification_target {
            properties.insert(
                "NotificationTarget".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.player_latency_policies {
            properties.insert(
                "PlayerLatencyPolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.priority_configuration {
            properties.insert(
                "PriorityConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.timeout_in_seconds {
            properties.insert(
                "TimeoutInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-location.html
pub struct Location_ {
    pub location_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_Location {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::Location"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_Location as Location;
impl crate::template::ToResource for Location_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Location"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LocationName".to_string(),
            crate::value::ToValue::to_value(&self.location_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingconfiguration.html
pub struct MatchmakingConfiguration_ {
    pub acceptance_required: crate::value::ExpBool,
    pub acceptance_timeout_seconds: Option<i32>,
    pub additional_player_count: Option<i32>,
    pub backfill_mode: Option<crate::value::ExpString>,
    pub creation_time: Option<crate::value::ExpString>,
    pub custom_event_data: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub flex_match_mode: Option<crate::value::ExpString>,
    pub game_properties: Option<Vec<super::gamelift::matchmakingconfiguration::GameProperty_>>,
    pub game_session_data: Option<crate::value::ExpString>,
    pub game_session_queue_arns: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub notification_target: Option<crate::value::ExpString>,
    pub request_timeout_seconds: i32,
    pub rule_set_arn: Option<crate::value::ExpString>,
    pub rule_set_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_MatchmakingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::MatchmakingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_MatchmakingConfiguration as MatchmakingConfiguration;
impl crate::template::ToResource for MatchmakingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MatchmakingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AcceptanceRequired".to_string(),
            crate::value::ToValue::to_value(&self.acceptance_required),
        );
        if let Some(ref value) = self.acceptance_timeout_seconds {
            properties.insert(
                "AcceptanceTimeoutSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.additional_player_count {
            properties.insert(
                "AdditionalPlayerCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backfill_mode {
            properties.insert(
                "BackfillMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.creation_time {
            properties.insert(
                "CreationTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_event_data {
            properties.insert(
                "CustomEventData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.flex_match_mode {
            properties.insert(
                "FlexMatchMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.game_properties {
            properties.insert(
                "GameProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.game_session_data {
            properties.insert(
                "GameSessionData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.game_session_queue_arns {
            properties.insert(
                "GameSessionQueueArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.notification_target {
            properties.insert(
                "NotificationTarget".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RequestTimeoutSeconds".to_string(),
            crate::value::ToValue::to_value(&self.request_timeout_seconds),
        );
        if let Some(ref value) = self.rule_set_arn {
            properties.insert(
                "RuleSetArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleSetName".to_string(),
            crate::value::ToValue::to_value(&self.rule_set_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-matchmakingruleset.html
pub struct MatchmakingRuleSet_ {
    pub name: crate::value::ExpString,
    pub rule_set_body: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_MatchmakingRuleSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::MatchmakingRuleSet"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_MatchmakingRuleSet as MatchmakingRuleSet;
impl crate::template::ToResource for MatchmakingRuleSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MatchmakingRuleSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RuleSetBody".to_string(),
            crate::value::ToValue::to_value(&self.rule_set_body),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-gamelift-script.html
pub struct Script_ {
    pub name: Option<crate::value::ExpString>,
    pub node_js_version: Option<crate::value::ExpString>,
    pub storage_location: super::gamelift::script::S3Location_,
    pub tags: Option<Vec<crate::Tag_>>,
    pub version: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_gamelift_Script {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::GameLift::Script"
        $($field $value)*)
    };
}
pub use crate::__aws_gamelift_Script as Script;
impl crate::template::ToResource for Script_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("GameLift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Script"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.node_js_version {
            properties.insert(
                "NodeJsVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StorageLocation".to_string(),
            crate::value::ToValue::to_value(&self.storage_location),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.version {
            properties.insert(
                "Version".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
