pub mod datarepositoryassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-datarepositoryassociation-autoexportpolicy.html
    pub struct AutoExportPolicy_ {
        pub events: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_DataRepositoryAssociation_AutoExportPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::DataRepositoryAssociation.AutoExportPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_DataRepositoryAssociation_AutoExportPolicy as AutoExportPolicy;
    impl crate::value::ToValue for AutoExportPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Events".to_string(),
                crate::value::ToValue::to_value(&self.events),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-datarepositoryassociation-autoimportpolicy.html
    pub struct AutoImportPolicy_ {
        pub events: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_DataRepositoryAssociation_AutoImportPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::DataRepositoryAssociation.AutoImportPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_DataRepositoryAssociation_AutoImportPolicy as AutoImportPolicy;
    impl crate::value::ToValue for AutoImportPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Events".to_string(),
                crate::value::ToValue::to_value(&self.events),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-datarepositoryassociation-s3.html
    pub struct S3_ {
        pub auto_export_policy: Option<Box<AutoExportPolicy_>>,
        pub auto_import_policy: Option<Box<AutoImportPolicy_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_DataRepositoryAssociation_S3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::DataRepositoryAssociation.S3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_DataRepositoryAssociation_S3 as S3;
    impl crate::value::ToValue for S3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_export_policy {
                properties.insert(
                    "AutoExportPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auto_import_policy {
                properties.insert(
                    "AutoImportPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod filesystem {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-auditlogconfiguration.html
    pub struct AuditLogConfiguration_ {
        pub audit_log_destination: Option<crate::value::ExpString>,
        pub file_access_audit_log_level: crate::value::ExpString,
        pub file_share_access_audit_log_level: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_AuditLogConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.AuditLogConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_AuditLogConfiguration as AuditLogConfiguration;
    impl crate::value::ToValue for AuditLogConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audit_log_destination {
                properties.insert(
                    "AuditLogDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FileAccessAuditLogLevel".to_string(),
                crate::value::ToValue::to_value(&self.file_access_audit_log_level),
            );
            properties.insert(
                "FileShareAccessAuditLogLevel".to_string(),
                crate::value::ToValue::to_value(&self.file_share_access_audit_log_level),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports-clientconfigurations.html
    pub struct ClientConfigurations_ {
        pub clients: Option<crate::value::ExpString>,
        pub options: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_ClientConfigurations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.ClientConfigurations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_ClientConfigurations as ClientConfigurations;
    impl crate::value::ToValue for ClientConfigurations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.clients {
                properties.insert(
                    "Clients".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration-datareadcacheconfiguration.html
    pub struct DataReadCacheConfiguration_ {
        pub size_gi_b: Option<i32>,
        pub sizing_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_DataReadCacheConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.DataReadCacheConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_DataReadCacheConfiguration as DataReadCacheConfiguration;
    impl crate::value::ToValue for DataReadCacheConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.size_gi_b {
                properties.insert(
                    "SizeGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sizing_mode {
                properties.insert(
                    "SizingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-diskiopsconfiguration.html
    pub struct DiskIopsConfiguration_ {
        pub iops: Option<i32>,
        pub mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_DiskIopsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.DiskIopsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_DiskIopsConfiguration as DiskIopsConfiguration;
    impl crate::value::ToValue for DiskIopsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration.html
    pub struct LustreConfiguration_ {
        pub auto_import_policy: Option<crate::value::ExpString>,
        pub automatic_backup_retention_days: Option<i32>,
        pub copy_tags_to_backups: Option<crate::value::ExpBool>,
        pub daily_automatic_backup_start_time: Option<crate::value::ExpString>,
        pub data_compression_type: Option<crate::value::ExpString>,
        pub data_read_cache_configuration: Option<Box<DataReadCacheConfiguration_>>,
        pub deployment_type: Option<crate::value::ExpString>,
        pub drive_cache_type: Option<crate::value::ExpString>,
        pub efa_enabled: Option<crate::value::ExpBool>,
        pub export_path: Option<crate::value::ExpString>,
        pub import_path: Option<crate::value::ExpString>,
        pub imported_file_chunk_size: Option<i32>,
        pub metadata_configuration: Option<Box<MetadataConfiguration_>>,
        pub per_unit_storage_throughput: Option<i32>,
        pub throughput_capacity: Option<i32>,
        pub weekly_maintenance_start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_LustreConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.LustreConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_LustreConfiguration as LustreConfiguration;
    impl crate::value::ToValue for LustreConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_import_policy {
                properties.insert(
                    "AutoImportPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.automatic_backup_retention_days {
                properties.insert(
                    "AutomaticBackupRetentionDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_tags_to_backups {
                properties.insert(
                    "CopyTagsToBackups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.daily_automatic_backup_start_time {
                properties.insert(
                    "DailyAutomaticBackupStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_compression_type {
                properties.insert(
                    "DataCompressionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_read_cache_configuration {
                properties.insert(
                    "DataReadCacheConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deployment_type {
                properties.insert(
                    "DeploymentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drive_cache_type {
                properties.insert(
                    "DriveCacheType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.efa_enabled {
                properties.insert(
                    "EfaEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.export_path {
                properties.insert(
                    "ExportPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.import_path {
                properties.insert(
                    "ImportPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.imported_file_chunk_size {
                properties.insert(
                    "ImportedFileChunkSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata_configuration {
                properties.insert(
                    "MetadataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.per_unit_storage_throughput {
                properties.insert(
                    "PerUnitStorageThroughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput_capacity {
                properties.insert(
                    "ThroughputCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weekly_maintenance_start_time {
                properties.insert(
                    "WeeklyMaintenanceStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-lustreconfiguration-metadataconfiguration.html
    pub struct MetadataConfiguration_ {
        pub iops: Option<i32>,
        pub mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_MetadataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.MetadataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_MetadataConfiguration as MetadataConfiguration;
    impl crate::value::ToValue for MetadataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-nfsexports.html
    pub struct NfsExports_ {
        pub client_configurations: Option<Vec<ClientConfigurations_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_NfsExports {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.NfsExports"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_NfsExports as NfsExports;
    impl crate::value::ToValue for NfsExports_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.client_configurations {
                properties.insert(
                    "ClientConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-ontapconfiguration.html
    pub struct OntapConfiguration_ {
        pub automatic_backup_retention_days: Option<i32>,
        pub daily_automatic_backup_start_time: Option<crate::value::ExpString>,
        pub deployment_type: crate::value::ExpString,
        pub disk_iops_configuration: Option<Box<DiskIopsConfiguration_>>,
        pub endpoint_ip_address_range: Option<crate::value::ExpString>,
        pub endpoint_ipv6_address_range: Option<crate::value::ExpString>,
        pub fsx_admin_password: Option<crate::value::ExpString>,
        pub ha_pairs: Option<i32>,
        pub preferred_subnet_id: Option<crate::value::ExpString>,
        pub route_table_ids: Option<Vec<crate::value::ExpString>>,
        pub throughput_capacity: Option<i32>,
        pub throughput_capacity_per_ha_pair: Option<i32>,
        pub weekly_maintenance_start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_OntapConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.OntapConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_OntapConfiguration as OntapConfiguration;
    impl crate::value::ToValue for OntapConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic_backup_retention_days {
                properties.insert(
                    "AutomaticBackupRetentionDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.daily_automatic_backup_start_time {
                properties.insert(
                    "DailyAutomaticBackupStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DeploymentType".to_string(),
                crate::value::ToValue::to_value(&self.deployment_type),
            );
            if let Some(ref value) = self.disk_iops_configuration {
                properties.insert(
                    "DiskIopsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_ip_address_range {
                properties.insert(
                    "EndpointIpAddressRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_ipv6_address_range {
                properties.insert(
                    "EndpointIpv6AddressRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fsx_admin_password {
                properties.insert(
                    "FsxAdminPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ha_pairs {
                properties.insert(
                    "HAPairs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_subnet_id {
                properties.insert(
                    "PreferredSubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route_table_ids {
                properties.insert(
                    "RouteTableIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput_capacity {
                properties.insert(
                    "ThroughputCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput_capacity_per_ha_pair {
                properties.insert(
                    "ThroughputCapacityPerHAPair".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weekly_maintenance_start_time {
                properties.insert(
                    "WeeklyMaintenanceStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration.html
    pub struct OpenZFSConfiguration_ {
        pub automatic_backup_retention_days: Option<i32>,
        pub copy_tags_to_backups: Option<crate::value::ExpBool>,
        pub copy_tags_to_volumes: Option<crate::value::ExpBool>,
        pub daily_automatic_backup_start_time: Option<crate::value::ExpString>,
        pub deployment_type: crate::value::ExpString,
        pub disk_iops_configuration: Option<Box<DiskIopsConfiguration_>>,
        pub endpoint_ip_address_range: Option<crate::value::ExpString>,
        pub endpoint_ipv6_address_range: Option<crate::value::ExpString>,
        pub options: Option<Vec<crate::value::ExpString>>,
        pub preferred_subnet_id: Option<crate::value::ExpString>,
        pub read_cache_configuration: Option<Box<ReadCacheConfiguration_>>,
        pub root_volume_configuration: Option<Box<RootVolumeConfiguration_>>,
        pub route_table_ids: Option<Vec<crate::value::ExpString>>,
        pub throughput_capacity: Option<i32>,
        pub weekly_maintenance_start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_OpenZFSConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.OpenZFSConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_OpenZFSConfiguration as OpenZFSConfiguration;
    impl crate::value::ToValue for OpenZFSConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic_backup_retention_days {
                properties.insert(
                    "AutomaticBackupRetentionDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_tags_to_backups {
                properties.insert(
                    "CopyTagsToBackups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_tags_to_volumes {
                properties.insert(
                    "CopyTagsToVolumes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.daily_automatic_backup_start_time {
                properties.insert(
                    "DailyAutomaticBackupStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DeploymentType".to_string(),
                crate::value::ToValue::to_value(&self.deployment_type),
            );
            if let Some(ref value) = self.disk_iops_configuration {
                properties.insert(
                    "DiskIopsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_ip_address_range {
                properties.insert(
                    "EndpointIpAddressRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_ipv6_address_range {
                properties.insert(
                    "EndpointIpv6AddressRange".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_subnet_id {
                properties.insert(
                    "PreferredSubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_cache_configuration {
                properties.insert(
                    "ReadCacheConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.root_volume_configuration {
                properties.insert(
                    "RootVolumeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.route_table_ids {
                properties.insert(
                    "RouteTableIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput_capacity {
                properties.insert(
                    "ThroughputCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weekly_maintenance_start_time {
                properties.insert(
                    "WeeklyMaintenanceStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-readcacheconfiguration.html
    pub struct ReadCacheConfiguration_ {
        pub size_gi_b: Option<i32>,
        pub sizing_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_ReadCacheConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.ReadCacheConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_ReadCacheConfiguration as ReadCacheConfiguration;
    impl crate::value::ToValue for ReadCacheConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.size_gi_b {
                properties.insert(
                    "SizeGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sizing_mode {
                properties.insert(
                    "SizingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration.html
    pub struct RootVolumeConfiguration_ {
        pub copy_tags_to_snapshots: Option<crate::value::ExpBool>,
        pub data_compression_type: Option<crate::value::ExpString>,
        pub nfs_exports: Option<Vec<NfsExports_>>,
        pub read_only: Option<crate::value::ExpBool>,
        pub record_size_ki_b: Option<i32>,
        pub user_and_group_quotas: Option<Vec<UserAndGroupQuotas_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_RootVolumeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.RootVolumeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_RootVolumeConfiguration as RootVolumeConfiguration;
    impl crate::value::ToValue for RootVolumeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copy_tags_to_snapshots {
                properties.insert(
                    "CopyTagsToSnapshots".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_compression_type {
                properties.insert(
                    "DataCompressionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nfs_exports {
                properties.insert(
                    "NfsExports".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_size_ki_b {
                properties.insert(
                    "RecordSizeKiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_and_group_quotas {
                properties.insert(
                    "UserAndGroupQuotas".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration-selfmanagedactivedirectoryconfiguration.html
    pub struct SelfManagedActiveDirectoryConfiguration_ {
        pub dns_ips: Option<Vec<crate::value::ExpString>>,
        pub domain_join_service_account_secret: Option<crate::value::ExpString>,
        pub domain_name: Option<crate::value::ExpString>,
        pub file_system_administrators_group: Option<crate::value::ExpString>,
        pub organizational_unit_distinguished_name: Option<crate::value::ExpString>,
        pub password: Option<crate::value::ExpString>,
        pub user_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_SelfManagedActiveDirectoryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.SelfManagedActiveDirectoryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_SelfManagedActiveDirectoryConfiguration as SelfManagedActiveDirectoryConfiguration;
    impl crate::value::ToValue for SelfManagedActiveDirectoryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dns_ips {
                properties.insert("DnsIps".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.domain_join_service_account_secret {
                properties.insert(
                    "DomainJoinServiceAccountSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_name {
                properties.insert(
                    "DomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_system_administrators_group {
                properties.insert(
                    "FileSystemAdministratorsGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit_distinguished_name {
                properties.insert(
                    "OrganizationalUnitDistinguishedName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_name {
                properties.insert(
                    "UserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-openzfsconfiguration-rootvolumeconfiguration-userandgroupquotas.html
    pub struct UserAndGroupQuotas_ {
        pub id: Option<i32>,
        pub storage_capacity_quota_gi_b: Option<i32>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_UserAndGroupQuotas {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.UserAndGroupQuotas"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_UserAndGroupQuotas as UserAndGroupQuotas;
    impl crate::value::ToValue for UserAndGroupQuotas_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.storage_capacity_quota_gi_b {
                properties.insert(
                    "StorageCapacityQuotaGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-filesystem-windowsconfiguration.html
    pub struct WindowsConfiguration_ {
        pub active_directory_id: Option<crate::value::ExpString>,
        pub aliases: Option<Vec<crate::value::ExpString>>,
        pub audit_log_configuration: Option<Box<AuditLogConfiguration_>>,
        pub automatic_backup_retention_days: Option<i32>,
        pub copy_tags_to_backups: Option<crate::value::ExpBool>,
        pub daily_automatic_backup_start_time: Option<crate::value::ExpString>,
        pub deployment_type: Option<crate::value::ExpString>,
        pub disk_iops_configuration: Option<Box<DiskIopsConfiguration_>>,
        pub preferred_subnet_id: Option<crate::value::ExpString>,
        pub self_managed_active_directory_configuration:
            Option<Box<SelfManagedActiveDirectoryConfiguration_>>,
        pub throughput_capacity: i32,
        pub weekly_maintenance_start_time: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_FileSystem_WindowsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::FileSystem.WindowsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_FileSystem_WindowsConfiguration as WindowsConfiguration;
    impl crate::value::ToValue for WindowsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.active_directory_id {
                properties.insert(
                    "ActiveDirectoryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aliases {
                properties.insert(
                    "Aliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.audit_log_configuration {
                properties.insert(
                    "AuditLogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.automatic_backup_retention_days {
                properties.insert(
                    "AutomaticBackupRetentionDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_tags_to_backups {
                properties.insert(
                    "CopyTagsToBackups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.daily_automatic_backup_start_time {
                properties.insert(
                    "DailyAutomaticBackupStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deployment_type {
                properties.insert(
                    "DeploymentType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disk_iops_configuration {
                properties.insert(
                    "DiskIopsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preferred_subnet_id {
                properties.insert(
                    "PreferredSubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_managed_active_directory_configuration {
                properties.insert(
                    "SelfManagedActiveDirectoryConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ThroughputCapacity".to_string(),
                crate::value::ToValue::to_value(&self.throughput_capacity),
            );
            if let Some(ref value) = self.weekly_maintenance_start_time {
                properties.insert(
                    "WeeklyMaintenanceStartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod s3accesspointattachment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-filesystemgid.html
    pub struct FileSystemGID_ {
        pub gid: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_FileSystemGID {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.FileSystemGID"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_FileSystemGID as FileSystemGID;
    impl crate::value::ToValue for FileSystemGID_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Gid".to_string(),
                crate::value::ToValue::to_value(&self.gid),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-ontapfilesystemidentity.html
    pub struct OntapFileSystemIdentity_ {
        pub r#type: crate::value::ExpString,
        pub unix_user: Option<Box<OntapUnixFileSystemUser_>>,
        pub windows_user: Option<Box<OntapWindowsFileSystemUser_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_OntapFileSystemIdentity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.OntapFileSystemIdentity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_OntapFileSystemIdentity as OntapFileSystemIdentity;
    impl crate::value::ToValue for OntapFileSystemIdentity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.unix_user {
                properties.insert(
                    "UnixUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.windows_user {
                properties.insert(
                    "WindowsUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-ontapunixfilesystemuser.html
    pub struct OntapUnixFileSystemUser_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_OntapUnixFileSystemUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.OntapUnixFileSystemUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_OntapUnixFileSystemUser as OntapUnixFileSystemUser;
    impl crate::value::ToValue for OntapUnixFileSystemUser_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-ontapwindowsfilesystemuser.html
    pub struct OntapWindowsFileSystemUser_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_OntapWindowsFileSystemUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.OntapWindowsFileSystemUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_OntapWindowsFileSystemUser as OntapWindowsFileSystemUser;
    impl crate::value::ToValue for OntapWindowsFileSystemUser_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-openzfsfilesystemidentity.html
    pub struct OpenZFSFileSystemIdentity_ {
        pub posix_user: Box<OpenZFSPosixFileSystemUser_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_OpenZFSFileSystemIdentity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.OpenZFSFileSystemIdentity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_OpenZFSFileSystemIdentity as OpenZFSFileSystemIdentity;
    impl crate::value::ToValue for OpenZFSFileSystemIdentity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PosixUser".to_string(),
                crate::value::ToValue::to_value(&self.posix_user),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-openzfsposixfilesystemuser.html
    pub struct OpenZFSPosixFileSystemUser_ {
        pub gid: f64,
        pub secondary_gids: Option<Vec<FileSystemGID_>>,
        pub uid: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_OpenZFSPosixFileSystemUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.OpenZFSPosixFileSystemUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_OpenZFSPosixFileSystemUser as OpenZFSPosixFileSystemUser;
    impl crate::value::ToValue for OpenZFSPosixFileSystemUser_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Gid".to_string(),
                crate::value::ToValue::to_value(&self.gid),
            );
            if let Some(ref value) = self.secondary_gids {
                properties.insert(
                    "SecondaryGids".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Uid".to_string(),
                crate::value::ToValue::to_value(&self.uid),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-s3accesspoint.html
    pub struct S3AccessPoint_ {
        pub alias: Option<crate::value::ExpString>,
        pub policy: Option<serde_json::Value>,
        pub resource_arn: Option<crate::value::ExpString>,
        pub vpc_configuration: Option<Box<S3AccessPointVpcConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_S3AccessPoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.S3AccessPoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_S3AccessPoint as S3AccessPoint;
    impl crate::value::ToValue for S3AccessPoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alias {
                properties.insert("Alias".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.policy {
                properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource_arn {
                properties.insert(
                    "ResourceARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_configuration {
                properties.insert(
                    "VpcConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-s3accesspointontapconfiguration.html
    pub struct S3AccessPointOntapConfiguration_ {
        pub file_system_identity: Box<OntapFileSystemIdentity_>,
        pub volume_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_S3AccessPointOntapConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.S3AccessPointOntapConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_S3AccessPointOntapConfiguration as S3AccessPointOntapConfiguration;
    impl crate::value::ToValue for S3AccessPointOntapConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemIdentity".to_string(),
                crate::value::ToValue::to_value(&self.file_system_identity),
            );
            properties.insert(
                "VolumeId".to_string(),
                crate::value::ToValue::to_value(&self.volume_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-s3accesspointopenzfsconfiguration.html
    pub struct S3AccessPointOpenZFSConfiguration_ {
        pub file_system_identity: Box<OpenZFSFileSystemIdentity_>,
        pub volume_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_S3AccessPointOpenZFSConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.S3AccessPointOpenZFSConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_S3AccessPointOpenZFSConfiguration as S3AccessPointOpenZFSConfiguration;
    impl crate::value::ToValue for S3AccessPointOpenZFSConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FileSystemIdentity".to_string(),
                crate::value::ToValue::to_value(&self.file_system_identity),
            );
            properties.insert(
                "VolumeId".to_string(),
                crate::value::ToValue::to_value(&self.volume_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-s3accesspointattachment-s3accesspointvpcconfiguration.html
    pub struct S3AccessPointVpcConfiguration_ {
        pub vpc_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_S3AccessPointAttachment_S3AccessPointVpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::S3AccessPointAttachment.S3AccessPointVpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_S3AccessPointAttachment_S3AccessPointVpcConfiguration as S3AccessPointVpcConfiguration;
    impl crate::value::ToValue for S3AccessPointVpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VpcId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_id),
            );
            properties.into()
        }
    }
}
pub mod storagevirtualmachine {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration.html
    pub struct ActiveDirectoryConfiguration_ {
        pub net_bios_name: Option<crate::value::ExpString>,
        pub self_managed_active_directory_configuration:
            Option<Box<SelfManagedActiveDirectoryConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_StorageVirtualMachine_ActiveDirectoryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::StorageVirtualMachine.ActiveDirectoryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_StorageVirtualMachine_ActiveDirectoryConfiguration as ActiveDirectoryConfiguration;
    impl crate::value::ToValue for ActiveDirectoryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.net_bios_name {
                properties.insert(
                    "NetBiosName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_managed_active_directory_configuration {
                properties.insert(
                    "SelfManagedActiveDirectoryConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-storagevirtualmachine-activedirectoryconfiguration-selfmanagedactivedirectoryconfiguration.html
    pub struct SelfManagedActiveDirectoryConfiguration_ {
        pub dns_ips: Option<Vec<crate::value::ExpString>>,
        pub domain_join_service_account_secret: Option<crate::value::ExpString>,
        pub domain_name: Option<crate::value::ExpString>,
        pub file_system_administrators_group: Option<crate::value::ExpString>,
        pub organizational_unit_distinguished_name: Option<crate::value::ExpString>,
        pub password: Option<crate::value::ExpString>,
        pub user_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_StorageVirtualMachine_SelfManagedActiveDirectoryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::StorageVirtualMachine.SelfManagedActiveDirectoryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_StorageVirtualMachine_SelfManagedActiveDirectoryConfiguration as SelfManagedActiveDirectoryConfiguration;
    impl crate::value::ToValue for SelfManagedActiveDirectoryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dns_ips {
                properties.insert("DnsIps".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.domain_join_service_account_secret {
                properties.insert(
                    "DomainJoinServiceAccountSecret".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_name {
                properties.insert(
                    "DomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_system_administrators_group {
                properties.insert(
                    "FileSystemAdministratorsGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.organizational_unit_distinguished_name {
                properties.insert(
                    "OrganizationalUnitDistinguishedName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password {
                properties.insert(
                    "Password".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_name {
                properties.insert(
                    "UserName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod volume {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration-aggregateconfiguration.html
    pub struct AggregateConfiguration_ {
        pub aggregates: Option<Vec<crate::value::ExpString>>,
        pub constituents_per_aggregate: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_AggregateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.AggregateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_AggregateConfiguration as AggregateConfiguration;
    impl crate::value::ToValue for AggregateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregates {
                properties.insert(
                    "Aggregates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.constituents_per_aggregate {
                properties.insert(
                    "ConstituentsPerAggregate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration-snaplockconfiguration-autocommitperiod.html
    pub struct AutocommitPeriod_ {
        pub r#type: crate::value::ExpString,
        pub value: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_AutocommitPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.AutocommitPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_AutocommitPeriod as AutocommitPeriod;
    impl crate::value::ToValue for AutocommitPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-nfsexports-clientconfigurations.html
    pub struct ClientConfigurations_ {
        pub clients: crate::value::ExpString,
        pub options: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_ClientConfigurations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.ClientConfigurations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_ClientConfigurations as ClientConfigurations;
    impl crate::value::ToValue for ClientConfigurations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Clients".to_string(),
                crate::value::ToValue::to_value(&self.clients),
            );
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(&self.options),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-nfsexports.html
    pub struct NfsExports_ {
        pub client_configurations: Vec<ClientConfigurations_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_NfsExports {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.NfsExports"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_NfsExports as NfsExports;
    impl crate::value::ToValue for NfsExports_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.client_configurations),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration.html
    pub struct OntapConfiguration_ {
        pub aggregate_configuration: Option<Box<AggregateConfiguration_>>,
        pub copy_tags_to_backups: Option<crate::value::ExpString>,
        pub junction_path: Option<crate::value::ExpString>,
        pub ontap_volume_type: Option<crate::value::ExpString>,
        pub security_style: Option<crate::value::ExpString>,
        pub size_in_bytes: Option<crate::value::ExpString>,
        pub size_in_megabytes: Option<crate::value::ExpString>,
        pub snaplock_configuration: Option<Box<SnaplockConfiguration_>>,
        pub snapshot_policy: Option<crate::value::ExpString>,
        pub storage_efficiency_enabled: Option<crate::value::ExpString>,
        pub storage_virtual_machine_id: crate::value::ExpString,
        pub tiering_policy: Option<Box<TieringPolicy_>>,
        pub volume_style: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_OntapConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.OntapConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_OntapConfiguration as OntapConfiguration;
    impl crate::value::ToValue for OntapConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aggregate_configuration {
                properties.insert(
                    "AggregateConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.copy_tags_to_backups {
                properties.insert(
                    "CopyTagsToBackups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.junction_path {
                properties.insert(
                    "JunctionPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ontap_volume_type {
                properties.insert(
                    "OntapVolumeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_style {
                properties.insert(
                    "SecurityStyle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_bytes {
                properties.insert(
                    "SizeInBytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_in_megabytes {
                properties.insert(
                    "SizeInMegabytes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snaplock_configuration {
                properties.insert(
                    "SnaplockConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshot_policy {
                properties.insert(
                    "SnapshotPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_efficiency_enabled {
                properties.insert(
                    "StorageEfficiencyEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StorageVirtualMachineId".to_string(),
                crate::value::ToValue::to_value(&self.storage_virtual_machine_id),
            );
            if let Some(ref value) = self.tiering_policy {
                properties.insert(
                    "TieringPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_style {
                properties.insert(
                    "VolumeStyle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration.html
    pub struct OpenZFSConfiguration_ {
        pub copy_tags_to_snapshots: Option<crate::value::ExpBool>,
        pub data_compression_type: Option<crate::value::ExpString>,
        pub nfs_exports: Option<Vec<NfsExports_>>,
        pub options: Option<Vec<crate::value::ExpString>>,
        pub origin_snapshot: Option<Box<OriginSnapshot_>>,
        pub parent_volume_id: crate::value::ExpString,
        pub read_only: Option<crate::value::ExpBool>,
        pub record_size_ki_b: Option<i32>,
        pub storage_capacity_quota_gi_b: Option<i32>,
        pub storage_capacity_reservation_gi_b: Option<i32>,
        pub user_and_group_quotas: Option<Vec<UserAndGroupQuotas_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_OpenZFSConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.OpenZFSConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_OpenZFSConfiguration as OpenZFSConfiguration;
    impl crate::value::ToValue for OpenZFSConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.copy_tags_to_snapshots {
                properties.insert(
                    "CopyTagsToSnapshots".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_compression_type {
                properties.insert(
                    "DataCompressionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nfs_exports {
                properties.insert(
                    "NfsExports".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origin_snapshot {
                properties.insert(
                    "OriginSnapshot".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ParentVolumeId".to_string(),
                crate::value::ToValue::to_value(&self.parent_volume_id),
            );
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_size_ki_b {
                properties.insert(
                    "RecordSizeKiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_capacity_quota_gi_b {
                properties.insert(
                    "StorageCapacityQuotaGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_capacity_reservation_gi_b {
                properties.insert(
                    "StorageCapacityReservationGiB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_and_group_quotas {
                properties.insert(
                    "UserAndGroupQuotas".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-originsnapshot.html
    pub struct OriginSnapshot_ {
        pub copy_strategy: crate::value::ExpString,
        pub snapshot_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_OriginSnapshot {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.OriginSnapshot"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_OriginSnapshot as OriginSnapshot;
    impl crate::value::ToValue for OriginSnapshot_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CopyStrategy".to_string(),
                crate::value::ToValue::to_value(&self.copy_strategy),
            );
            properties.insert(
                "SnapshotARN".to_string(),
                crate::value::ToValue::to_value(&self.snapshot_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-retentionperiod.html
    pub struct RetentionPeriod_ {
        pub r#type: crate::value::ExpString,
        pub value: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_RetentionPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.RetentionPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_RetentionPeriod as RetentionPeriod;
    impl crate::value::ToValue for RetentionPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration-snaplockconfiguration.html
    pub struct SnaplockConfiguration_ {
        pub audit_log_volume: Option<crate::value::ExpString>,
        pub autocommit_period: Option<Box<AutocommitPeriod_>>,
        pub privileged_delete: Option<crate::value::ExpString>,
        pub retention_period: Option<Box<SnaplockRetentionPeriod_>>,
        pub snaplock_type: crate::value::ExpString,
        pub volume_append_mode_enabled: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_SnaplockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.SnaplockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_SnaplockConfiguration as SnaplockConfiguration;
    impl crate::value::ToValue for SnaplockConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audit_log_volume {
                properties.insert(
                    "AuditLogVolume".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.autocommit_period {
                properties.insert(
                    "AutocommitPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.privileged_delete {
                properties.insert(
                    "PrivilegedDelete".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_period {
                properties.insert(
                    "RetentionPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SnaplockType".to_string(),
                crate::value::ToValue::to_value(&self.snaplock_type),
            );
            if let Some(ref value) = self.volume_append_mode_enabled {
                properties.insert(
                    "VolumeAppendModeEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-snaplockretentionperiod.html
    pub struct SnaplockRetentionPeriod_ {
        pub default_retention: Box<RetentionPeriod_>,
        pub maximum_retention: Box<RetentionPeriod_>,
        pub minimum_retention: Box<RetentionPeriod_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_SnaplockRetentionPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.SnaplockRetentionPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_SnaplockRetentionPeriod as SnaplockRetentionPeriod;
    impl crate::value::ToValue for SnaplockRetentionPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultRetention".to_string(),
                crate::value::ToValue::to_value(&self.default_retention),
            );
            properties.insert(
                "MaximumRetention".to_string(),
                crate::value::ToValue::to_value(&self.maximum_retention),
            );
            properties.insert(
                "MinimumRetention".to_string(),
                crate::value::ToValue::to_value(&self.minimum_retention),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-ontapconfiguration-tieringpolicy.html
    pub struct TieringPolicy_ {
        pub cooling_period: Option<i32>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_TieringPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.TieringPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_TieringPolicy as TieringPolicy;
    impl crate::value::ToValue for TieringPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cooling_period {
                properties.insert(
                    "CoolingPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-fsx-volume-openzfsconfiguration-userandgroupquotas.html
    pub struct UserAndGroupQuotas_ {
        pub id: i32,
        pub storage_capacity_quota_gi_b: i32,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_fsx_Volume_UserAndGroupQuotas {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FSx::Volume.UserAndGroupQuotas"
            $($field $value)*)
        };
    }
    pub use crate::__aws_fsx_Volume_UserAndGroupQuotas as UserAndGroupQuotas;
    impl crate::value::ToValue for UserAndGroupQuotas_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "StorageCapacityQuotaGiB".to_string(),
                crate::value::ToValue::to_value(&self.storage_capacity_quota_gi_b),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-datarepositoryassociation.html
pub struct DataRepositoryAssociation_ {
    pub batch_import_meta_data_on_create: Option<crate::value::ExpBool>,
    pub data_repository_path: crate::value::ExpString,
    pub file_system_id: crate::value::ExpString,
    pub file_system_path: crate::value::ExpString,
    pub imported_file_chunk_size: Option<i32>,
    pub s3: Option<super::fsx::datarepositoryassociation::S3_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fsx_DataRepositoryAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FSx::DataRepositoryAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_fsx_DataRepositoryAssociation as DataRepositoryAssociation;
impl crate::template::ToResource for DataRepositoryAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FSx"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataRepositoryAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.batch_import_meta_data_on_create {
            properties.insert(
                "BatchImportMetaDataOnCreate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataRepositoryPath".to_string(),
            crate::value::ToValue::to_value(&self.data_repository_path),
        );
        properties.insert(
            "FileSystemId".to_string(),
            crate::value::ToValue::to_value(&self.file_system_id),
        );
        properties.insert(
            "FileSystemPath".to_string(),
            crate::value::ToValue::to_value(&self.file_system_path),
        );
        if let Some(ref value) = self.imported_file_chunk_size {
            properties.insert(
                "ImportedFileChunkSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3 {
            properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-filesystem.html
pub struct FileSystem_ {
    pub backup_id: Option<crate::value::ExpString>,
    pub file_system_type: crate::value::ExpString,
    pub file_system_type_version: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub lustre_configuration: Option<super::fsx::filesystem::LustreConfiguration_>,
    pub network_type: Option<crate::value::ExpString>,
    pub ontap_configuration: Option<super::fsx::filesystem::OntapConfiguration_>,
    pub open_zfs_configuration: Option<super::fsx::filesystem::OpenZFSConfiguration_>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub storage_capacity: Option<i32>,
    pub storage_type: Option<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub windows_configuration: Option<super::fsx::filesystem::WindowsConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fsx_FileSystem {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FSx::FileSystem" $($field
        $value)*)
    };
}
pub use crate::__aws_fsx_FileSystem as FileSystem;
impl crate::template::ToResource for FileSystem_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FSx"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FileSystem"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.backup_id {
            properties.insert(
                "BackupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FileSystemType".to_string(),
            crate::value::ToValue::to_value(&self.file_system_type),
        );
        if let Some(ref value) = self.file_system_type_version {
            properties.insert(
                "FileSystemTypeVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lustre_configuration {
            properties.insert(
                "LustreConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_type {
            properties.insert(
                "NetworkType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ontap_configuration {
            properties.insert(
                "OntapConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.open_zfs_configuration {
            properties.insert(
                "OpenZFSConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_capacity {
            properties.insert(
                "StorageCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_type {
            properties.insert(
                "StorageType".to_string(),
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
        if let Some(ref value) = self.windows_configuration {
            properties.insert(
                "WindowsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-s3accesspointattachment.html
pub struct S3AccessPointAttachment_ {
    pub name: crate::value::ExpString,
    pub ontap_configuration:
        Option<super::fsx::s3accesspointattachment::S3AccessPointOntapConfiguration_>,
    pub open_zfs_configuration:
        Option<super::fsx::s3accesspointattachment::S3AccessPointOpenZFSConfiguration_>,
    pub s3_access_point: Option<super::fsx::s3accesspointattachment::S3AccessPoint_>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fsx_S3AccessPointAttachment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FSx::S3AccessPointAttachment"
        $($field $value)*)
    };
}
pub use crate::__aws_fsx_S3AccessPointAttachment as S3AccessPointAttachment;
impl crate::template::ToResource for S3AccessPointAttachment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FSx"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("S3AccessPointAttachment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.ontap_configuration {
            properties.insert(
                "OntapConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.open_zfs_configuration {
            properties.insert(
                "OpenZFSConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_access_point {
            properties.insert(
                "S3AccessPoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-snapshot.html
pub struct Snapshot_ {
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub volume_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fsx_Snapshot {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FSx::Snapshot" $($field
        $value)*)
    };
}
pub use crate::__aws_fsx_Snapshot as Snapshot;
impl crate::template::ToResource for Snapshot_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FSx"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Snapshot"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VolumeId".to_string(),
            crate::value::ToValue::to_value(&self.volume_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-storagevirtualmachine.html
pub struct StorageVirtualMachine_ {
    pub active_directory_configuration:
        Option<super::fsx::storagevirtualmachine::ActiveDirectoryConfiguration_>,
    pub file_system_id: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub root_volume_security_style: Option<crate::value::ExpString>,
    pub svm_admin_password: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fsx_StorageVirtualMachine {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FSx::StorageVirtualMachine"
        $($field $value)*)
    };
}
pub use crate::__aws_fsx_StorageVirtualMachine as StorageVirtualMachine;
impl crate::template::ToResource for StorageVirtualMachine_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FSx"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StorageVirtualMachine"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.active_directory_configuration {
            properties.insert(
                "ActiveDirectoryConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FileSystemId".to_string(),
            crate::value::ToValue::to_value(&self.file_system_id),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.root_volume_security_style {
            properties.insert(
                "RootVolumeSecurityStyle".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.svm_admin_password {
            properties.insert(
                "SvmAdminPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-fsx-volume.html
pub struct Volume_ {
    pub backup_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub ontap_configuration: Option<super::fsx::volume::OntapConfiguration_>,
    pub open_zfs_configuration: Option<super::fsx::volume::OpenZFSConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub volume_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_fsx_Volume {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FSx::Volume" $($field
        $value)*)
    };
}
pub use crate::__aws_fsx_Volume as Volume;
impl crate::template::ToResource for Volume_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FSx"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Volume"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.backup_id {
            properties.insert(
                "BackupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.ontap_configuration {
            properties.insert(
                "OntapConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.open_zfs_configuration {
            properties.insert(
                "OpenZFSConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.volume_type {
            properties.insert(
                "VolumeType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
