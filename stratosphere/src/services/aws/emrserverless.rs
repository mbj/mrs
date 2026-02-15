pub mod application {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostartconfiguration.html
    pub struct AutoStartConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_AutoStartConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.AutoStartConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_AutoStartConfiguration as AutoStartConfiguration;
    impl crate::value::ToValue for AutoStartConfiguration_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-autostopconfiguration.html
    pub struct AutoStopConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub idle_timeout_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_AutoStopConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.AutoStopConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_AutoStopConfiguration as AutoStopConfiguration;
    impl crate::value::ToValue for AutoStopConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.idle_timeout_minutes {
                properties.insert(
                    "IdleTimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-cloudwatchloggingconfiguration.html
    pub struct CloudWatchLoggingConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub encryption_key_arn: Option<crate::value::ExpString>,
        pub log_group_name: Option<crate::value::ExpString>,
        pub log_stream_name_prefix: Option<crate::value::ExpString>,
        pub log_type_map: Option<Vec<LogTypeMapKeyValuePair_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_CloudWatchLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.CloudWatchLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_CloudWatchLoggingConfiguration as CloudWatchLoggingConfiguration;
    impl crate::value::ToValue for CloudWatchLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_key_arn {
                properties.insert(
                    "EncryptionKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group_name {
                properties.insert(
                    "LogGroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_stream_name_prefix {
                properties.insert(
                    "LogStreamNamePrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_type_map {
                properties.insert(
                    "LogTypeMap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-configurationobject.html
    pub struct ConfigurationObject_ {
        pub classification: crate::value::ExpString,
        pub configurations: Option<Vec<ConfigurationObject_>>,
        pub properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_ConfigurationObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.ConfigurationObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_ConfigurationObject as ConfigurationObject;
    impl crate::value::ToValue for ConfigurationObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Classification".to_string(),
                crate::value::ToValue::to_value(&self.classification),
            );
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-identitycenterconfiguration.html
    pub struct IdentityCenterConfiguration_ {
        pub identity_center_instance_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_IdentityCenterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.IdentityCenterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_IdentityCenterConfiguration as IdentityCenterConfiguration;
    impl crate::value::ToValue for IdentityCenterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.identity_center_instance_arn {
                properties.insert(
                    "IdentityCenterInstanceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-imageconfigurationinput.html
    pub struct ImageConfigurationInput_ {
        pub image_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_ImageConfigurationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.ImageConfigurationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_ImageConfigurationInput as ImageConfigurationInput;
    impl crate::value::ToValue for ImageConfigurationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_uri {
                properties.insert(
                    "ImageUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfig.html
    pub struct InitialCapacityConfig_ {
        pub worker_configuration: Box<WorkerConfiguration_>,
        pub worker_count: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_InitialCapacityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.InitialCapacityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_InitialCapacityConfig as InitialCapacityConfig;
    impl crate::value::ToValue for InitialCapacityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkerConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.worker_configuration),
            );
            properties.insert(
                "WorkerCount".to_string(),
                crate::value::ToValue::to_value(&self.worker_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-initialcapacityconfigkeyvaluepair.html
    pub struct InitialCapacityConfigKeyValuePair_ {
        pub key: crate::value::ExpString,
        pub value: Box<InitialCapacityConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_InitialCapacityConfigKeyValuePair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.InitialCapacityConfigKeyValuePair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_InitialCapacityConfigKeyValuePair as InitialCapacityConfigKeyValuePair;
    impl crate::value::ToValue for InitialCapacityConfigKeyValuePair_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-interactiveconfiguration.html
    pub struct InteractiveConfiguration_ {
        pub livy_endpoint_enabled: Option<crate::value::ExpBool>,
        pub studio_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_InteractiveConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.InteractiveConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_InteractiveConfiguration as InteractiveConfiguration;
    impl crate::value::ToValue for InteractiveConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.livy_endpoint_enabled {
                properties.insert(
                    "LivyEndpointEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.studio_enabled {
                properties.insert(
                    "StudioEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-logtypemapkeyvaluepair.html
    pub struct LogTypeMapKeyValuePair_ {
        pub key: crate::value::ExpString,
        pub value: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_LogTypeMapKeyValuePair {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.LogTypeMapKeyValuePair"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_LogTypeMapKeyValuePair as LogTypeMapKeyValuePair;
    impl crate::value::ToValue for LogTypeMapKeyValuePair_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-managedpersistencemonitoringconfiguration.html
    pub struct ManagedPersistenceMonitoringConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub encryption_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_ManagedPersistenceMonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.ManagedPersistenceMonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_ManagedPersistenceMonitoringConfiguration as ManagedPersistenceMonitoringConfiguration;
    impl crate::value::ToValue for ManagedPersistenceMonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_key_arn {
                properties.insert(
                    "EncryptionKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-maximumallowedresources.html
    pub struct MaximumAllowedResources_ {
        pub cpu: crate::value::ExpString,
        pub disk: Option<crate::value::ExpString>,
        pub memory: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_MaximumAllowedResources {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.MaximumAllowedResources"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_MaximumAllowedResources as MaximumAllowedResources;
    impl crate::value::ToValue for MaximumAllowedResources_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cpu".to_string(),
                crate::value::ToValue::to_value(&self.cpu),
            );
            if let Some(ref value) = self.disk {
                properties.insert("Disk".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Memory".to_string(),
                crate::value::ToValue::to_value(&self.memory),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-monitoringconfiguration.html
    pub struct MonitoringConfiguration_ {
        pub cloud_watch_logging_configuration: Option<Box<CloudWatchLoggingConfiguration_>>,
        pub managed_persistence_monitoring_configuration:
            Option<Box<ManagedPersistenceMonitoringConfiguration_>>,
        pub prometheus_monitoring_configuration: Option<Box<PrometheusMonitoringConfiguration_>>,
        pub s3_monitoring_configuration: Option<Box<S3MonitoringConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_MonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.MonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_MonitoringConfiguration as MonitoringConfiguration;
    impl crate::value::ToValue for MonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logging_configuration {
                properties.insert(
                    "CloudWatchLoggingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_persistence_monitoring_configuration {
                properties.insert(
                    "ManagedPersistenceMonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prometheus_monitoring_configuration {
                properties.insert(
                    "PrometheusMonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_monitoring_configuration {
                properties.insert(
                    "S3MonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-prometheusmonitoringconfiguration.html
    pub struct PrometheusMonitoringConfiguration_ {
        pub remote_write_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_PrometheusMonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.PrometheusMonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_PrometheusMonitoringConfiguration as PrometheusMonitoringConfiguration;
    impl crate::value::ToValue for PrometheusMonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.remote_write_url {
                properties.insert(
                    "RemoteWriteUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-s3monitoringconfiguration.html
    pub struct S3MonitoringConfiguration_ {
        pub encryption_key_arn: Option<crate::value::ExpString>,
        pub log_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_S3MonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.S3MonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_S3MonitoringConfiguration as S3MonitoringConfiguration;
    impl crate::value::ToValue for S3MonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_key_arn {
                properties.insert(
                    "EncryptionKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_uri {
                properties.insert("LogUri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-schedulerconfiguration.html
    pub struct SchedulerConfiguration_ {
        pub max_concurrent_runs: Option<i32>,
        pub queue_timeout_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_SchedulerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.SchedulerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_SchedulerConfiguration as SchedulerConfiguration;
    impl crate::value::ToValue for SchedulerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_concurrent_runs {
                properties.insert(
                    "MaxConcurrentRuns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.queue_timeout_minutes {
                properties.insert(
                    "QueueTimeoutMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workerconfiguration.html
    pub struct WorkerConfiguration_ {
        pub cpu: crate::value::ExpString,
        pub disk: Option<crate::value::ExpString>,
        pub disk_type: Option<crate::value::ExpString>,
        pub memory: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_WorkerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.WorkerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_WorkerConfiguration as WorkerConfiguration;
    impl crate::value::ToValue for WorkerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cpu".to_string(),
                crate::value::ToValue::to_value(&self.cpu),
            );
            if let Some(ref value) = self.disk {
                properties.insert("Disk".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.disk_type {
                properties.insert(
                    "DiskType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Memory".to_string(),
                crate::value::ToValue::to_value(&self.memory),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-emrserverless-application-workertypespecificationinput.html
    pub struct WorkerTypeSpecificationInput_ {
        pub image_configuration: Option<Box<ImageConfigurationInput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_emrserverless_Application_WorkerTypeSpecificationInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EMRServerless::Application.WorkerTypeSpecificationInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_emrserverless_Application_WorkerTypeSpecificationInput as WorkerTypeSpecificationInput;
    impl crate::value::ToValue for WorkerTypeSpecificationInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_configuration {
                properties.insert(
                    "ImageConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-emrserverless-application.html
pub struct Application_ {
    pub architecture: Option<crate::value::ExpString>,
    pub auto_start_configuration:
        Option<super::emrserverless::application::AutoStartConfiguration_>,
    pub auto_stop_configuration: Option<super::emrserverless::application::AutoStopConfiguration_>,
    pub identity_center_configuration:
        Option<super::emrserverless::application::IdentityCenterConfiguration_>,
    pub image_configuration: Option<super::emrserverless::application::ImageConfigurationInput_>,
    pub initial_capacity:
        Option<Vec<super::emrserverless::application::InitialCapacityConfigKeyValuePair_>>,
    pub interactive_configuration:
        Option<super::emrserverless::application::InteractiveConfiguration_>,
    pub maximum_capacity: Option<super::emrserverless::application::MaximumAllowedResources_>,
    pub monitoring_configuration:
        Option<super::emrserverless::application::MonitoringConfiguration_>,
    pub name: Option<crate::value::ExpString>,
    pub network_configuration: Option<super::emrserverless::application::NetworkConfiguration_>,
    pub release_label: crate::value::ExpString,
    pub runtime_configuration: Option<Vec<super::emrserverless::application::ConfigurationObject_>>,
    pub scheduler_configuration: Option<super::emrserverless::application::SchedulerConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
    pub worker_type_specifications: Option<
        std::collections::BTreeMap<
            String,
            super::emrserverless::application::WorkerTypeSpecificationInput_,
        >,
    >,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_emrserverless_Application {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EMRServerless::Application"
        $($field $value)*)
    };
}
pub use crate::__aws_emrserverless_Application as Application;
impl crate::template::ToResource for Application_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EMRServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Application"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.architecture {
            properties.insert(
                "Architecture".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_start_configuration {
            properties.insert(
                "AutoStartConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_stop_configuration {
            properties.insert(
                "AutoStopConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_center_configuration {
            properties.insert(
                "IdentityCenterConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.image_configuration {
            properties.insert(
                "ImageConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.initial_capacity {
            properties.insert(
                "InitialCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.interactive_configuration {
            properties.insert(
                "InteractiveConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_capacity {
            properties.insert(
                "MaximumCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.monitoring_configuration {
            properties.insert(
                "MonitoringConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.network_configuration {
            properties.insert(
                "NetworkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ReleaseLabel".to_string(),
            crate::value::ToValue::to_value(&self.release_label),
        );
        if let Some(ref value) = self.runtime_configuration {
            properties.insert(
                "RuntimeConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scheduler_configuration {
            properties.insert(
                "SchedulerConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.worker_type_specifications {
            properties.insert(
                "WorkerTypeSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
