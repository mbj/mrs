pub mod locationazureblob {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationazureblob-azureblobsasconfiguration.html
    pub struct AzureBlobSasConfiguration_ {
        pub azure_blob_sas_token: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationAzureBlob_AzureBlobSasConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationAzureBlob.AzureBlobSasConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationAzureBlob_AzureBlobSasConfiguration as AzureBlobSasConfiguration;
    impl crate::value::ToValue for AzureBlobSasConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AzureBlobSasToken".to_string(),
                crate::value::ToValue::to_value(&self.azure_blob_sas_token),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationazureblob-cmksecretconfig.html
    pub struct CmkSecretConfig_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationAzureBlob_CmkSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationAzureBlob.CmkSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationAzureBlob_CmkSecretConfig as CmkSecretConfig;
    impl crate::value::ToValue for CmkSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationazureblob-customsecretconfig.html
    pub struct CustomSecretConfig_ {
        pub secret_access_role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationAzureBlob_CustomSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationAzureBlob.CustomSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationAzureBlob_CustomSecretConfig as CustomSecretConfig;
    impl crate::value::ToValue for CustomSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretAccessRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_access_role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationazureblob-managedsecretconfig.html
    pub struct ManagedSecretConfig_ {
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationAzureBlob_ManagedSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationAzureBlob.ManagedSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationAzureBlob_ManagedSecretConfig as ManagedSecretConfig;
    impl crate::value::ToValue for ManagedSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
}
pub mod locationefs {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationefs-ec2config.html
    pub struct Ec2Config_ {
        pub security_group_arns: Vec<crate::value::ExpString>,
        pub subnet_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationEFS_Ec2Config {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationEFS.Ec2Config"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationEFS_Ec2Config as Ec2Config;
    impl crate::value::ToValue for Ec2Config_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecurityGroupArns".to_string(),
                crate::value::ToValue::to_value(&self.security_group_arns),
            );
            properties.insert(
                "SubnetArn".to_string(),
                crate::value::ToValue::to_value(&self.subnet_arn),
            );
            properties.into()
        }
    }
}
pub mod locationfsxontap {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-nfs.html
    pub struct NFS_ {
        pub mount_options: Box<NfsMountOptions_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxONTAP_NFS {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxONTAP.NFS"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxONTAP_NFS as NFS;
    impl crate::value::ToValue for NFS_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MountOptions".to_string(),
                crate::value::ToValue::to_value(&self.mount_options),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-nfsmountoptions.html
    pub struct NfsMountOptions_ {
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxONTAP_NfsMountOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxONTAP.NfsMountOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxONTAP_NfsMountOptions as NfsMountOptions;
    impl crate::value::ToValue for NfsMountOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-protocol.html
    pub struct Protocol_ {
        pub nfs: Option<Box<NFS_>>,
        pub smb: Option<Box<SMB_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxONTAP_Protocol {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxONTAP.Protocol"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxONTAP_Protocol as Protocol;
    impl crate::value::ToValue for Protocol_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.nfs {
                properties.insert("NFS".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.smb {
                properties.insert("SMB".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smb.html
    pub struct SMB_ {
        pub domain: Option<crate::value::ExpString>,
        pub mount_options: Box<SmbMountOptions_>,
        pub password: crate::value::ExpString,
        pub user: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxONTAP_SMB {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxONTAP.SMB"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxONTAP_SMB as SMB;
    impl crate::value::ToValue for SMB_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain {
                properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "MountOptions".to_string(),
                crate::value::ToValue::to_value(&self.mount_options),
            );
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(&self.password),
            );
            properties.insert(
                "User".to_string(),
                crate::value::ToValue::to_value(&self.user),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxontap-smbmountoptions.html
    pub struct SmbMountOptions_ {
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxONTAP_SmbMountOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxONTAP.SmbMountOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxONTAP_SmbMountOptions as SmbMountOptions;
    impl crate::value::ToValue for SmbMountOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod locationfsxopenzfs {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-mountoptions.html
    pub struct MountOptions_ {
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxOpenZFS_MountOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxOpenZFS.MountOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxOpenZFS_MountOptions as MountOptions;
    impl crate::value::ToValue for MountOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-nfs.html
    pub struct NFS_ {
        pub mount_options: Box<MountOptions_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxOpenZFS_NFS {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxOpenZFS.NFS"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxOpenZFS_NFS as NFS;
    impl crate::value::ToValue for NFS_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MountOptions".to_string(),
                crate::value::ToValue::to_value(&self.mount_options),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationfsxopenzfs-protocol.html
    pub struct Protocol_ {
        pub nfs: Option<Box<NFS_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationFSxOpenZFS_Protocol {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationFSxOpenZFS.Protocol"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationFSxOpenZFS_Protocol as Protocol;
    impl crate::value::ToValue for Protocol_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.nfs {
                properties.insert("NFS".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod locationhdfs {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-namenode.html
    pub struct NameNode_ {
        pub hostname: crate::value::ExpString,
        pub port: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationHDFS_NameNode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationHDFS.NameNode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationHDFS_NameNode as NameNode;
    impl crate::value::ToValue for NameNode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Hostname".to_string(),
                crate::value::ToValue::to_value(&self.hostname),
            );
            properties.insert(
                "Port".to_string(),
                crate::value::ToValue::to_value(&self.port),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationhdfs-qopconfiguration.html
    pub struct QopConfiguration_ {
        pub data_transfer_protection: Option<crate::value::ExpString>,
        pub rpc_protection: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationHDFS_QopConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationHDFS.QopConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationHDFS_QopConfiguration as QopConfiguration;
    impl crate::value::ToValue for QopConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_transfer_protection {
                properties.insert(
                    "DataTransferProtection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rpc_protection {
                properties.insert(
                    "RpcProtection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod locationnfs {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationnfs-mountoptions.html
    pub struct MountOptions_ {
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationNFS_MountOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationNFS.MountOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationNFS_MountOptions as MountOptions;
    impl crate::value::ToValue for MountOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationnfs-onpremconfig.html
    pub struct OnPremConfig_ {
        pub agent_arns: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationNFS_OnPremConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationNFS.OnPremConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationNFS_OnPremConfig as OnPremConfig;
    impl crate::value::ToValue for OnPremConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentArns".to_string(),
                crate::value::ToValue::to_value(&self.agent_arns),
            );
            properties.into()
        }
    }
}
pub mod locationobjectstorage {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationobjectstorage-cmksecretconfig.html
    pub struct CmkSecretConfig_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationObjectStorage_CmkSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationObjectStorage.CmkSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationObjectStorage_CmkSecretConfig as CmkSecretConfig;
    impl crate::value::ToValue for CmkSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationobjectstorage-customsecretconfig.html
    pub struct CustomSecretConfig_ {
        pub secret_access_role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationObjectStorage_CustomSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationObjectStorage.CustomSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationObjectStorage_CustomSecretConfig as CustomSecretConfig;
    impl crate::value::ToValue for CustomSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretAccessRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_access_role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationobjectstorage-managedsecretconfig.html
    pub struct ManagedSecretConfig_ {
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationObjectStorage_ManagedSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationObjectStorage.ManagedSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationObjectStorage_ManagedSecretConfig as ManagedSecretConfig;
    impl crate::value::ToValue for ManagedSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
}
pub mod locations3 {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locations3-s3config.html
    pub struct S3Config_ {
        pub bucket_access_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationS3_S3Config {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationS3.S3Config"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationS3_S3Config as S3Config;
    impl crate::value::ToValue for S3Config_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketAccessRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.bucket_access_role_arn),
            );
            properties.into()
        }
    }
}
pub mod locationsmb {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationsmb-cmksecretconfig.html
    pub struct CmkSecretConfig_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationSMB_CmkSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationSMB.CmkSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationSMB_CmkSecretConfig as CmkSecretConfig;
    impl crate::value::ToValue for CmkSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationsmb-customsecretconfig.html
    pub struct CustomSecretConfig_ {
        pub secret_access_role_arn: crate::value::ExpString,
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationSMB_CustomSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationSMB.CustomSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationSMB_CustomSecretConfig as CustomSecretConfig;
    impl crate::value::ToValue for CustomSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretAccessRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_access_role_arn),
            );
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationsmb-managedsecretconfig.html
    pub struct ManagedSecretConfig_ {
        pub secret_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationSMB_ManagedSecretConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationSMB.ManagedSecretConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationSMB_ManagedSecretConfig as ManagedSecretConfig;
    impl crate::value::ToValue for ManagedSecretConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SecretArn".to_string(),
                crate::value::ToValue::to_value(&self.secret_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-locationsmb-mountoptions.html
    pub struct MountOptions_ {
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_LocationSMB_MountOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::LocationSMB.MountOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_LocationSMB_MountOptions as MountOptions;
    impl crate::value::ToValue for MountOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod task {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-deleted.html
    pub struct Deleted_ {
        pub report_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Deleted {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Deleted"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Deleted as Deleted;
    impl crate::value::ToValue for Deleted_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.report_level {
                properties.insert(
                    "ReportLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-destination.html
    pub struct Destination_ {
        pub s3: Option<Box<TaskReportConfigDestinationS3_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Destination as Destination;
    impl crate::value::ToValue for Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-filterrule.html
    pub struct FilterRule_ {
        pub filter_type: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_FilterRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.FilterRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_FilterRule as FilterRule;
    impl crate::value::ToValue for FilterRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filter_type {
                properties.insert(
                    "FilterType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-manifestconfig.html
    pub struct ManifestConfig_ {
        pub action: Option<crate::value::ExpString>,
        pub format: Option<crate::value::ExpString>,
        pub source: Box<Source_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_ManifestConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.ManifestConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_ManifestConfig as ManifestConfig;
    impl crate::value::ToValue for ManifestConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.format {
                properties.insert("Format".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-manifestconfigsources3.html
    pub struct ManifestConfigSourceS3_ {
        pub bucket_access_role_arn: Option<crate::value::ExpString>,
        pub manifest_object_path: Option<crate::value::ExpString>,
        pub manifest_object_version_id: Option<crate::value::ExpString>,
        pub s3_bucket_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_ManifestConfigSourceS3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.ManifestConfigSourceS3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_ManifestConfigSourceS3 as ManifestConfigSourceS3;
    impl crate::value::ToValue for ManifestConfigSourceS3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_access_role_arn {
                properties.insert(
                    "BucketAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_object_path {
                properties.insert(
                    "ManifestObjectPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manifest_object_version_id {
                properties.insert(
                    "ManifestObjectVersionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_arn {
                properties.insert(
                    "S3BucketArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-options.html
    pub struct Options_ {
        pub atime: Option<crate::value::ExpString>,
        pub bytes_per_second: Option<i64>,
        pub gid: Option<crate::value::ExpString>,
        pub log_level: Option<crate::value::ExpString>,
        pub mtime: Option<crate::value::ExpString>,
        pub object_tags: Option<crate::value::ExpString>,
        pub overwrite_mode: Option<crate::value::ExpString>,
        pub posix_permissions: Option<crate::value::ExpString>,
        pub preserve_deleted_files: Option<crate::value::ExpString>,
        pub preserve_devices: Option<crate::value::ExpString>,
        pub security_descriptor_copy_flags: Option<crate::value::ExpString>,
        pub task_queueing: Option<crate::value::ExpString>,
        pub transfer_mode: Option<crate::value::ExpString>,
        pub uid: Option<crate::value::ExpString>,
        pub verify_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Options {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Options"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Options as Options;
    impl crate::value::ToValue for Options_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.atime {
                properties.insert("Atime".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.bytes_per_second {
                properties.insert(
                    "BytesPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gid {
                properties.insert("Gid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.log_level {
                properties.insert(
                    "LogLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mtime {
                properties.insert("Mtime".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.object_tags {
                properties.insert(
                    "ObjectTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.overwrite_mode {
                properties.insert(
                    "OverwriteMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.posix_permissions {
                properties.insert(
                    "PosixPermissions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preserve_deleted_files {
                properties.insert(
                    "PreserveDeletedFiles".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preserve_devices {
                properties.insert(
                    "PreserveDevices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_descriptor_copy_flags {
                properties.insert(
                    "SecurityDescriptorCopyFlags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_queueing {
                properties.insert(
                    "TaskQueueing".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transfer_mode {
                properties.insert(
                    "TransferMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uid {
                properties.insert("Uid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.verify_mode {
                properties.insert(
                    "VerifyMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-overrides.html
    pub struct Overrides_ {
        pub deleted: Option<Box<Deleted_>>,
        pub skipped: Option<Box<Skipped_>>,
        pub transferred: Option<Box<Transferred_>>,
        pub verified: Option<Box<Verified_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Overrides {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Overrides"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Overrides as Overrides;
    impl crate::value::ToValue for Overrides_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deleted {
                properties.insert(
                    "Deleted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skipped {
                properties.insert(
                    "Skipped".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transferred {
                properties.insert(
                    "Transferred".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.verified {
                properties.insert(
                    "Verified".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-skipped.html
    pub struct Skipped_ {
        pub report_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Skipped {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Skipped"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Skipped as Skipped;
    impl crate::value::ToValue for Skipped_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.report_level {
                properties.insert(
                    "ReportLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-source.html
    pub struct Source_ {
        pub s3: Option<Box<ManifestConfigSourceS3_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfig.html
    pub struct TaskReportConfig_ {
        pub destination: Box<Destination_>,
        pub object_version_ids: Option<crate::value::ExpString>,
        pub output_type: crate::value::ExpString,
        pub overrides: Option<Box<Overrides_>>,
        pub report_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_TaskReportConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.TaskReportConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_TaskReportConfig as TaskReportConfig;
    impl crate::value::ToValue for TaskReportConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            if let Some(ref value) = self.object_version_ids {
                properties.insert(
                    "ObjectVersionIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutputType".to_string(),
                crate::value::ToValue::to_value(&self.output_type),
            );
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.report_level {
                properties.insert(
                    "ReportLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskreportconfigdestinations3.html
    pub struct TaskReportConfigDestinationS3_ {
        pub bucket_access_role_arn: Option<crate::value::ExpString>,
        pub s3_bucket_arn: Option<crate::value::ExpString>,
        pub subdirectory: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_TaskReportConfigDestinationS3 {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.TaskReportConfigDestinationS3"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_TaskReportConfigDestinationS3 as TaskReportConfigDestinationS3;
    impl crate::value::ToValue for TaskReportConfigDestinationS3_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_access_role_arn {
                properties.insert(
                    "BucketAccessRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_arn {
                properties.insert(
                    "S3BucketArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subdirectory {
                properties.insert(
                    "Subdirectory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-taskschedule.html
    pub struct TaskSchedule_ {
        pub schedule_expression: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_TaskSchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.TaskSchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_TaskSchedule as TaskSchedule;
    impl crate::value::ToValue for TaskSchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.schedule_expression {
                properties.insert(
                    "ScheduleExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-transferred.html
    pub struct Transferred_ {
        pub report_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Transferred {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Transferred"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Transferred as Transferred;
    impl crate::value::ToValue for Transferred_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.report_level {
                properties.insert(
                    "ReportLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datasync-task-verified.html
    pub struct Verified_ {
        pub report_level: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datasync_Task_Verified {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DataSync::Task.Verified"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datasync_Task_Verified as Verified;
    impl crate::value::ToValue for Verified_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.report_level {
                properties.insert(
                    "ReportLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-agent.html
pub struct Agent_ {
    pub activation_key: Option<crate::value::ExpString>,
    pub agent_name: Option<crate::value::ExpString>,
    pub security_group_arns: Option<Vec<crate::value::ExpString>>,
    pub subnet_arns: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_endpoint_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_Agent {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::Agent" $($field
        $value)*)
    };
}
pub use crate::__aws_datasync_Agent as Agent;
impl crate::template::ToResource for Agent_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Agent"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.activation_key {
            properties.insert(
                "ActivationKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.agent_name {
            properties.insert(
                "AgentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_arns {
            properties.insert(
                "SecurityGroupArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_arns {
            properties.insert(
                "SubnetArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_endpoint_id {
            properties.insert(
                "VpcEndpointId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationazureblob.html
pub struct LocationAzureBlob_ {
    pub agent_arns: Option<Vec<crate::value::ExpString>>,
    pub azure_access_tier: Option<crate::value::ExpString>,
    pub azure_blob_authentication_type: crate::value::ExpString,
    pub azure_blob_container_url: Option<crate::value::ExpString>,
    pub azure_blob_sas_configuration:
        Option<super::datasync::locationazureblob::AzureBlobSasConfiguration_>,
    pub azure_blob_type: Option<crate::value::ExpString>,
    pub cmk_secret_config: Option<super::datasync::locationazureblob::CmkSecretConfig_>,
    pub custom_secret_config: Option<super::datasync::locationazureblob::CustomSecretConfig_>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationAzureBlob {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationAzureBlob"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationAzureBlob as LocationAzureBlob;
impl crate::template::ToResource for LocationAzureBlob_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationAzureBlob"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.agent_arns {
            properties.insert(
                "AgentArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.azure_access_tier {
            properties.insert(
                "AzureAccessTier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AzureBlobAuthenticationType".to_string(),
            crate::value::ToValue::to_value(&self.azure_blob_authentication_type),
        );
        if let Some(ref value) = self.azure_blob_container_url {
            properties.insert(
                "AzureBlobContainerUrl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.azure_blob_sas_configuration {
            properties.insert(
                "AzureBlobSasConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.azure_blob_type {
            properties.insert(
                "AzureBlobType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cmk_secret_config {
            properties.insert(
                "CmkSecretConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_secret_config {
            properties.insert(
                "CustomSecretConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationefs.html
pub struct LocationEFS_ {
    pub access_point_arn: Option<crate::value::ExpString>,
    pub ec2_config: super::datasync::locationefs::Ec2Config_,
    pub efs_filesystem_arn: Option<crate::value::ExpString>,
    pub file_system_access_role_arn: Option<crate::value::ExpString>,
    pub in_transit_encryption: Option<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationEFS {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationEFS"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationEFS as LocationEFS;
impl crate::template::ToResource for LocationEFS_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationEFS"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_point_arn {
            properties.insert(
                "AccessPointArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Ec2Config".to_string(),
            crate::value::ToValue::to_value(&self.ec2_config),
        );
        if let Some(ref value) = self.efs_filesystem_arn {
            properties.insert(
                "EfsFilesystemArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.file_system_access_role_arn {
            properties.insert(
                "FileSystemAccessRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.in_transit_encryption {
            properties.insert(
                "InTransitEncryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxlustre.html
pub struct LocationFSxLustre_ {
    pub fsx_filesystem_arn: Option<crate::value::ExpString>,
    pub security_group_arns: Vec<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationFSxLustre {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationFSxLustre"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationFSxLustre as LocationFSxLustre;
impl crate::template::ToResource for LocationFSxLustre_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationFSxLustre"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.fsx_filesystem_arn {
            properties.insert(
                "FsxFilesystemArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecurityGroupArns".to_string(),
            crate::value::ToValue::to_value(&self.security_group_arns),
        );
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxontap.html
pub struct LocationFSxONTAP_ {
    pub protocol: Option<super::datasync::locationfsxontap::Protocol_>,
    pub security_group_arns: Vec<crate::value::ExpString>,
    pub storage_virtual_machine_arn: crate::value::ExpString,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationFSxONTAP {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationFSxONTAP"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationFSxONTAP as LocationFSxONTAP;
impl crate::template::ToResource for LocationFSxONTAP_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationFSxONTAP"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.protocol {
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecurityGroupArns".to_string(),
            crate::value::ToValue::to_value(&self.security_group_arns),
        );
        properties.insert(
            "StorageVirtualMachineArn".to_string(),
            crate::value::ToValue::to_value(&self.storage_virtual_machine_arn),
        );
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxopenzfs.html
pub struct LocationFSxOpenZFS_ {
    pub fsx_filesystem_arn: Option<crate::value::ExpString>,
    pub protocol: super::datasync::locationfsxopenzfs::Protocol_,
    pub security_group_arns: Vec<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationFSxOpenZFS {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationFSxOpenZFS"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationFSxOpenZFS as LocationFSxOpenZFS;
impl crate::template::ToResource for LocationFSxOpenZFS_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationFSxOpenZFS"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.fsx_filesystem_arn {
            properties.insert(
                "FsxFilesystemArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Protocol".to_string(),
            crate::value::ToValue::to_value(&self.protocol),
        );
        properties.insert(
            "SecurityGroupArns".to_string(),
            crate::value::ToValue::to_value(&self.security_group_arns),
        );
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationfsxwindows.html
pub struct LocationFSxWindows_ {
    pub domain: Option<crate::value::ExpString>,
    pub fsx_filesystem_arn: Option<crate::value::ExpString>,
    pub password: Option<crate::value::ExpString>,
    pub security_group_arns: Vec<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationFSxWindows {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationFSxWindows"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationFSxWindows as LocationFSxWindows;
impl crate::template::ToResource for LocationFSxWindows_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationFSxWindows"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.fsx_filesystem_arn {
            properties.insert(
                "FsxFilesystemArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.password {
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecurityGroupArns".to_string(),
            crate::value::ToValue::to_value(&self.security_group_arns),
        );
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "User".to_string(),
            crate::value::ToValue::to_value(&self.user),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationhdfs.html
pub struct LocationHDFS_ {
    pub agent_arns: Vec<crate::value::ExpString>,
    pub authentication_type: crate::value::ExpString,
    pub block_size: Option<i64>,
    pub kerberos_keytab: Option<crate::value::ExpString>,
    pub kerberos_krb5_conf: Option<crate::value::ExpString>,
    pub kerberos_principal: Option<crate::value::ExpString>,
    pub kms_key_provider_uri: Option<crate::value::ExpString>,
    pub name_nodes: Vec<super::datasync::locationhdfs::NameNode_>,
    pub qop_configuration: Option<super::datasync::locationhdfs::QopConfiguration_>,
    pub replication_factor: Option<i64>,
    pub simple_user: Option<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationHDFS {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationHDFS"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationHDFS as LocationHDFS;
impl crate::template::ToResource for LocationHDFS_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationHDFS"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AgentArns".to_string(),
            crate::value::ToValue::to_value(&self.agent_arns),
        );
        properties.insert(
            "AuthenticationType".to_string(),
            crate::value::ToValue::to_value(&self.authentication_type),
        );
        if let Some(ref value) = self.block_size {
            properties.insert(
                "BlockSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kerberos_keytab {
            properties.insert(
                "KerberosKeytab".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kerberos_krb5_conf {
            properties.insert(
                "KerberosKrb5Conf".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kerberos_principal {
            properties.insert(
                "KerberosPrincipal".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_provider_uri {
            properties.insert(
                "KmsKeyProviderUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NameNodes".to_string(),
            crate::value::ToValue::to_value(&self.name_nodes),
        );
        if let Some(ref value) = self.qop_configuration {
            properties.insert(
                "QopConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_factor {
            properties.insert(
                "ReplicationFactor".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.simple_user {
            properties.insert(
                "SimpleUser".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationnfs.html
pub struct LocationNFS_ {
    pub mount_options: Option<super::datasync::locationnfs::MountOptions_>,
    pub on_prem_config: super::datasync::locationnfs::OnPremConfig_,
    pub server_hostname: Option<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationNFS {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationNFS"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationNFS as LocationNFS;
impl crate::template::ToResource for LocationNFS_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationNFS"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.mount_options {
            properties.insert(
                "MountOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OnPremConfig".to_string(),
            crate::value::ToValue::to_value(&self.on_prem_config),
        );
        if let Some(ref value) = self.server_hostname {
            properties.insert(
                "ServerHostname".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationobjectstorage.html
pub struct LocationObjectStorage_ {
    pub access_key: Option<crate::value::ExpString>,
    pub agent_arns: Option<Vec<crate::value::ExpString>>,
    pub bucket_name: Option<crate::value::ExpString>,
    pub cmk_secret_config: Option<super::datasync::locationobjectstorage::CmkSecretConfig_>,
    pub custom_secret_config: Option<super::datasync::locationobjectstorage::CustomSecretConfig_>,
    pub secret_key: Option<crate::value::ExpString>,
    pub server_certificate: Option<crate::value::ExpString>,
    pub server_hostname: Option<crate::value::ExpString>,
    pub server_port: Option<i32>,
    pub server_protocol: Option<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationObjectStorage {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationObjectStorage"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationObjectStorage as LocationObjectStorage;
impl crate::template::ToResource for LocationObjectStorage_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationObjectStorage"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_key {
            properties.insert(
                "AccessKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.agent_arns {
            properties.insert(
                "AgentArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bucket_name {
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cmk_secret_config {
            properties.insert(
                "CmkSecretConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_secret_config {
            properties.insert(
                "CustomSecretConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.secret_key {
            properties.insert(
                "SecretKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_certificate {
            properties.insert(
                "ServerCertificate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_hostname {
            properties.insert(
                "ServerHostname".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_port {
            properties.insert(
                "ServerPort".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_protocol {
            properties.insert(
                "ServerProtocol".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locations3.html
pub struct LocationS3_ {
    pub s3_bucket_arn: Option<crate::value::ExpString>,
    pub s3_config: super::datasync::locations3::S3Config_,
    pub s3_storage_class: Option<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationS3 {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationS3"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationS3 as LocationS3;
impl crate::template::ToResource for LocationS3_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationS3"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.s3_bucket_arn {
            properties.insert(
                "S3BucketArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "S3Config".to_string(),
            crate::value::ToValue::to_value(&self.s3_config),
        );
        if let Some(ref value) = self.s3_storage_class {
            properties.insert(
                "S3StorageClass".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-locationsmb.html
pub struct LocationSMB_ {
    pub agent_arns: Vec<crate::value::ExpString>,
    pub authentication_type: Option<crate::value::ExpString>,
    pub cmk_secret_config: Option<super::datasync::locationsmb::CmkSecretConfig_>,
    pub custom_secret_config: Option<super::datasync::locationsmb::CustomSecretConfig_>,
    pub dns_ip_addresses: Option<Vec<crate::value::ExpString>>,
    pub domain: Option<crate::value::ExpString>,
    pub kerberos_keytab: Option<crate::value::ExpString>,
    pub kerberos_krb5_conf: Option<crate::value::ExpString>,
    pub kerberos_principal: Option<crate::value::ExpString>,
    pub mount_options: Option<super::datasync::locationsmb::MountOptions_>,
    pub password: Option<crate::value::ExpString>,
    pub server_hostname: Option<crate::value::ExpString>,
    pub subdirectory: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_LocationSMB {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::LocationSMB"
        $($field $value)*)
    };
}
pub use crate::__aws_datasync_LocationSMB as LocationSMB;
impl crate::template::ToResource for LocationSMB_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LocationSMB"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AgentArns".to_string(),
            crate::value::ToValue::to_value(&self.agent_arns),
        );
        if let Some(ref value) = self.authentication_type {
            properties.insert(
                "AuthenticationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cmk_secret_config {
            properties.insert(
                "CmkSecretConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_secret_config {
            properties.insert(
                "CustomSecretConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dns_ip_addresses {
            properties.insert(
                "DnsIpAddresses".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kerberos_keytab {
            properties.insert(
                "KerberosKeytab".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kerberos_krb5_conf {
            properties.insert(
                "KerberosKrb5Conf".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kerberos_principal {
            properties.insert(
                "KerberosPrincipal".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mount_options {
            properties.insert(
                "MountOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.password {
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_hostname {
            properties.insert(
                "ServerHostname".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subdirectory {
            properties.insert(
                "Subdirectory".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user {
            properties.insert("User".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datasync-task.html
pub struct Task_ {
    pub cloud_watch_log_group_arn: Option<crate::value::ExpString>,
    pub destination_location_arn: crate::value::ExpString,
    pub excludes: Option<Vec<super::datasync::task::FilterRule_>>,
    pub includes: Option<Vec<super::datasync::task::FilterRule_>>,
    pub manifest_config: Option<super::datasync::task::ManifestConfig_>,
    pub name: Option<crate::value::ExpString>,
    pub options: Option<super::datasync::task::Options_>,
    pub schedule: Option<super::datasync::task::TaskSchedule_>,
    pub source_location_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub task_mode: Option<crate::value::ExpString>,
    pub task_report_config: Option<super::datasync::task::TaskReportConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datasync_Task {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DataSync::Task" $($field
        $value)*)
    };
}
pub use crate::__aws_datasync_Task as Task;
impl crate::template::ToResource for Task_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataSync"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Task"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cloud_watch_log_group_arn {
            properties.insert(
                "CloudWatchLogGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DestinationLocationArn".to_string(),
            crate::value::ToValue::to_value(&self.destination_location_arn),
        );
        if let Some(ref value) = self.excludes {
            properties.insert(
                "Excludes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.includes {
            properties.insert(
                "Includes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manifest_config {
            properties.insert(
                "ManifestConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.options {
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SourceLocationArn".to_string(),
            crate::value::ToValue::to_value(&self.source_location_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.task_mode {
            properties.insert(
                "TaskMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.task_report_config {
            properties.insert(
                "TaskReportConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
