pub mod accessgrant {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-accessgrant-accessgrantslocationconfiguration.html
    pub struct AccessGrantsLocationConfiguration_ {
        pub s3_sub_prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_AccessGrant_AccessGrantsLocationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::AccessGrant.AccessGrantsLocationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_AccessGrant_AccessGrantsLocationConfiguration as AccessGrantsLocationConfiguration;
    impl crate::value::ToValue for AccessGrantsLocationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3SubPrefix".to_string(),
                crate::value::ToValue::to_value(&self.s3_sub_prefix),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-accessgrant-grantee.html
    pub struct Grantee_ {
        pub grantee_identifier: crate::value::ExpString,
        pub grantee_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_AccessGrant_Grantee {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::AccessGrant.Grantee"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_AccessGrant_Grantee as Grantee;
    impl crate::value::ToValue for Grantee_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GranteeIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.grantee_identifier),
            );
            properties.insert(
                "GranteeType".to_string(),
                crate::value::ToValue::to_value(&self.grantee_type),
            );
            properties.into()
        }
    }
}
pub mod accesspoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-accesspoint-publicaccessblockconfiguration.html
    pub struct PublicAccessBlockConfiguration_ {
        pub block_public_acls: Option<crate::value::ExpBool>,
        pub block_public_policy: Option<crate::value::ExpBool>,
        pub ignore_public_acls: Option<crate::value::ExpBool>,
        pub restrict_public_buckets: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_AccessPoint_PublicAccessBlockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::AccessPoint.PublicAccessBlockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_AccessPoint_PublicAccessBlockConfiguration as PublicAccessBlockConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-accesspoint-vpcconfiguration.html
    pub struct VpcConfiguration_ {
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_AccessPoint_VpcConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::AccessPoint.VpcConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_AccessPoint_VpcConfiguration as VpcConfiguration;
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
pub mod bucket {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-abortincompletemultipartupload.html
    pub struct AbortIncompleteMultipartUpload_ {
        pub days_after_initiation: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_AbortIncompleteMultipartUpload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.AbortIncompleteMultipartUpload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_AbortIncompleteMultipartUpload as AbortIncompleteMultipartUpload;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accelerateconfiguration.html
    pub struct AccelerateConfiguration_ {
        pub acceleration_status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_AccelerateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.AccelerateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_AccelerateConfiguration as AccelerateConfiguration;
    impl crate::value::ToValue for AccelerateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccelerationStatus".to_string(),
                crate::value::ToValue::to_value(&self.acceleration_status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-accesscontroltranslation.html
    pub struct AccessControlTranslation_ {
        pub owner: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_AccessControlTranslation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.AccessControlTranslation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_AccessControlTranslation as AccessControlTranslation;
    impl crate::value::ToValue for AccessControlTranslation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Owner".to_string(),
                crate::value::ToValue::to_value(&self.owner),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-analyticsconfiguration.html
    pub struct AnalyticsConfiguration_ {
        pub id: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
        pub storage_class_analysis: Box<StorageClassAnalysis_>,
        pub tag_filters: Option<Vec<TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_AnalyticsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.AnalyticsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_AnalyticsConfiguration as AnalyticsConfiguration;
    impl crate::value::ToValue for AnalyticsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "StorageClassAnalysis".to_string(),
                crate::value::ToValue::to_value(&self.storage_class_analysis),
            );
            if let Some(ref value) = self.tag_filters {
                properties.insert(
                    "TagFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-bucketencryption.html
    pub struct BucketEncryption_ {
        pub server_side_encryption_configuration: Vec<ServerSideEncryptionRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_BucketEncryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.BucketEncryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_BucketEncryption as BucketEncryption;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-corsconfiguration.html
    pub struct CorsConfiguration_ {
        pub cors_rules: Vec<CorsRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_CorsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.CorsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_CorsConfiguration as CorsConfiguration;
    impl crate::value::ToValue for CorsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CorsRules".to_string(),
                crate::value::ToValue::to_value(&self.cors_rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-corsrule.html
    pub struct CorsRule_ {
        pub allowed_headers: Option<Vec<crate::value::ExpString>>,
        pub allowed_methods: Vec<crate::value::ExpString>,
        pub allowed_origins: Vec<crate::value::ExpString>,
        pub exposed_headers: Option<Vec<crate::value::ExpString>>,
        pub id: Option<crate::value::ExpString>,
        pub max_age: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_CorsRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.CorsRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_CorsRule as CorsRule;
    impl crate::value::ToValue for CorsRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_headers {
                properties.insert(
                    "AllowedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AllowedMethods".to_string(),
                crate::value::ToValue::to_value(&self.allowed_methods),
            );
            properties.insert(
                "AllowedOrigins".to_string(),
                crate::value::ToValue::to_value(&self.allowed_origins),
            );
            if let Some(ref value) = self.exposed_headers {
                properties.insert(
                    "ExposedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.max_age {
                properties.insert("MaxAge".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-dataexport.html
    pub struct DataExport_ {
        pub destination: Box<Destination_>,
        pub output_schema_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_DataExport {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.DataExport"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_DataExport as DataExport;
    impl crate::value::ToValue for DataExport_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "OutputSchemaVersion".to_string(),
                crate::value::ToValue::to_value(&self.output_schema_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-defaultretention.html
    pub struct DefaultRetention_ {
        pub days: Option<i64>,
        pub mode: Option<crate::value::ExpString>,
        pub years: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_DefaultRetention {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.DefaultRetention"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_DefaultRetention as DefaultRetention;
    impl crate::value::ToValue for DefaultRetention_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.days {
                properties.insert("Days".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.years {
                properties.insert("Years".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-deletemarkerreplication.html
    pub struct DeleteMarkerReplication_ {
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_DeleteMarkerReplication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.DeleteMarkerReplication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_DeleteMarkerReplication as DeleteMarkerReplication;
    impl crate::value::ToValue for DeleteMarkerReplication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-destination.html
    pub struct Destination_ {
        pub bucket_account_id: Option<crate::value::ExpString>,
        pub bucket_arn: crate::value::ExpString,
        pub format: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_Destination as Destination;
    impl crate::value::ToValue for Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_account_id {
                properties.insert(
                    "BucketAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BucketArn".to_string(),
                crate::value::ToValue::to_value(&self.bucket_arn),
            );
            properties.insert(
                "Format".to_string(),
                crate::value::ToValue::to_value(&self.format),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub replica_kms_key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ReplicaKmsKeyID".to_string(),
                crate::value::ToValue::to_value(&self.replica_kms_key_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-eventbridgeconfiguration.html
    pub struct EventBridgeConfiguration_ {
        pub event_bridge_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_EventBridgeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.EventBridgeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_EventBridgeConfiguration as EventBridgeConfiguration;
    impl crate::value::ToValue for EventBridgeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventBridgeEnabled".to_string(),
                crate::value::ToValue::to_value(&self.event_bridge_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-filterrule.html
    pub struct FilterRule_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_FilterRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.FilterRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_FilterRule as FilterRule;
    impl crate::value::ToValue for FilterRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-intelligenttieringconfiguration.html
    pub struct IntelligentTieringConfiguration_ {
        pub id: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
        pub tag_filters: Option<Vec<TagFilter_>>,
        pub tierings: Vec<Tiering_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_IntelligentTieringConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.IntelligentTieringConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_IntelligentTieringConfiguration as IntelligentTieringConfiguration;
    impl crate::value::ToValue for IntelligentTieringConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            if let Some(ref value) = self.tag_filters {
                properties.insert(
                    "TagFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Tierings".to_string(),
                crate::value::ToValue::to_value(&self.tierings),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventoryconfiguration.html
    pub struct InventoryConfiguration_ {
        pub destination: Box<Destination_>,
        pub enabled: crate::value::ExpBool,
        pub id: crate::value::ExpString,
        pub included_object_versions: crate::value::ExpString,
        pub optional_fields: Option<Vec<crate::value::ExpString>>,
        pub prefix: Option<crate::value::ExpString>,
        pub schedule_frequency: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_InventoryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.InventoryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_InventoryConfiguration as InventoryConfiguration;
    impl crate::value::ToValue for InventoryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "IncludedObjectVersions".to_string(),
                crate::value::ToValue::to_value(&self.included_object_versions),
            );
            if let Some(ref value) = self.optional_fields {
                properties.insert(
                    "OptionalFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "ScheduleFrequency".to_string(),
                crate::value::ToValue::to_value(&self.schedule_frequency),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-inventorytableconfiguration.html
    pub struct InventoryTableConfiguration_ {
        pub configuration_state: crate::value::ExpString,
        pub encryption_configuration: Option<Box<MetadataTableEncryptionConfiguration_>>,
        pub table_arn: Option<crate::value::ExpString>,
        pub table_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_InventoryTableConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.InventoryTableConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_InventoryTableConfiguration as InventoryTableConfiguration;
    impl crate::value::ToValue for InventoryTableConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConfigurationState".to_string(),
                crate::value::ToValue::to_value(&self.configuration_state),
            );
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_arn {
                properties.insert(
                    "TableArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_name {
                properties.insert(
                    "TableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-journaltableconfiguration.html
    pub struct JournalTableConfiguration_ {
        pub encryption_configuration: Option<Box<MetadataTableEncryptionConfiguration_>>,
        pub record_expiration: Box<RecordExpiration_>,
        pub table_arn: Option<crate::value::ExpString>,
        pub table_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_JournalTableConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.JournalTableConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_JournalTableConfiguration as JournalTableConfiguration;
    impl crate::value::ToValue for JournalTableConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RecordExpiration".to_string(),
                crate::value::ToValue::to_value(&self.record_expiration),
            );
            if let Some(ref value) = self.table_arn {
                properties.insert(
                    "TableArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.table_name {
                properties.insert(
                    "TableName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lambdaconfiguration.html
    pub struct LambdaConfiguration_ {
        pub event: crate::value::ExpString,
        pub filter: Option<Box<NotificationFilter_>>,
        pub function: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_LambdaConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.LambdaConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_LambdaConfiguration as LambdaConfiguration;
    impl crate::value::ToValue for LambdaConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Event".to_string(),
                crate::value::ToValue::to_value(&self.event),
            );
            if let Some(ref value) = self.filter {
                properties.insert("Filter".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Function".to_string(),
                crate::value::ToValue::to_value(&self.function),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-lifecycleconfiguration.html
    pub struct LifecycleConfiguration_ {
        pub rules: Vec<Rule_>,
        pub transition_default_minimum_object_size: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_LifecycleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.LifecycleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_LifecycleConfiguration as LifecycleConfiguration;
    impl crate::value::ToValue for LifecycleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            if let Some(ref value) = self.transition_default_minimum_object_size {
                properties.insert(
                    "TransitionDefaultMinimumObjectSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-loggingconfiguration.html
    pub struct LoggingConfiguration_ {
        pub destination_bucket_name: Option<crate::value::ExpString>,
        pub log_file_prefix: Option<crate::value::ExpString>,
        pub target_object_key_format: Option<Box<TargetObjectKeyFormat_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_LoggingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.LoggingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_LoggingConfiguration as LoggingConfiguration;
    impl crate::value::ToValue for LoggingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_bucket_name {
                properties.insert(
                    "DestinationBucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_file_prefix {
                properties.insert(
                    "LogFilePrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_object_key_format {
                properties.insert(
                    "TargetObjectKeyFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metadataconfiguration.html
    pub struct MetadataConfiguration_ {
        pub destination: Option<Box<MetadataDestination_>>,
        pub inventory_table_configuration: Option<Box<InventoryTableConfiguration_>>,
        pub journal_table_configuration: Box<JournalTableConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_MetadataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.MetadataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_MetadataConfiguration as MetadataConfiguration;
    impl crate::value::ToValue for MetadataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination {
                properties.insert(
                    "Destination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inventory_table_configuration {
                properties.insert(
                    "InventoryTableConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "JournalTableConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.journal_table_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metadatadestination.html
    pub struct MetadataDestination_ {
        pub table_bucket_arn: Option<crate::value::ExpString>,
        pub table_bucket_type: crate::value::ExpString,
        pub table_namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_MetadataDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.MetadataDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_MetadataDestination as MetadataDestination;
    impl crate::value::ToValue for MetadataDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.table_bucket_arn {
                properties.insert(
                    "TableBucketArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableBucketType".to_string(),
                crate::value::ToValue::to_value(&self.table_bucket_type),
            );
            if let Some(ref value) = self.table_namespace {
                properties.insert(
                    "TableNamespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metadatatableconfiguration.html
    pub struct MetadataTableConfiguration_ {
        pub s3_tables_destination: Box<S3TablesDestination_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_MetadataTableConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.MetadataTableConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_MetadataTableConfiguration as MetadataTableConfiguration;
    impl crate::value::ToValue for MetadataTableConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3TablesDestination".to_string(),
                crate::value::ToValue::to_value(&self.s3_tables_destination),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metadatatableencryptionconfiguration.html
    pub struct MetadataTableEncryptionConfiguration_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub sse_algorithm: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_MetadataTableEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.MetadataTableEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_MetadataTableEncryptionConfiguration as MetadataTableEncryptionConfiguration;
    impl crate::value::ToValue for MetadataTableEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SseAlgorithm".to_string(),
                crate::value::ToValue::to_value(&self.sse_algorithm),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metrics.html
    pub struct Metrics_ {
        pub event_threshold: Option<Box<ReplicationTimeValue_>>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_Metrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.Metrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_Metrics as Metrics;
    impl crate::value::ToValue for Metrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.event_threshold {
                properties.insert(
                    "EventThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-metricsconfiguration.html
    pub struct MetricsConfiguration_ {
        pub access_point_arn: Option<crate::value::ExpString>,
        pub id: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
        pub tag_filters: Option<Vec<TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_MetricsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.MetricsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_MetricsConfiguration as MetricsConfiguration;
    impl crate::value::ToValue for MetricsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_point_arn {
                properties.insert(
                    "AccessPointArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tag_filters {
                properties.insert(
                    "TagFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-noncurrentversionexpiration.html
    pub struct NoncurrentVersionExpiration_ {
        pub newer_noncurrent_versions: Option<i64>,
        pub noncurrent_days: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_NoncurrentVersionExpiration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.NoncurrentVersionExpiration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_NoncurrentVersionExpiration as NoncurrentVersionExpiration;
    impl crate::value::ToValue for NoncurrentVersionExpiration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.newer_noncurrent_versions {
                properties.insert(
                    "NewerNoncurrentVersions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "NoncurrentDays".to_string(),
                crate::value::ToValue::to_value(&self.noncurrent_days),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-noncurrentversiontransition.html
    pub struct NoncurrentVersionTransition_ {
        pub newer_noncurrent_versions: Option<i64>,
        pub storage_class: crate::value::ExpString,
        pub transition_in_days: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_NoncurrentVersionTransition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.NoncurrentVersionTransition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_NoncurrentVersionTransition as NoncurrentVersionTransition;
    impl crate::value::ToValue for NoncurrentVersionTransition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.newer_noncurrent_versions {
                properties.insert(
                    "NewerNoncurrentVersions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StorageClass".to_string(),
                crate::value::ToValue::to_value(&self.storage_class),
            );
            properties.insert(
                "TransitionInDays".to_string(),
                crate::value::ToValue::to_value(&self.transition_in_days),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationconfiguration.html
    pub struct NotificationConfiguration_ {
        pub event_bridge_configuration: Option<Box<EventBridgeConfiguration_>>,
        pub lambda_configurations: Option<Vec<LambdaConfiguration_>>,
        pub queue_configurations: Option<Vec<QueueConfiguration_>>,
        pub topic_configurations: Option<Vec<TopicConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_NotificationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.NotificationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_NotificationConfiguration as NotificationConfiguration;
    impl crate::value::ToValue for NotificationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.event_bridge_configuration {
                properties.insert(
                    "EventBridgeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_configurations {
                properties.insert(
                    "LambdaConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.queue_configurations {
                properties.insert(
                    "QueueConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic_configurations {
                properties.insert(
                    "TopicConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-notificationfilter.html
    pub struct NotificationFilter_ {
        pub s3_key: Box<S3KeyFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_NotificationFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.NotificationFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_NotificationFilter as NotificationFilter;
    impl crate::value::ToValue for NotificationFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Key".to_string(),
                crate::value::ToValue::to_value(&self.s3_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-objectlockconfiguration.html
    pub struct ObjectLockConfiguration_ {
        pub object_lock_enabled: Option<crate::value::ExpString>,
        pub rule: Option<Box<ObjectLockRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ObjectLockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ObjectLockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ObjectLockConfiguration as ObjectLockConfiguration;
    impl crate::value::ToValue for ObjectLockConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.object_lock_enabled {
                properties.insert(
                    "ObjectLockEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule {
                properties.insert("Rule".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-objectlockrule.html
    pub struct ObjectLockRule_ {
        pub default_retention: Option<Box<DefaultRetention_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ObjectLockRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ObjectLockRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ObjectLockRule as ObjectLockRule;
    impl crate::value::ToValue for ObjectLockRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_retention {
                properties.insert(
                    "DefaultRetention".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-ownershipcontrols.html
    pub struct OwnershipControls_ {
        pub rules: Vec<OwnershipControlsRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_OwnershipControls {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.OwnershipControls"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_OwnershipControls as OwnershipControls;
    impl crate::value::ToValue for OwnershipControls_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-ownershipcontrolsrule.html
    pub struct OwnershipControlsRule_ {
        pub object_ownership: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_OwnershipControlsRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.OwnershipControlsRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_OwnershipControlsRule as OwnershipControlsRule;
    impl crate::value::ToValue for OwnershipControlsRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.object_ownership {
                properties.insert(
                    "ObjectOwnership".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-partitionedprefix.html
    pub struct PartitionedPrefix_ {
        pub partition_date_source: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_PartitionedPrefix {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.PartitionedPrefix"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_PartitionedPrefix as PartitionedPrefix;
    impl crate::value::ToValue for PartitionedPrefix_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.partition_date_source {
                properties.insert(
                    "PartitionDateSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-publicaccessblockconfiguration.html
    pub struct PublicAccessBlockConfiguration_ {
        pub block_public_acls: Option<crate::value::ExpBool>,
        pub block_public_policy: Option<crate::value::ExpBool>,
        pub ignore_public_acls: Option<crate::value::ExpBool>,
        pub restrict_public_buckets: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_PublicAccessBlockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.PublicAccessBlockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_PublicAccessBlockConfiguration as PublicAccessBlockConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-queueconfiguration.html
    pub struct QueueConfiguration_ {
        pub event: crate::value::ExpString,
        pub filter: Option<Box<NotificationFilter_>>,
        pub queue: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_QueueConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.QueueConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_QueueConfiguration as QueueConfiguration;
    impl crate::value::ToValue for QueueConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Event".to_string(),
                crate::value::ToValue::to_value(&self.event),
            );
            if let Some(ref value) = self.filter {
                properties.insert("Filter".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Queue".to_string(),
                crate::value::ToValue::to_value(&self.queue),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-recordexpiration.html
    pub struct RecordExpiration_ {
        pub days: Option<i64>,
        pub expiration: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_RecordExpiration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.RecordExpiration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_RecordExpiration as RecordExpiration;
    impl crate::value::ToValue for RecordExpiration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.days {
                properties.insert("Days".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Expiration".to_string(),
                crate::value::ToValue::to_value(&self.expiration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-redirectallrequeststo.html
    pub struct RedirectAllRequestsTo_ {
        pub host_name: crate::value::ExpString,
        pub protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_RedirectAllRequestsTo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.RedirectAllRequestsTo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_RedirectAllRequestsTo as RedirectAllRequestsTo;
    impl crate::value::ToValue for RedirectAllRequestsTo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HostName".to_string(),
                crate::value::ToValue::to_value(&self.host_name),
            );
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-redirectrule.html
    pub struct RedirectRule_ {
        pub host_name: Option<crate::value::ExpString>,
        pub http_redirect_code: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
        pub replace_key_prefix_with: Option<crate::value::ExpString>,
        pub replace_key_with: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_RedirectRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.RedirectRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_RedirectRule as RedirectRule;
    impl crate::value::ToValue for RedirectRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.host_name {
                properties.insert(
                    "HostName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_redirect_code {
                properties.insert(
                    "HttpRedirectCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replace_key_prefix_with {
                properties.insert(
                    "ReplaceKeyPrefixWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replace_key_with {
                properties.insert(
                    "ReplaceKeyWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicamodifications.html
    pub struct ReplicaModifications_ {
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicaModifications {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicaModifications"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicaModifications as ReplicaModifications;
    impl crate::value::ToValue for ReplicaModifications_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationconfiguration.html
    pub struct ReplicationConfiguration_ {
        pub role: crate::value::ExpString,
        pub rules: Vec<ReplicationRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicationConfiguration as ReplicationConfiguration;
    impl crate::value::ToValue for ReplicationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Role".to_string(),
                crate::value::ToValue::to_value(&self.role),
            );
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationdestination.html
    pub struct ReplicationDestination_ {
        pub access_control_translation: Option<Box<AccessControlTranslation_>>,
        pub account: Option<crate::value::ExpString>,
        pub bucket: crate::value::ExpString,
        pub encryption_configuration: Option<Box<EncryptionConfiguration_>>,
        pub metrics: Option<Box<Metrics_>>,
        pub replication_time: Option<Box<ReplicationTime_>>,
        pub storage_class: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicationDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicationDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicationDestination as ReplicationDestination;
    impl crate::value::ToValue for ReplicationDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_control_translation {
                properties.insert(
                    "AccessControlTranslation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.account {
                properties.insert(
                    "Account".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            if let Some(ref value) = self.encryption_configuration {
                properties.insert(
                    "EncryptionConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.replication_time {
                properties.insert(
                    "ReplicationTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_class {
                properties.insert(
                    "StorageClass".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationrule.html
    pub struct ReplicationRule_ {
        pub delete_marker_replication: Option<Box<DeleteMarkerReplication_>>,
        pub destination: Box<ReplicationDestination_>,
        pub filter: Option<Box<ReplicationRuleFilter_>>,
        pub id: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub priority: Option<i64>,
        pub source_selection_criteria: Option<Box<SourceSelectionCriteria_>>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicationRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicationRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicationRule as ReplicationRule;
    impl crate::value::ToValue for ReplicationRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delete_marker_replication {
                properties.insert(
                    "DeleteMarkerReplication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            if let Some(ref value) = self.filter {
                properties.insert("Filter".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_selection_criteria {
                properties.insert(
                    "SourceSelectionCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationruleandoperator.html
    pub struct ReplicationRuleAndOperator_ {
        pub prefix: Option<crate::value::ExpString>,
        pub tag_filters: Option<Vec<TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicationRuleAndOperator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicationRuleAndOperator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicationRuleAndOperator as ReplicationRuleAndOperator;
    impl crate::value::ToValue for ReplicationRuleAndOperator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tag_filters {
                properties.insert(
                    "TagFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationrulefilter.html
    pub struct ReplicationRuleFilter_ {
        pub and: Option<Box<ReplicationRuleAndOperator_>>,
        pub prefix: Option<crate::value::ExpString>,
        pub tag_filter: Option<Box<TagFilter_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicationRuleFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicationRuleFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicationRuleFilter as ReplicationRuleFilter;
    impl crate::value::ToValue for ReplicationRuleFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and {
                properties.insert("And".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tag_filter {
                properties.insert(
                    "TagFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationtime.html
    pub struct ReplicationTime_ {
        pub status: crate::value::ExpString,
        pub time: Box<ReplicationTimeValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicationTime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicationTime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicationTime as ReplicationTime;
    impl crate::value::ToValue for ReplicationTime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.insert(
                "Time".to_string(),
                crate::value::ToValue::to_value(&self.time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-replicationtimevalue.html
    pub struct ReplicationTimeValue_ {
        pub minutes: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ReplicationTimeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ReplicationTimeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ReplicationTimeValue as ReplicationTimeValue;
    impl crate::value::ToValue for ReplicationTimeValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Minutes".to_string(),
                crate::value::ToValue::to_value(&self.minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-routingrule.html
    pub struct RoutingRule_ {
        pub redirect_rule: Box<RedirectRule_>,
        pub routing_rule_condition: Option<Box<RoutingRuleCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_RoutingRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.RoutingRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_RoutingRule as RoutingRule;
    impl crate::value::ToValue for RoutingRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RedirectRule".to_string(),
                crate::value::ToValue::to_value(&self.redirect_rule),
            );
            if let Some(ref value) = self.routing_rule_condition {
                properties.insert(
                    "RoutingRuleCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-routingrulecondition.html
    pub struct RoutingRuleCondition_ {
        pub http_error_code_returned_equals: Option<crate::value::ExpString>,
        pub key_prefix_equals: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_RoutingRuleCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.RoutingRuleCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_RoutingRuleCondition as RoutingRuleCondition;
    impl crate::value::ToValue for RoutingRuleCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http_error_code_returned_equals {
                properties.insert(
                    "HttpErrorCodeReturnedEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_prefix_equals {
                properties.insert(
                    "KeyPrefixEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-rule.html
    pub struct Rule_ {
        pub abort_incomplete_multipart_upload: Option<Box<AbortIncompleteMultipartUpload_>>,
        pub expiration_date: Option<crate::value::ExpString>,
        pub expiration_in_days: Option<i64>,
        pub expired_object_delete_marker: Option<crate::value::ExpBool>,
        pub id: Option<crate::value::ExpString>,
        pub noncurrent_version_expiration: Option<Box<NoncurrentVersionExpiration_>>,
        pub noncurrent_version_expiration_in_days: Option<i64>,
        pub noncurrent_version_transition: Option<Box<NoncurrentVersionTransition_>>,
        pub noncurrent_version_transitions: Option<Vec<NoncurrentVersionTransition_>>,
        pub object_size_greater_than: Option<crate::value::ExpString>,
        pub object_size_less_than: Option<crate::value::ExpString>,
        pub prefix: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
        pub tag_filters: Option<Vec<TagFilter_>>,
        pub transition: Option<Box<Transition_>>,
        pub transitions: Option<Vec<Transition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.abort_incomplete_multipart_upload {
                properties.insert(
                    "AbortIncompleteMultipartUpload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expiration_date {
                properties.insert(
                    "ExpirationDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expiration_in_days {
                properties.insert(
                    "ExpirationInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expired_object_delete_marker {
                properties.insert(
                    "ExpiredObjectDeleteMarker".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.noncurrent_version_expiration {
                properties.insert(
                    "NoncurrentVersionExpiration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.noncurrent_version_expiration_in_days {
                properties.insert(
                    "NoncurrentVersionExpirationInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.noncurrent_version_transition {
                properties.insert(
                    "NoncurrentVersionTransition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.noncurrent_version_transitions {
                properties.insert(
                    "NoncurrentVersionTransitions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
            if let Some(ref value) = self.tag_filters {
                properties.insert(
                    "TagFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transition {
                properties.insert(
                    "Transition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transitions {
                properties.insert(
                    "Transitions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-s3keyfilter.html
    pub struct S3KeyFilter_ {
        pub rules: Vec<FilterRule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_S3KeyFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.S3KeyFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_S3KeyFilter as S3KeyFilter;
    impl crate::value::ToValue for S3KeyFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-s3tablesdestination.html
    pub struct S3TablesDestination_ {
        pub table_arn: Option<crate::value::ExpString>,
        pub table_bucket_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
        pub table_namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_S3TablesDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.S3TablesDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_S3TablesDestination as S3TablesDestination;
    impl crate::value::ToValue for S3TablesDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.table_arn {
                properties.insert(
                    "TableArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TableBucketArn".to_string(),
                crate::value::ToValue::to_value(&self.table_bucket_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            if let Some(ref value) = self.table_namespace {
                properties.insert(
                    "TableNamespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionbydefault.html
    pub struct ServerSideEncryptionByDefault_ {
        pub kms_master_key_id: Option<crate::value::ExpString>,
        pub sse_algorithm: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ServerSideEncryptionByDefault {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ServerSideEncryptionByDefault"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ServerSideEncryptionByDefault as ServerSideEncryptionByDefault;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-serversideencryptionrule.html
    pub struct ServerSideEncryptionRule_ {
        pub bucket_key_enabled: Option<crate::value::ExpBool>,
        pub server_side_encryption_by_default: Option<Box<ServerSideEncryptionByDefault_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_ServerSideEncryptionRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.ServerSideEncryptionRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_ServerSideEncryptionRule as ServerSideEncryptionRule;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-sourceselectioncriteria.html
    pub struct SourceSelectionCriteria_ {
        pub replica_modifications: Option<Box<ReplicaModifications_>>,
        pub sse_kms_encrypted_objects: Option<Box<SseKmsEncryptedObjects_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_SourceSelectionCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.SourceSelectionCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_SourceSelectionCriteria as SourceSelectionCriteria;
    impl crate::value::ToValue for SourceSelectionCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.replica_modifications {
                properties.insert(
                    "ReplicaModifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sse_kms_encrypted_objects {
                properties.insert(
                    "SseKmsEncryptedObjects".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-ssekmsencryptedobjects.html
    pub struct SseKmsEncryptedObjects_ {
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_SseKmsEncryptedObjects {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.SseKmsEncryptedObjects"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_SseKmsEncryptedObjects as SseKmsEncryptedObjects;
    impl crate::value::ToValue for SseKmsEncryptedObjects_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-storageclassanalysis.html
    pub struct StorageClassAnalysis_ {
        pub data_export: Option<Box<DataExport_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_StorageClassAnalysis {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.StorageClassAnalysis"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_StorageClassAnalysis as StorageClassAnalysis;
    impl crate::value::ToValue for StorageClassAnalysis_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_export {
                properties.insert(
                    "DataExport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-tagfilter.html
    pub struct TagFilter_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_TagFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.TagFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_TagFilter as TagFilter;
    impl crate::value::ToValue for TagFilter_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-targetobjectkeyformat.html
    pub struct TargetObjectKeyFormat_ {
        pub partitioned_prefix: Option<Box<PartitionedPrefix_>>,
        pub simple_prefix: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_TargetObjectKeyFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.TargetObjectKeyFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_TargetObjectKeyFormat as TargetObjectKeyFormat;
    impl crate::value::ToValue for TargetObjectKeyFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.partitioned_prefix {
                properties.insert(
                    "PartitionedPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.simple_prefix {
                properties.insert(
                    "SimplePrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-tiering.html
    pub struct Tiering_ {
        pub access_tier: crate::value::ExpString,
        pub days: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_Tiering {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.Tiering"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_Tiering as Tiering;
    impl crate::value::ToValue for Tiering_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccessTier".to_string(),
                crate::value::ToValue::to_value(&self.access_tier),
            );
            properties.insert(
                "Days".to_string(),
                crate::value::ToValue::to_value(&self.days),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-topicconfiguration.html
    pub struct TopicConfiguration_ {
        pub event: crate::value::ExpString,
        pub filter: Option<Box<NotificationFilter_>>,
        pub topic: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_TopicConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.TopicConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_TopicConfiguration as TopicConfiguration;
    impl crate::value::ToValue for TopicConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Event".to_string(),
                crate::value::ToValue::to_value(&self.event),
            );
            if let Some(ref value) = self.filter {
                properties.insert("Filter".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Topic".to_string(),
                crate::value::ToValue::to_value(&self.topic),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-transition.html
    pub struct Transition_ {
        pub storage_class: crate::value::ExpString,
        pub transition_date: Option<crate::value::ExpString>,
        pub transition_in_days: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_Transition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.Transition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_Transition as Transition;
    impl crate::value::ToValue for Transition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StorageClass".to_string(),
                crate::value::ToValue::to_value(&self.storage_class),
            );
            if let Some(ref value) = self.transition_date {
                properties.insert(
                    "TransitionDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.transition_in_days {
                properties.insert(
                    "TransitionInDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-versioningconfiguration.html
    pub struct VersioningConfiguration_ {
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_VersioningConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.VersioningConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_VersioningConfiguration as VersioningConfiguration;
    impl crate::value::ToValue for VersioningConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-bucket-websiteconfiguration.html
    pub struct WebsiteConfiguration_ {
        pub error_document: Option<crate::value::ExpString>,
        pub index_document: Option<crate::value::ExpString>,
        pub redirect_all_requests_to: Option<Box<RedirectAllRequestsTo_>>,
        pub routing_rules: Option<Vec<RoutingRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_Bucket_WebsiteConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::Bucket.WebsiteConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_Bucket_WebsiteConfiguration as WebsiteConfiguration;
    impl crate::value::ToValue for WebsiteConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error_document {
                properties.insert(
                    "ErrorDocument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.index_document {
                properties.insert(
                    "IndexDocument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redirect_all_requests_to {
                properties.insert(
                    "RedirectAllRequestsTo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.routing_rules {
                properties.insert(
                    "RoutingRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod multiregionaccesspoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-multiregionaccesspoint-publicaccessblockconfiguration.html
    pub struct PublicAccessBlockConfiguration_ {
        pub block_public_acls: Option<crate::value::ExpBool>,
        pub block_public_policy: Option<crate::value::ExpBool>,
        pub ignore_public_acls: Option<crate::value::ExpBool>,
        pub restrict_public_buckets: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_MultiRegionAccessPoint_PublicAccessBlockConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::MultiRegionAccessPoint.PublicAccessBlockConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_MultiRegionAccessPoint_PublicAccessBlockConfiguration as PublicAccessBlockConfiguration;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-multiregionaccesspoint-region.html
    pub struct Region_ {
        pub bucket: crate::value::ExpString,
        pub bucket_account_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_MultiRegionAccessPoint_Region {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::MultiRegionAccessPoint.Region"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_MultiRegionAccessPoint_Region as Region;
    impl crate::value::ToValue for Region_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            properties.into()
        }
    }
}
pub mod multiregionaccesspointpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-multiregionaccesspointpolicy-policystatus.html
    pub struct PolicyStatus_ {
        pub is_public: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_MultiRegionAccessPointPolicy_PolicyStatus {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::MultiRegionAccessPointPolicy.PolicyStatus"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_MultiRegionAccessPointPolicy_PolicyStatus as PolicyStatus;
    impl crate::value::ToValue for PolicyStatus_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsPublic".to_string(),
                crate::value::ToValue::to_value(&self.is_public),
            );
            properties.into()
        }
    }
}
pub mod storagelens {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-accountlevel.html
    pub struct AccountLevel_ {
        pub activity_metrics: Option<Box<ActivityMetrics_>>,
        pub advanced_cost_optimization_metrics: Option<Box<AdvancedCostOptimizationMetrics_>>,
        pub advanced_data_protection_metrics: Option<Box<AdvancedDataProtectionMetrics_>>,
        pub bucket_level: Box<BucketLevel_>,
        pub detailed_status_codes_metrics: Option<Box<DetailedStatusCodesMetrics_>>,
        pub storage_lens_group_level: Option<Box<StorageLensGroupLevel_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_AccountLevel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.AccountLevel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_AccountLevel as AccountLevel;
    impl crate::value::ToValue for AccountLevel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.activity_metrics {
                properties.insert(
                    "ActivityMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.advanced_cost_optimization_metrics {
                properties.insert(
                    "AdvancedCostOptimizationMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.advanced_data_protection_metrics {
                properties.insert(
                    "AdvancedDataProtectionMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "BucketLevel".to_string(),
                crate::value::ToValue::to_value(&self.bucket_level),
            );
            if let Some(ref value) = self.detailed_status_codes_metrics {
                properties.insert(
                    "DetailedStatusCodesMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_lens_group_level {
                properties.insert(
                    "StorageLensGroupLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-activitymetrics.html
    pub struct ActivityMetrics_ {
        pub is_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_ActivityMetrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.ActivityMetrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_ActivityMetrics as ActivityMetrics;
    impl crate::value::ToValue for ActivityMetrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_enabled {
                properties.insert(
                    "IsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-advancedcostoptimizationmetrics.html
    pub struct AdvancedCostOptimizationMetrics_ {
        pub is_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_AdvancedCostOptimizationMetrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.AdvancedCostOptimizationMetrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_AdvancedCostOptimizationMetrics as AdvancedCostOptimizationMetrics;
    impl crate::value::ToValue for AdvancedCostOptimizationMetrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_enabled {
                properties.insert(
                    "IsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-advanceddataprotectionmetrics.html
    pub struct AdvancedDataProtectionMetrics_ {
        pub is_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_AdvancedDataProtectionMetrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.AdvancedDataProtectionMetrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_AdvancedDataProtectionMetrics as AdvancedDataProtectionMetrics;
    impl crate::value::ToValue for AdvancedDataProtectionMetrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_enabled {
                properties.insert(
                    "IsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-awsorg.html
    pub struct AwsOrg_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_AwsOrg {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.AwsOrg"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_AwsOrg as AwsOrg;
    impl crate::value::ToValue for AwsOrg_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-bucketlevel.html
    pub struct BucketLevel_ {
        pub activity_metrics: Option<Box<ActivityMetrics_>>,
        pub advanced_cost_optimization_metrics: Option<Box<AdvancedCostOptimizationMetrics_>>,
        pub advanced_data_protection_metrics: Option<Box<AdvancedDataProtectionMetrics_>>,
        pub detailed_status_codes_metrics: Option<Box<DetailedStatusCodesMetrics_>>,
        pub prefix_level: Option<Box<PrefixLevel_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_BucketLevel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.BucketLevel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_BucketLevel as BucketLevel;
    impl crate::value::ToValue for BucketLevel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.activity_metrics {
                properties.insert(
                    "ActivityMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.advanced_cost_optimization_metrics {
                properties.insert(
                    "AdvancedCostOptimizationMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.advanced_data_protection_metrics {
                properties.insert(
                    "AdvancedDataProtectionMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.detailed_status_codes_metrics {
                properties.insert(
                    "DetailedStatusCodesMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prefix_level {
                properties.insert(
                    "PrefixLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-bucketsandregions.html
    pub struct BucketsAndRegions_ {
        pub buckets: Option<Vec<crate::value::ExpString>>,
        pub regions: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_BucketsAndRegions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.BucketsAndRegions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_BucketsAndRegions as BucketsAndRegions;
    impl crate::value::ToValue for BucketsAndRegions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buckets {
                properties.insert(
                    "Buckets".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regions {
                properties.insert(
                    "Regions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-cloudwatchmetrics.html
    pub struct CloudWatchMetrics_ {
        pub is_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_CloudWatchMetrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.CloudWatchMetrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_CloudWatchMetrics as CloudWatchMetrics;
    impl crate::value::ToValue for CloudWatchMetrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IsEnabled".to_string(),
                crate::value::ToValue::to_value(&self.is_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-dataexport.html
    pub struct DataExport_ {
        pub cloud_watch_metrics: Option<Box<CloudWatchMetrics_>>,
        pub s3_bucket_destination: Option<Box<S3BucketDestination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_DataExport {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.DataExport"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_DataExport as DataExport;
    impl crate::value::ToValue for DataExport_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_metrics {
                properties.insert(
                    "CloudWatchMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_bucket_destination {
                properties.insert(
                    "S3BucketDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-detailedstatuscodesmetrics.html
    pub struct DetailedStatusCodesMetrics_ {
        pub is_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_DetailedStatusCodesMetrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.DetailedStatusCodesMetrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_DetailedStatusCodesMetrics as DetailedStatusCodesMetrics;
    impl crate::value::ToValue for DetailedStatusCodesMetrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_enabled {
                properties.insert(
                    "IsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-encryption.html
    pub struct Encryption_ {
        pub ssekms: Option<Box<SSEKMS_>>,
        pub sses3: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_Encryption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.Encryption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_Encryption as Encryption;
    impl crate::value::ToValue for Encryption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ssekms {
                properties.insert("SSEKMS".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sses3 {
                properties.insert("SSES3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-prefixlevel.html
    pub struct PrefixLevel_ {
        pub storage_metrics: Box<PrefixLevelStorageMetrics_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_PrefixLevel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.PrefixLevel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_PrefixLevel as PrefixLevel;
    impl crate::value::ToValue for PrefixLevel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StorageMetrics".to_string(),
                crate::value::ToValue::to_value(&self.storage_metrics),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-prefixlevelstoragemetrics.html
    pub struct PrefixLevelStorageMetrics_ {
        pub is_enabled: Option<crate::value::ExpBool>,
        pub selection_criteria: Option<Box<SelectionCriteria_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_PrefixLevelStorageMetrics {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.PrefixLevelStorageMetrics"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_PrefixLevelStorageMetrics as PrefixLevelStorageMetrics;
    impl crate::value::ToValue for PrefixLevelStorageMetrics_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_enabled {
                properties.insert(
                    "IsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.selection_criteria {
                properties.insert(
                    "SelectionCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-s3bucketdestination.html
    pub struct S3BucketDestination_ {
        pub account_id: crate::value::ExpString,
        pub arn: crate::value::ExpString,
        pub encryption: Option<Box<Encryption_>>,
        pub format: crate::value::ExpString,
        pub output_schema_version: crate::value::ExpString,
        pub prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_S3BucketDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.S3BucketDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_S3BucketDestination as S3BucketDestination;
    impl crate::value::ToValue for S3BucketDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountId".to_string(),
                crate::value::ToValue::to_value(&self.account_id),
            );
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.encryption {
                properties.insert(
                    "Encryption".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Format".to_string(),
                crate::value::ToValue::to_value(&self.format),
            );
            properties.insert(
                "OutputSchemaVersion".to_string(),
                crate::value::ToValue::to_value(&self.output_schema_version),
            );
            if let Some(ref value) = self.prefix {
                properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-ssekms.html
    pub struct SSEKMS_ {
        pub key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_SSEKMS {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.SSEKMS"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_SSEKMS as SSEKMS;
    impl crate::value::ToValue for SSEKMS_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KeyId".to_string(),
                crate::value::ToValue::to_value(&self.key_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-selectioncriteria.html
    pub struct SelectionCriteria_ {
        pub delimiter: Option<crate::value::ExpString>,
        pub max_depth: Option<i64>,
        pub min_storage_bytes_percentage: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_SelectionCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.SelectionCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_SelectionCriteria as SelectionCriteria;
    impl crate::value::ToValue for SelectionCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delimiter {
                properties.insert(
                    "Delimiter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_depth {
                properties.insert(
                    "MaxDepth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_storage_bytes_percentage {
                properties.insert(
                    "MinStorageBytesPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-storagelensconfiguration.html
    pub struct StorageLensConfiguration_ {
        pub account_level: Box<AccountLevel_>,
        pub aws_org: Option<Box<AwsOrg_>>,
        pub data_export: Option<Box<DataExport_>>,
        pub exclude: Option<Box<BucketsAndRegions_>>,
        pub id: crate::value::ExpString,
        pub include: Option<Box<BucketsAndRegions_>>,
        pub is_enabled: crate::value::ExpBool,
        pub storage_lens_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_StorageLensConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.StorageLensConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_StorageLensConfiguration as StorageLensConfiguration;
    impl crate::value::ToValue for StorageLensConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AccountLevel".to_string(),
                crate::value::ToValue::to_value(&self.account_level),
            );
            if let Some(ref value) = self.aws_org {
                properties.insert("AwsOrg".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.data_export {
                properties.insert(
                    "DataExport".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IsEnabled".to_string(),
                crate::value::ToValue::to_value(&self.is_enabled),
            );
            if let Some(ref value) = self.storage_lens_arn {
                properties.insert(
                    "StorageLensArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-storagelensgrouplevel.html
    pub struct StorageLensGroupLevel_ {
        pub storage_lens_group_selection_criteria: Option<Box<StorageLensGroupSelectionCriteria_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_StorageLensGroupLevel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.StorageLensGroupLevel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_StorageLensGroupLevel as StorageLensGroupLevel;
    impl crate::value::ToValue for StorageLensGroupLevel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.storage_lens_group_selection_criteria {
                properties.insert(
                    "StorageLensGroupSelectionCriteria".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelens-storagelensgroupselectioncriteria.html
    pub struct StorageLensGroupSelectionCriteria_ {
        pub exclude: Option<Vec<crate::value::ExpString>>,
        pub include: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLens_StorageLensGroupSelectionCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLens.StorageLensGroupSelectionCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLens_StorageLensGroupSelectionCriteria as StorageLensGroupSelectionCriteria;
    impl crate::value::ToValue for StorageLensGroupSelectionCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exclude {
                properties.insert(
                    "Exclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod storagelensgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelensgroup-and.html
    pub struct And_ {
        pub match_any_prefix: Option<Vec<crate::value::ExpString>>,
        pub match_any_suffix: Option<Vec<crate::value::ExpString>>,
        pub match_any_tag: Option<Vec<crate::Tag_>>,
        pub match_object_age: Option<Box<MatchObjectAge_>>,
        pub match_object_size: Option<Box<MatchObjectSize_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLensGroup_And {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLensGroup.And"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLensGroup_And as And;
    impl crate::value::ToValue for And_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.match_any_prefix {
                properties.insert(
                    "MatchAnyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_any_suffix {
                properties.insert(
                    "MatchAnySuffix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_any_tag {
                properties.insert(
                    "MatchAnyTag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_object_age {
                properties.insert(
                    "MatchObjectAge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_object_size {
                properties.insert(
                    "MatchObjectSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelensgroup-filter.html
    pub struct Filter_ {
        pub and: Option<Box<And_>>,
        pub match_any_prefix: Option<Vec<crate::value::ExpString>>,
        pub match_any_suffix: Option<Vec<crate::value::ExpString>>,
        pub match_any_tag: Option<Vec<crate::Tag_>>,
        pub match_object_age: Option<Box<MatchObjectAge_>>,
        pub match_object_size: Option<Box<MatchObjectSize_>>,
        pub or: Option<Box<Or_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLensGroup_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLensGroup.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLensGroup_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and {
                properties.insert("And".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.match_any_prefix {
                properties.insert(
                    "MatchAnyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_any_suffix {
                properties.insert(
                    "MatchAnySuffix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_any_tag {
                properties.insert(
                    "MatchAnyTag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_object_age {
                properties.insert(
                    "MatchObjectAge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_object_size {
                properties.insert(
                    "MatchObjectSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.or {
                properties.insert("Or".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelensgroup-matchobjectage.html
    pub struct MatchObjectAge_ {
        pub days_greater_than: Option<i64>,
        pub days_less_than: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLensGroup_MatchObjectAge {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLensGroup.MatchObjectAge"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLensGroup_MatchObjectAge as MatchObjectAge;
    impl crate::value::ToValue for MatchObjectAge_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.days_greater_than {
                properties.insert(
                    "DaysGreaterThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.days_less_than {
                properties.insert(
                    "DaysLessThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelensgroup-matchobjectsize.html
    pub struct MatchObjectSize_ {
        pub bytes_greater_than: Option<i64>,
        pub bytes_less_than: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLensGroup_MatchObjectSize {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLensGroup.MatchObjectSize"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLensGroup_MatchObjectSize as MatchObjectSize;
    impl crate::value::ToValue for MatchObjectSize_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bytes_greater_than {
                properties.insert(
                    "BytesGreaterThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bytes_less_than {
                properties.insert(
                    "BytesLessThan".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3-storagelensgroup-or.html
    pub struct Or_ {
        pub match_any_prefix: Option<Vec<crate::value::ExpString>>,
        pub match_any_suffix: Option<Vec<crate::value::ExpString>>,
        pub match_any_tag: Option<Vec<crate::Tag_>>,
        pub match_object_age: Option<Box<MatchObjectAge_>>,
        pub match_object_size: Option<Box<MatchObjectSize_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3_StorageLensGroup_Or {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3::StorageLensGroup.Or"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3_StorageLensGroup_Or as Or;
    impl crate::value::ToValue for Or_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.match_any_prefix {
                properties.insert(
                    "MatchAnyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_any_suffix {
                properties.insert(
                    "MatchAnySuffix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_any_tag {
                properties.insert(
                    "MatchAnyTag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_object_age {
                properties.insert(
                    "MatchObjectAge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.match_object_size {
                properties.insert(
                    "MatchObjectSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-accessgrant.html
pub struct AccessGrant_ {
    pub access_grants_location_configuration:
        Option<super::s3::accessgrant::AccessGrantsLocationConfiguration_>,
    pub access_grants_location_id: crate::value::ExpString,
    pub application_arn: Option<crate::value::ExpString>,
    pub grantee: super::s3::accessgrant::Grantee_,
    pub permission: crate::value::ExpString,
    pub s3_prefix_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_AccessGrant {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::AccessGrant" $($field
        $value)*)
    };
}
pub use crate::__aws_s3_AccessGrant as AccessGrant;
impl crate::template::ToResource for AccessGrant_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessGrant"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_grants_location_configuration {
            properties.insert(
                "AccessGrantsLocationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AccessGrantsLocationId".to_string(),
            crate::value::ToValue::to_value(&self.access_grants_location_id),
        );
        if let Some(ref value) = self.application_arn {
            properties.insert(
                "ApplicationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Grantee".to_string(),
            crate::value::ToValue::to_value(&self.grantee),
        );
        properties.insert(
            "Permission".to_string(),
            crate::value::ToValue::to_value(&self.permission),
        );
        if let Some(ref value) = self.s3_prefix_type {
            properties.insert(
                "S3PrefixType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-accessgrantsinstance.html
pub struct AccessGrantsInstance_ {
    pub identity_center_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_AccessGrantsInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::AccessGrantsInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_s3_AccessGrantsInstance as AccessGrantsInstance;
impl crate::template::ToResource for AccessGrantsInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessGrantsInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.identity_center_arn {
            properties.insert(
                "IdentityCenterArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-accessgrantslocation.html
pub struct AccessGrantsLocation_ {
    pub iam_role_arn: Option<crate::value::ExpString>,
    pub location_scope: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_AccessGrantsLocation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::AccessGrantsLocation"
        $($field $value)*)
    };
}
pub use crate::__aws_s3_AccessGrantsLocation as AccessGrantsLocation;
impl crate::template::ToResource for AccessGrantsLocation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AccessGrantsLocation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.iam_role_arn {
            properties.insert(
                "IamRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.location_scope {
            properties.insert(
                "LocationScope".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-accesspoint.html
pub struct AccessPoint_ {
    pub bucket: crate::value::ExpString,
    pub bucket_account_id: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub policy: Option<serde_json::Value>,
    pub public_access_block_configuration:
        Option<super::s3::accesspoint::PublicAccessBlockConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_configuration: Option<super::s3::accesspoint::VpcConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_AccessPoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::AccessPoint" $($field
        $value)*)
    };
}
pub use crate::__aws_s3_AccessPoint as AccessPoint;
impl crate::template::ToResource for AccessPoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-bucket.html
pub struct Bucket_ {
    pub accelerate_configuration: Option<super::s3::bucket::AccelerateConfiguration_>,
    pub access_control: Option<crate::value::ExpString>,
    pub analytics_configurations: Option<Vec<super::s3::bucket::AnalyticsConfiguration_>>,
    pub bucket_encryption: Option<super::s3::bucket::BucketEncryption_>,
    pub bucket_name: Option<crate::value::ExpString>,
    pub cors_configuration: Option<super::s3::bucket::CorsConfiguration_>,
    pub intelligent_tiering_configurations:
        Option<Vec<super::s3::bucket::IntelligentTieringConfiguration_>>,
    pub inventory_configurations: Option<Vec<super::s3::bucket::InventoryConfiguration_>>,
    pub lifecycle_configuration: Option<super::s3::bucket::LifecycleConfiguration_>,
    pub logging_configuration: Option<super::s3::bucket::LoggingConfiguration_>,
    pub metadata_configuration: Option<super::s3::bucket::MetadataConfiguration_>,
    pub metadata_table_configuration: Option<super::s3::bucket::MetadataTableConfiguration_>,
    pub metrics_configurations: Option<Vec<super::s3::bucket::MetricsConfiguration_>>,
    pub notification_configuration: Option<super::s3::bucket::NotificationConfiguration_>,
    pub object_lock_configuration: Option<super::s3::bucket::ObjectLockConfiguration_>,
    pub object_lock_enabled: Option<crate::value::ExpBool>,
    pub ownership_controls: Option<super::s3::bucket::OwnershipControls_>,
    pub public_access_block_configuration:
        Option<super::s3::bucket::PublicAccessBlockConfiguration_>,
    pub replication_configuration: Option<super::s3::bucket::ReplicationConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub versioning_configuration: Option<super::s3::bucket::VersioningConfiguration_>,
    pub website_configuration: Option<super::s3::bucket::WebsiteConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_Bucket {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::Bucket" $($field
        $value)*)
    };
}
pub use crate::__aws_s3_Bucket as Bucket;
impl crate::template::ToResource for Bucket_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Bucket"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accelerate_configuration {
            properties.insert(
                "AccelerateConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.access_control {
            properties.insert(
                "AccessControl".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.analytics_configurations {
            properties.insert(
                "AnalyticsConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.cors_configuration {
            properties.insert(
                "CorsConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.intelligent_tiering_configurations {
            properties.insert(
                "IntelligentTieringConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.inventory_configurations {
            properties.insert(
                "InventoryConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.lifecycle_configuration {
            properties.insert(
                "LifecycleConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_configuration {
            properties.insert(
                "LoggingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metadata_configuration {
            properties.insert(
                "MetadataConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metadata_table_configuration {
            properties.insert(
                "MetadataTableConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metrics_configurations {
            properties.insert(
                "MetricsConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_configuration {
            properties.insert(
                "NotificationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.object_lock_configuration {
            properties.insert(
                "ObjectLockConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.object_lock_enabled {
            properties.insert(
                "ObjectLockEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ownership_controls {
            properties.insert(
                "OwnershipControls".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_access_block_configuration {
            properties.insert(
                "PublicAccessBlockConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.replication_configuration {
            properties.insert(
                "ReplicationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.versioning_configuration {
            properties.insert(
                "VersioningConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.website_configuration {
            properties.insert(
                "WebsiteConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-bucketpolicy.html
pub struct BucketPolicy_ {
    pub bucket: crate::value::ExpString,
    pub policy_document: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_BucketPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::BucketPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3_BucketPolicy as BucketPolicy;
impl crate::template::ToResource for BucketPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-multiregionaccesspoint.html
pub struct MultiRegionAccessPoint_ {
    pub name: Option<crate::value::ExpString>,
    pub public_access_block_configuration:
        Option<super::s3::multiregionaccesspoint::PublicAccessBlockConfiguration_>,
    pub regions: Vec<super::s3::multiregionaccesspoint::Region_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_MultiRegionAccessPoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::MultiRegionAccessPoint"
        $($field $value)*)
    };
}
pub use crate::__aws_s3_MultiRegionAccessPoint as MultiRegionAccessPoint;
impl crate::template::ToResource for MultiRegionAccessPoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MultiRegionAccessPoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.public_access_block_configuration {
            properties.insert(
                "PublicAccessBlockConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Regions".to_string(),
            crate::value::ToValue::to_value(&self.regions),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-multiregionaccesspointpolicy.html
pub struct MultiRegionAccessPointPolicy_ {
    pub mrap_name: crate::value::ExpString,
    pub policy: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_MultiRegionAccessPointPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::MultiRegionAccessPointPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3_MultiRegionAccessPointPolicy as MultiRegionAccessPointPolicy;
impl crate::template::ToResource for MultiRegionAccessPointPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "MultiRegionAccessPointPolicy",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MrapName".to_string(),
            crate::value::ToValue::to_value(&self.mrap_name),
        );
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-storagelens.html
pub struct StorageLens_ {
    pub storage_lens_configuration: super::s3::storagelens::StorageLensConfiguration_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_StorageLens {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::StorageLens" $($field
        $value)*)
    };
}
pub use crate::__aws_s3_StorageLens as StorageLens;
impl crate::template::ToResource for StorageLens_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StorageLens"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "StorageLensConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.storage_lens_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3-storagelensgroup.html
pub struct StorageLensGroup_ {
    pub filter: super::s3::storagelensgroup::Filter_,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3_StorageLensGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3::StorageLensGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_s3_StorageLensGroup as StorageLensGroup;
impl crate::template::ToResource for StorageLensGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StorageLensGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Filter".to_string(),
            crate::value::ToValue::to_value(&self.filter),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
