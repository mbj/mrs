pub mod keyspace {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-keyspace-replicationspecification.html
    pub struct ReplicationSpecification_ {
        pub region_list: Option<Vec<crate::value::ExpString>>,
        pub replication_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Keyspace_ReplicationSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Keyspace.ReplicationSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Keyspace_ReplicationSpecification as ReplicationSpecification;
    impl crate::value::ToValue for ReplicationSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.region_list {
                properties.insert(
                    "RegionList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replication_strategy {
                properties.insert(
                    "ReplicationStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod table {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-autoscalingsetting.html
    pub struct AutoScalingSetting_ {
        pub auto_scaling_disabled: Option<crate::value::ExpBool>,
        pub maximum_units: Option<i64>,
        pub minimum_units: Option<i64>,
        pub scaling_policy: Option<Box<ScalingPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_AutoScalingSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.AutoScalingSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_AutoScalingSetting as AutoScalingSetting;
    impl crate::value::ToValue for AutoScalingSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_scaling_disabled {
                properties.insert(
                    "AutoScalingDisabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_units {
                properties.insert(
                    "MaximumUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_units {
                properties.insert(
                    "MinimumUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scaling_policy {
                properties.insert(
                    "ScalingPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-autoscalingspecification.html
    pub struct AutoScalingSpecification_ {
        pub read_capacity_auto_scaling: Option<Box<AutoScalingSetting_>>,
        pub write_capacity_auto_scaling: Option<Box<AutoScalingSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_AutoScalingSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.AutoScalingSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_AutoScalingSpecification as AutoScalingSpecification;
    impl crate::value::ToValue for AutoScalingSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.read_capacity_auto_scaling {
                properties.insert(
                    "ReadCapacityAutoScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_capacity_auto_scaling {
                properties.insert(
                    "WriteCapacityAutoScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-billingmode.html
    pub struct BillingMode_ {
        pub mode: crate::value::ExpString,
        pub provisioned_throughput: Option<Box<ProvisionedThroughput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_BillingMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.BillingMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_BillingMode as BillingMode;
    impl crate::value::ToValue for BillingMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            if let Some(ref value) = self.provisioned_throughput {
                properties.insert(
                    "ProvisionedThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-cdcspecification.html
    pub struct CdcSpecification_ {
        pub status: crate::value::ExpString,
        pub tags: Option<Vec<crate::Tag_>>,
        pub view_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_CdcSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.CdcSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_CdcSpecification as CdcSpecification;
    impl crate::value::ToValue for CdcSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.view_type {
                properties.insert(
                    "ViewType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-clusteringkeycolumn.html
    pub struct ClusteringKeyColumn_ {
        pub column: Box<Column_>,
        pub order_by: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_ClusteringKeyColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.ClusteringKeyColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_ClusteringKeyColumn as ClusteringKeyColumn;
    impl crate::value::ToValue for ClusteringKeyColumn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Column".to_string(),
                crate::value::ToValue::to_value(&self.column),
            );
            if let Some(ref value) = self.order_by {
                properties.insert(
                    "OrderBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-column.html
    pub struct Column_ {
        pub column_name: crate::value::ExpString,
        pub column_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_Column {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.Column"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_Column as Column;
    impl crate::value::ToValue for Column_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnName".to_string(),
                crate::value::ToValue::to_value(&self.column_name),
            );
            properties.insert(
                "ColumnType".to_string(),
                crate::value::ToValue::to_value(&self.column_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-encryptionspecification.html
    pub struct EncryptionSpecification_ {
        pub encryption_type: crate::value::ExpString,
        pub kms_key_identifier: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_EncryptionSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.EncryptionSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_EncryptionSpecification as EncryptionSpecification;
    impl crate::value::ToValue for EncryptionSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionType".to_string(),
                crate::value::ToValue::to_value(&self.encryption_type),
            );
            if let Some(ref value) = self.kms_key_identifier {
                properties.insert(
                    "KmsKeyIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-provisionedthroughput.html
    pub struct ProvisionedThroughput_ {
        pub read_capacity_units: i64,
        pub write_capacity_units: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_ProvisionedThroughput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.ProvisionedThroughput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_ProvisionedThroughput as ProvisionedThroughput;
    impl crate::value::ToValue for ProvisionedThroughput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ReadCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.read_capacity_units),
            );
            properties.insert(
                "WriteCapacityUnits".to_string(),
                crate::value::ToValue::to_value(&self.write_capacity_units),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-replicaspecification.html
    pub struct ReplicaSpecification_ {
        pub read_capacity_auto_scaling: Option<Box<AutoScalingSetting_>>,
        pub read_capacity_units: Option<i64>,
        pub region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_ReplicaSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.ReplicaSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_ReplicaSpecification as ReplicaSpecification;
    impl crate::value::ToValue for ReplicaSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.read_capacity_auto_scaling {
                properties.insert(
                    "ReadCapacityAutoScaling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_capacity_units {
                properties.insert(
                    "ReadCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-scalingpolicy.html
    pub struct ScalingPolicy_ {
        pub target_tracking_scaling_policy_configuration:
            Option<Box<TargetTrackingScalingPolicyConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_ScalingPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.ScalingPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_ScalingPolicy as ScalingPolicy;
    impl crate::value::ToValue for ScalingPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_tracking_scaling_policy_configuration {
                properties.insert(
                    "TargetTrackingScalingPolicyConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-table-targettrackingscalingpolicyconfiguration.html
    pub struct TargetTrackingScalingPolicyConfiguration_ {
        pub disable_scale_in: Option<crate::value::ExpBool>,
        pub scale_in_cooldown: Option<i64>,
        pub scale_out_cooldown: Option<i64>,
        pub target_value: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Table_TargetTrackingScalingPolicyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Table.TargetTrackingScalingPolicyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Table_TargetTrackingScalingPolicyConfiguration as TargetTrackingScalingPolicyConfiguration;
    impl crate::value::ToValue for TargetTrackingScalingPolicyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.disable_scale_in {
                properties.insert(
                    "DisableScaleIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scale_in_cooldown {
                properties.insert(
                    "ScaleInCooldown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scale_out_cooldown {
                properties.insert(
                    "ScaleOutCooldown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetValue".to_string(),
                crate::value::ToValue::to_value(&self.target_value),
            );
            properties.into()
        }
    }
}
pub mod r#type {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cassandra-type-field.html
    pub struct Field_ {
        pub field_name: crate::value::ExpString,
        pub field_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cassandra_Type_Field {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cassandra::Type.Field"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cassandra_Type_Field as Field;
    impl crate::value::ToValue for Field_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.insert(
                "FieldType".to_string(),
                crate::value::ToValue::to_value(&self.field_type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-keyspace.html
pub struct Keyspace_ {
    pub client_side_timestamps_enabled: Option<crate::value::ExpBool>,
    pub keyspace_name: Option<crate::value::ExpString>,
    pub replication_specification: Option<super::cassandra::keyspace::ReplicationSpecification_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cassandra_Keyspace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cassandra::Keyspace"
        $($field $value)*)
    };
}
pub use crate::__aws_cassandra_Keyspace as Keyspace;
impl crate::template::ToResource for Keyspace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cassandra"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Keyspace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.client_side_timestamps_enabled {
            properties.insert(
                "ClientSideTimestampsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.keyspace_name {
            properties.insert(
                "KeyspaceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_specification {
            properties.insert(
                "ReplicationSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-table.html
pub struct Table_ {
    pub auto_scaling_specifications: Option<super::cassandra::table::AutoScalingSpecification_>,
    pub billing_mode: Option<super::cassandra::table::BillingMode_>,
    pub cdc_specification: Option<super::cassandra::table::CdcSpecification_>,
    pub client_side_timestamps_enabled: Option<crate::value::ExpBool>,
    pub clustering_key_columns: Option<Vec<super::cassandra::table::ClusteringKeyColumn_>>,
    pub default_time_to_live: Option<i64>,
    pub encryption_specification: Option<super::cassandra::table::EncryptionSpecification_>,
    pub keyspace_name: crate::value::ExpString,
    pub partition_key_columns: Vec<super::cassandra::table::Column_>,
    pub point_in_time_recovery_enabled: Option<crate::value::ExpBool>,
    pub regular_columns: Option<Vec<super::cassandra::table::Column_>>,
    pub replica_specifications: Option<Vec<super::cassandra::table::ReplicaSpecification_>>,
    pub table_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cassandra_Table {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cassandra::Table"
        $($field $value)*)
    };
}
pub use crate::__aws_cassandra_Table as Table;
impl crate::template::ToResource for Table_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cassandra"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Table"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_scaling_specifications {
            properties.insert(
                "AutoScalingSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.billing_mode {
            properties.insert(
                "BillingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cdc_specification {
            properties.insert(
                "CdcSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_side_timestamps_enabled {
            properties.insert(
                "ClientSideTimestampsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.clustering_key_columns {
            properties.insert(
                "ClusteringKeyColumns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_time_to_live {
            properties.insert(
                "DefaultTimeToLive".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encryption_specification {
            properties.insert(
                "EncryptionSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KeyspaceName".to_string(),
            crate::value::ToValue::to_value(&self.keyspace_name),
        );
        properties.insert(
            "PartitionKeyColumns".to_string(),
            crate::value::ToValue::to_value(&self.partition_key_columns),
        );
        if let Some(ref value) = self.point_in_time_recovery_enabled {
            properties.insert(
                "PointInTimeRecoveryEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.regular_columns {
            properties.insert(
                "RegularColumns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replica_specifications {
            properties.insert(
                "ReplicaSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.table_name {
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cassandra-type.html
pub struct Type_ {
    pub fields: Vec<super::cassandra::r#type::Field_>,
    pub keyspace_name: crate::value::ExpString,
    pub type_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cassandra_Type {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cassandra::Type" $($field
        $value)*)
    };
}
pub use crate::__aws_cassandra_Type as Type;
impl crate::template::ToResource for Type_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cassandra"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Type"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Fields".to_string(),
            crate::value::ToValue::to_value(&self.fields),
        );
        properties.insert(
            "KeyspaceName".to_string(),
            crate::value::ToValue::to_value(&self.keyspace_name),
        );
        properties.insert(
            "TypeName".to_string(),
            crate::value::ToValue::to_value(&self.type_name),
        );
        properties
    }
}
