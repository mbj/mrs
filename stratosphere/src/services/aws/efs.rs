pub mod accesspoint {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-accesspointtag.html>
    pub struct AccessPointTag_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_AccessPoint_AccessPointTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::AccessPoint.AccessPointTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_AccessPoint_AccessPointTag as AccessPointTag;
    impl crate::value::ToValue for AccessPointTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-creationinfo.html>
    pub struct CreationInfo_ {
        pub owner_gid: crate::value::ExpString,
        pub owner_uid: crate::value::ExpString,
        pub permissions: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_AccessPoint_CreationInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::AccessPoint.CreationInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_AccessPoint_CreationInfo as CreationInfo;
    impl crate::value::ToValue for CreationInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OwnerGid".to_string(),
                crate::value::ToValue::to_value(&self.owner_gid),
            );
            properties.insert(
                "OwnerUid".to_string(),
                crate::value::ToValue::to_value(&self.owner_uid),
            );
            properties.insert(
                "Permissions".to_string(),
                crate::value::ToValue::to_value(&self.permissions),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-posixuser.html>
    pub struct PosixUser_ {
        pub gid: crate::value::ExpString,
        pub secondary_gids: Option<Vec<crate::value::ExpString>>,
        pub uid: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_AccessPoint_PosixUser {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::AccessPoint.PosixUser"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_AccessPoint_PosixUser as PosixUser;
    impl crate::value::ToValue for PosixUser_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-accesspoint-rootdirectory.html>
    pub struct RootDirectory_ {
        pub creation_info: Option<Box<CreationInfo_>>,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_AccessPoint_RootDirectory {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::AccessPoint.RootDirectory"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_AccessPoint_RootDirectory as RootDirectory;
    impl crate::value::ToValue for RootDirectory_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.creation_info {
                properties.insert(
                    "CreationInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod filesystem {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-backuppolicy.html>
    pub struct BackupPolicy_ {
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_FileSystem_BackupPolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::FileSystem.BackupPolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_FileSystem_BackupPolicy as BackupPolicy;
    impl crate::value::ToValue for BackupPolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-elasticfilesystemtag.html>
    pub struct ElasticFileSystemTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_FileSystem_ElasticFileSystemTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::FileSystem.ElasticFileSystemTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_FileSystem_ElasticFileSystemTag as ElasticFileSystemTag;
    impl crate::value::ToValue for ElasticFileSystemTag_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-filesystemprotection.html>
    pub struct FileSystemProtection_ {
        pub replication_overwrite_protection: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_FileSystem_FileSystemProtection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::FileSystem.FileSystemProtection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_FileSystem_FileSystemProtection as FileSystemProtection;
    impl crate::value::ToValue for FileSystemProtection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.replication_overwrite_protection {
                properties.insert(
                    "ReplicationOverwriteProtection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-lifecyclepolicy.html>
    pub struct LifecyclePolicy_ {
        pub transition_to_archive: Option<crate::value::ExpString>,
        pub transition_to_ia: Option<crate::value::ExpString>,
        pub transition_to_primary_storage_class: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_FileSystem_LifecyclePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::FileSystem.LifecyclePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_FileSystem_LifecyclePolicy as LifecyclePolicy;
    impl crate::value::ToValue for LifecyclePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.transition_to_archive {
                properties.insert(
                    "TransitionToArchive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transition_to_ia {
                properties.insert(
                    "TransitionToIA".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transition_to_primary_storage_class {
                properties.insert(
                    "TransitionToPrimaryStorageClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-replicationconfiguration.html>
    pub struct ReplicationConfiguration_ {
        pub destinations: Option<Vec<ReplicationDestination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_FileSystem_ReplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::FileSystem.ReplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_FileSystem_ReplicationConfiguration as ReplicationConfiguration;
    impl crate::value::ToValue for ReplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destinations {
                properties.insert(
                    "Destinations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-efs-filesystem-replicationdestination.html>
    pub struct ReplicationDestination_ {
        pub availability_zone_name: Option<crate::value::ExpString>,
        pub file_system_id: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub region: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
        pub status_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_efs_FileSystem_ReplicationDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EFS::FileSystem.ReplicationDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_efs_FileSystem_ReplicationDestination as ReplicationDestination;
    impl crate::value::ToValue for ReplicationDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone_name {
                properties.insert(
                    "AvailabilityZoneName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_system_id {
                properties.insert(
                    "FileSystemId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status_message {
                properties.insert(
                    "StatusMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-accesspoint.html>
pub struct AccessPoint_ {
    pub access_point_tags: Option<Vec<super::efs::accesspoint::AccessPointTag_>>,
    pub client_token: Option<crate::value::ExpString>,
    pub file_system_id: crate::value::ExpString,
    pub posix_user: Option<super::efs::accesspoint::PosixUser_>,
    pub root_directory: Option<super::efs::accesspoint::RootDirectory_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_efs_AccessPoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EFS::AccessPoint"
        $($field $value)*)
    };
}
pub use crate::__aws_efs_AccessPoint as AccessPoint;
impl crate::template::ToResource for AccessPoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EFS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessPoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_point_tags {
            properties.insert(
                "AccessPointTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_token {
            properties.insert(
                "ClientToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FileSystemId".to_string(),
            crate::value::ToValue::to_value(&self.file_system_id),
        );
        if let Some(ref value) = self.posix_user {
            properties.insert(
                "PosixUser".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.root_directory {
            properties.insert(
                "RootDirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-filesystem.html>
pub struct FileSystem_ {
    pub availability_zone_name: Option<crate::value::ExpString>,
    pub backup_policy: Option<super::efs::filesystem::BackupPolicy_>,
    pub bypass_policy_lockout_safety_check: Option<crate::value::ExpBool>,
    pub encrypted: Option<crate::value::ExpBool>,
    pub file_system_policy: Option<serde_json::Value>,
    pub file_system_protection: Option<super::efs::filesystem::FileSystemProtection_>,
    pub file_system_tags: Option<Vec<super::efs::filesystem::ElasticFileSystemTag_>>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub lifecycle_policies: Option<Vec<super::efs::filesystem::LifecyclePolicy_>>,
    pub performance_mode: Option<crate::value::ExpString>,
    pub provisioned_throughput_in_mibps: Option<f64>,
    pub replication_configuration: Option<super::efs::filesystem::ReplicationConfiguration_>,
    pub throughput_mode: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_efs_FileSystem {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EFS::FileSystem" $($field
        $value)*)
    };
}
pub use crate::__aws_efs_FileSystem as FileSystem;
impl crate::template::ToResource for FileSystem_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EFS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FileSystem"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone_name {
            properties.insert(
                "AvailabilityZoneName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_policy {
            properties.insert(
                "BackupPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bypass_policy_lockout_safety_check {
            properties.insert(
                "BypassPolicyLockoutSafetyCheck".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encrypted {
            properties.insert(
                "Encrypted".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_system_policy {
            properties.insert(
                "FileSystemPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_system_protection {
            properties.insert(
                "FileSystemProtection".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_system_tags {
            properties.insert(
                "FileSystemTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_policies {
            properties.insert(
                "LifecyclePolicies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.performance_mode {
            properties.insert(
                "PerformanceMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.provisioned_throughput_in_mibps {
            properties.insert(
                "ProvisionedThroughputInMibps".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_configuration {
            properties.insert(
                "ReplicationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.throughput_mode {
            properties.insert(
                "ThroughputMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-efs-mounttarget.html>
pub struct MountTarget_ {
    pub file_system_id: crate::value::ExpString,
    pub ip_address: Option<crate::value::ExpString>,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub ipv6_address: Option<crate::value::ExpString>,
    pub security_groups: Vec<crate::value::ExpString>,
    pub subnet_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_efs_MountTarget {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EFS::MountTarget"
        $($field $value)*)
    };
}
pub use crate::__aws_efs_MountTarget as MountTarget;
impl crate::template::ToResource for MountTarget_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EFS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MountTarget"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "FileSystemId".to_string(),
            crate::value::ToValue::to_value(&self.file_system_id),
        );
        if let Some(ref value) = self.ip_address {
            properties.insert(
                "IpAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ipv6_address {
            properties.insert(
                "Ipv6Address".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecurityGroups".to_string(),
            crate::value::ToValue::to_value(&self.security_groups),
        );
        properties.insert(
            "SubnetId".to_string(),
            crate::value::ToValue::to_value(&self.subnet_id),
        );
        properties
    }
}
