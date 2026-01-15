pub mod cluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-memorydb-cluster-endpoint.html
    pub struct Endpoint_ {
        pub address: Option<crate::value::ExpString>,
        pub port: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_memorydb_Cluster_Endpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MemoryDB::Cluster.Endpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_memorydb_Cluster_Endpoint as Endpoint;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-memorydb-user-authenticationmode.html
    pub struct AuthenticationMode_ {
        pub passwords: Option<Vec<crate::value::ExpString>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_memorydb_User_AuthenticationMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::MemoryDB::User.AuthenticationMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_memorydb_User_AuthenticationMode as AuthenticationMode;
    impl crate::value::ToValue for AuthenticationMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.passwords {
                properties.insert(
                    "Passwords".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-memorydb-acl.html
pub struct ACL_ {
    pub acl_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_names: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_memorydb_ACL {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MemoryDB::ACL" $($field
        $value)*)
    };
}
pub use crate::__aws_memorydb_ACL as ACL;
impl crate::template::ToResource for ACL_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MemoryDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ACL"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ACLName".to_string(),
            crate::value::ToValue::to_value(&self.acl_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_names {
            properties.insert(
                "UserNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-memorydb-cluster.html
pub struct Cluster_ {
    pub acl_name: crate::value::ExpString,
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub cluster_endpoint: Option<super::memorydb::cluster::Endpoint_>,
    pub cluster_name: crate::value::ExpString,
    pub data_tiering: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub final_snapshot_name: Option<crate::value::ExpString>,
    pub ip_discovery: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub maintenance_window: Option<crate::value::ExpString>,
    pub multi_region_cluster_name: Option<crate::value::ExpString>,
    pub network_type: Option<crate::value::ExpString>,
    pub node_type: crate::value::ExpString,
    pub num_replicas_per_shard: Option<i64>,
    pub num_shards: Option<i64>,
    pub parameter_group_name: Option<crate::value::ExpString>,
    pub port: Option<i64>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub snapshot_arns: Option<Vec<crate::value::ExpString>>,
    pub snapshot_name: Option<crate::value::ExpString>,
    pub snapshot_retention_limit: Option<i64>,
    pub snapshot_window: Option<crate::value::ExpString>,
    pub sns_topic_arn: Option<crate::value::ExpString>,
    pub sns_topic_status: Option<crate::value::ExpString>,
    pub subnet_group_name: Option<crate::value::ExpString>,
    pub tls_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_memorydb_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MemoryDB::Cluster"
        $($field $value)*)
    };
}
pub use crate::__aws_memorydb_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MemoryDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ACLName".to_string(),
            crate::value::ToValue::to_value(&self.acl_name),
        );
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_endpoint {
            properties.insert(
                "ClusterEndpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClusterName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_name),
        );
        if let Some(ref value) = self.data_tiering {
            properties.insert(
                "DataTiering".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
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
        if let Some(ref value) = self.final_snapshot_name {
            properties.insert(
                "FinalSnapshotName".to_string(),
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
        if let Some(ref value) = self.maintenance_window {
            properties.insert(
                "MaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_region_cluster_name {
            properties.insert(
                "MultiRegionClusterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NodeType".to_string(),
            crate::value::ToValue::to_value(&self.node_type),
        );
        if let Some(ref value) = self.num_replicas_per_shard {
            properties.insert(
                "NumReplicasPerShard".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.num_shards {
            properties.insert(
                "NumShards".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameter_group_name {
            properties.insert(
                "ParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
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
        if let Some(ref value) = self.sns_topic_arn {
            properties.insert(
                "SnsTopicArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sns_topic_status {
            properties.insert(
                "SnsTopicStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_group_name {
            properties.insert(
                "SubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tls_enabled {
            properties.insert(
                "TLSEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-memorydb-multiregioncluster.html
pub struct MultiRegionCluster_ {
    pub description: Option<crate::value::ExpString>,
    pub engine: Option<crate::value::ExpString>,
    pub engine_version: Option<crate::value::ExpString>,
    pub multi_region_cluster_name_suffix: Option<crate::value::ExpString>,
    pub multi_region_parameter_group_name: Option<crate::value::ExpString>,
    pub node_type: crate::value::ExpString,
    pub num_shards: Option<i64>,
    pub tls_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub update_strategy: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_memorydb_MultiRegionCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MemoryDB::MultiRegionCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_memorydb_MultiRegionCluster as MultiRegionCluster;
impl crate::template::ToResource for MultiRegionCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MemoryDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MultiRegionCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
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
        if let Some(ref value) = self.multi_region_cluster_name_suffix {
            properties.insert(
                "MultiRegionClusterNameSuffix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_region_parameter_group_name {
            properties.insert(
                "MultiRegionParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NodeType".to_string(),
            crate::value::ToValue::to_value(&self.node_type),
        );
        if let Some(ref value) = self.num_shards {
            properties.insert(
                "NumShards".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tls_enabled {
            properties.insert(
                "TLSEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.update_strategy {
            properties.insert(
                "UpdateStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-memorydb-parametergroup.html
pub struct ParameterGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub family: crate::value::ExpString,
    pub parameter_group_name: crate::value::ExpString,
    pub parameters: Option<serde_json::Value>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_memorydb_ParameterGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MemoryDB::ParameterGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_memorydb_ParameterGroup as ParameterGroup;
impl crate::template::ToResource for ParameterGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MemoryDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ParameterGroup"),
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
            "Family".to_string(),
            crate::value::ToValue::to_value(&self.family),
        );
        properties.insert(
            "ParameterGroupName".to_string(),
            crate::value::ToValue::to_value(&self.parameter_group_name),
        );
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-memorydb-subnetgroup.html
pub struct SubnetGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub subnet_group_name: crate::value::ExpString,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_memorydb_SubnetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MemoryDB::SubnetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_memorydb_SubnetGroup as SubnetGroup;
impl crate::template::ToResource for SubnetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MemoryDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SubnetGroup"),
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
            "SubnetGroupName".to_string(),
            crate::value::ToValue::to_value(&self.subnet_group_name),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-memorydb-user.html
pub struct User_ {
    pub access_string: Option<crate::value::ExpString>,
    pub authentication_mode: Option<super::memorydb::user::AuthenticationMode_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_memorydb_User {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::MemoryDB::User" $($field
        $value)*)
    };
}
pub use crate::__aws_memorydb_User as User;
impl crate::template::ToResource for User_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MemoryDB"),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "UserName".to_string(),
            crate::value::ToValue::to_value(&self.user_name),
        );
        properties
    }
}
