pub mod app {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-datasource.html>
    pub struct DataSource_ {
        pub arn: Option<crate::value::ExpString>,
        pub database_name: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_App_DataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::App.DataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_App_DataSource as DataSource;
    impl crate::value::ToValue for DataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.database_name {
                properties.insert(
                    "DatabaseName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-environment.html>
    pub struct EnvironmentVariable_ {
        pub key: crate::value::ExpString,
        pub secure: Option<crate::value::ExpBool>,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_App_EnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::App.EnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_App_EnvironmentVariable as EnvironmentVariable;
    impl crate::value::ToValue for EnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.secure {
                properties.insert("Secure".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html>
    pub struct Source_ {
        pub password: Option<crate::value::ExpString>,
        pub revision: Option<crate::value::ExpString>,
        pub ssh_key: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_App_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::App.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_App_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revision {
                properties.insert(
                    "Revision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssh_key {
                properties.insert("SshKey".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-app-sslconfiguration.html>
    pub struct SslConfiguration_ {
        pub certificate: Option<crate::value::ExpString>,
        pub chain: Option<crate::value::ExpString>,
        pub private_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_App_SslConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::App.SslConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_App_SslConfiguration as SslConfiguration;
    impl crate::value::ToValue for SslConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate {
                properties.insert(
                    "Certificate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.chain {
                properties.insert("Chain".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.private_key {
                properties.insert(
                    "PrivateKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod instance {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-blockdevicemapping.html>
    pub struct BlockDeviceMapping_ {
        pub device_name: Option<crate::value::ExpString>,
        pub ebs: Option<Box<EbsBlockDevice_>>,
        pub no_device: Option<crate::value::ExpString>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Instance_BlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Instance.BlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Instance_BlockDeviceMapping as BlockDeviceMapping;
    impl crate::value::ToValue for BlockDeviceMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.device_name {
                properties.insert(
                    "DeviceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs {
                properties.insert("Ebs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.no_device {
                properties.insert(
                    "NoDevice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.virtual_name {
                properties.insert(
                    "VirtualName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-ebsblockdevice.html>
    pub struct EbsBlockDevice_ {
        pub delete_on_termination: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub snapshot_id: Option<crate::value::ExpString>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Instance_EbsBlockDevice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Instance.EbsBlockDevice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Instance_EbsBlockDevice as EbsBlockDevice;
    impl crate::value::ToValue for EbsBlockDevice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_on_termination {
                properties.insert(
                    "DeleteOnTermination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.snapshot_id {
                properties.insert(
                    "SnapshotId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_size {
                properties.insert(
                    "VolumeSize".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-instance-timebasedautoscaling.html>
    pub struct TimeBasedAutoScaling_ {
        pub friday: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub monday: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub saturday: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub sunday: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub thursday: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub tuesday: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub wednesday: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Instance_TimeBasedAutoScaling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Instance.TimeBasedAutoScaling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Instance_TimeBasedAutoScaling as TimeBasedAutoScaling;
    impl crate::value::ToValue for TimeBasedAutoScaling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.friday {
                properties.insert("Friday".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.monday {
                properties.insert("Monday".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.saturday {
                properties.insert(
                    "Saturday".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sunday {
                properties.insert("Sunday".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.thursday {
                properties.insert(
                    "Thursday".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tuesday {
                properties.insert(
                    "Tuesday".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wednesday {
                properties.insert(
                    "Wednesday".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod layer {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling-autoscalingthresholds.html>
    pub struct AutoScalingThresholds_ {
        pub cpu_threshold: Option<f64>,
        pub ignore_metrics_time: Option<i32>,
        pub instance_count: Option<i32>,
        pub load_threshold: Option<f64>,
        pub memory_threshold: Option<f64>,
        pub thresholds_wait_time: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Layer_AutoScalingThresholds {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Layer.AutoScalingThresholds"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Layer_AutoScalingThresholds as AutoScalingThresholds;
    impl crate::value::ToValue for AutoScalingThresholds_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu_threshold {
                properties.insert(
                    "CpuThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_metrics_time {
                properties.insert(
                    "IgnoreMetricsTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_count {
                properties.insert(
                    "InstanceCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.load_threshold {
                properties.insert(
                    "LoadThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_threshold {
                properties.insert(
                    "MemoryThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.thresholds_wait_time {
                properties.insert(
                    "ThresholdsWaitTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration.html>
    pub struct LifecycleEventConfiguration_ {
        pub shutdown_event_configuration: Option<Box<ShutdownEventConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Layer_LifecycleEventConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Layer.LifecycleEventConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Layer_LifecycleEventConfiguration as LifecycleEventConfiguration;
    impl crate::value::ToValue for LifecycleEventConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.shutdown_event_configuration {
                properties.insert(
                    "ShutdownEventConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-loadbasedautoscaling.html>
    pub struct LoadBasedAutoScaling_ {
        pub down_scaling: Option<Box<AutoScalingThresholds_>>,
        pub enable: Option<crate::value::ExpBool>,
        pub up_scaling: Option<Box<AutoScalingThresholds_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Layer_LoadBasedAutoScaling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Layer.LoadBasedAutoScaling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Layer_LoadBasedAutoScaling as LoadBasedAutoScaling;
    impl crate::value::ToValue for LoadBasedAutoScaling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.down_scaling {
                properties.insert(
                    "DownScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable {
                properties.insert("Enable".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.up_scaling {
                properties.insert(
                    "UpScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-recipes.html>
    pub struct Recipes_ {
        pub configure: Option<Vec<crate::value::ExpString>>,
        pub deploy: Option<Vec<crate::value::ExpString>>,
        pub setup: Option<Vec<crate::value::ExpString>>,
        pub shutdown: Option<Vec<crate::value::ExpString>>,
        pub undeploy: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Layer_Recipes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Layer.Recipes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Layer_Recipes as Recipes;
    impl crate::value::ToValue for Recipes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configure {
                properties.insert(
                    "Configure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deploy {
                properties.insert("Deploy".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.setup {
                properties.insert("Setup".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.shutdown {
                properties.insert(
                    "Shutdown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.undeploy {
                properties.insert(
                    "Undeploy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-lifecycleeventconfiguration-shutdowneventconfiguration.html>
    pub struct ShutdownEventConfiguration_ {
        pub delay_until_elb_connections_drained: Option<crate::value::ExpBool>,
        pub execution_timeout: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Layer_ShutdownEventConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Layer.ShutdownEventConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Layer_ShutdownEventConfiguration as ShutdownEventConfiguration;
    impl crate::value::ToValue for ShutdownEventConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delay_until_elb_connections_drained {
                properties.insert(
                    "DelayUntilElbConnectionsDrained".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_timeout {
                properties.insert(
                    "ExecutionTimeout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-layer-volumeconfiguration.html>
    pub struct VolumeConfiguration_ {
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub mount_point: Option<crate::value::ExpString>,
        pub number_of_disks: Option<i32>,
        pub raid_level: Option<i32>,
        pub size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Layer_VolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Layer.VolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Layer_VolumeConfiguration as VolumeConfiguration;
    impl crate::value::ToValue for VolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encrypted {
                properties.insert(
                    "Encrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mount_point {
                properties.insert(
                    "MountPoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_disks {
                properties.insert(
                    "NumberOfDisks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.raid_level {
                properties.insert(
                    "RaidLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size {
                properties.insert("Size".to_string(), crate::value::ToValue::to_value(value));
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
}
pub mod stack {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-chefconfiguration.html>
    pub struct ChefConfiguration_ {
        pub berkshelf_version: Option<crate::value::ExpString>,
        pub manage_berkshelf: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Stack_ChefConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Stack.ChefConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Stack_ChefConfiguration as ChefConfiguration;
    impl crate::value::ToValue for ChefConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.berkshelf_version {
                properties.insert(
                    "BerkshelfVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manage_berkshelf {
                properties.insert(
                    "ManageBerkshelf".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-elasticip.html>
    pub struct ElasticIp_ {
        pub ip: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Stack_ElasticIp {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Stack.ElasticIp"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Stack_ElasticIp as ElasticIp;
    impl crate::value::ToValue for ElasticIp_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Ip".to_string(), crate::value::ToValue::to_value(&self.ip));
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-rdsdbinstance.html>
    pub struct RdsDbInstance_ {
        pub db_password: crate::value::ExpString,
        pub db_user: crate::value::ExpString,
        pub rds_db_instance_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Stack_RdsDbInstance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Stack.RdsDbInstance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Stack_RdsDbInstance as RdsDbInstance;
    impl crate::value::ToValue for RdsDbInstance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DbPassword".to_string(),
                crate::value::ToValue::to_value(&self.db_password),
            );
            properties.insert(
                "DbUser".to_string(),
                crate::value::ToValue::to_value(&self.db_user),
            );
            properties.insert(
                "RdsDbInstanceArn".to_string(),
                crate::value::ToValue::to_value(&self.rds_db_instance_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-source.html>
    pub struct Source_ {
        pub password: Option<crate::value::ExpString>,
        pub revision: Option<crate::value::ExpString>,
        pub ssh_key: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
        pub username: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Stack_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Stack.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Stack_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revision {
                properties.insert(
                    "Revision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssh_key {
                properties.insert("SshKey".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.username {
                properties.insert(
                    "Username".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-opsworks-stack-stackconfigmanager.html>
    pub struct StackConfigurationManager_ {
        pub name: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_opsworks_Stack_StackConfigurationManager {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::OpsWorks::Stack.StackConfigurationManager"
            $($field $value)*)
        };
    }
    pub use crate::__aws_opsworks_Stack_StackConfigurationManager as StackConfigurationManager;
    impl crate::value::ToValue for StackConfigurationManager_ {
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-app.html>
pub struct App_ {
    pub app_source: Option<super::opsworks::app::Source_>,
    pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub data_sources: Option<Vec<super::opsworks::app::DataSource_>>,
    pub description: Option<crate::value::ExpString>,
    pub domains: Option<Vec<crate::value::ExpString>>,
    pub enable_ssl: Option<crate::value::ExpBool>,
    pub environment: Option<Vec<super::opsworks::app::EnvironmentVariable_>>,
    pub name: crate::value::ExpString,
    pub shortname: Option<crate::value::ExpString>,
    pub ssl_configuration: Option<super::opsworks::app::SslConfiguration_>,
    pub stack_id: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworks_App {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorks::App" $($field
        $value)*)
    };
}
pub use crate::__aws_opsworks_App as App;
impl crate::template::ToResource for App_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorks"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("App"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_source {
            properties.insert(
                "AppSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.attributes {
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_sources {
            properties.insert(
                "DataSources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domains {
            properties.insert(
                "Domains".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_ssl {
            properties.insert(
                "EnableSsl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment {
            properties.insert(
                "Environment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.shortname {
            properties.insert(
                "Shortname".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ssl_configuration {
            properties.insert(
                "SslConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StackId".to_string(),
            crate::value::ToValue::to_value(&self.stack_id),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-elbattachment.html>
pub struct ElasticLoadBalancerAttachment_ {
    pub elastic_load_balancer_name: crate::value::ExpString,
    pub layer_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworks_ElasticLoadBalancerAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorks::ElasticLoadBalancerAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_opsworks_ElasticLoadBalancerAttachment as ElasticLoadBalancerAttachment;
impl crate::template::ToResource for ElasticLoadBalancerAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorks"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ElasticLoadBalancerAttachment",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ElasticLoadBalancerName".to_string(),
            crate::value::ToValue::to_value(&self.elastic_load_balancer_name),
        );
        properties.insert(
            "LayerId".to_string(),
            crate::value::ToValue::to_value(&self.layer_id),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-instance.html>
pub struct Instance_ {
    pub agent_version: Option<crate::value::ExpString>,
    pub ami_id: Option<crate::value::ExpString>,
    pub architecture: Option<crate::value::ExpString>,
    pub auto_scaling_type: Option<crate::value::ExpString>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub block_device_mappings: Option<Vec<super::opsworks::instance::BlockDeviceMapping_>>,
    pub ebs_optimized: Option<crate::value::ExpBool>,
    pub elastic_ips: Option<Vec<crate::value::ExpString>>,
    pub hostname: Option<crate::value::ExpString>,
    pub install_updates_on_boot: Option<crate::value::ExpBool>,
    pub instance_type: crate::value::ExpString,
    pub layer_ids: Vec<crate::value::ExpString>,
    pub os: Option<crate::value::ExpString>,
    pub root_device_type: Option<crate::value::ExpString>,
    pub ssh_key_name: Option<crate::value::ExpString>,
    pub stack_id: crate::value::ExpString,
    pub subnet_id: Option<crate::value::ExpString>,
    pub tenancy: Option<crate::value::ExpString>,
    pub time_based_auto_scaling: Option<super::opsworks::instance::TimeBasedAutoScaling_>,
    pub virtualization_type: Option<crate::value::ExpString>,
    pub volumes: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworks_Instance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorks::Instance"
        $($field $value)*)
    };
}
pub use crate::__aws_opsworks_Instance as Instance;
impl crate::template::ToResource for Instance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorks"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Instance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.agent_version {
            properties.insert(
                "AgentVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ami_id {
            properties.insert("AmiId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.architecture {
            properties.insert(
                "Architecture".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_scaling_type {
            properties.insert(
                "AutoScalingType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.block_device_mappings {
            properties.insert(
                "BlockDeviceMappings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ebs_optimized {
            properties.insert(
                "EbsOptimized".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elastic_ips {
            properties.insert(
                "ElasticIps".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hostname {
            properties.insert(
                "Hostname".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.install_updates_on_boot {
            properties.insert(
                "InstallUpdatesOnBoot".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceType".to_string(),
            crate::value::ToValue::to_value(&self.instance_type),
        );
        properties.insert(
            "LayerIds".to_string(),
            crate::value::ToValue::to_value(&self.layer_ids),
        );
        if let Some(ref value) = self.os {
            properties.insert("Os".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.root_device_type {
            properties.insert(
                "RootDeviceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ssh_key_name {
            properties.insert(
                "SshKeyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StackId".to_string(),
            crate::value::ToValue::to_value(&self.stack_id),
        );
        if let Some(ref value) = self.subnet_id {
            properties.insert(
                "SubnetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tenancy {
            properties.insert(
                "Tenancy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.time_based_auto_scaling {
            properties.insert(
                "TimeBasedAutoScaling".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.virtualization_type {
            properties.insert(
                "VirtualizationType".to_string(),
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-layer.html>
pub struct Layer_ {
    pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub auto_assign_elastic_ips: crate::value::ExpBool,
    pub auto_assign_public_ips: crate::value::ExpBool,
    pub custom_instance_profile_arn: Option<crate::value::ExpString>,
    pub custom_json: Option<serde_json::Value>,
    pub custom_recipes: Option<super::opsworks::layer::Recipes_>,
    pub custom_security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub enable_auto_healing: crate::value::ExpBool,
    pub install_updates_on_boot: Option<crate::value::ExpBool>,
    pub lifecycle_event_configuration: Option<super::opsworks::layer::LifecycleEventConfiguration_>,
    pub load_based_auto_scaling: Option<super::opsworks::layer::LoadBasedAutoScaling_>,
    pub name: crate::value::ExpString,
    pub packages: Option<Vec<crate::value::ExpString>>,
    pub shortname: crate::value::ExpString,
    pub stack_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
    pub use_ebs_optimized_instances: Option<crate::value::ExpBool>,
    pub volume_configurations: Option<Vec<super::opsworks::layer::VolumeConfiguration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworks_Layer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorks::Layer" $($field
        $value)*)
    };
}
pub use crate::__aws_opsworks_Layer as Layer;
impl crate::template::ToResource for Layer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorks"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Layer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attributes {
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AutoAssignElasticIps".to_string(),
            crate::value::ToValue::to_value(&self.auto_assign_elastic_ips),
        );
        properties.insert(
            "AutoAssignPublicIps".to_string(),
            crate::value::ToValue::to_value(&self.auto_assign_public_ips),
        );
        if let Some(ref value) = self.custom_instance_profile_arn {
            properties.insert(
                "CustomInstanceProfileArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_json {
            properties.insert(
                "CustomJson".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_recipes {
            properties.insert(
                "CustomRecipes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_security_group_ids {
            properties.insert(
                "CustomSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EnableAutoHealing".to_string(),
            crate::value::ToValue::to_value(&self.enable_auto_healing),
        );
        if let Some(ref value) = self.install_updates_on_boot {
            properties.insert(
                "InstallUpdatesOnBoot".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_event_configuration {
            properties.insert(
                "LifecycleEventConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.load_based_auto_scaling {
            properties.insert(
                "LoadBasedAutoScaling".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.packages {
            properties.insert(
                "Packages".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Shortname".to_string(),
            crate::value::ToValue::to_value(&self.shortname),
        );
        properties.insert(
            "StackId".to_string(),
            crate::value::ToValue::to_value(&self.stack_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.use_ebs_optimized_instances {
            properties.insert(
                "UseEbsOptimizedInstances".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volume_configurations {
            properties.insert(
                "VolumeConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-stack.html>
pub struct Stack_ {
    pub agent_version: Option<crate::value::ExpString>,
    pub attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub chef_configuration: Option<super::opsworks::stack::ChefConfiguration_>,
    pub clone_app_ids: Option<Vec<crate::value::ExpString>>,
    pub clone_permissions: Option<crate::value::ExpBool>,
    pub configuration_manager: Option<super::opsworks::stack::StackConfigurationManager_>,
    pub custom_cookbooks_source: Option<super::opsworks::stack::Source_>,
    pub custom_json: Option<serde_json::Value>,
    pub default_availability_zone: Option<crate::value::ExpString>,
    pub default_instance_profile_arn: crate::value::ExpString,
    pub default_os: Option<crate::value::ExpString>,
    pub default_root_device_type: Option<crate::value::ExpString>,
    pub default_ssh_key_name: Option<crate::value::ExpString>,
    pub default_subnet_id: Option<crate::value::ExpString>,
    pub ecs_cluster_arn: Option<crate::value::ExpString>,
    pub elastic_ips: Option<Vec<super::opsworks::stack::ElasticIp_>>,
    pub hostname_theme: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub rds_db_instances: Option<Vec<super::opsworks::stack::RdsDbInstance_>>,
    pub service_role_arn: crate::value::ExpString,
    pub source_stack_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub use_custom_cookbooks: Option<crate::value::ExpBool>,
    pub use_opsworks_security_groups: Option<crate::value::ExpBool>,
    pub vpc_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworks_Stack {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorks::Stack" $($field
        $value)*)
    };
}
pub use crate::__aws_opsworks_Stack as Stack;
impl crate::template::ToResource for Stack_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorks"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Stack"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.agent_version {
            properties.insert(
                "AgentVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.attributes {
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.chef_configuration {
            properties.insert(
                "ChefConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.clone_app_ids {
            properties.insert(
                "CloneAppIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.clone_permissions {
            properties.insert(
                "ClonePermissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.configuration_manager {
            properties.insert(
                "ConfigurationManager".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_cookbooks_source {
            properties.insert(
                "CustomCookbooksSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_json {
            properties.insert(
                "CustomJson".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_availability_zone {
            properties.insert(
                "DefaultAvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultInstanceProfileArn".to_string(),
            crate::value::ToValue::to_value(&self.default_instance_profile_arn),
        );
        if let Some(ref value) = self.default_os {
            properties.insert(
                "DefaultOs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_root_device_type {
            properties.insert(
                "DefaultRootDeviceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_ssh_key_name {
            properties.insert(
                "DefaultSshKeyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_subnet_id {
            properties.insert(
                "DefaultSubnetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ecs_cluster_arn {
            properties.insert(
                "EcsClusterArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elastic_ips {
            properties.insert(
                "ElasticIps".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hostname_theme {
            properties.insert(
                "HostnameTheme".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.rds_db_instances {
            properties.insert(
                "RdsDbInstances".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServiceRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.service_role_arn),
        );
        if let Some(ref value) = self.source_stack_id {
            properties.insert(
                "SourceStackId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.use_custom_cookbooks {
            properties.insert(
                "UseCustomCookbooks".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.use_opsworks_security_groups {
            properties.insert(
                "UseOpsworksSecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_id {
            properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-userprofile.html>
pub struct UserProfile_ {
    pub allow_self_management: Option<crate::value::ExpBool>,
    pub iam_user_arn: crate::value::ExpString,
    pub ssh_public_key: Option<crate::value::ExpString>,
    pub ssh_username: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworks_UserProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorks::UserProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_opsworks_UserProfile as UserProfile;
impl crate::template::ToResource for UserProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorks"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allow_self_management {
            properties.insert(
                "AllowSelfManagement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IamUserArn".to_string(),
            crate::value::ToValue::to_value(&self.iam_user_arn),
        );
        if let Some(ref value) = self.ssh_public_key {
            properties.insert(
                "SshPublicKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ssh_username {
            properties.insert(
                "SshUsername".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-opsworks-volume.html>
pub struct Volume_ {
    pub ec2_volume_id: crate::value::ExpString,
    pub mount_point: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub stack_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_opsworks_Volume {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::OpsWorks::Volume"
        $($field $value)*)
    };
}
pub use crate::__aws_opsworks_Volume as Volume;
impl crate::template::ToResource for Volume_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("OpsWorks"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Volume"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Ec2VolumeId".to_string(),
            crate::value::ToValue::to_value(&self.ec2_volume_id),
        );
        if let Some(ref value) = self.mount_point {
            properties.insert(
                "MountPoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "StackId".to_string(),
            crate::value::ToValue::to_value(&self.stack_id),
        );
        properties
    }
}
