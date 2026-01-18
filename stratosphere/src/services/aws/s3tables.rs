pub mod table {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3tables-table-compaction.html
    pub struct Compaction_ {
        pub status: Option<crate::value::ExpString>,
        pub target_file_size_mb: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3tables_Table_Compaction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Tables::Table.Compaction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3tables_Table_Compaction as Compaction;
    impl crate::value::ToValue for Compaction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.target_file_size_mb {
                properties.insert(
                    "TargetFileSizeMB".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3tables-table-icebergmetadata.html
    pub struct IcebergMetadata_ {
        pub iceberg_schema: Box<IcebergSchema_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3tables_Table_IcebergMetadata {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Tables::Table.IcebergMetadata"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3tables_Table_IcebergMetadata as IcebergMetadata;
    impl crate::value::ToValue for IcebergMetadata_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IcebergSchema".to_string(),
                crate::value::ToValue::to_value(&self.iceberg_schema),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3tables-table-icebergschema.html
    pub struct IcebergSchema_ {
        pub schema_field_list: Vec<SchemaField_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3tables_Table_IcebergSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Tables::Table.IcebergSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3tables_Table_IcebergSchema as IcebergSchema;
    impl crate::value::ToValue for IcebergSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SchemaFieldList".to_string(),
                crate::value::ToValue::to_value(&self.schema_field_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3tables-table-schemafield.html
    pub struct SchemaField_ {
        pub name: crate::value::ExpString,
        pub required: Option<crate::value::ExpBool>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3tables_Table_SchemaField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Tables::Table.SchemaField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3tables_Table_SchemaField as SchemaField;
    impl crate::value::ToValue for SchemaField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.required {
                properties.insert(
                    "Required".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3tables-table-snapshotmanagement.html
    pub struct SnapshotManagement_ {
        pub max_snapshot_age_hours: Option<i64>,
        pub min_snapshots_to_keep: Option<i64>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3tables_Table_SnapshotManagement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Tables::Table.SnapshotManagement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3tables_Table_SnapshotManagement as SnapshotManagement;
    impl crate::value::ToValue for SnapshotManagement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_snapshot_age_hours {
                properties.insert(
                    "MaxSnapshotAgeHours".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_snapshots_to_keep {
                properties.insert(
                    "MinSnapshotsToKeep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod tablebucket {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3tables-tablebucket-encryptionconfiguration.html
    pub struct EncryptionConfiguration_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub sse_algorithm: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3tables_TableBucket_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Tables::TableBucket.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3tables_TableBucket_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KMSKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sse_algorithm {
                properties.insert(
                    "SSEAlgorithm".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3tables-tablebucket-unreferencedfileremoval.html
    pub struct UnreferencedFileRemoval_ {
        pub noncurrent_days: Option<i64>,
        pub status: Option<crate::value::ExpString>,
        pub unreferenced_days: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3tables_TableBucket_UnreferencedFileRemoval {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Tables::TableBucket.UnreferencedFileRemoval"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3tables_TableBucket_UnreferencedFileRemoval as UnreferencedFileRemoval;
    impl crate::value::ToValue for UnreferencedFileRemoval_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.noncurrent_days {
                properties.insert(
                    "NoncurrentDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unreferenced_days {
                properties.insert(
                    "UnreferencedDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3tables-namespace.html
pub struct Namespace_ {
    pub namespace: crate::value::ExpString,
    pub table_bucket_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3tables_Namespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Tables::Namespace"
        $($field $value)*)
    };
}
pub use crate::__aws_s3tables_Namespace as Namespace;
impl crate::template::ToResource for Namespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Tables"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Namespace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Namespace".to_string(),
            crate::value::ToValue::to_value(&self.namespace),
        );
        properties.insert(
            "TableBucketARN".to_string(),
            crate::value::ToValue::to_value(&self.table_bucket_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3tables-table.html
pub struct Table_ {
    pub compaction: Option<super::s3tables::table::Compaction_>,
    pub iceberg_metadata: Option<super::s3tables::table::IcebergMetadata_>,
    pub namespace: crate::value::ExpString,
    pub open_table_format: crate::value::ExpString,
    pub snapshot_management: Option<super::s3tables::table::SnapshotManagement_>,
    pub table_bucket_arn: crate::value::ExpString,
    pub table_name: crate::value::ExpString,
    pub without_metadata: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3tables_Table {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Tables::Table" $($field
        $value)*)
    };
}
pub use crate::__aws_s3tables_Table as Table;
impl crate::template::ToResource for Table_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Tables"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Table"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compaction {
            properties.insert(
                "Compaction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iceberg_metadata {
            properties.insert(
                "IcebergMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Namespace".to_string(),
            crate::value::ToValue::to_value(&self.namespace),
        );
        properties.insert(
            "OpenTableFormat".to_string(),
            crate::value::ToValue::to_value(&self.open_table_format),
        );
        if let Some(ref value) = self.snapshot_management {
            properties.insert(
                "SnapshotManagement".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TableBucketARN".to_string(),
            crate::value::ToValue::to_value(&self.table_bucket_arn),
        );
        properties.insert(
            "TableName".to_string(),
            crate::value::ToValue::to_value(&self.table_name),
        );
        if let Some(ref value) = self.without_metadata {
            properties.insert(
                "WithoutMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3tables-tablebucket.html
pub struct TableBucket_ {
    pub encryption_configuration: Option<super::s3tables::tablebucket::EncryptionConfiguration_>,
    pub table_bucket_name: crate::value::ExpString,
    pub unreferenced_file_removal: Option<super::s3tables::tablebucket::UnreferencedFileRemoval_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3tables_TableBucket {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Tables::TableBucket"
        $($field $value)*)
    };
}
pub use crate::__aws_s3tables_TableBucket as TableBucket;
impl crate::template::ToResource for TableBucket_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Tables"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TableBucket"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TableBucketName".to_string(),
            crate::value::ToValue::to_value(&self.table_bucket_name),
        );
        if let Some(ref value) = self.unreferenced_file_removal {
            properties.insert(
                "UnreferencedFileRemoval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3tables-tablebucketpolicy.html
pub struct TableBucketPolicy_ {
    pub resource_policy: serde_json::Value,
    pub table_bucket_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3tables_TableBucketPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Tables::TableBucketPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3tables_TableBucketPolicy as TableBucketPolicy;
impl crate::template::ToResource for TableBucketPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Tables"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TableBucketPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourcePolicy".to_string(),
            crate::value::ToValue::to_value(&self.resource_policy),
        );
        properties.insert(
            "TableBucketARN".to_string(),
            crate::value::ToValue::to_value(&self.table_bucket_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3tables-tablepolicy.html
pub struct TablePolicy_ {
    pub resource_policy: serde_json::Value,
    pub table_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3tables_TablePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Tables::TablePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3tables_TablePolicy as TablePolicy;
impl crate::template::ToResource for TablePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Tables"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TablePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourcePolicy".to_string(),
            crate::value::ToValue::to_value(&self.resource_policy),
        );
        properties.insert(
            "TableARN".to_string(),
            crate::value::ToValue::to_value(&self.table_arn),
        );
        properties
    }
}
