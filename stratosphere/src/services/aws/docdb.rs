pub mod dbcluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-docdb-dbcluster-serverlessv2scalingconfiguration.html
    pub struct ServerlessV2ScalingConfiguration_ {
        pub max_capacity: f64,
        pub min_capacity: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_docdb_DBCluster_ServerlessV2ScalingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DocDB::DBCluster.ServerlessV2ScalingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_docdb_DBCluster_ServerlessV2ScalingConfiguration as ServerlessV2ScalingConfiguration;
    impl crate::value::ToValue for ServerlessV2ScalingConfiguration_ {
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbcluster.html
pub struct DBCluster_ {
    pub availability_zones: Option<Vec<crate::value::ExpString>>,
    pub backup_retention_period: Option<i64>,
    pub copy_tags_to_snapshot: Option<crate::value::ExpBool>,
    pub db_cluster_identifier: Option<crate::value::ExpString>,
    pub db_cluster_parameter_group_name: Option<crate::value::ExpString>,
    pub db_subnet_group_name: Option<crate::value::ExpString>,
    pub deletion_protection: Option<crate::value::ExpBool>,
    pub enable_cloudwatch_logs_exports: Option<Vec<crate::value::ExpString>>,
    pub engine_version: Option<crate::value::ExpString>,
    pub global_cluster_identifier: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub manage_master_user_password: Option<crate::value::ExpBool>,
    pub master_user_password: Option<crate::value::ExpString>,
    pub master_user_secret_kms_key_id: Option<crate::value::ExpString>,
    pub master_username: Option<crate::value::ExpString>,
    pub network_type: Option<crate::value::ExpString>,
    pub port: Option<i64>,
    pub preferred_backup_window: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub restore_to_time: Option<crate::value::ExpString>,
    pub restore_type: Option<crate::value::ExpString>,
    pub rotate_master_user_password: Option<crate::value::ExpBool>,
    pub serverless_v2_scaling_configuration:
        Option<super::docdb::dbcluster::ServerlessV2ScalingConfiguration_>,
    pub snapshot_identifier: Option<crate::value::ExpString>,
    pub source_db_cluster_identifier: Option<crate::value::ExpString>,
    pub storage_encrypted: Option<crate::value::ExpBool>,
    pub storage_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub use_latest_restorable_time: Option<crate::value::ExpBool>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_docdb_DBCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DocDB::DBCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_docdb_DBCluster as DBCluster;
impl crate::template::ToResource for DBCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DocDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zones {
            properties.insert(
                "AvailabilityZones".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_retention_period {
            properties.insert(
                "BackupRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.copy_tags_to_snapshot {
            properties.insert(
                "CopyTagsToSnapshot".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_identifier {
            properties.insert(
                "DBClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_cluster_parameter_group_name {
            properties.insert(
                "DBClusterParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_subnet_group_name {
            properties.insert(
                "DBSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection {
            properties.insert(
                "DeletionProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_cloudwatch_logs_exports {
            properties.insert(
                "EnableCloudwatchLogsExports".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine_version {
            properties.insert(
                "EngineVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_cluster_identifier {
            properties.insert(
                "GlobalClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manage_master_user_password {
            properties.insert(
                "ManageMasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_password {
            properties.insert(
                "MasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_user_secret_kms_key_id {
            properties.insert(
                "MasterUserSecretKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_username {
            properties.insert(
                "MasterUsername".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.preferred_backup_window {
            properties.insert(
                "PreferredBackupWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.restore_to_time {
            properties.insert(
                "RestoreToTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.restore_type {
            properties.insert(
                "RestoreType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rotate_master_user_password {
            properties.insert(
                "RotateMasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.serverless_v2_scaling_configuration {
            properties.insert(
                "ServerlessV2ScalingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_identifier {
            properties.insert(
                "SnapshotIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_db_cluster_identifier {
            properties.insert(
                "SourceDBClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_encrypted {
            properties.insert(
                "StorageEncrypted".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_type {
            properties.insert(
                "StorageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.use_latest_restorable_time {
            properties.insert(
                "UseLatestRestorableTime".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbclusterparametergroup.html
pub struct DBClusterParameterGroup_ {
    pub description: crate::value::ExpString,
    pub family: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub parameters: serde_json::Value,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_docdb_DBClusterParameterGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DocDB::DBClusterParameterGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_docdb_DBClusterParameterGroup as DBClusterParameterGroup;
impl crate::template::ToResource for DBClusterParameterGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DocDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBClusterParameterGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "Family".to_string(),
            crate::value::ToValue::to_value(&self.family),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Parameters".to_string(),
            crate::value::ToValue::to_value(&self.parameters),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbinstance.html
pub struct DBInstance_ {
    pub auto_minor_version_upgrade: Option<crate::value::ExpBool>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub ca_certificate_identifier: Option<crate::value::ExpString>,
    pub certificate_rotation_restart: Option<crate::value::ExpBool>,
    pub db_cluster_identifier: crate::value::ExpString,
    pub db_instance_class: crate::value::ExpString,
    pub db_instance_identifier: Option<crate::value::ExpString>,
    pub enable_performance_insights: Option<crate::value::ExpBool>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_docdb_DBInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DocDB::DBInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_docdb_DBInstance as DBInstance;
impl crate::template::ToResource for DBInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DocDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_minor_version_upgrade {
            properties.insert(
                "AutoMinorVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ca_certificate_identifier {
            properties.insert(
                "CACertificateIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_rotation_restart {
            properties.insert(
                "CertificateRotationRestart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DBClusterIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.db_cluster_identifier),
        );
        properties.insert(
            "DBInstanceClass".to_string(),
            crate::value::ToValue::to_value(&self.db_instance_class),
        );
        if let Some(ref value) = self.db_instance_identifier {
            properties.insert(
                "DBInstanceIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_performance_insights {
            properties.insert(
                "EnablePerformanceInsights".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-dbsubnetgroup.html
pub struct DBSubnetGroup_ {
    pub db_subnet_group_description: crate::value::ExpString,
    pub db_subnet_group_name: Option<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_docdb_DBSubnetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DocDB::DBSubnetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_docdb_DBSubnetGroup as DBSubnetGroup;
impl crate::template::ToResource for DBSubnetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DocDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DBSubnetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DBSubnetGroupDescription".to_string(),
            crate::value::ToValue::to_value(&self.db_subnet_group_description),
        );
        if let Some(ref value) = self.db_subnet_group_name {
            properties.insert(
                "DBSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-docdb-eventsubscription.html
pub struct EventSubscription_ {
    pub enabled: Option<crate::value::ExpBool>,
    pub event_categories: Option<Vec<crate::value::ExpString>>,
    pub sns_topic_arn: crate::value::ExpString,
    pub source_ids: Option<Vec<crate::value::ExpString>>,
    pub source_type: Option<crate::value::ExpString>,
    pub subscription_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_docdb_EventSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DocDB::EventSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_docdb_EventSubscription as EventSubscription;
impl crate::template::ToResource for EventSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DocDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventSubscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_categories {
            properties.insert(
                "EventCategories".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SnsTopicArn".to_string(),
            crate::value::ToValue::to_value(&self.sns_topic_arn),
        );
        if let Some(ref value) = self.source_ids {
            properties.insert(
                "SourceIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_type {
            properties.insert(
                "SourceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subscription_name {
            properties.insert(
                "SubscriptionName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
