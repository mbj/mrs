pub mod capacityreservation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-capacityreservation-capacityassignment.html
    pub struct CapacityAssignment_ {
        pub workgroup_names: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_CapacityReservation_CapacityAssignment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::CapacityReservation.CapacityAssignment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_CapacityReservation_CapacityAssignment as CapacityAssignment;
    impl crate::value::ToValue for CapacityAssignment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WorkgroupNames".to_string(),
                crate::value::ToValue::to_value(&self.workgroup_names),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-capacityreservation-capacityassignmentconfiguration.html
    pub struct CapacityAssignmentConfiguration_ {
        pub capacity_assignments: Vec<CapacityAssignment_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_CapacityReservation_CapacityAssignmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::CapacityReservation.CapacityAssignmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_CapacityReservation_CapacityAssignmentConfiguration as CapacityAssignmentConfiguration;
    impl crate::value::ToValue for CapacityAssignmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CapacityAssignments".to_string(),
                crate::value::ToValue::to_value(&self.capacity_assignments),
            );
            properties.into()
        }
    }
}
pub mod workgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-aclconfiguration.html
    pub struct AclConfiguration_ {
        pub s3_acl_option: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_AclConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.AclConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_AclConfiguration as AclConfiguration;
    impl crate::value::ToValue for AclConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3AclOption".to_string(),
                crate::value::ToValue::to_value(&self.s3_acl_option),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-classification.html
    pub struct Classification_ {
        pub name: Option<crate::value::ExpString>,
        pub properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_Classification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.Classification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_Classification as Classification;
    impl crate::value::ToValue for Classification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-cloudwatchloggingconfiguration.html
    pub struct CloudWatchLoggingConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub log_group: Option<crate::value::ExpString>,
        pub log_stream_name_prefix: Option<crate::value::ExpString>,
        pub log_types: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_CloudWatchLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.CloudWatchLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_CloudWatchLoggingConfiguration as CloudWatchLoggingConfiguration;
    impl crate::value::ToValue for CloudWatchLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_group {
                properties.insert(
                    "LogGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_stream_name_prefix {
                properties.insert(
                    "LogStreamNamePrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_types {
                properties.insert(
                    "LogTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-customercontentencryptionconfiguration.html
    pub struct CustomerContentEncryptionConfiguration_ {
        pub kms_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_CustomerContentEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.CustomerContentEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_CustomerContentEncryptionConfiguration as CustomerContentEncryptionConfiguration;
    impl crate::value::ToValue for CustomerContentEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KmsKey".to_string(),
                crate::value::ToValue::to_value(&self.kms_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub encryption_option: crate::value::ExpString,
        pub kms_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionOption".to_string(),
                crate::value::ToValue::to_value(&self.encryption_option),
            );
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-engineconfiguration.html
    pub struct EngineConfiguration_ {
        pub additional_configs: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub classifications: Option<Vec<Classification_>>,
        pub coordinator_dpu_size: Option<i32>,
        pub default_executor_dpu_size: Option<i32>,
        pub max_concurrent_dpus: Option<i32>,
        pub spark_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_EngineConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.EngineConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_EngineConfiguration as EngineConfiguration;
    impl crate::value::ToValue for EngineConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_configs {
                properties.insert(
                    "AdditionalConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.classifications {
                properties.insert(
                    "Classifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.coordinator_dpu_size {
                properties.insert(
                    "CoordinatorDpuSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_executor_dpu_size {
                properties.insert(
                    "DefaultExecutorDpuSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_concurrent_dpus {
                properties.insert(
                    "MaxConcurrentDpus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spark_properties {
                properties.insert(
                    "SparkProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-engineversion.html
    pub struct EngineVersion_ {
        pub effective_engine_version: Option<crate::value::ExpString>,
        pub selected_engine_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_EngineVersion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.EngineVersion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_EngineVersion as EngineVersion;
    impl crate::value::ToValue for EngineVersion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.effective_engine_version {
                properties.insert(
                    "EffectiveEngineVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.selected_engine_version {
                properties.insert(
                    "SelectedEngineVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-managedloggingconfiguration.html
    pub struct ManagedLoggingConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub kms_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_ManagedLoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.ManagedLoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_ManagedLoggingConfiguration as ManagedLoggingConfiguration;
    impl crate::value::ToValue for ManagedLoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-managedqueryresultsconfiguration.html
    pub struct ManagedQueryResultsConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub encryption_configuration: Option<Box<ManagedStorageEncryptionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_ManagedQueryResultsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.ManagedQueryResultsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_ManagedQueryResultsConfiguration as ManagedQueryResultsConfiguration;
    impl crate::value::ToValue for ManagedQueryResultsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-managedstorageencryptionconfiguration.html
    pub struct ManagedStorageEncryptionConfiguration_ {
        pub kms_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_ManagedStorageEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.ManagedStorageEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_ManagedStorageEncryptionConfiguration as ManagedStorageEncryptionConfiguration;
    impl crate::value::ToValue for ManagedStorageEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-monitoringconfiguration.html
    pub struct MonitoringConfiguration_ {
        pub cloud_watch_logging_configuration: Option<Box<CloudWatchLoggingConfiguration_>>,
        pub managed_logging_configuration: Option<Box<ManagedLoggingConfiguration_>>,
        pub s3_logging_configuration: Option<Box<S3LoggingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_MonitoringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.MonitoringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_MonitoringConfiguration as MonitoringConfiguration;
    impl crate::value::ToValue for MonitoringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logging_configuration {
                properties.insert(
                    "CloudWatchLoggingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_logging_configuration {
                properties.insert(
                    "ManagedLoggingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_logging_configuration {
                properties.insert(
                    "S3LoggingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-resultconfiguration.html
    pub struct ResultConfiguration_ {
        pub acl_configuration: Option<Box<AclConfiguration_>>,
        pub encryption_configuration: Option<Box<EncryptionConfiguration_>>,
        pub expected_bucket_owner: Option<crate::value::ExpString>,
        pub output_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_ResultConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.ResultConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_ResultConfiguration as ResultConfiguration;
    impl crate::value::ToValue for ResultConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.acl_configuration {
                properties.insert(
                    "AclConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expected_bucket_owner {
                properties.insert(
                    "ExpectedBucketOwner".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_location {
                properties.insert(
                    "OutputLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-s3loggingconfiguration.html
    pub struct S3LoggingConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
        pub kms_key: Option<crate::value::ExpString>,
        pub log_location: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_S3LoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.S3LoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_S3LoggingConfiguration as S3LoggingConfiguration;
    impl crate::value::ToValue for S3LoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key {
                properties.insert("KmsKey".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.log_location {
                properties.insert(
                    "LogLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-athena-workgroup-workgroupconfiguration.html
    pub struct WorkGroupConfiguration_ {
        pub additional_configuration: Option<crate::value::ExpString>,
        pub bytes_scanned_cutoff_per_query: Option<i64>,
        pub customer_content_encryption_configuration:
            Option<Box<CustomerContentEncryptionConfiguration_>>,
        pub enforce_work_group_configuration: Option<crate::value::ExpBool>,
        pub engine_configuration: Option<Box<EngineConfiguration_>>,
        pub engine_version: Option<Box<EngineVersion_>>,
        pub execution_role: Option<crate::value::ExpString>,
        pub managed_query_results_configuration: Option<Box<ManagedQueryResultsConfiguration_>>,
        pub monitoring_configuration: Option<Box<MonitoringConfiguration_>>,
        pub publish_cloud_watch_metrics_enabled: Option<crate::value::ExpBool>,
        pub requester_pays_enabled: Option<crate::value::ExpBool>,
        pub result_configuration: Option<Box<ResultConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_athena_WorkGroup_WorkGroupConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Athena::WorkGroup.WorkGroupConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_athena_WorkGroup_WorkGroupConfiguration as WorkGroupConfiguration;
    impl crate::value::ToValue for WorkGroupConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_configuration {
                properties.insert(
                    "AdditionalConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bytes_scanned_cutoff_per_query {
                properties.insert(
                    "BytesScannedCutoffPerQuery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customer_content_encryption_configuration {
                properties.insert(
                    "CustomerContentEncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enforce_work_group_configuration {
                properties.insert(
                    "EnforceWorkGroupConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.engine_configuration {
                properties.insert(
                    "EngineConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.engine_version {
                properties.insert(
                    "EngineVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.execution_role {
                properties.insert(
                    "ExecutionRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_query_results_configuration {
                properties.insert(
                    "ManagedQueryResultsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitoring_configuration {
                properties.insert(
                    "MonitoringConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.publish_cloud_watch_metrics_enabled {
                properties.insert(
                    "PublishCloudWatchMetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.requester_pays_enabled {
                properties.insert(
                    "RequesterPaysEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.result_configuration {
                properties.insert(
                    "ResultConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-capacityreservation.html
pub struct CapacityReservation_ {
    pub capacity_assignment_configuration:
        Option<super::athena::capacityreservation::CapacityAssignmentConfiguration_>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_dpus: i64,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_athena_CapacityReservation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Athena::CapacityReservation"
        $($field $value)*)
    };
}
pub use crate::__aws_athena_CapacityReservation as CapacityReservation;
impl crate::template::ToResource for CapacityReservation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Athena"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CapacityReservation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.capacity_assignment_configuration {
            properties.insert(
                "CapacityAssignmentConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetDpus".to_string(),
            crate::value::ToValue::to_value(&self.target_dpus),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-datacatalog.html
pub struct DataCatalog_ {
    pub connection_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub error: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub parameters: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_athena_DataCatalog {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Athena::DataCatalog"
        $($field $value)*)
    };
}
pub use crate::__aws_athena_DataCatalog as DataCatalog;
impl crate::template::ToResource for DataCatalog_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Athena"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataCatalog"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.connection_type {
            properties.insert(
                "ConnectionType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.error {
            properties.insert("Error".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-namedquery.html
pub struct NamedQuery_ {
    pub database: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub query_string: crate::value::ExpString,
    pub work_group: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_athena_NamedQuery {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Athena::NamedQuery"
        $($field $value)*)
    };
}
pub use crate::__aws_athena_NamedQuery as NamedQuery;
impl crate::template::ToResource for NamedQuery_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Athena"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NamedQuery"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Database".to_string(),
            crate::value::ToValue::to_value(&self.database),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "QueryString".to_string(),
            crate::value::ToValue::to_value(&self.query_string),
        );
        if let Some(ref value) = self.work_group {
            properties.insert(
                "WorkGroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-preparedstatement.html
pub struct PreparedStatement_ {
    pub description: Option<crate::value::ExpString>,
    pub query_statement: crate::value::ExpString,
    pub statement_name: crate::value::ExpString,
    pub work_group: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_athena_PreparedStatement {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Athena::PreparedStatement"
        $($field $value)*)
    };
}
pub use crate::__aws_athena_PreparedStatement as PreparedStatement;
impl crate::template::ToResource for PreparedStatement_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Athena"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PreparedStatement"),
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
            "QueryStatement".to_string(),
            crate::value::ToValue::to_value(&self.query_statement),
        );
        properties.insert(
            "StatementName".to_string(),
            crate::value::ToValue::to_value(&self.statement_name),
        );
        properties.insert(
            "WorkGroup".to_string(),
            crate::value::ToValue::to_value(&self.work_group),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-athena-workgroup.html
pub struct WorkGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub recursive_delete_option: Option<crate::value::ExpBool>,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub work_group_configuration: Option<super::athena::workgroup::WorkGroupConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_athena_WorkGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Athena::WorkGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_athena_WorkGroup as WorkGroup;
impl crate::template::ToResource for WorkGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Athena"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WorkGroup"),
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
        if let Some(ref value) = self.recursive_delete_option {
            properties.insert(
                "RecursiveDeleteOption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.work_group_configuration {
            properties.insert(
                "WorkGroupConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
