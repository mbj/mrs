pub mod connector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-apachekafkacluster.html
    pub struct ApacheKafkaCluster_ {
        pub bootstrap_servers: crate::value::ExpString,
        pub vpc: Box<Vpc_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_ApacheKafkaCluster {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.ApacheKafkaCluster"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_ApacheKafkaCluster as ApacheKafkaCluster;
    impl crate::value::ToValue for ApacheKafkaCluster_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BootstrapServers".to_string(),
                crate::value::ToValue::to_value(&self.bootstrap_servers),
            );
            properties.insert(
                "Vpc".to_string(),
                crate::value::ToValue::to_value(&self.vpc),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-autoscaling.html
    pub struct AutoScaling_ {
        pub max_worker_count: i64,
        pub mcu_count: i64,
        pub min_worker_count: i64,
        pub scale_in_policy: Box<ScaleInPolicy_>,
        pub scale_out_policy: Box<ScaleOutPolicy_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_AutoScaling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.AutoScaling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_AutoScaling as AutoScaling;
    impl crate::value::ToValue for AutoScaling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxWorkerCount".to_string(),
                crate::value::ToValue::to_value(&self.max_worker_count),
            );
            properties.insert(
                "McuCount".to_string(),
                crate::value::ToValue::to_value(&self.mcu_count),
            );
            properties.insert(
                "MinWorkerCount".to_string(),
                crate::value::ToValue::to_value(&self.min_worker_count),
            );
            properties.insert(
                "ScaleInPolicy".to_string(),
                crate::value::ToValue::to_value(&self.scale_in_policy),
            );
            properties.insert(
                "ScaleOutPolicy".to_string(),
                crate::value::ToValue::to_value(&self.scale_out_policy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-capacity.html
    pub struct Capacity_ {
        pub auto_scaling: Option<Box<AutoScaling_>>,
        pub provisioned_capacity: Option<Box<ProvisionedCapacity_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_Capacity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.Capacity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_Capacity as Capacity;
    impl crate::value::ToValue for Capacity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_scaling {
                properties.insert(
                    "AutoScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provisioned_capacity {
                properties.insert(
                    "ProvisionedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-cloudwatchlogslogdelivery.html
    pub struct CloudWatchLogsLogDelivery_ {
        pub enabled: crate::value::ExpBool,
        pub log_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_CloudWatchLogsLogDelivery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.CloudWatchLogsLogDelivery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_CloudWatchLogsLogDelivery as CloudWatchLogsLogDelivery;
    impl crate::value::ToValue for CloudWatchLogsLogDelivery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.log_group {
                properties.insert(
                    "LogGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-customplugin.html
    pub struct CustomPlugin_ {
        pub custom_plugin_arn: crate::value::ExpString,
        pub revision: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_CustomPlugin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.CustomPlugin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_CustomPlugin as CustomPlugin;
    impl crate::value::ToValue for CustomPlugin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CustomPluginArn".to_string(),
                crate::value::ToValue::to_value(&self.custom_plugin_arn),
            );
            properties.insert(
                "Revision".to_string(),
                crate::value::ToValue::to_value(&self.revision),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-firehoselogdelivery.html
    pub struct FirehoseLogDelivery_ {
        pub delivery_stream: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_FirehoseLogDelivery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.FirehoseLogDelivery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_FirehoseLogDelivery as FirehoseLogDelivery;
    impl crate::value::ToValue for FirehoseLogDelivery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delivery_stream {
                properties.insert(
                    "DeliveryStream".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkacluster.html
    pub struct KafkaCluster_ {
        pub apache_kafka_cluster: Box<ApacheKafkaCluster_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_KafkaCluster {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.KafkaCluster"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_KafkaCluster as KafkaCluster;
    impl crate::value::ToValue for KafkaCluster_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApacheKafkaCluster".to_string(),
                crate::value::ToValue::to_value(&self.apache_kafka_cluster),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkaclusterclientauthentication.html
    pub struct KafkaClusterClientAuthentication_ {
        pub authentication_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_KafkaClusterClientAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.KafkaClusterClientAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_KafkaClusterClientAuthentication as KafkaClusterClientAuthentication;
    impl crate::value::ToValue for KafkaClusterClientAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(&self.authentication_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-kafkaclusterencryptionintransit.html
    pub struct KafkaClusterEncryptionInTransit_ {
        pub encryption_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_KafkaClusterEncryptionInTransit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.KafkaClusterEncryptionInTransit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_KafkaClusterEncryptionInTransit as KafkaClusterEncryptionInTransit;
    impl crate::value::ToValue for KafkaClusterEncryptionInTransit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionType".to_string(),
                crate::value::ToValue::to_value(&self.encryption_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-logdelivery.html
    pub struct LogDelivery_ {
        pub worker_log_delivery: Box<WorkerLogDelivery_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_LogDelivery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.LogDelivery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_LogDelivery as LogDelivery;
    impl crate::value::ToValue for LogDelivery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkerLogDelivery".to_string(),
                crate::value::ToValue::to_value(&self.worker_log_delivery),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-plugin.html
    pub struct Plugin_ {
        pub custom_plugin: Box<CustomPlugin_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_Plugin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.Plugin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_Plugin as Plugin;
    impl crate::value::ToValue for Plugin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CustomPlugin".to_string(),
                crate::value::ToValue::to_value(&self.custom_plugin),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-provisionedcapacity.html
    pub struct ProvisionedCapacity_ {
        pub mcu_count: Option<i64>,
        pub worker_count: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_ProvisionedCapacity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.ProvisionedCapacity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_ProvisionedCapacity as ProvisionedCapacity;
    impl crate::value::ToValue for ProvisionedCapacity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mcu_count {
                properties.insert(
                    "McuCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "WorkerCount".to_string(),
                crate::value::ToValue::to_value(&self.worker_count),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-s3logdelivery.html
    pub struct S3LogDelivery_ {
        pub bucket: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_S3LogDelivery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.S3LogDelivery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_S3LogDelivery as S3LogDelivery;
    impl crate::value::ToValue for S3LogDelivery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket {
                properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-scaleinpolicy.html
    pub struct ScaleInPolicy_ {
        pub cpu_utilization_percentage: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_ScaleInPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.ScaleInPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_ScaleInPolicy as ScaleInPolicy;
    impl crate::value::ToValue for ScaleInPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CpuUtilizationPercentage".to_string(),
                crate::value::ToValue::to_value(&self.cpu_utilization_percentage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-scaleoutpolicy.html
    pub struct ScaleOutPolicy_ {
        pub cpu_utilization_percentage: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_ScaleOutPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.ScaleOutPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_ScaleOutPolicy as ScaleOutPolicy;
    impl crate::value::ToValue for ScaleOutPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CpuUtilizationPercentage".to_string(),
                crate::value::ToValue::to_value(&self.cpu_utilization_percentage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-vpc.html
    pub struct Vpc_ {
        pub security_groups: Vec<crate::value::ExpString>,
        pub subnets: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_Vpc {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.Vpc"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_Vpc as Vpc;
    impl crate::value::ToValue for Vpc_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroups".to_string(),
                crate::value::ToValue::to_value(&self.security_groups),
            );
            properties.insert(
                "Subnets".to_string(),
                crate::value::ToValue::to_value(&self.subnets),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerconfiguration.html
    pub struct WorkerConfiguration_ {
        pub revision: i64,
        pub worker_configuration_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_WorkerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.WorkerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_WorkerConfiguration as WorkerConfiguration;
    impl crate::value::ToValue for WorkerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Revision".to_string(),
                crate::value::ToValue::to_value(&self.revision),
            );
            properties.insert(
                "WorkerConfigurationArn".to_string(),
                crate::value::ToValue::to_value(&self.worker_configuration_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-connector-workerlogdelivery.html
    pub struct WorkerLogDelivery_ {
        pub cloud_watch_logs: Option<Box<CloudWatchLogsLogDelivery_>>,
        pub firehose: Option<Box<FirehoseLogDelivery_>>,
        pub s3: Option<Box<S3LogDelivery_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_Connector_WorkerLogDelivery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::Connector.WorkerLogDelivery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_Connector_WorkerLogDelivery as WorkerLogDelivery;
    impl crate::value::ToValue for WorkerLogDelivery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs {
                properties.insert(
                    "CloudWatchLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.firehose {
                properties.insert(
                    "Firehose".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod customplugin {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-customplugin-custompluginfiledescription.html
    pub struct CustomPluginFileDescription_ {
        pub file_md5: Option<crate::value::ExpString>,
        pub file_size: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_CustomPlugin_CustomPluginFileDescription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::CustomPlugin.CustomPluginFileDescription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_CustomPlugin_CustomPluginFileDescription as CustomPluginFileDescription;
    impl crate::value::ToValue for CustomPluginFileDescription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.file_md5 {
                properties.insert(
                    "FileMd5".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_size {
                properties.insert(
                    "FileSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-customplugin-custompluginlocation.html
    pub struct CustomPluginLocation_ {
        pub s3_location: Box<S3Location_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_CustomPlugin_CustomPluginLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::CustomPlugin.CustomPluginLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_CustomPlugin_CustomPluginLocation as CustomPluginLocation;
    impl crate::value::ToValue for CustomPluginLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Location".to_string(),
                crate::value::ToValue::to_value(&self.s3_location),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-kafkaconnect-customplugin-s3location.html
    pub struct S3Location_ {
        pub bucket_arn: crate::value::ExpString,
        pub file_key: crate::value::ExpString,
        pub object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_kafkaconnect_CustomPlugin_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::KafkaConnect::CustomPlugin.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_kafkaconnect_CustomPlugin_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketArn".to_string(),
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-connector.html
pub struct Connector_ {
    pub capacity: super::kafkaconnect::connector::Capacity_,
    pub connector_configuration: std::collections::BTreeMap<String, crate::value::ExpString>,
    pub connector_description: Option<crate::value::ExpString>,
    pub connector_name: crate::value::ExpString,
    pub kafka_cluster: super::kafkaconnect::connector::KafkaCluster_,
    pub kafka_cluster_client_authentication:
        super::kafkaconnect::connector::KafkaClusterClientAuthentication_,
    pub kafka_cluster_encryption_in_transit:
        super::kafkaconnect::connector::KafkaClusterEncryptionInTransit_,
    pub kafka_connect_version: crate::value::ExpString,
    pub log_delivery: Option<super::kafkaconnect::connector::LogDelivery_>,
    pub plugins: Vec<super::kafkaconnect::connector::Plugin_>,
    pub service_execution_role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub worker_configuration: Option<super::kafkaconnect::connector::WorkerConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kafkaconnect_Connector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KafkaConnect::Connector"
        $($field $value)*)
    };
}
pub use crate::__aws_kafkaconnect_Connector as Connector;
impl crate::template::ToResource for Connector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KafkaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Connector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Capacity".to_string(),
            crate::value::ToValue::to_value(&self.capacity),
        );
        properties.insert(
            "ConnectorConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.connector_configuration),
        );
        if let Some(ref value) = self.connector_description {
            properties.insert(
                "ConnectorDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConnectorName".to_string(),
            crate::value::ToValue::to_value(&self.connector_name),
        );
        properties.insert(
            "KafkaCluster".to_string(),
            crate::value::ToValue::to_value(&self.kafka_cluster),
        );
        properties.insert(
            "KafkaClusterClientAuthentication".to_string(),
            crate::value::ToValue::to_value(&self.kafka_cluster_client_authentication),
        );
        properties.insert(
            "KafkaClusterEncryptionInTransit".to_string(),
            crate::value::ToValue::to_value(&self.kafka_cluster_encryption_in_transit),
        );
        properties.insert(
            "KafkaConnectVersion".to_string(),
            crate::value::ToValue::to_value(&self.kafka_connect_version),
        );
        if let Some(ref value) = self.log_delivery {
            properties.insert(
                "LogDelivery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Plugins".to_string(),
            crate::value::ToValue::to_value(&self.plugins),
        );
        properties.insert(
            "ServiceExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.service_execution_role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.worker_configuration {
            properties.insert(
                "WorkerConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-customplugin.html
pub struct CustomPlugin_ {
    pub content_type: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub location: super::kafkaconnect::customplugin::CustomPluginLocation_,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kafkaconnect_CustomPlugin {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KafkaConnect::CustomPlugin"
        $($field $value)*)
    };
}
pub use crate::__aws_kafkaconnect_CustomPlugin as CustomPlugin;
impl crate::template::ToResource for CustomPlugin_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KafkaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CustomPlugin"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContentType".to_string(),
            crate::value::ToValue::to_value(&self.content_type),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Location".to_string(),
            crate::value::ToValue::to_value(&self.location),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-kafkaconnect-workerconfiguration.html
pub struct WorkerConfiguration_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub properties_file_content: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_kafkaconnect_WorkerConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::KafkaConnect::WorkerConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_kafkaconnect_WorkerConfiguration as WorkerConfiguration;
impl crate::template::ToResource for WorkerConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("KafkaConnect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WorkerConfiguration"),
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
            "PropertiesFileContent".to_string(),
            crate::value::ToValue::to_value(&self.properties_file_content),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
