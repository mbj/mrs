pub mod accesspoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-accesspoint-publicaccessblockconfiguration.html
    pub struct PublicAccessBlockConfiguration_ {
        pub block_public_acls: Option<crate::value::ExpBool>,
        pub block_public_policy: Option<crate::value::ExpBool>,
        pub ignore_public_acls: Option<crate::value::ExpBool>,
        pub restrict_public_buckets: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_AccessPoint_PublicAccessBlockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::AccessPoint.PublicAccessBlockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_AccessPoint_PublicAccessBlockConfiguration as PublicAccessBlockConfiguration;
    impl crate::value::ToValue for PublicAccessBlockConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_public_acls {
                properties.insert(
                    "BlockPublicAcls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.block_public_policy {
                properties.insert(
                    "BlockPublicPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ignore_public_acls {
                properties.insert(
                    "IgnorePublicAcls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.restrict_public_buckets {
                properties.insert(
                    "RestrictPublicBuckets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-accesspoint-scope.html
    pub struct Scope_ {
        pub permissions: Option<Vec<crate::value::ExpString>>,
        pub prefixes: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_AccessPoint_Scope {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::AccessPoint.Scope"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_AccessPoint_Scope as Scope;
    impl crate::value::ToValue for Scope_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.permissions {
                properties.insert(
                    "Permissions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefixes {
                properties.insert(
                    "Prefixes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-accesspoint-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_AccessPoint_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::AccessPoint.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_AccessPoint_VpcConfiguration as VpcConfiguration;
    impl crate::value::ToValue for VpcConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod directorybucket {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-directorybucket-abortincompletemultipartupload.html
    pub struct AbortIncompleteMultipartUpload_ {
        pub days_after_initiation: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_DirectoryBucket_AbortIncompleteMultipartUpload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::DirectoryBucket.AbortIncompleteMultipartUpload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_DirectoryBucket_AbortIncompleteMultipartUpload as AbortIncompleteMultipartUpload;
    impl crate::value::ToValue for AbortIncompleteMultipartUpload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DaysAfterInitiation".to_string(),
                crate::value::ToValue::to_value(&self.days_after_initiation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-directorybucket-bucketencryption.html
    pub struct BucketEncryption_ {
        pub server_side_encryption_configuration: Vec<ServerSideEncryptionRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_DirectoryBucket_BucketEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::DirectoryBucket.BucketEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_DirectoryBucket_BucketEncryption as BucketEncryption;
    impl crate::value::ToValue for BucketEncryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ServerSideEncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.server_side_encryption_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-directorybucket-lifecycleconfiguration.html
    pub struct LifecycleConfiguration_ {
        pub rules: Vec<Rule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_DirectoryBucket_LifecycleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::DirectoryBucket.LifecycleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_DirectoryBucket_LifecycleConfiguration as LifecycleConfiguration;
    impl crate::value::ToValue for LifecycleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-directorybucket-rule.html
    pub struct Rule_ {
        pub abort_incomplete_multipart_upload: Option<Box<AbortIncompleteMultipartUpload_>>,
        pub expiration_in_days: Option<i64>,
        pub id: Option<crate::value::ExpString>,
        pub object_size_greater_than: Option<crate::value::ExpString>,
        pub object_size_less_than: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_DirectoryBucket_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::DirectoryBucket.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_DirectoryBucket_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.abort_incomplete_multipart_upload {
                properties.insert(
                    "AbortIncompleteMultipartUpload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expiration_in_days {
                properties.insert(
                    "ExpirationInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.object_size_greater_than {
                properties.insert(
                    "ObjectSizeGreaterThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.object_size_less_than {
                properties.insert(
                    "ObjectSizeLessThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-directorybucket-serversideencryptionbydefault.html
    pub struct ServerSideEncryptionByDefault_ {
        pub kms_master_key_id: Option<crate::value::ExpString>,
        pub sse_algorithm: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_DirectoryBucket_ServerSideEncryptionByDefault {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::DirectoryBucket.ServerSideEncryptionByDefault"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_DirectoryBucket_ServerSideEncryptionByDefault as ServerSideEncryptionByDefault;
    impl crate::value::ToValue for ServerSideEncryptionByDefault_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_master_key_id {
                properties.insert(
                    "KMSMasterKeyID".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SSEAlgorithm".to_string(),
                crate::value::ToValue::to_value(&self.sse_algorithm),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3express-directorybucket-serversideencryptionrule.html
    pub struct ServerSideEncryptionRule_ {
        pub bucket_key_enabled: Option<crate::value::ExpBool>,
        pub server_side_encryption_by_default: Option<Box<ServerSideEncryptionByDefault_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3express_DirectoryBucket_ServerSideEncryptionRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Express::DirectoryBucket.ServerSideEncryptionRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3express_DirectoryBucket_ServerSideEncryptionRule as ServerSideEncryptionRule;
    impl crate::value::ToValue for ServerSideEncryptionRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_key_enabled {
                properties.insert(
                    "BucketKeyEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.server_side_encryption_by_default {
                properties.insert(
                    "ServerSideEncryptionByDefault".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3express-accesspoint.html
pub struct AccessPoint_ {
    pub bucket: crate::value::ExpString,
    pub bucket_account_id: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub policy: Option<serde_json::Value>,
    pub public_access_block_configuration:
        Option<super::s3express::accesspoint::PublicAccessBlockConfiguration_>,
    pub scope: Option<super::s3express::accesspoint::Scope_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_configuration: Option<super::s3express::accesspoint::VpcConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3express_AccessPoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Express::AccessPoint"
        $($field $value)*)
    };
}
pub use crate::__aws_s3express_AccessPoint as AccessPoint;
impl crate::template::ToResource for AccessPoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Express"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessPoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Bucket".to_string(),
            crate::value::ToValue::to_value(&self.bucket),
        );
        if let Some(ref value) = self.bucket_account_id {
            properties.insert(
                "BucketAccountId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.policy {
            properties.insert("Policy".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.public_access_block_configuration {
            properties.insert(
                "PublicAccessBlockConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scope {
            properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_configuration {
            properties.insert(
                "VpcConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3express-bucketpolicy.html
pub struct BucketPolicy_ {
    pub bucket: crate::value::ExpString,
    pub policy_document: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3express_BucketPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Express::BucketPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3express_BucketPolicy as BucketPolicy;
impl crate::template::ToResource for BucketPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Express"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BucketPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Bucket".to_string(),
            crate::value::ToValue::to_value(&self.bucket),
        );
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3express-directorybucket.html
pub struct DirectoryBucket_ {
    pub bucket_encryption: Option<super::s3express::directorybucket::BucketEncryption_>,
    pub bucket_name: Option<crate::value::ExpString>,
    pub data_redundancy: crate::value::ExpString,
    pub lifecycle_configuration: Option<super::s3express::directorybucket::LifecycleConfiguration_>,
    pub location_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3express_DirectoryBucket {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Express::DirectoryBucket"
        $($field $value)*)
    };
}
pub use crate::__aws_s3express_DirectoryBucket as DirectoryBucket;
impl crate::template::ToResource for DirectoryBucket_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Express"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DirectoryBucket"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bucket_encryption {
            properties.insert(
                "BucketEncryption".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bucket_name {
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataRedundancy".to_string(),
            crate::value::ToValue::to_value(&self.data_redundancy),
        );
        if let Some(ref value) = self.lifecycle_configuration {
            properties.insert(
                "LifecycleConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LocationName".to_string(),
            crate::value::ToValue::to_value(&self.location_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
