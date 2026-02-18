pub mod cluster {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokerlogs.html>
    pub struct BrokerLogs_ {
        pub cloud_watch_logs: Option<Box<CloudWatchLogs_>>,
        pub firehose: Option<Box<Firehose_>>,
        pub s3: Option<Box<S3_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_BrokerLogs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.BrokerLogs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_BrokerLogs as BrokerLogs;
    impl crate::value::ToValue for BrokerLogs_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-brokernodegroupinfo.html>
    pub struct BrokerNodeGroupInfo_ {
        pub broker_az_distribution: Option<crate::value::ExpString>,
        pub client_subnets: Vec<crate::value::ExpString>,
        pub connectivity_info: Option<Box<ConnectivityInfo_>>,
        pub instance_type: crate::value::ExpString,
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub storage_info: Option<Box<StorageInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_BrokerNodeGroupInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.BrokerNodeGroupInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_BrokerNodeGroupInfo as BrokerNodeGroupInfo;
    impl crate::value::ToValue for BrokerNodeGroupInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.broker_az_distribution {
                properties.insert(
                    "BrokerAZDistribution".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ClientSubnets".to_string(),
                crate::value::ToValue::to_value(&self.client_subnets),
            );
            if let Some(ref value) = self.connectivity_info {
                properties.insert(
                    "ConnectivityInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_info {
                properties.insert(
                    "StorageInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-clientauthentication.html>
    pub struct ClientAuthentication_ {
        pub sasl: Option<Box<Sasl_>>,
        pub tls: Option<Box<Tls_>>,
        pub unauthenticated: Option<Box<Unauthenticated_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_ClientAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.ClientAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_ClientAuthentication as ClientAuthentication;
    impl crate::value::ToValue for ClientAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sasl {
                properties.insert("Sasl".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tls {
                properties.insert("Tls".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unauthenticated {
                properties.insert(
                    "Unauthenticated".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-cloudwatchlogs.html>
    pub struct CloudWatchLogs_ {
        pub enabled: crate::value::ExpBool,
        pub log_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_CloudWatchLogs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.CloudWatchLogs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_CloudWatchLogs as CloudWatchLogs;
    impl crate::value::ToValue for CloudWatchLogs_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-configurationinfo.html>
    pub struct ConfigurationInfo_ {
        pub arn: crate::value::ExpString,
        pub revision: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_ConfigurationInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.ConfigurationInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_ConfigurationInfo as ConfigurationInfo;
    impl crate::value::ToValue for ConfigurationInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.insert(
                "Revision".to_string(),
                crate::value::ToValue::to_value(&self.revision),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-connectivityinfo.html>
    pub struct ConnectivityInfo_ {
        pub network_type: Option<crate::value::ExpString>,
        pub public_access: Option<Box<PublicAccess_>>,
        pub vpc_connectivity: Option<Box<VpcConnectivity_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_ConnectivityInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.ConnectivityInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_ConnectivityInfo as ConnectivityInfo;
    impl crate::value::ToValue for ConnectivityInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_type {
                properties.insert(
                    "NetworkType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_access {
                properties.insert(
                    "PublicAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_connectivity {
                properties.insert(
                    "VpcConnectivity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-ebsstorageinfo.html>
    pub struct EBSStorageInfo_ {
        pub provisioned_throughput: Option<Box<ProvisionedThroughput_>>,
        pub volume_size: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_EBSStorageInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.EBSStorageInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_EBSStorageInfo as EBSStorageInfo;
    impl crate::value::ToValue for EBSStorageInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provisioned_throughput {
                properties.insert(
                    "ProvisionedThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_size {
                properties.insert(
                    "VolumeSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptionatrest.html>
    pub struct EncryptionAtRest_ {
        pub data_volume_kms_key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_EncryptionAtRest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.EncryptionAtRest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_EncryptionAtRest as EncryptionAtRest;
    impl crate::value::ToValue for EncryptionAtRest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataVolumeKMSKeyId".to_string(),
                crate::value::ToValue::to_value(&self.data_volume_kms_key_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptionintransit.html>
    pub struct EncryptionInTransit_ {
        pub client_broker: Option<crate::value::ExpString>,
        pub in_cluster: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_EncryptionInTransit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.EncryptionInTransit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_EncryptionInTransit as EncryptionInTransit;
    impl crate::value::ToValue for EncryptionInTransit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_broker {
                properties.insert(
                    "ClientBroker".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.in_cluster {
                properties.insert(
                    "InCluster".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-encryptioninfo.html>
    pub struct EncryptionInfo_ {
        pub encryption_at_rest: Option<Box<EncryptionAtRest_>>,
        pub encryption_in_transit: Option<Box<EncryptionInTransit_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_EncryptionInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.EncryptionInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_EncryptionInfo as EncryptionInfo;
    impl crate::value::ToValue for EncryptionInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_at_rest {
                properties.insert(
                    "EncryptionAtRest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_in_transit {
                properties.insert(
                    "EncryptionInTransit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-firehose.html>
    pub struct Firehose_ {
        pub delivery_stream: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Firehose {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Firehose"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Firehose as Firehose;
    impl crate::value::ToValue for Firehose_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-iam.html>
    pub struct Iam_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Iam {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Iam"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Iam as Iam;
    impl crate::value::ToValue for Iam_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-jmxexporter.html>
    pub struct JmxExporter_ {
        pub enabled_in_broker: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_JmxExporter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.JmxExporter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_JmxExporter as JmxExporter;
    impl crate::value::ToValue for JmxExporter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnabledInBroker".to_string(),
                crate::value::ToValue::to_value(&self.enabled_in_broker),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-logginginfo.html>
    pub struct LoggingInfo_ {
        pub broker_logs: Box<BrokerLogs_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_LoggingInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.LoggingInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_LoggingInfo as LoggingInfo;
    impl crate::value::ToValue for LoggingInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BrokerLogs".to_string(),
                crate::value::ToValue::to_value(&self.broker_logs),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-nodeexporter.html>
    pub struct NodeExporter_ {
        pub enabled_in_broker: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_NodeExporter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.NodeExporter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_NodeExporter as NodeExporter;
    impl crate::value::ToValue for NodeExporter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnabledInBroker".to_string(),
                crate::value::ToValue::to_value(&self.enabled_in_broker),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-openmonitoring.html>
    pub struct OpenMonitoring_ {
        pub prometheus: Box<Prometheus_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_OpenMonitoring {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.OpenMonitoring"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_OpenMonitoring as OpenMonitoring;
    impl crate::value::ToValue for OpenMonitoring_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Prometheus".to_string(),
                crate::value::ToValue::to_value(&self.prometheus),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-prometheus.html>
    pub struct Prometheus_ {
        pub jmx_exporter: Option<Box<JmxExporter_>>,
        pub node_exporter: Option<Box<NodeExporter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Prometheus {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Prometheus"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Prometheus as Prometheus;
    impl crate::value::ToValue for Prometheus_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.jmx_exporter {
                properties.insert(
                    "JmxExporter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_exporter {
                properties.insert(
                    "NodeExporter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-provisionedthroughput.html>
    pub struct ProvisionedThroughput_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub volume_throughput: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_ProvisionedThroughput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.ProvisionedThroughput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_ProvisionedThroughput as ProvisionedThroughput;
    impl crate::value::ToValue for ProvisionedThroughput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_throughput {
                properties.insert(
                    "VolumeThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-publicaccess.html>
    pub struct PublicAccess_ {
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_PublicAccess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.PublicAccess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_PublicAccess as PublicAccess;
    impl crate::value::ToValue for PublicAccess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-rebalancing.html>
    pub struct Rebalancing_ {
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Rebalancing {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Rebalancing"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Rebalancing as Rebalancing;
    impl crate::value::ToValue for Rebalancing_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-s3.html>
    pub struct S3_ {
        pub bucket: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_S3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.S3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_S3 as S3;
    impl crate::value::ToValue for S3_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-sasl.html>
    pub struct Sasl_ {
        pub iam: Option<Box<Iam_>>,
        pub scram: Option<Box<Scram_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Sasl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Sasl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Sasl as Sasl;
    impl crate::value::ToValue for Sasl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iam {
                properties.insert("Iam".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scram {
                properties.insert("Scram".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-scram.html>
    pub struct Scram_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Scram {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Scram"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Scram as Scram;
    impl crate::value::ToValue for Scram_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-storageinfo.html>
    pub struct StorageInfo_ {
        pub ebs_storage_info: Option<Box<EBSStorageInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_StorageInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.StorageInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_StorageInfo as StorageInfo;
    impl crate::value::ToValue for StorageInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ebs_storage_info {
                properties.insert(
                    "EBSStorageInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-tls.html>
    pub struct Tls_ {
        pub certificate_authority_arn_list: Option<Vec<crate::value::ExpString>>,
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Tls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Tls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Tls as Tls;
    impl crate::value::ToValue for Tls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_authority_arn_list {
                properties.insert(
                    "CertificateAuthorityArnList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-unauthenticated.html>
    pub struct Unauthenticated_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_Unauthenticated {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.Unauthenticated"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_Unauthenticated as Unauthenticated;
    impl crate::value::ToValue for Unauthenticated_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivity.html>
    pub struct VpcConnectivity_ {
        pub client_authentication: Option<Box<VpcConnectivityClientAuthentication_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_VpcConnectivity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.VpcConnectivity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_VpcConnectivity as VpcConnectivity;
    impl crate::value::ToValue for VpcConnectivity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_authentication {
                properties.insert(
                    "ClientAuthentication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityclientauthentication.html>
    pub struct VpcConnectivityClientAuthentication_ {
        pub sasl: Option<Box<VpcConnectivitySasl_>>,
        pub tls: Option<Box<VpcConnectivityTls_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_VpcConnectivityClientAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.VpcConnectivityClientAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_VpcConnectivityClientAuthentication as VpcConnectivityClientAuthentication;
    impl crate::value::ToValue for VpcConnectivityClientAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sasl {
                properties.insert("Sasl".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tls {
                properties.insert("Tls".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityiam.html>
    pub struct VpcConnectivityIam_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_VpcConnectivityIam {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.VpcConnectivityIam"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_VpcConnectivityIam as VpcConnectivityIam;
    impl crate::value::ToValue for VpcConnectivityIam_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivitysasl.html>
    pub struct VpcConnectivitySasl_ {
        pub iam: Option<Box<VpcConnectivityIam_>>,
        pub scram: Option<Box<VpcConnectivityScram_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_VpcConnectivitySasl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.VpcConnectivitySasl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_VpcConnectivitySasl as VpcConnectivitySasl;
    impl crate::value::ToValue for VpcConnectivitySasl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iam {
                properties.insert("Iam".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.scram {
                properties.insert("Scram".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivityscram.html>
    pub struct VpcConnectivityScram_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_VpcConnectivityScram {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.VpcConnectivityScram"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_VpcConnectivityScram as VpcConnectivityScram;
    impl crate::value::ToValue for VpcConnectivityScram_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-cluster-vpcconnectivitytls.html>
    pub struct VpcConnectivityTls_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Cluster_VpcConnectivityTls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Cluster.VpcConnectivityTls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Cluster_VpcConnectivityTls as VpcConnectivityTls;
    impl crate::value::ToValue for VpcConnectivityTls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
}
pub mod configuration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-configuration-latestrevision.html>
    pub struct LatestRevision_ {
        pub creation_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub revision: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Configuration_LatestRevision {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Configuration.LatestRevision"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Configuration_LatestRevision as LatestRevision;
    impl crate::value::ToValue for LatestRevision_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.creation_time {
                properties.insert(
                    "CreationTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.revision {
                properties.insert(
                    "Revision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod replicator {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-amazonmskcluster.html>
    pub struct AmazonMskCluster_ {
        pub msk_cluster_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_AmazonMskCluster {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.AmazonMskCluster"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_AmazonMskCluster as AmazonMskCluster;
    impl crate::value::ToValue for AmazonMskCluster_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MskClusterArn".to_string(),
                crate::value::ToValue::to_value(&self.msk_cluster_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-consumergroupreplication.html>
    pub struct ConsumerGroupReplication_ {
        pub consumer_groups_to_exclude: Option<Vec<crate::value::ExpString>>,
        pub consumer_groups_to_replicate: Vec<crate::value::ExpString>,
        pub detect_and_copy_new_consumer_groups: Option<crate::value::ExpBool>,
        pub synchronise_consumer_group_offsets: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_ConsumerGroupReplication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.ConsumerGroupReplication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_ConsumerGroupReplication as ConsumerGroupReplication;
    impl crate::value::ToValue for ConsumerGroupReplication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.consumer_groups_to_exclude {
                properties.insert(
                    "ConsumerGroupsToExclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConsumerGroupsToReplicate".to_string(),
                crate::value::ToValue::to_value(&self.consumer_groups_to_replicate),
            );
            if let Some(ref value) = self.detect_and_copy_new_consumer_groups {
                properties.insert(
                    "DetectAndCopyNewConsumerGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.synchronise_consumer_group_offsets {
                properties.insert(
                    "SynchroniseConsumerGroupOffsets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkacluster.html>
    pub struct KafkaCluster_ {
        pub amazon_msk_cluster: Box<AmazonMskCluster_>,
        pub vpc_config: Box<KafkaClusterClientVpcConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_KafkaCluster {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.KafkaCluster"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_KafkaCluster as KafkaCluster;
    impl crate::value::ToValue for KafkaCluster_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AmazonMskCluster".to_string(),
                crate::value::ToValue::to_value(&self.amazon_msk_cluster),
            );
            properties.insert(
                "VpcConfig".to_string(),
                crate::value::ToValue::to_value(&self.vpc_config),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-kafkaclusterclientvpcconfig.html>
    pub struct KafkaClusterClientVpcConfig_ {
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_KafkaClusterClientVpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.KafkaClusterClientVpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_KafkaClusterClientVpcConfig as KafkaClusterClientVpcConfig;
    impl crate::value::ToValue for KafkaClusterClientVpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationinfo.html>
    pub struct ReplicationInfo_ {
        pub consumer_group_replication: Box<ConsumerGroupReplication_>,
        pub source_kafka_cluster_arn: crate::value::ExpString,
        pub target_compression_type: crate::value::ExpString,
        pub target_kafka_cluster_arn: crate::value::ExpString,
        pub topic_replication: Box<TopicReplication_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_ReplicationInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.ReplicationInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_ReplicationInfo as ReplicationInfo;
    impl crate::value::ToValue for ReplicationInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConsumerGroupReplication".to_string(),
                crate::value::ToValue::to_value(&self.consumer_group_replication),
            );
            properties.insert(
                "SourceKafkaClusterArn".to_string(),
                crate::value::ToValue::to_value(&self.source_kafka_cluster_arn),
            );
            properties.insert(
                "TargetCompressionType".to_string(),
                crate::value::ToValue::to_value(&self.target_compression_type),
            );
            properties.insert(
                "TargetKafkaClusterArn".to_string(),
                crate::value::ToValue::to_value(&self.target_kafka_cluster_arn),
            );
            properties.insert(
                "TopicReplication".to_string(),
                crate::value::ToValue::to_value(&self.topic_replication),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationstartingposition.html>
    pub struct ReplicationStartingPosition_ {
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_ReplicationStartingPosition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.ReplicationStartingPosition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_ReplicationStartingPosition as ReplicationStartingPosition;
    impl crate::value::ToValue for ReplicationStartingPosition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-replicationtopicnameconfiguration.html>
    pub struct ReplicationTopicNameConfiguration_ {
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_ReplicationTopicNameConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.ReplicationTopicNameConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_ReplicationTopicNameConfiguration as ReplicationTopicNameConfiguration;
    impl crate::value::ToValue for ReplicationTopicNameConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-replicator-topicreplication.html>
    pub struct TopicReplication_ {
        pub copy_access_control_lists_for_topics: Option<crate::value::ExpBool>,
        pub copy_topic_configurations: Option<crate::value::ExpBool>,
        pub detect_and_copy_new_topics: Option<crate::value::ExpBool>,
        pub starting_position: Option<Box<ReplicationStartingPosition_>>,
        pub topic_name_configuration: Option<Box<ReplicationTopicNameConfiguration_>>,
        pub topics_to_exclude: Option<Vec<crate::value::ExpString>>,
        pub topics_to_replicate: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_Replicator_TopicReplication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::Replicator.TopicReplication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_Replicator_TopicReplication as TopicReplication;
    impl crate::value::ToValue for TopicReplication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copy_access_control_lists_for_topics {
                properties.insert(
                    "CopyAccessControlListsForTopics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_topic_configurations {
                properties.insert(
                    "CopyTopicConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.detect_and_copy_new_topics {
                properties.insert(
                    "DetectAndCopyNewTopics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starting_position {
                properties.insert(
                    "StartingPosition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic_name_configuration {
                properties.insert(
                    "TopicNameConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topics_to_exclude {
                properties.insert(
                    "TopicsToExclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TopicsToReplicate".to_string(),
                crate::value::ToValue::to_value(&self.topics_to_replicate),
            );
            properties.into()
        }
    }
}
pub mod serverlesscluster {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-clientauthentication.html>
    pub struct ClientAuthentication_ {
        pub sasl: Box<Sasl_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_ServerlessCluster_ClientAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::ServerlessCluster.ClientAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_ServerlessCluster_ClientAuthentication as ClientAuthentication;
    impl crate::value::ToValue for ClientAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Sasl".to_string(),
                crate::value::ToValue::to_value(&self.sasl),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-iam.html>
    pub struct Iam_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_ServerlessCluster_Iam {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::ServerlessCluster.Iam"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_ServerlessCluster_Iam as Iam;
    impl crate::value::ToValue for Iam_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-sasl.html>
    pub struct Sasl_ {
        pub iam: Box<Iam_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_ServerlessCluster_Sasl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::ServerlessCluster.Sasl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_ServerlessCluster_Sasl as Sasl;
    impl crate::value::ToValue for Sasl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Iam".to_string(),
                crate::value::ToValue::to_value(&self.iam),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-msk-serverlesscluster-vpcconfig.html>
    pub struct VpcConfig_ {
        pub security_groups: Option<Vec<crate::value::ExpString>>,
        pub subnet_ids: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_msk_ServerlessCluster_VpcConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MSK::ServerlessCluster.VpcConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_msk_ServerlessCluster_VpcConfig as VpcConfig;
    impl crate::value::ToValue for VpcConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.security_groups {
                properties.insert(
                    "SecurityGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(&self.subnet_ids),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-batchscramsecret.html>
pub struct BatchScramSecret_ {
    pub cluster_arn: crate::value::ExpString,
    pub secret_arn_list: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_BatchScramSecret {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::BatchScramSecret"
        $($field $value)*)
    };
}
pub use crate::__aws_msk_BatchScramSecret as BatchScramSecret;
impl crate::template::ToResource for BatchScramSecret_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BatchScramSecret"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterArn".to_string(),
            crate::value::ToValue::to_value(&self.cluster_arn),
        );
        if let Some(ref value) = self.secret_arn_list {
            properties.insert(
                "SecretArnList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-cluster.html>
pub struct Cluster_ {
    pub broker_node_group_info: super::msk::cluster::BrokerNodeGroupInfo_,
    pub client_authentication: Option<super::msk::cluster::ClientAuthentication_>,
    pub cluster_name: crate::value::ExpString,
    pub configuration_info: Option<super::msk::cluster::ConfigurationInfo_>,
    pub encryption_info: Option<super::msk::cluster::EncryptionInfo_>,
    pub enhanced_monitoring: Option<crate::value::ExpString>,
    pub kafka_version: crate::value::ExpString,
    pub logging_info: Option<super::msk::cluster::LoggingInfo_>,
    pub number_of_broker_nodes: i32,
    pub open_monitoring: Option<super::msk::cluster::OpenMonitoring_>,
    pub rebalancing: Option<super::msk::cluster::Rebalancing_>,
    pub storage_mode: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::Cluster" $($field
        $value)*)
    };
}
pub use crate::__aws_msk_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BrokerNodeGroupInfo".to_string(),
            crate::value::ToValue::to_value(&self.broker_node_group_info),
        );
        if let Some(ref value) = self.client_authentication {
            properties.insert(
                "ClientAuthentication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.configuration_info {
            properties.insert(
                "ConfigurationInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_info {
            properties.insert(
                "EncryptionInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enhanced_monitoring {
            properties.insert(
                "EnhancedMonitoring".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KafkaVersion".to_string(),
            crate::value::ToValue::to_value(&self.kafka_version),
        );
        if let Some(ref value) = self.logging_info {
            properties.insert(
                "LoggingInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NumberOfBrokerNodes".to_string(),
            crate::value::ToValue::to_value(&self.number_of_broker_nodes),
        );
        if let Some(ref value) = self.open_monitoring {
            properties.insert(
                "OpenMonitoring".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rebalancing {
            properties.insert(
                "Rebalancing".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_mode {
            properties.insert(
                "StorageMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-clusterpolicy.html>
pub struct ClusterPolicy_ {
    pub cluster_arn: crate::value::ExpString,
    pub policy: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_ClusterPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::ClusterPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_msk_ClusterPolicy as ClusterPolicy;
impl crate::template::ToResource for ClusterPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ClusterPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterArn".to_string(),
            crate::value::ToValue::to_value(&self.cluster_arn),
        );
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-configuration.html>
pub struct Configuration_ {
    pub description: Option<crate::value::ExpString>,
    pub kafka_versions_list: Option<Vec<crate::value::ExpString>>,
    pub latest_revision: Option<super::msk::configuration::LatestRevision_>,
    pub name: crate::value::ExpString,
    pub server_properties: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_Configuration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::Configuration"
        $($field $value)*)
    };
}
pub use crate::__aws_msk_Configuration as Configuration;
impl crate::template::ToResource for Configuration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Configuration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kafka_versions_list {
            properties.insert(
                "KafkaVersionsList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.latest_revision {
            properties.insert(
                "LatestRevision".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "ServerProperties".to_string(),
            crate::value::ToValue::to_value(&self.server_properties),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-replicator.html>
pub struct Replicator_ {
    pub description: Option<crate::value::ExpString>,
    pub kafka_clusters: Vec<super::msk::replicator::KafkaCluster_>,
    pub replication_info_list: Vec<super::msk::replicator::ReplicationInfo_>,
    pub replicator_name: crate::value::ExpString,
    pub service_execution_role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_Replicator {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::Replicator" $($field
        $value)*)
    };
}
pub use crate::__aws_msk_Replicator as Replicator;
impl crate::template::ToResource for Replicator_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Replicator"),
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
            "KafkaClusters".to_string(),
            crate::value::ToValue::to_value(&self.kafka_clusters),
        );
        properties.insert(
            "ReplicationInfoList".to_string(),
            crate::value::ToValue::to_value(&self.replication_info_list),
        );
        properties.insert(
            "ReplicatorName".to_string(),
            crate::value::ToValue::to_value(&self.replicator_name),
        );
        properties.insert(
            "ServiceExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.service_execution_role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-serverlesscluster.html>
pub struct ServerlessCluster_ {
    pub client_authentication: super::msk::serverlesscluster::ClientAuthentication_,
    pub cluster_name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub vpc_configs: Vec<super::msk::serverlesscluster::VpcConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_ServerlessCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::ServerlessCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_msk_ServerlessCluster as ServerlessCluster;
impl crate::template::ToResource for ServerlessCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServerlessCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClientAuthentication".to_string(),
            crate::value::ToValue::to_value(&self.client_authentication),
        );
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcConfigs".to_string(),
            crate::value::ToValue::to_value(&self.vpc_configs),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-topic.html>
pub struct Topic_ {
    pub cluster_arn: crate::value::ExpString,
    pub configs: Option<crate::value::ExpString>,
    pub partition_count: i32,
    pub replication_factor: i32,
    pub topic_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_Topic {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::Topic" $($field
        $value)*)
    };
}
pub use crate::__aws_msk_Topic as Topic;
impl crate::template::ToResource for Topic_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Topic"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterArn".to_string(),
            crate::value::ToValue::to_value(&self.cluster_arn),
        );
        if let Some(ref value) = self.configs {
            properties.insert(
                "Configs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PartitionCount".to_string(),
            crate::value::ToValue::to_value(&self.partition_count),
        );
        properties.insert(
            "ReplicationFactor".to_string(),
            crate::value::ToValue::to_value(&self.replication_factor),
        );
        properties.insert(
            "TopicName".to_string(),
            crate::value::ToValue::to_value(&self.topic_name),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-msk-vpcconnection.html>
pub struct VpcConnection_ {
    pub authentication: crate::value::ExpString,
    pub client_subnets: Vec<crate::value::ExpString>,
    pub security_groups: Vec<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub target_cluster_arn: crate::value::ExpString,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_msk_VpcConnection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MSK::VpcConnection"
        $($field $value)*)
    };
}
pub use crate::__aws_msk_VpcConnection as VpcConnection;
impl crate::template::ToResource for VpcConnection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MSK"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VpcConnection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Authentication".to_string(),
            crate::value::ToValue::to_value(&self.authentication),
        );
        properties.insert(
            "ClientSubnets".to_string(),
            crate::value::ToValue::to_value(&self.client_subnets),
        );
        properties.insert(
            "SecurityGroups".to_string(),
            crate::value::ToValue::to_value(&self.security_groups),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetClusterArn".to_string(),
            crate::value::ToValue::to_value(&self.target_cluster_arn),
        );
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
