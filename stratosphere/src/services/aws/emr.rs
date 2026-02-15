pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-application.html
    pub struct Application_ {
        pub additional_info: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub args: Option<Vec<crate::value::ExpString>>,
        pub name: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_Application {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.Application"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_Application as Application;
    impl crate::value::ToValue for Application_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_info {
                properties.insert(
                    "AdditionalInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.args {
                properties.insert("Args".to_string(), crate::value::ToValue::to_value(value));
            }
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-autoscalingpolicy.html
    pub struct AutoScalingPolicy_ {
        pub constraints: Box<ScalingConstraints_>,
        pub rules: Vec<ScalingRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_AutoScalingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.AutoScalingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_AutoScalingPolicy as AutoScalingPolicy;
    impl crate::value::ToValue for AutoScalingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Constraints".to_string(),
                crate::value::ToValue::to_value(&self.constraints),
            );
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-autoterminationpolicy.html
    pub struct AutoTerminationPolicy_ {
        pub idle_timeout: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_AutoTerminationPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.AutoTerminationPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_AutoTerminationPolicy as AutoTerminationPolicy;
    impl crate::value::ToValue for AutoTerminationPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.idle_timeout {
                properties.insert(
                    "IdleTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-bootstrapactionconfig.html
    pub struct BootstrapActionConfig_ {
        pub name: crate::value::ExpString,
        pub script_bootstrap_action: Box<ScriptBootstrapActionConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_BootstrapActionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.BootstrapActionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_BootstrapActionConfig as BootstrapActionConfig;
    impl crate::value::ToValue for BootstrapActionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "ScriptBootstrapAction".to_string(),
                crate::value::ToValue::to_value(&self.script_bootstrap_action),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-cloudwatchalarmdefinition.html
    pub struct CloudWatchAlarmDefinition_ {
        pub comparison_operator: crate::value::ExpString,
        pub dimensions: Option<Vec<MetricDimension_>>,
        pub evaluation_periods: Option<i32>,
        pub metric_name: crate::value::ExpString,
        pub namespace: Option<crate::value::ExpString>,
        pub period: i32,
        pub statistic: Option<crate::value::ExpString>,
        pub threshold: f64,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_CloudWatchAlarmDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.CloudWatchAlarmDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_CloudWatchAlarmDefinition as CloudWatchAlarmDefinition;
    impl crate::value::ToValue for CloudWatchAlarmDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComparisonOperator".to_string(),
                crate::value::ToValue::to_value(&self.comparison_operator),
            );
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
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
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Period".to_string(),
                crate::value::ToValue::to_value(&self.period),
            );
            if let Some(ref value) = self.statistic {
                properties.insert(
                    "Statistic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(&self.threshold),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-computelimits.html
    pub struct ComputeLimits_ {
        pub maximum_capacity_units: i32,
        pub maximum_core_capacity_units: Option<i32>,
        pub maximum_on_demand_capacity_units: Option<i32>,
        pub minimum_capacity_units: i32,
        pub unit_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_ComputeLimits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.ComputeLimits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_ComputeLimits as ComputeLimits;
    impl crate::value::ToValue for ComputeLimits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaximumCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.maximum_capacity_units),
            );
            if let Some(ref value) = self.maximum_core_capacity_units {
                properties.insert(
                    "MaximumCoreCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_on_demand_capacity_units {
                properties.insert(
                    "MaximumOnDemandCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MinimumCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.minimum_capacity_units),
            );
            properties.insert(
                "UnitType".to_string(),
                crate::value::ToValue::to_value(&self.unit_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-configuration.html
    pub struct Configuration_ {
        pub classification: Option<crate::value::ExpString>,
        pub configuration_properties:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub configurations: Option<Vec<Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_Configuration as Configuration;
    impl crate::value::ToValue for Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.classification {
                properties.insert(
                    "Classification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configuration_properties {
                properties.insert(
                    "ConfigurationProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsblockdeviceconfig.html
    pub struct EbsBlockDeviceConfig_ {
        pub volume_specification: Box<VolumeSpecification_>,
        pub volumes_per_instance: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_EbsBlockDeviceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.EbsBlockDeviceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_EbsBlockDeviceConfig as EbsBlockDeviceConfig;
    impl crate::value::ToValue for EbsBlockDeviceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VolumeSpecification".to_string(),
                crate::value::ToValue::to_value(&self.volume_specification),
            );
            if let Some(ref value) = self.volumes_per_instance {
                properties.insert(
                    "VolumesPerInstance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ebsconfiguration.html
    pub struct EbsConfiguration_ {
        pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig_>>,
        pub ebs_optimized: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_EbsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.EbsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_EbsConfiguration as EbsConfiguration;
    impl crate::value::ToValue for EbsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_block_device_configs {
                properties.insert(
                    "EbsBlockDeviceConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_optimized {
                properties.insert(
                    "EbsOptimized".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-hadoopjarstepconfig.html
    pub struct HadoopJarStepConfig_ {
        pub args: Option<Vec<crate::value::ExpString>>,
        pub jar: crate::value::ExpString,
        pub main_class: Option<crate::value::ExpString>,
        pub step_properties: Option<Vec<KeyValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_HadoopJarStepConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.HadoopJarStepConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_HadoopJarStepConfig as HadoopJarStepConfig;
    impl crate::value::ToValue for HadoopJarStepConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.args {
                properties.insert("Args".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Jar".to_string(),
                crate::value::ToValue::to_value(&self.jar),
            );
            if let Some(ref value) = self.main_class {
                properties.insert(
                    "MainClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.step_properties {
                properties.insert(
                    "StepProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetconfig.html
    pub struct InstanceFleetConfig_ {
        pub instance_type_configs: Option<Vec<InstanceTypeConfig_>>,
        pub launch_specifications: Option<Box<InstanceFleetProvisioningSpecifications_>>,
        pub name: Option<crate::value::ExpString>,
        pub resize_specifications: Option<Box<InstanceFleetResizingSpecifications_>>,
        pub target_on_demand_capacity: Option<i32>,
        pub target_spot_capacity: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_InstanceFleetConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.InstanceFleetConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_InstanceFleetConfig as InstanceFleetConfig;
    impl crate::value::ToValue for InstanceFleetConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_type_configs {
                properties.insert(
                    "InstanceTypeConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.launch_specifications {
                properties.insert(
                    "LaunchSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resize_specifications {
                properties.insert(
                    "ResizeSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_on_demand_capacity {
                properties.insert(
                    "TargetOnDemandCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_spot_capacity {
                properties.insert(
                    "TargetSpotCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetprovisioningspecifications.html
    pub struct InstanceFleetProvisioningSpecifications_ {
        pub on_demand_specification: Option<Box<OnDemandProvisioningSpecification_>>,
        pub spot_specification: Option<Box<SpotProvisioningSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_InstanceFleetProvisioningSpecifications {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.InstanceFleetProvisioningSpecifications"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_InstanceFleetProvisioningSpecifications as InstanceFleetProvisioningSpecifications;
    impl crate::value::ToValue for InstanceFleetProvisioningSpecifications_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_demand_specification {
                properties.insert(
                    "OnDemandSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_specification {
                properties.insert(
                    "SpotSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancefleetresizingspecifications.html
    pub struct InstanceFleetResizingSpecifications_ {
        pub on_demand_resize_specification: Option<Box<OnDemandResizingSpecification_>>,
        pub spot_resize_specification: Option<Box<SpotResizingSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_InstanceFleetResizingSpecifications {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.InstanceFleetResizingSpecifications"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_InstanceFleetResizingSpecifications as InstanceFleetResizingSpecifications;
    impl crate::value::ToValue for InstanceFleetResizingSpecifications_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_demand_resize_specification {
                properties.insert(
                    "OnDemandResizeSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_resize_specification {
                properties.insert(
                    "SpotResizeSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancegroupconfig.html
    pub struct InstanceGroupConfig_ {
        pub auto_scaling_policy: Option<Box<AutoScalingPolicy_>>,
        pub bid_price: Option<crate::value::ExpString>,
        pub configurations: Option<Vec<Configuration_>>,
        pub custom_ami_id: Option<crate::value::ExpString>,
        pub ebs_configuration: Option<Box<EbsConfiguration_>>,
        pub instance_count: i32,
        pub instance_type: crate::value::ExpString,
        pub market: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_InstanceGroupConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.InstanceGroupConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_InstanceGroupConfig as InstanceGroupConfig;
    impl crate::value::ToValue for InstanceGroupConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_scaling_policy {
                properties.insert(
                    "AutoScalingPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bid_price {
                properties.insert(
                    "BidPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_ami_id {
                properties.insert(
                    "CustomAmiId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_configuration {
                properties.insert(
                    "EbsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceCount".to_string(),
                crate::value::ToValue::to_value(&self.instance_count),
            );
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.market {
                properties.insert("Market".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-instancetypeconfig.html
    pub struct InstanceTypeConfig_ {
        pub bid_price: Option<crate::value::ExpString>,
        pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
        pub configurations: Option<Vec<Configuration_>>,
        pub custom_ami_id: Option<crate::value::ExpString>,
        pub ebs_configuration: Option<Box<EbsConfiguration_>>,
        pub instance_type: crate::value::ExpString,
        pub priority: Option<f64>,
        pub weighted_capacity: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_InstanceTypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.InstanceTypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_InstanceTypeConfig as InstanceTypeConfig;
    impl crate::value::ToValue for InstanceTypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bid_price {
                properties.insert(
                    "BidPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bid_price_as_percentage_of_on_demand_price {
                properties.insert(
                    "BidPriceAsPercentageOfOnDemandPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_ami_id {
                properties.insert(
                    "CustomAmiId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_configuration {
                properties.insert(
                    "EbsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weighted_capacity {
                properties.insert(
                    "WeightedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-jobflowinstancesconfig.html
    pub struct JobFlowInstancesConfig_ {
        pub additional_master_security_groups: Option<Vec<crate::value::ExpString>>,
        pub additional_slave_security_groups: Option<Vec<crate::value::ExpString>>,
        pub core_instance_fleet: Option<Box<InstanceFleetConfig_>>,
        pub core_instance_group: Option<Box<InstanceGroupConfig_>>,
        pub ec2_key_name: Option<crate::value::ExpString>,
        pub ec2_subnet_id: Option<crate::value::ExpString>,
        pub ec2_subnet_ids: Option<Vec<crate::value::ExpString>>,
        pub emr_managed_master_security_group: Option<crate::value::ExpString>,
        pub emr_managed_slave_security_group: Option<crate::value::ExpString>,
        pub hadoop_version: Option<crate::value::ExpString>,
        pub keep_job_flow_alive_when_no_steps: Option<crate::value::ExpBool>,
        pub master_instance_fleet: Option<Box<InstanceFleetConfig_>>,
        pub master_instance_group: Option<Box<InstanceGroupConfig_>>,
        pub placement: Option<Box<PlacementType_>>,
        pub service_access_security_group: Option<crate::value::ExpString>,
        pub task_instance_fleets: Option<Vec<InstanceFleetConfig_>>,
        pub task_instance_groups: Option<Vec<InstanceGroupConfig_>>,
        pub termination_protected: Option<crate::value::ExpBool>,
        pub unhealthy_node_replacement: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_JobFlowInstancesConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.JobFlowInstancesConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_JobFlowInstancesConfig as JobFlowInstancesConfig;
    impl crate::value::ToValue for JobFlowInstancesConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_master_security_groups {
                properties.insert(
                    "AdditionalMasterSecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.additional_slave_security_groups {
                properties.insert(
                    "AdditionalSlaveSecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.core_instance_fleet {
                properties.insert(
                    "CoreInstanceFleet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.core_instance_group {
                properties.insert(
                    "CoreInstanceGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_key_name {
                properties.insert(
                    "Ec2KeyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_subnet_id {
                properties.insert(
                    "Ec2SubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ec2_subnet_ids {
                properties.insert(
                    "Ec2SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.emr_managed_master_security_group {
                properties.insert(
                    "EmrManagedMasterSecurityGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.emr_managed_slave_security_group {
                properties.insert(
                    "EmrManagedSlaveSecurityGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hadoop_version {
                properties.insert(
                    "HadoopVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.keep_job_flow_alive_when_no_steps {
                properties.insert(
                    "KeepJobFlowAliveWhenNoSteps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_instance_fleet {
                properties.insert(
                    "MasterInstanceFleet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.master_instance_group {
                properties.insert(
                    "MasterInstanceGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement {
                properties.insert(
                    "Placement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_access_security_group {
                properties.insert(
                    "ServiceAccessSecurityGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_instance_fleets {
                properties.insert(
                    "TaskInstanceFleets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_instance_groups {
                properties.insert(
                    "TaskInstanceGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.termination_protected {
                properties.insert(
                    "TerminationProtected".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unhealthy_node_replacement {
                properties.insert(
                    "UnhealthyNodeReplacement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-kerberosattributes.html
    pub struct KerberosAttributes_ {
        pub ad_domain_join_password: Option<crate::value::ExpString>,
        pub ad_domain_join_user: Option<crate::value::ExpString>,
        pub cross_realm_trust_principal_password: Option<crate::value::ExpString>,
        pub kdc_admin_password: crate::value::ExpString,
        pub realm: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_KerberosAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.KerberosAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_KerberosAttributes as KerberosAttributes;
    impl crate::value::ToValue for KerberosAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ad_domain_join_password {
                properties.insert(
                    "ADDomainJoinPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ad_domain_join_user {
                properties.insert(
                    "ADDomainJoinUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cross_realm_trust_principal_password {
                properties.insert(
                    "CrossRealmTrustPrincipalPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KdcAdminPassword".to_string(),
                crate::value::ToValue::to_value(&self.kdc_admin_password),
            );
            properties.insert(
                "Realm".to_string(),
                crate::value::ToValue::to_value(&self.realm),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-keyvalue.html
    pub struct KeyValue_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_KeyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.KeyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_KeyValue as KeyValue;
    impl crate::value::ToValue for KeyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-managedscalingpolicy.html
    pub struct ManagedScalingPolicy_ {
        pub compute_limits: Option<Box<ComputeLimits_>>,
        pub scaling_strategy: Option<crate::value::ExpString>,
        pub utilization_performance_index: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_ManagedScalingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.ManagedScalingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_ManagedScalingPolicy as ManagedScalingPolicy;
    impl crate::value::ToValue for ManagedScalingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.compute_limits {
                properties.insert(
                    "ComputeLimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_strategy {
                properties.insert(
                    "ScalingStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.utilization_performance_index {
                properties.insert(
                    "UtilizationPerformanceIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-metricdimension.html
    pub struct MetricDimension_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_MetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.MetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_MetricDimension as MetricDimension;
    impl crate::value::ToValue for MetricDimension_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ondemandcapacityreservationoptions.html
    pub struct OnDemandCapacityReservationOptions_ {
        pub capacity_reservation_preference: Option<crate::value::ExpString>,
        pub capacity_reservation_resource_group_arn: Option<crate::value::ExpString>,
        pub usage_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_OnDemandCapacityReservationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.OnDemandCapacityReservationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_OnDemandCapacityReservationOptions as OnDemandCapacityReservationOptions;
    impl crate::value::ToValue for OnDemandCapacityReservationOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_reservation_preference {
                properties.insert(
                    "CapacityReservationPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_resource_group_arn {
                properties.insert(
                    "CapacityReservationResourceGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.usage_strategy {
                properties.insert(
                    "UsageStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ondemandprovisioningspecification.html
    pub struct OnDemandProvisioningSpecification_ {
        pub allocation_strategy: crate::value::ExpString,
        pub capacity_reservation_options: Option<Box<OnDemandCapacityReservationOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_OnDemandProvisioningSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.OnDemandProvisioningSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_OnDemandProvisioningSpecification as OnDemandProvisioningSpecification;
    impl crate::value::ToValue for OnDemandProvisioningSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllocationStrategy".to_string(),
                crate::value::ToValue::to_value(&self.allocation_strategy),
            );
            if let Some(ref value) = self.capacity_reservation_options {
                properties.insert(
                    "CapacityReservationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-ondemandresizingspecification.html
    pub struct OnDemandResizingSpecification_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub capacity_reservation_options: Option<Box<OnDemandCapacityReservationOptions_>>,
        pub timeout_duration_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_OnDemandResizingSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.OnDemandResizingSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_OnDemandResizingSpecification as OnDemandResizingSpecification;
    impl crate::value::ToValue for OnDemandResizingSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_options {
                properties.insert(
                    "CapacityReservationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_duration_minutes {
                properties.insert(
                    "TimeoutDurationMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-placementgroupconfig.html
    pub struct PlacementGroupConfig_ {
        pub instance_role: crate::value::ExpString,
        pub placement_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_PlacementGroupConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.PlacementGroupConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_PlacementGroupConfig as PlacementGroupConfig;
    impl crate::value::ToValue for PlacementGroupConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InstanceRole".to_string(),
                crate::value::ToValue::to_value(&self.instance_role),
            );
            if let Some(ref value) = self.placement_strategy {
                properties.insert(
                    "PlacementStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-placementtype.html
    pub struct PlacementType_ {
        pub availability_zone: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_PlacementType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.PlacementType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_PlacementType as PlacementType;
    impl crate::value::ToValue for PlacementType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(&self.availability_zone),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingaction.html
    pub struct ScalingAction_ {
        pub market: Option<crate::value::ExpString>,
        pub simple_scaling_policy_configuration: Box<SimpleScalingPolicyConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_ScalingAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.ScalingAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_ScalingAction as ScalingAction;
    impl crate::value::ToValue for ScalingAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.market {
                properties.insert("Market".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SimpleScalingPolicyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.simple_scaling_policy_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingconstraints.html
    pub struct ScalingConstraints_ {
        pub max_capacity: i32,
        pub min_capacity: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_ScalingConstraints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.ScalingConstraints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_ScalingConstraints as ScalingConstraints;
    impl crate::value::ToValue for ScalingConstraints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxCapacity".to_string(),
                crate::value::ToValue::to_value(&self.max_capacity),
            );
            properties.insert(
                "MinCapacity".to_string(),
                crate::value::ToValue::to_value(&self.min_capacity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingrule.html
    pub struct ScalingRule_ {
        pub action: Box<ScalingAction_>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub trigger: Box<ScalingTrigger_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_ScalingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.ScalingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_ScalingRule as ScalingRule;
    impl crate::value::ToValue for ScalingRule_ {
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
            properties.insert(
                "Trigger".to_string(),
                crate::value::ToValue::to_value(&self.trigger),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scalingtrigger.html
    pub struct ScalingTrigger_ {
        pub cloud_watch_alarm_definition: Box<CloudWatchAlarmDefinition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_ScalingTrigger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.ScalingTrigger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_ScalingTrigger as ScalingTrigger;
    impl crate::value::ToValue for ScalingTrigger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchAlarmDefinition".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_alarm_definition),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-scriptbootstrapactionconfig.html
    pub struct ScriptBootstrapActionConfig_ {
        pub args: Option<Vec<crate::value::ExpString>>,
        pub path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_ScriptBootstrapActionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.ScriptBootstrapActionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_ScriptBootstrapActionConfig as ScriptBootstrapActionConfig;
    impl crate::value::ToValue for ScriptBootstrapActionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.args {
                properties.insert("Args".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-simplescalingpolicyconfiguration.html
    pub struct SimpleScalingPolicyConfiguration_ {
        pub adjustment_type: Option<crate::value::ExpString>,
        pub cool_down: Option<i32>,
        pub scaling_adjustment: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_SimpleScalingPolicyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.SimpleScalingPolicyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_SimpleScalingPolicyConfiguration as SimpleScalingPolicyConfiguration;
    impl crate::value::ToValue for SimpleScalingPolicyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.adjustment_type {
                properties.insert(
                    "AdjustmentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cool_down {
                properties.insert(
                    "CoolDown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScalingAdjustment".to_string(),
                crate::value::ToValue::to_value(&self.scaling_adjustment),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotprovisioningspecification.html
    pub struct SpotProvisioningSpecification_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub block_duration_minutes: Option<i32>,
        pub timeout_action: crate::value::ExpString,
        pub timeout_duration_minutes: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_SpotProvisioningSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.SpotProvisioningSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_SpotProvisioningSpecification as SpotProvisioningSpecification;
    impl crate::value::ToValue for SpotProvisioningSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.block_duration_minutes {
                properties.insert(
                    "BlockDurationMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeoutAction".to_string(),
                crate::value::ToValue::to_value(&self.timeout_action),
            );
            properties.insert(
                "TimeoutDurationMinutes".to_string(),
                crate::value::ToValue::to_value(&self.timeout_duration_minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-spotresizingspecification.html
    pub struct SpotResizingSpecification_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub timeout_duration_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_SpotResizingSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.SpotResizingSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_SpotResizingSpecification as SpotResizingSpecification;
    impl crate::value::ToValue for SpotResizingSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_duration_minutes {
                properties.insert(
                    "TimeoutDurationMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-stepconfig.html
    pub struct StepConfig_ {
        pub action_on_failure: Option<crate::value::ExpString>,
        pub hadoop_jar_step: Box<HadoopJarStepConfig_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_StepConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.StepConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_StepConfig as StepConfig;
    impl crate::value::ToValue for StepConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_on_failure {
                properties.insert(
                    "ActionOnFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HadoopJarStep".to_string(),
                crate::value::ToValue::to_value(&self.hadoop_jar_step),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-cluster-volumespecification.html
    pub struct VolumeSpecification_ {
        pub iops: Option<i32>,
        pub size_in_gb: i32,
        pub throughput: Option<i32>,
        pub volume_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Cluster_VolumeSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Cluster.VolumeSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Cluster_VolumeSpecification as VolumeSpecification;
    impl crate::value::ToValue for VolumeSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.size_in_gb),
            );
            if let Some(ref value) = self.throughput {
                properties.insert(
                    "Throughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeType".to_string(),
                crate::value::ToValue::to_value(&self.volume_type),
            );
            properties.into()
        }
    }
}
pub mod instancefleetconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-configuration.html
    pub struct Configuration_ {
        pub classification: Option<crate::value::ExpString>,
        pub configuration_properties:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub configurations: Option<Vec<Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_Configuration as Configuration;
    impl crate::value::ToValue for Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.classification {
                properties.insert(
                    "Classification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configuration_properties {
                properties.insert(
                    "ConfigurationProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsblockdeviceconfig.html
    pub struct EbsBlockDeviceConfig_ {
        pub volume_specification: Box<VolumeSpecification_>,
        pub volumes_per_instance: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_EbsBlockDeviceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.EbsBlockDeviceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_EbsBlockDeviceConfig as EbsBlockDeviceConfig;
    impl crate::value::ToValue for EbsBlockDeviceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VolumeSpecification".to_string(),
                crate::value::ToValue::to_value(&self.volume_specification),
            );
            if let Some(ref value) = self.volumes_per_instance {
                properties.insert(
                    "VolumesPerInstance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ebsconfiguration.html
    pub struct EbsConfiguration_ {
        pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig_>>,
        pub ebs_optimized: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_EbsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.EbsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_EbsConfiguration as EbsConfiguration;
    impl crate::value::ToValue for EbsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_block_device_configs {
                properties.insert(
                    "EbsBlockDeviceConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_optimized {
                properties.insert(
                    "EbsOptimized".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancefleetprovisioningspecifications.html
    pub struct InstanceFleetProvisioningSpecifications_ {
        pub on_demand_specification: Option<Box<OnDemandProvisioningSpecification_>>,
        pub spot_specification: Option<Box<SpotProvisioningSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_InstanceFleetProvisioningSpecifications {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.InstanceFleetProvisioningSpecifications"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_InstanceFleetProvisioningSpecifications as InstanceFleetProvisioningSpecifications;
    impl crate::value::ToValue for InstanceFleetProvisioningSpecifications_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_demand_specification {
                properties.insert(
                    "OnDemandSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_specification {
                properties.insert(
                    "SpotSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancefleetresizingspecifications.html
    pub struct InstanceFleetResizingSpecifications_ {
        pub on_demand_resize_specification: Option<Box<OnDemandResizingSpecification_>>,
        pub spot_resize_specification: Option<Box<SpotResizingSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_InstanceFleetResizingSpecifications {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.InstanceFleetResizingSpecifications"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_InstanceFleetResizingSpecifications as InstanceFleetResizingSpecifications;
    impl crate::value::ToValue for InstanceFleetResizingSpecifications_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.on_demand_resize_specification {
                properties.insert(
                    "OnDemandResizeSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_resize_specification {
                properties.insert(
                    "SpotResizeSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-instancetypeconfig.html
    pub struct InstanceTypeConfig_ {
        pub bid_price: Option<crate::value::ExpString>,
        pub bid_price_as_percentage_of_on_demand_price: Option<f64>,
        pub configurations: Option<Vec<Configuration_>>,
        pub custom_ami_id: Option<crate::value::ExpString>,
        pub ebs_configuration: Option<Box<EbsConfiguration_>>,
        pub instance_type: crate::value::ExpString,
        pub priority: Option<f64>,
        pub weighted_capacity: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_InstanceTypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.InstanceTypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_InstanceTypeConfig as InstanceTypeConfig;
    impl crate::value::ToValue for InstanceTypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bid_price {
                properties.insert(
                    "BidPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bid_price_as_percentage_of_on_demand_price {
                properties.insert(
                    "BidPriceAsPercentageOfOnDemandPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_ami_id {
                properties.insert(
                    "CustomAmiId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_configuration {
                properties.insert(
                    "EbsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weighted_capacity {
                properties.insert(
                    "WeightedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ondemandcapacityreservationoptions.html
    pub struct OnDemandCapacityReservationOptions_ {
        pub capacity_reservation_preference: Option<crate::value::ExpString>,
        pub capacity_reservation_resource_group_arn: Option<crate::value::ExpString>,
        pub usage_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_OnDemandCapacityReservationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.OnDemandCapacityReservationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_OnDemandCapacityReservationOptions as OnDemandCapacityReservationOptions;
    impl crate::value::ToValue for OnDemandCapacityReservationOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_reservation_preference {
                properties.insert(
                    "CapacityReservationPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_resource_group_arn {
                properties.insert(
                    "CapacityReservationResourceGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.usage_strategy {
                properties.insert(
                    "UsageStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ondemandprovisioningspecification.html
    pub struct OnDemandProvisioningSpecification_ {
        pub allocation_strategy: crate::value::ExpString,
        pub capacity_reservation_options: Option<Box<OnDemandCapacityReservationOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_OnDemandProvisioningSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.OnDemandProvisioningSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_OnDemandProvisioningSpecification as OnDemandProvisioningSpecification;
    impl crate::value::ToValue for OnDemandProvisioningSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllocationStrategy".to_string(),
                crate::value::ToValue::to_value(&self.allocation_strategy),
            );
            if let Some(ref value) = self.capacity_reservation_options {
                properties.insert(
                    "CapacityReservationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-ondemandresizingspecification.html
    pub struct OnDemandResizingSpecification_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub capacity_reservation_options: Option<Box<OnDemandCapacityReservationOptions_>>,
        pub timeout_duration_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_OnDemandResizingSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.OnDemandResizingSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_OnDemandResizingSpecification as OnDemandResizingSpecification;
    impl crate::value::ToValue for OnDemandResizingSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_options {
                properties.insert(
                    "CapacityReservationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_duration_minutes {
                properties.insert(
                    "TimeoutDurationMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotprovisioningspecification.html
    pub struct SpotProvisioningSpecification_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub block_duration_minutes: Option<i32>,
        pub timeout_action: crate::value::ExpString,
        pub timeout_duration_minutes: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_SpotProvisioningSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.SpotProvisioningSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_SpotProvisioningSpecification as SpotProvisioningSpecification;
    impl crate::value::ToValue for SpotProvisioningSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.block_duration_minutes {
                properties.insert(
                    "BlockDurationMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TimeoutAction".to_string(),
                crate::value::ToValue::to_value(&self.timeout_action),
            );
            properties.insert(
                "TimeoutDurationMinutes".to_string(),
                crate::value::ToValue::to_value(&self.timeout_duration_minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-spotresizingspecification.html
    pub struct SpotResizingSpecification_ {
        pub allocation_strategy: Option<crate::value::ExpString>,
        pub timeout_duration_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_SpotResizingSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.SpotResizingSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_SpotResizingSpecification as SpotResizingSpecification;
    impl crate::value::ToValue for SpotResizingSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allocation_strategy {
                properties.insert(
                    "AllocationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_duration_minutes {
                properties.insert(
                    "TimeoutDurationMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancefleetconfig-volumespecification.html
    pub struct VolumeSpecification_ {
        pub iops: Option<i32>,
        pub size_in_gb: i32,
        pub throughput: Option<i32>,
        pub volume_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceFleetConfig_VolumeSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceFleetConfig.VolumeSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceFleetConfig_VolumeSpecification as VolumeSpecification;
    impl crate::value::ToValue for VolumeSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.size_in_gb),
            );
            if let Some(ref value) = self.throughput {
                properties.insert(
                    "Throughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeType".to_string(),
                crate::value::ToValue::to_value(&self.volume_type),
            );
            properties.into()
        }
    }
}
pub mod instancegroupconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-autoscalingpolicy.html
    pub struct AutoScalingPolicy_ {
        pub constraints: Box<ScalingConstraints_>,
        pub rules: Vec<ScalingRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_AutoScalingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.AutoScalingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_AutoScalingPolicy as AutoScalingPolicy;
    impl crate::value::ToValue for AutoScalingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Constraints".to_string(),
                crate::value::ToValue::to_value(&self.constraints),
            );
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-cloudwatchalarmdefinition.html
    pub struct CloudWatchAlarmDefinition_ {
        pub comparison_operator: crate::value::ExpString,
        pub dimensions: Option<Vec<MetricDimension_>>,
        pub evaluation_periods: Option<i32>,
        pub metric_name: crate::value::ExpString,
        pub namespace: Option<crate::value::ExpString>,
        pub period: i32,
        pub statistic: Option<crate::value::ExpString>,
        pub threshold: f64,
        pub unit: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_CloudWatchAlarmDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.CloudWatchAlarmDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_CloudWatchAlarmDefinition as CloudWatchAlarmDefinition;
    impl crate::value::ToValue for CloudWatchAlarmDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComparisonOperator".to_string(),
                crate::value::ToValue::to_value(&self.comparison_operator),
            );
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
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
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Period".to_string(),
                crate::value::ToValue::to_value(&self.period),
            );
            if let Some(ref value) = self.statistic {
                properties.insert(
                    "Statistic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(&self.threshold),
            );
            if let Some(ref value) = self.unit {
                properties.insert("Unit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-cluster-configuration.html
    pub struct Configuration_ {
        pub classification: Option<crate::value::ExpString>,
        pub configuration_properties:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub configurations: Option<Vec<Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_Configuration as Configuration;
    impl crate::value::ToValue for Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.classification {
                properties.insert(
                    "Classification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configuration_properties {
                properties.insert(
                    "ConfigurationProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig.html
    pub struct EbsBlockDeviceConfig_ {
        pub volume_specification: Box<VolumeSpecification_>,
        pub volumes_per_instance: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_EbsBlockDeviceConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.EbsBlockDeviceConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_EbsBlockDeviceConfig as EbsBlockDeviceConfig;
    impl crate::value::ToValue for EbsBlockDeviceConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VolumeSpecification".to_string(),
                crate::value::ToValue::to_value(&self.volume_specification),
            );
            if let Some(ref value) = self.volumes_per_instance {
                properties.insert(
                    "VolumesPerInstance".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration.html
    pub struct EbsConfiguration_ {
        pub ebs_block_device_configs: Option<Vec<EbsBlockDeviceConfig_>>,
        pub ebs_optimized: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_EbsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.EbsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_EbsConfiguration as EbsConfiguration;
    impl crate::value::ToValue for EbsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_block_device_configs {
                properties.insert(
                    "EbsBlockDeviceConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_optimized {
                properties.insert(
                    "EbsOptimized".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-metricdimension.html
    pub struct MetricDimension_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_MetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.MetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_MetricDimension as MetricDimension;
    impl crate::value::ToValue for MetricDimension_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingaction.html
    pub struct ScalingAction_ {
        pub market: Option<crate::value::ExpString>,
        pub simple_scaling_policy_configuration: Box<SimpleScalingPolicyConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_ScalingAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.ScalingAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_ScalingAction as ScalingAction;
    impl crate::value::ToValue for ScalingAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.market {
                properties.insert("Market".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SimpleScalingPolicyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.simple_scaling_policy_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingconstraints.html
    pub struct ScalingConstraints_ {
        pub max_capacity: i32,
        pub min_capacity: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_ScalingConstraints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.ScalingConstraints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_ScalingConstraints as ScalingConstraints;
    impl crate::value::ToValue for ScalingConstraints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxCapacity".to_string(),
                crate::value::ToValue::to_value(&self.max_capacity),
            );
            properties.insert(
                "MinCapacity".to_string(),
                crate::value::ToValue::to_value(&self.min_capacity),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingrule.html
    pub struct ScalingRule_ {
        pub action: Box<ScalingAction_>,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub trigger: Box<ScalingTrigger_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_ScalingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.ScalingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_ScalingRule as ScalingRule;
    impl crate::value::ToValue for ScalingRule_ {
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
            properties.insert(
                "Trigger".to_string(),
                crate::value::ToValue::to_value(&self.trigger),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-scalingtrigger.html
    pub struct ScalingTrigger_ {
        pub cloud_watch_alarm_definition: Box<CloudWatchAlarmDefinition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_ScalingTrigger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.ScalingTrigger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_ScalingTrigger as ScalingTrigger;
    impl crate::value::ToValue for ScalingTrigger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchAlarmDefinition".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_alarm_definition),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticmapreduce-instancegroupconfig-simplescalingpolicyconfiguration.html
    pub struct SimpleScalingPolicyConfiguration_ {
        pub adjustment_type: Option<crate::value::ExpString>,
        pub cool_down: Option<i32>,
        pub scaling_adjustment: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_SimpleScalingPolicyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.SimpleScalingPolicyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_SimpleScalingPolicyConfiguration as SimpleScalingPolicyConfiguration;
    impl crate::value::ToValue for SimpleScalingPolicyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.adjustment_type {
                properties.insert(
                    "AdjustmentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cool_down {
                properties.insert(
                    "CoolDown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ScalingAdjustment".to_string(),
                crate::value::ToValue::to_value(&self.scaling_adjustment),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-ebsconfiguration-ebsblockdeviceconfig-volumespecification.html
    pub struct VolumeSpecification_ {
        pub iops: Option<i32>,
        pub size_in_gb: i32,
        pub throughput: Option<i32>,
        pub volume_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_InstanceGroupConfig_VolumeSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::InstanceGroupConfig.VolumeSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_InstanceGroupConfig_VolumeSpecification as VolumeSpecification;
    impl crate::value::ToValue for VolumeSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "SizeInGB".to_string(),
                crate::value::ToValue::to_value(&self.size_in_gb),
            );
            if let Some(ref value) = self.throughput {
                properties.insert(
                    "Throughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VolumeType".to_string(),
                crate::value::ToValue::to_value(&self.volume_type),
            );
            properties.into()
        }
    }
}
pub mod step {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-step-hadoopjarstepconfig.html
    pub struct HadoopJarStepConfig_ {
        pub args: Option<Vec<crate::value::ExpString>>,
        pub jar: crate::value::ExpString,
        pub main_class: Option<crate::value::ExpString>,
        pub step_properties: Option<Vec<KeyValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Step_HadoopJarStepConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Step.HadoopJarStepConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Step_HadoopJarStepConfig as HadoopJarStepConfig;
    impl crate::value::ToValue for HadoopJarStepConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.args {
                properties.insert("Args".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Jar".to_string(),
                crate::value::ToValue::to_value(&self.jar),
            );
            if let Some(ref value) = self.main_class {
                properties.insert(
                    "MainClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.step_properties {
                properties.insert(
                    "StepProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emr-step-keyvalue.html
    pub struct KeyValue_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emr_Step_KeyValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMR::Step.KeyValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emr_Step_KeyValue as KeyValue;
    impl crate::value::ToValue for KeyValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-cluster.html
pub struct Cluster_ {
    pub additional_info: Option<serde_json::Value>,
    pub applications: Option<Vec<super::emr::cluster::Application_>>,
    pub auto_scaling_role: Option<crate::value::ExpString>,
    pub auto_termination_policy: Option<super::emr::cluster::AutoTerminationPolicy_>,
    pub bootstrap_actions: Option<Vec<super::emr::cluster::BootstrapActionConfig_>>,
    pub configurations: Option<Vec<super::emr::cluster::Configuration_>>,
    pub custom_ami_id: Option<crate::value::ExpString>,
    pub ebs_root_volume_iops: Option<i32>,
    pub ebs_root_volume_size: Option<i32>,
    pub ebs_root_volume_throughput: Option<i32>,
    pub instances: super::emr::cluster::JobFlowInstancesConfig_,
    pub job_flow_role: crate::value::ExpString,
    pub kerberos_attributes: Option<super::emr::cluster::KerberosAttributes_>,
    pub log_encryption_kms_key_id: Option<crate::value::ExpString>,
    pub log_uri: Option<crate::value::ExpString>,
    pub managed_scaling_policy: Option<super::emr::cluster::ManagedScalingPolicy_>,
    pub name: crate::value::ExpString,
    pub os_release_label: Option<crate::value::ExpString>,
    pub placement_group_configs: Option<Vec<super::emr::cluster::PlacementGroupConfig_>>,
    pub release_label: Option<crate::value::ExpString>,
    pub scale_down_behavior: Option<crate::value::ExpString>,
    pub security_configuration: Option<crate::value::ExpString>,
    pub service_role: crate::value::ExpString,
    pub step_concurrency_level: Option<i32>,
    pub steps: Option<Vec<super::emr::cluster::StepConfig_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub visible_to_all_users: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::Cluster" $($field
        $value)*)
    };
}
pub use crate::__aws_emr_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_info {
            properties.insert(
                "AdditionalInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.applications {
            properties.insert(
                "Applications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_scaling_role {
            properties.insert(
                "AutoScalingRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_termination_policy {
            properties.insert(
                "AutoTerminationPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bootstrap_actions {
            properties.insert(
                "BootstrapActions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configurations {
            properties.insert(
                "Configurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_ami_id {
            properties.insert(
                "CustomAmiId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ebs_root_volume_iops {
            properties.insert(
                "EbsRootVolumeIops".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ebs_root_volume_size {
            properties.insert(
                "EbsRootVolumeSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ebs_root_volume_throughput {
            properties.insert(
                "EbsRootVolumeThroughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Instances".to_string(),
            crate::value::ToValue::to_value(&self.instances),
        );
        properties.insert(
            "JobFlowRole".to_string(),
            crate::value::ToValue::to_value(&self.job_flow_role),
        );
        if let Some(ref value) = self.kerberos_attributes {
            properties.insert(
                "KerberosAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_encryption_kms_key_id {
            properties.insert(
                "LogEncryptionKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_uri {
            properties.insert("LogUri".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.managed_scaling_policy {
            properties.insert(
                "ManagedScalingPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.os_release_label {
            properties.insert(
                "OSReleaseLabel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.placement_group_configs {
            properties.insert(
                "PlacementGroupConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.release_label {
            properties.insert(
                "ReleaseLabel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scale_down_behavior {
            properties.insert(
                "ScaleDownBehavior".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_configuration {
            properties.insert(
                "SecurityConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceRole".to_string(),
            crate::value::ToValue::to_value(&self.service_role),
        );
        if let Some(ref value) = self.step_concurrency_level {
            properties.insert(
                "StepConcurrencyLevel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.steps {
            properties.insert("Steps".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.visible_to_all_users {
            properties.insert(
                "VisibleToAllUsers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticmapreduce-instancefleetconfig.html
pub struct InstanceFleetConfig_ {
    pub cluster_id: crate::value::ExpString,
    pub instance_fleet_type: crate::value::ExpString,
    pub instance_type_configs: Option<Vec<super::emr::instancefleetconfig::InstanceTypeConfig_>>,
    pub launch_specifications:
        Option<super::emr::instancefleetconfig::InstanceFleetProvisioningSpecifications_>,
    pub name: Option<crate::value::ExpString>,
    pub resize_specifications:
        Option<super::emr::instancefleetconfig::InstanceFleetResizingSpecifications_>,
    pub target_on_demand_capacity: Option<i32>,
    pub target_spot_capacity: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_InstanceFleetConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::InstanceFleetConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_emr_InstanceFleetConfig as InstanceFleetConfig;
impl crate::template::ToResource for InstanceFleetConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InstanceFleetConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterId".to_string(),
            crate::value::ToValue::to_value(&self.cluster_id),
        );
        properties.insert(
            "InstanceFleetType".to_string(),
            crate::value::ToValue::to_value(&self.instance_fleet_type),
        );
        if let Some(ref value) = self.instance_type_configs {
            properties.insert(
                "InstanceTypeConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.launch_specifications {
            properties.insert(
                "LaunchSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.resize_specifications {
            properties.insert(
                "ResizeSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_on_demand_capacity {
            properties.insert(
                "TargetOnDemandCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_spot_capacity {
            properties.insert(
                "TargetSpotCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-instancegroupconfig.html
pub struct InstanceGroupConfig_ {
    pub auto_scaling_policy: Option<super::emr::instancegroupconfig::AutoScalingPolicy_>,
    pub bid_price: Option<crate::value::ExpString>,
    pub configurations: Option<Vec<super::emr::instancegroupconfig::Configuration_>>,
    pub custom_ami_id: Option<crate::value::ExpString>,
    pub ebs_configuration: Option<super::emr::instancegroupconfig::EbsConfiguration_>,
    pub instance_count: i32,
    pub instance_role: crate::value::ExpString,
    pub instance_type: crate::value::ExpString,
    pub job_flow_id: crate::value::ExpString,
    pub market: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_InstanceGroupConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::InstanceGroupConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_emr_InstanceGroupConfig as InstanceGroupConfig;
impl crate::template::ToResource for InstanceGroupConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InstanceGroupConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling_policy {
            properties.insert(
                "AutoScalingPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bid_price {
            properties.insert(
                "BidPrice".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configurations {
            properties.insert(
                "Configurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_ami_id {
            properties.insert(
                "CustomAmiId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ebs_configuration {
            properties.insert(
                "EbsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceCount".to_string(),
            crate::value::ToValue::to_value(&self.instance_count),
        );
        properties.insert(
            "InstanceRole".to_string(),
            crate::value::ToValue::to_value(&self.instance_role),
        );
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        properties.insert(
            "JobFlowId".to_string(),
            crate::value::ToValue::to_value(&self.job_flow_id),
        );
        if let Some(ref value) = self.market {
            properties.insert("Market".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-securityconfiguration.html
pub struct SecurityConfiguration_ {
    pub name: Option<crate::value::ExpString>,
    pub security_configuration: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_SecurityConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::SecurityConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_emr_SecurityConfiguration as SecurityConfiguration;
impl crate::template::ToResource for SecurityConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "SecurityConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.security_configuration),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-step.html
pub struct Step_ {
    pub action_on_failure: crate::value::ExpString,
    pub encryption_key_arn: Option<crate::value::ExpString>,
    pub hadoop_jar_step: super::emr::step::HadoopJarStepConfig_,
    pub job_flow_id: crate::value::ExpString,
    pub log_uri: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_Step {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::Step" $($field
        $value)*)
    };
}
pub use crate::__aws_emr_Step as Step;
impl crate::template::ToResource for Step_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Step"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ActionOnFailure".to_string(),
            crate::value::ToValue::to_value(&self.action_on_failure),
        );
        if let Some(ref value) = self.encryption_key_arn {
            properties.insert(
                "EncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "HadoopJarStep".to_string(),
            crate::value::ToValue::to_value(&self.hadoop_jar_step),
        );
        properties.insert(
            "JobFlowId".to_string(),
            crate::value::ToValue::to_value(&self.job_flow_id),
        );
        if let Some(ref value) = self.log_uri {
            properties.insert("LogUri".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-studio.html
pub struct Studio_ {
    pub auth_mode: crate::value::ExpString,
    pub default_s3_location: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub encryption_key_arn: Option<crate::value::ExpString>,
    pub engine_security_group_id: crate::value::ExpString,
    pub idc_instance_arn: Option<crate::value::ExpString>,
    pub idc_user_assignment: Option<crate::value::ExpString>,
    pub idp_auth_url: Option<crate::value::ExpString>,
    pub idp_relay_state_parameter_name: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub service_role: crate::value::ExpString,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trusted_identity_propagation_enabled: Option<crate::value::ExpBool>,
    pub user_role: Option<crate::value::ExpString>,
    pub vpc_id: crate::value::ExpString,
    pub workspace_security_group_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_Studio {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::Studio" $($field
        $value)*)
    };
}
pub use crate::__aws_emr_Studio as Studio;
impl crate::template::ToResource for Studio_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Studio"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AuthMode".to_string(),
            crate::value::ToValue::to_value(&self.auth_mode),
        );
        properties.insert(
            "DefaultS3Location".to_string(),
            crate::value::ToValue::to_value(&self.default_s3_location),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_key_arn {
            properties.insert(
                "EncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EngineSecurityGroupId".to_string(),
            crate::value::ToValue::to_value(&self.engine_security_group_id),
        );
        if let Some(ref value) = self.idc_instance_arn {
            properties.insert(
                "IdcInstanceArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.idc_user_assignment {
            properties.insert(
                "IdcUserAssignment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.idp_auth_url {
            properties.insert(
                "IdpAuthUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.idp_relay_state_parameter_name {
            properties.insert(
                "IdpRelayStateParameterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ServiceRole".to_string(),
            crate::value::ToValue::to_value(&self.service_role),
        );
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trusted_identity_propagation_enabled {
            properties.insert(
                "TrustedIdentityPropagationEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_role {
            properties.insert(
                "UserRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties.insert(
            "WorkspaceSecurityGroupId".to_string(),
            crate::value::ToValue::to_value(&self.workspace_security_group_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-studiosessionmapping.html
pub struct StudioSessionMapping_ {
    pub identity_name: crate::value::ExpString,
    pub identity_type: crate::value::ExpString,
    pub session_policy_arn: crate::value::ExpString,
    pub studio_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_StudioSessionMapping {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::StudioSessionMapping"
        $($field $value)*)
    };
}
pub use crate::__aws_emr_StudioSessionMapping as StudioSessionMapping;
impl crate::template::ToResource for StudioSessionMapping_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StudioSessionMapping"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "IdentityName".to_string(),
            crate::value::ToValue::to_value(&self.identity_name),
        );
        properties.insert(
            "IdentityType".to_string(),
            crate::value::ToValue::to_value(&self.identity_type),
        );
        properties.insert(
            "SessionPolicyArn".to_string(),
            crate::value::ToValue::to_value(&self.session_policy_arn),
        );
        properties.insert(
            "StudioId".to_string(),
            crate::value::ToValue::to_value(&self.studio_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emr-walworkspace.html
pub struct WALWorkspace_ {
    pub tags: Option<Vec<crate::Tag_>>,
    pub wal_workspace_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emr_WALWorkspace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMR::WALWorkspace"
        $($field $value)*)
    };
}
pub use crate::__aws_emr_WALWorkspace as WALWorkspace;
impl crate::template::ToResource for WALWorkspace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMR"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WALWorkspace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.wal_workspace_name {
            properties.insert(
                "WALWorkspaceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
