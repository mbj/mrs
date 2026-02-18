pub mod globaltable {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-attributedefinition.html>
    pub struct AttributeDefinition_ {
        pub attribute_name: crate::value::ExpString,
        pub attribute_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_AttributeDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.AttributeDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_AttributeDefinition as AttributeDefinition;
    impl crate::value::ToValue for AttributeDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            properties.insert(
                "AttributeType".to_string(),
                crate::value::ToValue::to_value(&self.attribute_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-capacityautoscalingsettings.html>
    pub struct CapacityAutoScalingSettings_ {
        pub max_capacity: i32,
        pub min_capacity: i32,
        pub seed_capacity: Option<i32>,
        pub target_tracking_scaling_policy_configuration:
            Box<TargetTrackingScalingPolicyConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_CapacityAutoScalingSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.CapacityAutoScalingSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_CapacityAutoScalingSettings as CapacityAutoScalingSettings;
    impl crate::value::ToValue for CapacityAutoScalingSettings_ {
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
            if let Some(ref value) = self.seed_capacity {
                properties.insert(
                    "SeedCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetTrackingScalingPolicyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.target_tracking_scaling_policy_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-contributorinsightsspecification.html>
    pub struct ContributorInsightsSpecification_ {
        pub enabled: crate::value::ExpBool,
        pub mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ContributorInsightsSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ContributorInsightsSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ContributorInsightsSpecification as ContributorInsightsSpecification;
    impl crate::value::ToValue for ContributorInsightsSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globalreadprovisionedthroughputsettings.html>
    pub struct GlobalReadProvisionedThroughputSettings_ {
        pub read_capacity_units: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_GlobalReadProvisionedThroughputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.GlobalReadProvisionedThroughputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_GlobalReadProvisionedThroughputSettings as GlobalReadProvisionedThroughputSettings;
    impl crate::value::ToValue for GlobalReadProvisionedThroughputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.read_capacity_units {
                properties.insert(
                    "ReadCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globalsecondaryindex.html>
    pub struct GlobalSecondaryIndex_ {
        pub index_name: crate::value::ExpString,
        pub key_schema: Vec<KeySchema_>,
        pub projection: Box<Projection_>,
        pub read_on_demand_throughput_settings: Option<Box<ReadOnDemandThroughputSettings_>>,
        pub read_provisioned_throughput_settings:
            Option<Box<GlobalReadProvisionedThroughputSettings_>>,
        pub warm_throughput: Option<Box<WarmThroughput_>>,
        pub write_on_demand_throughput_settings: Option<Box<WriteOnDemandThroughputSettings_>>,
        pub write_provisioned_throughput_settings: Option<Box<WriteProvisionedThroughputSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_GlobalSecondaryIndex {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.GlobalSecondaryIndex"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_GlobalSecondaryIndex as GlobalSecondaryIndex;
    impl crate::value::ToValue for GlobalSecondaryIndex_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            properties.insert(
                "KeySchema".to_string(),
                crate::value::ToValue::to_value(&self.key_schema),
            );
            properties.insert(
                "Projection".to_string(),
                crate::value::ToValue::to_value(&self.projection),
            );
            if let Some(ref value) = self.read_on_demand_throughput_settings {
                properties.insert(
                    "ReadOnDemandThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_provisioned_throughput_settings {
                properties.insert(
                    "ReadProvisionedThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.warm_throughput {
                properties.insert(
                    "WarmThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_on_demand_throughput_settings {
                properties.insert(
                    "WriteOnDemandThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_provisioned_throughput_settings {
                properties.insert(
                    "WriteProvisionedThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-globaltablewitness.html>
    pub struct GlobalTableWitness_ {
        pub region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_GlobalTableWitness {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.GlobalTableWitness"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_GlobalTableWitness as GlobalTableWitness;
    impl crate::value::ToValue for GlobalTableWitness_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-keyschema.html>
    pub struct KeySchema_ {
        pub attribute_name: crate::value::ExpString,
        pub key_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_KeySchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.KeySchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_KeySchema as KeySchema;
    impl crate::value::ToValue for KeySchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            properties.insert(
                "KeyType".to_string(),
                crate::value::ToValue::to_value(&self.key_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-kinesisstreamspecification.html>
    pub struct KinesisStreamSpecification_ {
        pub approximate_creation_date_time_precision: Option<crate::value::ExpString>,
        pub stream_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_KinesisStreamSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.KinesisStreamSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_KinesisStreamSpecification as KinesisStreamSpecification;
    impl crate::value::ToValue for KinesisStreamSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approximate_creation_date_time_precision {
                properties.insert(
                    "ApproximateCreationDateTimePrecision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StreamArn".to_string(),
                crate::value::ToValue::to_value(&self.stream_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-localsecondaryindex.html>
    pub struct LocalSecondaryIndex_ {
        pub index_name: crate::value::ExpString,
        pub key_schema: Vec<KeySchema_>,
        pub projection: Box<Projection_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_LocalSecondaryIndex {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.LocalSecondaryIndex"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_LocalSecondaryIndex as LocalSecondaryIndex;
    impl crate::value::ToValue for LocalSecondaryIndex_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            properties.insert(
                "KeySchema".to_string(),
                crate::value::ToValue::to_value(&self.key_schema),
            );
            properties.insert(
                "Projection".to_string(),
                crate::value::ToValue::to_value(&self.projection),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-pointintimerecoveryspecification.html>
    pub struct PointInTimeRecoverySpecification_ {
        pub point_in_time_recovery_enabled: Option<crate::value::ExpBool>,
        pub recovery_period_in_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_PointInTimeRecoverySpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.PointInTimeRecoverySpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_PointInTimeRecoverySpecification as PointInTimeRecoverySpecification;
    impl crate::value::ToValue for PointInTimeRecoverySpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.point_in_time_recovery_enabled {
                properties.insert(
                    "PointInTimeRecoveryEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recovery_period_in_days {
                properties.insert(
                    "RecoveryPeriodInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-projection.html>
    pub struct Projection_ {
        pub non_key_attributes: Option<Vec<crate::value::ExpString>>,
        pub projection_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_Projection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.Projection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_Projection as Projection;
    impl crate::value::ToValue for Projection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.non_key_attributes {
                properties.insert(
                    "NonKeyAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.projection_type {
                properties.insert(
                    "ProjectionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-readondemandthroughputsettings.html>
    pub struct ReadOnDemandThroughputSettings_ {
        pub max_read_request_units: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ReadOnDemandThroughputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ReadOnDemandThroughputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ReadOnDemandThroughputSettings as ReadOnDemandThroughputSettings;
    impl crate::value::ToValue for ReadOnDemandThroughputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_read_request_units {
                properties.insert(
                    "MaxReadRequestUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-readprovisionedthroughputsettings.html>
    pub struct ReadProvisionedThroughputSettings_ {
        pub read_capacity_auto_scaling_settings: Option<Box<CapacityAutoScalingSettings_>>,
        pub read_capacity_units: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ReadProvisionedThroughputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ReadProvisionedThroughputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ReadProvisionedThroughputSettings as ReadProvisionedThroughputSettings;
    impl crate::value::ToValue for ReadProvisionedThroughputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.read_capacity_auto_scaling_settings {
                properties.insert(
                    "ReadCapacityAutoScalingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_capacity_units {
                properties.insert(
                    "ReadCapacityUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaglobalsecondaryindexspecification.html>
    pub struct ReplicaGlobalSecondaryIndexSpecification_ {
        pub contributor_insights_specification: Option<Box<ContributorInsightsSpecification_>>,
        pub index_name: crate::value::ExpString,
        pub read_on_demand_throughput_settings: Option<Box<ReadOnDemandThroughputSettings_>>,
        pub read_provisioned_throughput_settings: Option<Box<ReadProvisionedThroughputSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ReplicaGlobalSecondaryIndexSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ReplicaGlobalSecondaryIndexSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ReplicaGlobalSecondaryIndexSpecification as ReplicaGlobalSecondaryIndexSpecification;
    impl crate::value::ToValue for ReplicaGlobalSecondaryIndexSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.contributor_insights_specification {
                properties.insert(
                    "ContributorInsightsSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            if let Some(ref value) = self.read_on_demand_throughput_settings {
                properties.insert(
                    "ReadOnDemandThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_provisioned_throughput_settings {
                properties.insert(
                    "ReadProvisionedThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicassespecification.html>
    pub struct ReplicaSSESpecification_ {
        pub kms_master_key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ReplicaSSESpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ReplicaSSESpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ReplicaSSESpecification as ReplicaSSESpecification;
    impl crate::value::ToValue for ReplicaSSESpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KMSMasterKeyId".to_string(),
                crate::value::ToValue::to_value(&self.kms_master_key_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicaspecification.html>
    pub struct ReplicaSpecification_ {
        pub contributor_insights_specification: Option<Box<ContributorInsightsSpecification_>>,
        pub deletion_protection_enabled: Option<crate::value::ExpBool>,
        pub global_secondary_indexes: Option<Vec<ReplicaGlobalSecondaryIndexSpecification_>>,
        pub global_table_settings_replication_mode: Option<crate::value::ExpString>,
        pub kinesis_stream_specification: Option<Box<KinesisStreamSpecification_>>,
        pub point_in_time_recovery_specification: Option<Box<PointInTimeRecoverySpecification_>>,
        pub read_on_demand_throughput_settings: Option<Box<ReadOnDemandThroughputSettings_>>,
        pub read_provisioned_throughput_settings: Option<Box<ReadProvisionedThroughputSettings_>>,
        pub region: crate::value::ExpString,
        pub replica_stream_specification: Option<Box<ReplicaStreamSpecification_>>,
        pub resource_policy: Option<Box<ResourcePolicy_>>,
        pub sse_specification: Option<Box<ReplicaSSESpecification_>>,
        pub table_class: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ReplicaSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ReplicaSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ReplicaSpecification as ReplicaSpecification;
    impl crate::value::ToValue for ReplicaSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.contributor_insights_specification {
                properties.insert(
                    "ContributorInsightsSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deletion_protection_enabled {
                properties.insert(
                    "DeletionProtectionEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.global_secondary_indexes {
                properties.insert(
                    "GlobalSecondaryIndexes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.global_table_settings_replication_mode {
                properties.insert(
                    "GlobalTableSettingsReplicationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kinesis_stream_specification {
                properties.insert(
                    "KinesisStreamSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.point_in_time_recovery_specification {
                properties.insert(
                    "PointInTimeRecoverySpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_on_demand_throughput_settings {
                properties.insert(
                    "ReadOnDemandThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_provisioned_throughput_settings {
                properties.insert(
                    "ReadProvisionedThroughputSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            if let Some(ref value) = self.replica_stream_specification {
                properties.insert(
                    "ReplicaStreamSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_policy {
                properties.insert(
                    "ResourcePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sse_specification {
                properties.insert(
                    "SSESpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_class {
                properties.insert(
                    "TableClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-replicastreamspecification.html>
    pub struct ReplicaStreamSpecification_ {
        pub resource_policy: Box<ResourcePolicy_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ReplicaStreamSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ReplicaStreamSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ReplicaStreamSpecification as ReplicaStreamSpecification;
    impl crate::value::ToValue for ReplicaStreamSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResourcePolicy".to_string(),
                crate::value::ToValue::to_value(&self.resource_policy),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-resourcepolicy.html>
    pub struct ResourcePolicy_ {
        pub policy_document: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_ResourcePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.ResourcePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_ResourcePolicy as ResourcePolicy;
    impl crate::value::ToValue for ResourcePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(&self.policy_document),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-ssespecification.html>
    pub struct SSESpecification_ {
        pub sse_enabled: crate::value::ExpBool,
        pub sse_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_SSESpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.SSESpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_SSESpecification as SSESpecification;
    impl crate::value::ToValue for SSESpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SSEEnabled".to_string(),
                crate::value::ToValue::to_value(&self.sse_enabled),
            );
            if let Some(ref value) = self.sse_type {
                properties.insert(
                    "SSEType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-streamspecification.html>
    pub struct StreamSpecification_ {
        pub stream_view_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_StreamSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.StreamSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_StreamSpecification as StreamSpecification;
    impl crate::value::ToValue for StreamSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StreamViewType".to_string(),
                crate::value::ToValue::to_value(&self.stream_view_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-targettrackingscalingpolicyconfiguration.html>
    pub struct TargetTrackingScalingPolicyConfiguration_ {
        pub disable_scale_in: Option<crate::value::ExpBool>,
        pub scale_in_cooldown: Option<i32>,
        pub scale_out_cooldown: Option<i32>,
        pub target_value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_TargetTrackingScalingPolicyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.TargetTrackingScalingPolicyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_TargetTrackingScalingPolicyConfiguration as TargetTrackingScalingPolicyConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-timetolivespecification.html>
    pub struct TimeToLiveSpecification_ {
        pub attribute_name: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_TimeToLiveSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.TimeToLiveSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_TimeToLiveSpecification as TimeToLiveSpecification;
    impl crate::value::ToValue for TimeToLiveSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_name {
                properties.insert(
                    "AttributeName".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-warmthroughput.html>
    pub struct WarmThroughput_ {
        pub read_units_per_second: Option<i32>,
        pub write_units_per_second: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_WarmThroughput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.WarmThroughput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_WarmThroughput as WarmThroughput;
    impl crate::value::ToValue for WarmThroughput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.read_units_per_second {
                properties.insert(
                    "ReadUnitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_units_per_second {
                properties.insert(
                    "WriteUnitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-writeondemandthroughputsettings.html>
    pub struct WriteOnDemandThroughputSettings_ {
        pub max_write_request_units: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_WriteOnDemandThroughputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.WriteOnDemandThroughputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_WriteOnDemandThroughputSettings as WriteOnDemandThroughputSettings;
    impl crate::value::ToValue for WriteOnDemandThroughputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_write_request_units {
                properties.insert(
                    "MaxWriteRequestUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-globaltable-writeprovisionedthroughputsettings.html>
    pub struct WriteProvisionedThroughputSettings_ {
        pub write_capacity_auto_scaling_settings: Option<Box<CapacityAutoScalingSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_GlobalTable_WriteProvisionedThroughputSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::GlobalTable.WriteProvisionedThroughputSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_GlobalTable_WriteProvisionedThroughputSettings as WriteProvisionedThroughputSettings;
    impl crate::value::ToValue for WriteProvisionedThroughputSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.write_capacity_auto_scaling_settings {
                properties.insert(
                    "WriteCapacityAutoScalingSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod table {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-attributedefinition.html>
    pub struct AttributeDefinition_ {
        pub attribute_name: crate::value::ExpString,
        pub attribute_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_AttributeDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.AttributeDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_AttributeDefinition as AttributeDefinition;
    impl crate::value::ToValue for AttributeDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            properties.insert(
                "AttributeType".to_string(),
                crate::value::ToValue::to_value(&self.attribute_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-contributorinsightsspecification.html>
    pub struct ContributorInsightsSpecification_ {
        pub enabled: crate::value::ExpBool,
        pub mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_ContributorInsightsSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.ContributorInsightsSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_ContributorInsightsSpecification as ContributorInsightsSpecification;
    impl crate::value::ToValue for ContributorInsightsSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-csv.html>
    pub struct Csv_ {
        pub delimiter: Option<crate::value::ExpString>,
        pub header_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_Csv {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.Csv"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_Csv as Csv;
    impl crate::value::ToValue for Csv_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_list {
                properties.insert(
                    "HeaderList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-globalsecondaryindex.html>
    pub struct GlobalSecondaryIndex_ {
        pub contributor_insights_specification: Option<Box<ContributorInsightsSpecification_>>,
        pub index_name: crate::value::ExpString,
        pub key_schema: Vec<KeySchema_>,
        pub on_demand_throughput: Option<Box<OnDemandThroughput_>>,
        pub projection: Box<Projection_>,
        pub provisioned_throughput: Option<Box<ProvisionedThroughput_>>,
        pub warm_throughput: Option<Box<WarmThroughput_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_GlobalSecondaryIndex {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.GlobalSecondaryIndex"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_GlobalSecondaryIndex as GlobalSecondaryIndex;
    impl crate::value::ToValue for GlobalSecondaryIndex_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.contributor_insights_specification {
                properties.insert(
                    "ContributorInsightsSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            properties.insert(
                "KeySchema".to_string(),
                crate::value::ToValue::to_value(&self.key_schema),
            );
            if let Some(ref value) = self.on_demand_throughput {
                properties.insert(
                    "OnDemandThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Projection".to_string(),
                crate::value::ToValue::to_value(&self.projection),
            );
            if let Some(ref value) = self.provisioned_throughput {
                properties.insert(
                    "ProvisionedThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.warm_throughput {
                properties.insert(
                    "WarmThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-importsourcespecification.html>
    pub struct ImportSourceSpecification_ {
        pub input_compression_type: Option<crate::value::ExpString>,
        pub input_format: crate::value::ExpString,
        pub input_format_options: Option<Box<InputFormatOptions_>>,
        pub s3_bucket_source: Box<S3BucketSource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_ImportSourceSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.ImportSourceSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_ImportSourceSpecification as ImportSourceSpecification;
    impl crate::value::ToValue for ImportSourceSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_compression_type {
                properties.insert(
                    "InputCompressionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputFormat".to_string(),
                crate::value::ToValue::to_value(&self.input_format),
            );
            if let Some(ref value) = self.input_format_options {
                properties.insert(
                    "InputFormatOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3BucketSource".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket_source),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-inputformatoptions.html>
    pub struct InputFormatOptions_ {
        pub csv: Option<Box<Csv_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_InputFormatOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.InputFormatOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_InputFormatOptions as InputFormatOptions;
    impl crate::value::ToValue for InputFormatOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.csv {
                properties.insert("Csv".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-keyschema.html>
    pub struct KeySchema_ {
        pub attribute_name: crate::value::ExpString,
        pub key_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_KeySchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.KeySchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_KeySchema as KeySchema;
    impl crate::value::ToValue for KeySchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            properties.insert(
                "KeyType".to_string(),
                crate::value::ToValue::to_value(&self.key_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-kinesisstreamspecification.html>
    pub struct KinesisStreamSpecification_ {
        pub approximate_creation_date_time_precision: Option<crate::value::ExpString>,
        pub stream_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_KinesisStreamSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.KinesisStreamSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_KinesisStreamSpecification as KinesisStreamSpecification;
    impl crate::value::ToValue for KinesisStreamSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.approximate_creation_date_time_precision {
                properties.insert(
                    "ApproximateCreationDateTimePrecision".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StreamArn".to_string(),
                crate::value::ToValue::to_value(&self.stream_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-localsecondaryindex.html>
    pub struct LocalSecondaryIndex_ {
        pub index_name: crate::value::ExpString,
        pub key_schema: Vec<KeySchema_>,
        pub projection: Box<Projection_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_LocalSecondaryIndex {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.LocalSecondaryIndex"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_LocalSecondaryIndex as LocalSecondaryIndex;
    impl crate::value::ToValue for LocalSecondaryIndex_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(&self.index_name),
            );
            properties.insert(
                "KeySchema".to_string(),
                crate::value::ToValue::to_value(&self.key_schema),
            );
            properties.insert(
                "Projection".to_string(),
                crate::value::ToValue::to_value(&self.projection),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-ondemandthroughput.html>
    pub struct OnDemandThroughput_ {
        pub max_read_request_units: Option<i32>,
        pub max_write_request_units: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_OnDemandThroughput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.OnDemandThroughput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_OnDemandThroughput as OnDemandThroughput;
    impl crate::value::ToValue for OnDemandThroughput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_read_request_units {
                properties.insert(
                    "MaxReadRequestUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_write_request_units {
                properties.insert(
                    "MaxWriteRequestUnits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-pointintimerecoveryspecification.html>
    pub struct PointInTimeRecoverySpecification_ {
        pub point_in_time_recovery_enabled: Option<crate::value::ExpBool>,
        pub recovery_period_in_days: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_PointInTimeRecoverySpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.PointInTimeRecoverySpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_PointInTimeRecoverySpecification as PointInTimeRecoverySpecification;
    impl crate::value::ToValue for PointInTimeRecoverySpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.point_in_time_recovery_enabled {
                properties.insert(
                    "PointInTimeRecoveryEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.recovery_period_in_days {
                properties.insert(
                    "RecoveryPeriodInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-projection.html>
    pub struct Projection_ {
        pub non_key_attributes: Option<Vec<crate::value::ExpString>>,
        pub projection_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_Projection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.Projection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_Projection as Projection;
    impl crate::value::ToValue for Projection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.non_key_attributes {
                properties.insert(
                    "NonKeyAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.projection_type {
                properties.insert(
                    "ProjectionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-provisionedthroughput.html>
    pub struct ProvisionedThroughput_ {
        pub read_capacity_units: i32,
        pub write_capacity_units: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_ProvisionedThroughput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.ProvisionedThroughput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_ProvisionedThroughput as ProvisionedThroughput;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-resourcepolicy.html>
    pub struct ResourcePolicy_ {
        pub policy_document: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_ResourcePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.ResourcePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_ResourcePolicy as ResourcePolicy;
    impl crate::value::ToValue for ResourcePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(&self.policy_document),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-s3bucketsource.html>
    pub struct S3BucketSource_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_bucket_owner: Option<crate::value::ExpString>,
        pub s3_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_S3BucketSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.S3BucketSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_S3BucketSource as S3BucketSource;
    impl crate::value::ToValue for S3BucketSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            if let Some(ref value) = self.s3_bucket_owner {
                properties.insert(
                    "S3BucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_key_prefix {
                properties.insert(
                    "S3KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-ssespecification.html>
    pub struct SSESpecification_ {
        pub kms_master_key_id: Option<crate::value::ExpString>,
        pub sse_enabled: crate::value::ExpBool,
        pub sse_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_SSESpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.SSESpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_SSESpecification as SSESpecification;
    impl crate::value::ToValue for SSESpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_master_key_id {
                properties.insert(
                    "KMSMasterKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SSEEnabled".to_string(),
                crate::value::ToValue::to_value(&self.sse_enabled),
            );
            if let Some(ref value) = self.sse_type {
                properties.insert(
                    "SSEType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-streamspecification.html>
    pub struct StreamSpecification_ {
        pub resource_policy: Option<Box<ResourcePolicy_>>,
        pub stream_view_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_StreamSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.StreamSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_StreamSpecification as StreamSpecification;
    impl crate::value::ToValue for StreamSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_policy {
                properties.insert(
                    "ResourcePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StreamViewType".to_string(),
                crate::value::ToValue::to_value(&self.stream_view_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-timetolivespecification.html>
    pub struct TimeToLiveSpecification_ {
        pub attribute_name: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_TimeToLiveSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.TimeToLiveSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_TimeToLiveSpecification as TimeToLiveSpecification;
    impl crate::value::ToValue for TimeToLiveSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_name {
                properties.insert(
                    "AttributeName".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dynamodb-table-warmthroughput.html>
    pub struct WarmThroughput_ {
        pub read_units_per_second: Option<i32>,
        pub write_units_per_second: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dynamodb_Table_WarmThroughput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DynamoDB::Table.WarmThroughput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dynamodb_Table_WarmThroughput as WarmThroughput;
    impl crate::value::ToValue for WarmThroughput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.read_units_per_second {
                properties.insert(
                    "ReadUnitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.write_units_per_second {
                properties.insert(
                    "WriteUnitsPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-globaltable.html>
pub struct GlobalTable_ {
    pub attribute_definitions: Option<Vec<super::dynamodb::globaltable::AttributeDefinition_>>,
    pub billing_mode: Option<crate::value::ExpString>,
    pub global_secondary_indexes: Option<Vec<super::dynamodb::globaltable::GlobalSecondaryIndex_>>,
    pub global_table_source_arn: Option<crate::value::ExpString>,
    pub global_table_witnesses: Option<Vec<super::dynamodb::globaltable::GlobalTableWitness_>>,
    pub key_schema: Option<Vec<super::dynamodb::globaltable::KeySchema_>>,
    pub local_secondary_indexes: Option<Vec<super::dynamodb::globaltable::LocalSecondaryIndex_>>,
    pub multi_region_consistency: Option<crate::value::ExpString>,
    pub read_on_demand_throughput_settings:
        Option<super::dynamodb::globaltable::ReadOnDemandThroughputSettings_>,
    pub read_provisioned_throughput_settings:
        Option<super::dynamodb::globaltable::GlobalReadProvisionedThroughputSettings_>,
    pub replicas: Vec<super::dynamodb::globaltable::ReplicaSpecification_>,
    pub sse_specification: Option<super::dynamodb::globaltable::SSESpecification_>,
    pub stream_specification: Option<super::dynamodb::globaltable::StreamSpecification_>,
    pub table_name: Option<crate::value::ExpString>,
    pub time_to_live_specification: Option<super::dynamodb::globaltable::TimeToLiveSpecification_>,
    pub warm_throughput: Option<super::dynamodb::globaltable::WarmThroughput_>,
    pub write_on_demand_throughput_settings:
        Option<super::dynamodb::globaltable::WriteOnDemandThroughputSettings_>,
    pub write_provisioned_throughput_settings:
        Option<super::dynamodb::globaltable::WriteProvisionedThroughputSettings_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dynamodb_GlobalTable {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DynamoDB::GlobalTable"
        $($field $value)*)
    };
}
pub use crate::__aws_dynamodb_GlobalTable as GlobalTable;
impl crate::template::ToResource for GlobalTable_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DynamoDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GlobalTable"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attribute_definitions {
            properties.insert(
                "AttributeDefinitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.billing_mode {
            properties.insert(
                "BillingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_secondary_indexes {
            properties.insert(
                "GlobalSecondaryIndexes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_table_source_arn {
            properties.insert(
                "GlobalTableSourceArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_table_witnesses {
            properties.insert(
                "GlobalTableWitnesses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.key_schema {
            properties.insert(
                "KeySchema".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.local_secondary_indexes {
            properties.insert(
                "LocalSecondaryIndexes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_region_consistency {
            properties.insert(
                "MultiRegionConsistency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.read_on_demand_throughput_settings {
            properties.insert(
                "ReadOnDemandThroughputSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.read_provisioned_throughput_settings {
            properties.insert(
                "ReadProvisionedThroughputSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Replicas".to_string(),
            crate::value::ToValue::to_value(&self.replicas),
        );
        if let Some(ref value) = self.sse_specification {
            properties.insert(
                "SSESpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stream_specification {
            properties.insert(
                "StreamSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.table_name {
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.time_to_live_specification {
            properties.insert(
                "TimeToLiveSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.warm_throughput {
            properties.insert(
                "WarmThroughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.write_on_demand_throughput_settings {
            properties.insert(
                "WriteOnDemandThroughputSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.write_provisioned_throughput_settings {
            properties.insert(
                "WriteProvisionedThroughputSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dynamodb-table.html>
pub struct Table_ {
    pub attribute_definitions: Option<Vec<super::dynamodb::table::AttributeDefinition_>>,
    pub billing_mode: Option<crate::value::ExpString>,
    pub contributor_insights_specification:
        Option<super::dynamodb::table::ContributorInsightsSpecification_>,
    pub deletion_protection_enabled: Option<crate::value::ExpBool>,
    pub global_secondary_indexes: Option<Vec<super::dynamodb::table::GlobalSecondaryIndex_>>,
    pub import_source_specification: Option<super::dynamodb::table::ImportSourceSpecification_>,
    pub key_schema: Vec<super::dynamodb::table::KeySchema_>,
    pub kinesis_stream_specification: Option<super::dynamodb::table::KinesisStreamSpecification_>,
    pub local_secondary_indexes: Option<Vec<super::dynamodb::table::LocalSecondaryIndex_>>,
    pub on_demand_throughput: Option<super::dynamodb::table::OnDemandThroughput_>,
    pub point_in_time_recovery_specification:
        Option<super::dynamodb::table::PointInTimeRecoverySpecification_>,
    pub provisioned_throughput: Option<super::dynamodb::table::ProvisionedThroughput_>,
    pub resource_policy: Option<super::dynamodb::table::ResourcePolicy_>,
    pub sse_specification: Option<super::dynamodb::table::SSESpecification_>,
    pub stream_specification: Option<super::dynamodb::table::StreamSpecification_>,
    pub table_class: Option<crate::value::ExpString>,
    pub table_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub time_to_live_specification: Option<super::dynamodb::table::TimeToLiveSpecification_>,
    pub warm_throughput: Option<super::dynamodb::table::WarmThroughput_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dynamodb_Table {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DynamoDB::Table" $($field
        $value)*)
    };
}
pub use crate::__aws_dynamodb_Table as Table;
impl crate::template::ToResource for Table_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DynamoDB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Table"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attribute_definitions {
            properties.insert(
                "AttributeDefinitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.billing_mode {
            properties.insert(
                "BillingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.contributor_insights_specification {
            properties.insert(
                "ContributorInsightsSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection_enabled {
            properties.insert(
                "DeletionProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.global_secondary_indexes {
            properties.insert(
                "GlobalSecondaryIndexes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.import_source_specification {
            properties.insert(
                "ImportSourceSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KeySchema".to_string(),
            crate::value::ToValue::to_value(&self.key_schema),
        );
        if let Some(ref value) = self.kinesis_stream_specification {
            properties.insert(
                "KinesisStreamSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.local_secondary_indexes {
            properties.insert(
                "LocalSecondaryIndexes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.on_demand_throughput {
            properties.insert(
                "OnDemandThroughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.point_in_time_recovery_specification {
            properties.insert(
                "PointInTimeRecoverySpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioned_throughput {
            properties.insert(
                "ProvisionedThroughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_policy {
            properties.insert(
                "ResourcePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sse_specification {
            properties.insert(
                "SSESpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.stream_specification {
            properties.insert(
                "StreamSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.table_class {
            properties.insert(
                "TableClass".to_string(),
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
        if let Some(ref value) = self.time_to_live_specification {
            properties.insert(
                "TimeToLiveSpecification".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.warm_throughput {
            properties.insert(
                "WarmThroughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
