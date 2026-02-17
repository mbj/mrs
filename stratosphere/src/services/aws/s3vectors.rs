pub mod index {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3vectors-index-encryptionconfiguration.html>
    pub struct EncryptionConfiguration_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub sse_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3vectors_Index_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Vectors::Index.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3vectors_Index_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sse_type {
                properties.insert(
                    "SseType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3vectors-index-metadataconfiguration.html>
    pub struct MetadataConfiguration_ {
        pub non_filterable_metadata_keys: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3vectors_Index_MetadataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Vectors::Index.MetadataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3vectors_Index_MetadataConfiguration as MetadataConfiguration;
    impl crate::value::ToValue for MetadataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.non_filterable_metadata_keys {
                properties.insert(
                    "NonFilterableMetadataKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod vectorbucket {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-s3vectors-vectorbucket-encryptionconfiguration.html>
    pub struct EncryptionConfiguration_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub sse_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_s3vectors_VectorBucket_EncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::S3Vectors::VectorBucket.EncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_s3vectors_VectorBucket_EncryptionConfiguration as EncryptionConfiguration;
    impl crate::value::ToValue for EncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sse_type {
                properties.insert(
                    "SseType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3vectors-index.html>
pub struct Index_ {
    pub data_type: crate::value::ExpString,
    pub dimension: i32,
    pub distance_metric: crate::value::ExpString,
    pub encryption_configuration: Option<super::s3vectors::index::EncryptionConfiguration_>,
    pub index_name: Option<crate::value::ExpString>,
    pub metadata_configuration: Option<super::s3vectors::index::MetadataConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vector_bucket_arn: Option<crate::value::ExpString>,
    pub vector_bucket_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3vectors_Index {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Vectors::Index"
        $($field $value)*)
    };
}
pub use crate::__aws_s3vectors_Index as Index;
impl crate::template::ToResource for Index_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Vectors"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Index"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DataType".to_string(),
            crate::value::ToValue::to_value(&self.data_type),
        );
        properties.insert(
            "Dimension".to_string(),
            crate::value::ToValue::to_value(&self.dimension),
        );
        properties.insert(
            "DistanceMetric".to_string(),
            crate::value::ToValue::to_value(&self.distance_metric),
        );
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.index_name {
            properties.insert(
                "IndexName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.metadata_configuration {
            properties.insert(
                "MetadataConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vector_bucket_arn {
            properties.insert(
                "VectorBucketArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vector_bucket_name {
            properties.insert(
                "VectorBucketName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3vectors-vectorbucket.html>
pub struct VectorBucket_ {
    pub encryption_configuration: Option<super::s3vectors::vectorbucket::EncryptionConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vector_bucket_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3vectors_VectorBucket {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Vectors::VectorBucket"
        $($field $value)*)
    };
}
pub use crate::__aws_s3vectors_VectorBucket as VectorBucket;
impl crate::template::ToResource for VectorBucket_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Vectors"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VectorBucket"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.encryption_configuration {
            properties.insert(
                "EncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vector_bucket_name {
            properties.insert(
                "VectorBucketName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-s3vectors-vectorbucketpolicy.html>
pub struct VectorBucketPolicy_ {
    pub policy: serde_json::Value,
    pub vector_bucket_arn: Option<crate::value::ExpString>,
    pub vector_bucket_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_s3vectors_VectorBucketPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::S3Vectors::VectorBucketPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_s3vectors_VectorBucketPolicy as VectorBucketPolicy;
impl crate::template::ToResource for VectorBucketPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("S3Vectors"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VectorBucketPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        if let Some(ref value) = self.vector_bucket_arn {
            properties.insert(
                "VectorBucketArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vector_bucket_name {
            properties.insert(
                "VectorBucketName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
