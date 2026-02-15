pub mod cachecluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-cloudwatchlogsdestinationdetails.html
    pub struct CloudWatchLogsDestinationDetails_ {
        pub log_group: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_CacheCluster_CloudWatchLogsDestinationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::CacheCluster.CloudWatchLogsDestinationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_CacheCluster_CloudWatchLogsDestinationDetails as CloudWatchLogsDestinationDetails;
    impl crate::value::ToValue for CloudWatchLogsDestinationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroup".to_string(),
                crate::value::ToValue::to_value(&self.log_group),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-destinationdetails.html
    pub struct DestinationDetails_ {
        pub cloud_watch_logs_details: Option<Box<CloudWatchLogsDestinationDetails_>>,
        pub kinesis_firehose_details: Option<Box<KinesisFirehoseDestinationDetails_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_CacheCluster_DestinationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::CacheCluster.DestinationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_CacheCluster_DestinationDetails as DestinationDetails;
    impl crate::value::ToValue for DestinationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_details {
                properties.insert(
                    "CloudWatchLogsDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_firehose_details {
                properties.insert(
                    "KinesisFirehoseDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-kinesisfirehosedestinationdetails.html
    pub struct KinesisFirehoseDestinationDetails_ {
        pub delivery_stream: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_CacheCluster_KinesisFirehoseDestinationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::CacheCluster.KinesisFirehoseDestinationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_CacheCluster_KinesisFirehoseDestinationDetails as KinesisFirehoseDestinationDetails;
    impl crate::value::ToValue for KinesisFirehoseDestinationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeliveryStream".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cachecluster-logdeliveryconfigurationrequest.html
    pub struct LogDeliveryConfigurationRequest_ {
        pub destination_details: Box<DestinationDetails_>,
        pub destination_type: crate::value::ExpString,
        pub log_format: crate::value::ExpString,
        pub log_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_CacheCluster_LogDeliveryConfigurationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::CacheCluster.LogDeliveryConfigurationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_CacheCluster_LogDeliveryConfigurationRequest as LogDeliveryConfigurationRequest;
    impl crate::value::ToValue for LogDeliveryConfigurationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationDetails".to_string(),
                crate::value::ToValue::to_value(&self.destination_details),
            );
            properties.insert(
                "DestinationType".to_string(),
                crate::value::ToValue::to_value(&self.destination_type),
            );
            properties.insert(
                "LogFormat".to_string(),
                crate::value::ToValue::to_value(&self.log_format),
            );
            properties.insert(
                "LogType".to_string(),
                crate::value::ToValue::to_value(&self.log_type),
            );
            properties.into()
        }
    }
}
pub mod globalreplicationgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-globalreplicationgroupmember.html
    pub struct GlobalReplicationGroupMember_ {
        pub replication_group_id: Option<crate::value::ExpString>,
        pub replication_group_region: Option<crate::value::ExpString>,
        pub role: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_GlobalReplicationGroup_GlobalReplicationGroupMember {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::GlobalReplicationGroup.GlobalReplicationGroupMember"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_GlobalReplicationGroup_GlobalReplicationGroupMember as GlobalReplicationGroupMember;
    impl crate::value::ToValue for GlobalReplicationGroupMember_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.replication_group_id {
                properties.insert(
                    "ReplicationGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replication_group_region {
                properties.insert(
                    "ReplicationGroupRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role {
                properties.insert("Role".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-regionalconfiguration.html
    pub struct RegionalConfiguration_ {
        pub replication_group_id: Option<crate::value::ExpString>,
        pub replication_group_region: Option<crate::value::ExpString>,
        pub resharding_configurations: Option<Vec<ReshardingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_GlobalReplicationGroup_RegionalConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::GlobalReplicationGroup.RegionalConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_GlobalReplicationGroup_RegionalConfiguration as RegionalConfiguration;
    impl crate::value::ToValue for RegionalConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.replication_group_id {
                properties.insert(
                    "ReplicationGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replication_group_region {
                properties.insert(
                    "ReplicationGroupRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resharding_configurations {
                properties.insert(
                    "ReshardingConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-globalreplicationgroup-reshardingconfiguration.html
    pub struct ReshardingConfiguration_ {
        pub node_group_id: Option<crate::value::ExpString>,
        pub preferred_availability_zones: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_GlobalReplicationGroup_ReshardingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::GlobalReplicationGroup.ReshardingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_GlobalReplicationGroup_ReshardingConfiguration as ReshardingConfiguration;
    impl crate::value::ToValue for ReshardingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.node_group_id {
                properties.insert(
                    "NodeGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_availability_zones {
                properties.insert(
                    "PreferredAvailabilityZones".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod replicationgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-cloudwatchlogsdestinationdetails.html
    pub struct CloudWatchLogsDestinationDetails_ {
        pub log_group: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ReplicationGroup_CloudWatchLogsDestinationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ReplicationGroup.CloudWatchLogsDestinationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ReplicationGroup_CloudWatchLogsDestinationDetails as CloudWatchLogsDestinationDetails;
    impl crate::value::ToValue for CloudWatchLogsDestinationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LogGroup".to_string(),
                crate::value::ToValue::to_value(&self.log_group),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-destinationdetails.html
    pub struct DestinationDetails_ {
        pub cloud_watch_logs_details: Option<Box<CloudWatchLogsDestinationDetails_>>,
        pub kinesis_firehose_details: Option<Box<KinesisFirehoseDestinationDetails_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ReplicationGroup_DestinationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ReplicationGroup.DestinationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ReplicationGroup_DestinationDetails as DestinationDetails;
    impl crate::value::ToValue for DestinationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_details {
                properties.insert(
                    "CloudWatchLogsDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_firehose_details {
                properties.insert(
                    "KinesisFirehoseDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-kinesisfirehosedestinationdetails.html
    pub struct KinesisFirehoseDestinationDetails_ {
        pub delivery_stream: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ReplicationGroup_KinesisFirehoseDestinationDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ReplicationGroup.KinesisFirehoseDestinationDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ReplicationGroup_KinesisFirehoseDestinationDetails as KinesisFirehoseDestinationDetails;
    impl crate::value::ToValue for KinesisFirehoseDestinationDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeliveryStream".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-logdeliveryconfigurationrequest.html
    pub struct LogDeliveryConfigurationRequest_ {
        pub destination_details: Box<DestinationDetails_>,
        pub destination_type: crate::value::ExpString,
        pub log_format: crate::value::ExpString,
        pub log_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ReplicationGroup_LogDeliveryConfigurationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ReplicationGroup.LogDeliveryConfigurationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ReplicationGroup_LogDeliveryConfigurationRequest as LogDeliveryConfigurationRequest;
    impl crate::value::ToValue for LogDeliveryConfigurationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DestinationDetails".to_string(),
                crate::value::ToValue::to_value(&self.destination_details),
            );
            properties.insert(
                "DestinationType".to_string(),
                crate::value::ToValue::to_value(&self.destination_type),
            );
            properties.insert(
                "LogFormat".to_string(),
                crate::value::ToValue::to_value(&self.log_format),
            );
            properties.insert(
                "LogType".to_string(),
                crate::value::ToValue::to_value(&self.log_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-replicationgroup-nodegroupconfiguration.html
    pub struct NodeGroupConfiguration_ {
        pub node_group_id: Option<crate::value::ExpString>,
        pub primary_availability_zone: Option<crate::value::ExpString>,
        pub replica_availability_zones: Option<Vec<crate::value::ExpString>>,
        pub replica_count: Option<i32>,
        pub slots: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ReplicationGroup_NodeGroupConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ReplicationGroup.NodeGroupConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ReplicationGroup_NodeGroupConfiguration as NodeGroupConfiguration;
    impl crate::value::ToValue for NodeGroupConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.node_group_id {
                properties.insert(
                    "NodeGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.primary_availability_zone {
                properties.insert(
                    "PrimaryAvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replica_availability_zones {
                properties.insert(
                    "ReplicaAvailabilityZones".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replica_count {
                properties.insert(
                    "ReplicaCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slots {
                properties.insert("Slots".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod serverlesscache {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-cacheusagelimits.html
    pub struct CacheUsageLimits_ {
        pub data_storage: Option<Box<DataStorage_>>,
        pub ecpu_per_second: Option<Box<ECPUPerSecond_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ServerlessCache_CacheUsageLimits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ServerlessCache.CacheUsageLimits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ServerlessCache_CacheUsageLimits as CacheUsageLimits;
    impl crate::value::ToValue for CacheUsageLimits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_storage {
                properties.insert(
                    "DataStorage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecpu_per_second {
                properties.insert(
                    "ECPUPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-datastorage.html
    pub struct DataStorage_ {
        pub maximum: Option<i32>,
        pub minimum: Option<i32>,
        pub unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ServerlessCache_DataStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ServerlessCache.DataStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ServerlessCache_DataStorage as DataStorage;
    impl crate::value::ToValue for DataStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum {
                properties.insert(
                    "Maximum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum {
                properties.insert(
                    "Minimum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-ecpupersecond.html
    pub struct ECPUPerSecond_ {
        pub maximum: Option<i32>,
        pub minimum: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ServerlessCache_ECPUPerSecond {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ServerlessCache.ECPUPerSecond"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ServerlessCache_ECPUPerSecond as ECPUPerSecond;
    impl crate::value::ToValue for ECPUPerSecond_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum {
                properties.insert(
                    "Maximum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum {
                properties.insert(
                    "Minimum".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-serverlesscache-endpoint.html
    pub struct Endpoint_ {
        pub address: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_ServerlessCache_Endpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::ServerlessCache.Endpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_ServerlessCache_Endpoint as Endpoint;
    impl crate::value::ToValue for Endpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod user {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-user-authenticationmode.html
    pub struct AuthenticationMode_ {
        pub passwords: Option<Vec<crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_elasticache_User_AuthenticationMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ElastiCache::User.AuthenticationMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_elasticache_User_AuthenticationMode as AuthenticationMode;
    impl crate::value::ToValue for AuthenticationMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.passwords {
                properties.insert(
                    "Passwords".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-cache-cluster.html
pub struct CacheCluster_ {
    pub az_mode: Option<crate::value::ExpString>,
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub cache_node_type: crate::value::ExpString,
    pub cache_parameter_group_name: Option<crate::value::ExpString>,
    pub cache_security_group_names: Option<Vec<crate::value::ExpString>>,
    pub cache_subnet_group_name: Option<crate::value::ExpString>,
    pub cluster_name: Option<crate::value::ExpString>,
    pub engine: crate::value::ExpString,
    pub engine_version: Option<crate::value::ExpString>,
    pub ip_discovery: Option<crate::value::ExpString>,
    pub log_delivery_configurations:
        Option<Vec<super::elasticache::cachecluster::LogDeliveryConfigurationRequest_>>,
    pub network_type: Option<crate::value::ExpString>,
    pub notification_topic_arn: Option<crate::value::ExpString>,
    pub num_cache_nodes: i32,
    pub port: Option<i32>,
    pub preferred_availability_zone: Option<crate::value::ExpString>,
    pub preferred_availability_zones: Option<Vec<crate::value::ExpString>>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub snapshot_arns: Option<Vec<crate::value::ExpString>>,
    pub snapshot_name: Option<crate::value::ExpString>,
    pub snapshot_retention_limit: Option<i32>,
    pub snapshot_window: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_encryption_enabled: Option<crate::value::ExpBool>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_CacheCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::CacheCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_CacheCluster as CacheCluster;
impl crate::template::ToResource for CacheCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CacheCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.az_mode {
            properties.insert("AZMode".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "CacheNodeType".to_string(),
            crate::value::ToValue::to_value(&self.cache_node_type),
        );
        if let Some(ref value) = self.cache_parameter_group_name {
            properties.insert(
                "CacheParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_security_group_names {
            properties.insert(
                "CacheSecurityGroupNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_subnet_group_name {
            properties.insert(
                "CacheSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_name {
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Engine".to_string(),
            crate::value::ToValue::to_value(&self.engine),
        );
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_discovery {
            properties.insert(
                "IpDiscovery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_delivery_configurations {
            properties.insert(
                "LogDeliveryConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_topic_arn {
            properties.insert(
                "NotificationTopicArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NumCacheNodes".to_string(),
            crate::value::ToValue::to_value(&self.num_cache_nodes),
        );
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.preferred_availability_zone {
            properties.insert(
                "PreferredAvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_availability_zones {
            properties.insert(
                "PreferredAvailabilityZones".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_arns {
            properties.insert(
                "SnapshotArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_name {
            properties.insert(
                "SnapshotName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_retention_limit {
            properties.insert(
                "SnapshotRetentionLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_window {
            properties.insert(
                "SnapshotWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.transit_encryption_enabled {
            properties.insert(
                "TransitEncryptionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vpc_security_group_ids {
            properties.insert(
                "VpcSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-globalreplicationgroup.html
pub struct GlobalReplicationGroup_ {
    pub automatic_failover_enabled: Option<crate::value::ExpBool>,
    pub cache_node_type: Option<crate::value::ExpString>,
    pub cache_parameter_group_name: Option<crate::value::ExpString>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub global_node_group_count: Option<i32>,
    pub global_replication_group_description: Option<crate::value::ExpString>,
    pub global_replication_group_id_suffix: Option<crate::value::ExpString>,
    pub members: Vec<super::elasticache::globalreplicationgroup::GlobalReplicationGroupMember_>,
    pub regional_configurations:
        Option<Vec<super::elasticache::globalreplicationgroup::RegionalConfiguration_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_GlobalReplicationGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::GlobalReplicationGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_GlobalReplicationGroup as GlobalReplicationGroup;
impl crate::template::ToResource for GlobalReplicationGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GlobalReplicationGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.automatic_failover_enabled {
            properties.insert(
                "AutomaticFailoverEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_node_type {
            properties.insert(
                "CacheNodeType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_parameter_group_name {
            properties.insert(
                "CacheParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_node_group_count {
            properties.insert(
                "GlobalNodeGroupCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_replication_group_description {
            properties.insert(
                "GlobalReplicationGroupDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_replication_group_id_suffix {
            properties.insert(
                "GlobalReplicationGroupIdSuffix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Members".to_string(),
            crate::value::ToValue::to_value(&self.members),
        );
        if let Some(ref value) = self.regional_configurations {
            properties.insert(
                "RegionalConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-parametergroup.html
pub struct ParameterGroup_ {
    pub cache_parameter_group_family: crate::value::ExpString,
    pub description: crate::value::ExpString,
    pub properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_ParameterGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::ParameterGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_ParameterGroup as ParameterGroup;
impl crate::template::ToResource for ParameterGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ParameterGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CacheParameterGroupFamily".to_string(),
            crate::value::ToValue::to_value(&self.cache_parameter_group_family),
        );
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.properties {
            properties.insert(
                "Properties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-replicationgroup.html
pub struct ReplicationGroup_ {
    pub at_rest_encryption_enabled: Option<crate::value::ExpBool>,
    pub auth_token: Option<crate::value::ExpString>,
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub automatic_failover_enabled: Option<crate::value::ExpBool>,
    pub cache_node_type: Option<crate::value::ExpString>,
    pub cache_parameter_group_name: Option<crate::value::ExpString>,
    pub cache_security_group_names: Option<Vec<crate::value::ExpString>>,
    pub cache_subnet_group_name: Option<crate::value::ExpString>,
    pub cluster_mode: Option<crate::value::ExpString>,
    pub data_tiering_enabled: Option<crate::value::ExpBool>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub global_replication_group_id: Option<crate::value::ExpString>,
    pub ip_discovery: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub log_delivery_configurations:
        Option<Vec<super::elasticache::replicationgroup::LogDeliveryConfigurationRequest_>>,
    pub multi_az_enabled: Option<crate::value::ExpBool>,
    pub network_type: Option<crate::value::ExpString>,
    pub node_group_configuration:
        Option<Vec<super::elasticache::replicationgroup::NodeGroupConfiguration_>>,
    pub notification_topic_arn: Option<crate::value::ExpString>,
    pub num_cache_clusters: Option<i32>,
    pub num_node_groups: Option<i32>,
    pub port: Option<i32>,
    pub preferred_cache_cluster_a_zs: Option<Vec<crate::value::ExpString>>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub primary_cluster_id: Option<crate::value::ExpString>,
    pub replicas_per_node_group: Option<i32>,
    pub replication_group_description: crate::value::ExpString,
    pub replication_group_id: Option<crate::value::ExpString>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub snapshot_arns: Option<Vec<crate::value::ExpString>>,
    pub snapshot_name: Option<crate::value::ExpString>,
    pub snapshot_retention_limit: Option<i32>,
    pub snapshot_window: Option<crate::value::ExpString>,
    pub snapshotting_cluster_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub transit_encryption_enabled: Option<crate::value::ExpBool>,
    pub transit_encryption_mode: Option<crate::value::ExpString>,
    pub user_group_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_ReplicationGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::ReplicationGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_ReplicationGroup as ReplicationGroup;
impl crate::template::ToResource for ReplicationGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReplicationGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.at_rest_encryption_enabled {
            properties.insert(
                "AtRestEncryptionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auth_token {
            properties.insert(
                "AuthToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.automatic_failover_enabled {
            properties.insert(
                "AutomaticFailoverEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_node_type {
            properties.insert(
                "CacheNodeType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_parameter_group_name {
            properties.insert(
                "CacheParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_security_group_names {
            properties.insert(
                "CacheSecurityGroupNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_subnet_group_name {
            properties.insert(
                "CacheSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_mode {
            properties.insert(
                "ClusterMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_tiering_enabled {
            properties.insert(
                "DataTieringEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_replication_group_id {
            properties.insert(
                "GlobalReplicationGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_discovery {
            properties.insert(
                "IpDiscovery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.log_delivery_configurations {
            properties.insert(
                "LogDeliveryConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_az_enabled {
            properties.insert(
                "MultiAZEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.node_group_configuration {
            properties.insert(
                "NodeGroupConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_topic_arn {
            properties.insert(
                "NotificationTopicArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.num_cache_clusters {
            properties.insert(
                "NumCacheClusters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.num_node_groups {
            properties.insert(
                "NumNodeGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.preferred_cache_cluster_a_zs {
            properties.insert(
                "PreferredCacheClusterAZs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.primary_cluster_id {
            properties.insert(
                "PrimaryClusterId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replicas_per_node_group {
            properties.insert(
                "ReplicasPerNodeGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ReplicationGroupDescription".to_string(),
            crate::value::ToValue::to_value(&self.replication_group_description),
        );
        if let Some(ref value) = self.replication_group_id {
            properties.insert(
                "ReplicationGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_arns {
            properties.insert(
                "SnapshotArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_name {
            properties.insert(
                "SnapshotName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_retention_limit {
            properties.insert(
                "SnapshotRetentionLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_window {
            properties.insert(
                "SnapshotWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshotting_cluster_id {
            properties.insert(
                "SnapshottingClusterId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.transit_encryption_enabled {
            properties.insert(
                "TransitEncryptionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.transit_encryption_mode {
            properties.insert(
                "TransitEncryptionMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.user_group_ids {
            properties.insert(
                "UserGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group.html
pub struct SecurityGroup_ {
    pub description: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_SecurityGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::SecurityGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_SecurityGroup as SecurityGroup;
impl crate::template::ToResource for SecurityGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-elasticache-security-group-ingress.html
pub struct SecurityGroupIngress_ {
    pub cache_security_group_name: crate::value::ExpString,
    pub ec2_security_group_name: crate::value::ExpString,
    pub ec2_security_group_owner_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_SecurityGroupIngress {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::SecurityGroupIngress"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_SecurityGroupIngress as SecurityGroupIngress;
impl crate::template::ToResource for SecurityGroupIngress_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityGroupIngress"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CacheSecurityGroupName".to_string(),
            crate::value::ToValue::to_value(&self.cache_security_group_name),
        );
        properties.insert(
            "EC2SecurityGroupName".to_string(),
            crate::value::ToValue::to_value(&self.ec2_security_group_name),
        );
        if let Some(ref value) = self.ec2_security_group_owner_id {
            properties.insert(
                "EC2SecurityGroupOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-serverlesscache.html
pub struct ServerlessCache_ {
    pub cache_usage_limits: Option<super::elasticache::serverlesscache::CacheUsageLimits_>,
    pub daily_snapshot_time: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub endpoint: Option<super::elasticache::serverlesscache::Endpoint_>,
    pub engine: crate::value::ExpString,
    pub final_snapshot_name: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub major_engine_version: Option<crate::value::ExpString>,
    pub reader_endpoint: Option<super::elasticache::serverlesscache::Endpoint_>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub serverless_cache_name: crate::value::ExpString,
    pub snapshot_arns_to_restore: Option<Vec<crate::value::ExpString>>,
    pub snapshot_retention_limit: Option<i32>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_group_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_ServerlessCache {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::ServerlessCache"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_ServerlessCache as ServerlessCache;
impl crate::template::ToResource for ServerlessCache_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ServerlessCache"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cache_usage_limits {
            properties.insert(
                "CacheUsageLimits".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.daily_snapshot_time {
            properties.insert(
                "DailySnapshotTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint {
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Engine".to_string(),
            crate::value::ToValue::to_value(&self.engine),
        );
        if let Some(ref value) = self.final_snapshot_name {
            properties.insert(
                "FinalSnapshotName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.major_engine_version {
            properties.insert(
                "MajorEngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.reader_endpoint {
            properties.insert(
                "ReaderEndpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServerlessCacheName".to_string(),
            crate::value::ToValue::to_value(&self.serverless_cache_name),
        );
        if let Some(ref value) = self.snapshot_arns_to_restore {
            properties.insert(
                "SnapshotArnsToRestore".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_retention_limit {
            properties.insert(
                "SnapshotRetentionLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_group_id {
            properties.insert(
                "UserGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-subnetgroup.html
pub struct SubnetGroup_ {
    pub cache_subnet_group_name: Option<crate::value::ExpString>,
    pub description: crate::value::ExpString,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_SubnetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::SubnetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_SubnetGroup as SubnetGroup;
impl crate::template::ToResource for SubnetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubnetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cache_subnet_group_name {
            properties.insert(
                "CacheSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-user.html
pub struct User_ {
    pub access_string: Option<crate::value::ExpString>,
    pub authentication_mode: Option<super::elasticache::user::AuthenticationMode_>,
    pub engine: crate::value::ExpString,
    pub no_password_required: Option<crate::value::ExpBool>,
    pub passwords: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_id: crate::value::ExpString,
    pub user_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_User {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::User"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_User as User;
impl crate::template::ToResource for User_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("User"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_string {
            properties.insert(
                "AccessString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.authentication_mode {
            properties.insert(
                "AuthenticationMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Engine".to_string(),
            crate::value::ToValue::to_value(&self.engine),
        );
        if let Some(ref value) = self.no_password_required {
            properties.insert(
                "NoPasswordRequired".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.passwords {
            properties.insert(
                "Passwords".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserId".to_string(),
            crate::value::ToValue::to_value(&self.user_id),
        );
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-elasticache-usergroup.html
pub struct UserGroup_ {
    pub engine: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_group_id: crate::value::ExpString,
    pub user_ids: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_elasticache_UserGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ElastiCache::UserGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_elasticache_UserGroup as UserGroup;
impl crate::template::ToResource for UserGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ElastiCache"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Engine".to_string(),
            crate::value::ToValue::to_value(&self.engine),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserGroupId".to_string(),
            crate::value::ToValue::to_value(&self.user_group_id),
        );
        properties.insert(
            "UserIds".to_string(),
            crate::value::ToValue::to_value(&self.user_ids),
        );
        properties
    }
}
