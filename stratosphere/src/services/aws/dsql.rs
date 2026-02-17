pub mod cluster {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dsql-cluster-encryptiondetails.html>
    pub struct EncryptionDetails_ {
        pub encryption_status: Option<crate::value::ExpString>,
        pub encryption_type: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dsql_Cluster_EncryptionDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DSQL::Cluster.EncryptionDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dsql_Cluster_EncryptionDetails as EncryptionDetails;
    impl crate::value::ToValue for EncryptionDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encryption_status {
                properties.insert(
                    "EncryptionStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encryption_type {
                properties.insert(
                    "EncryptionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-dsql-cluster-multiregionproperties.html>
    pub struct MultiRegionProperties_ {
        pub clusters: Option<Vec<crate::value::ExpString>>,
        pub witness_region: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_dsql_Cluster_MultiRegionProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DSQL::Cluster.MultiRegionProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_dsql_Cluster_MultiRegionProperties as MultiRegionProperties;
    impl crate::value::ToValue for MultiRegionProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.clusters {
                properties.insert(
                    "Clusters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.witness_region {
                properties.insert(
                    "WitnessRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-dsql-cluster.html>
pub struct Cluster_ {
    pub deletion_protection_enabled: Option<crate::value::ExpBool>,
    pub kms_encryption_key: Option<crate::value::ExpString>,
    pub multi_region_properties: Option<super::dsql::cluster::MultiRegionProperties_>,
    pub policy_document: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_dsql_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DSQL::Cluster" $($field
        $value)*)
    };
}
pub use crate::__aws_dsql_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DSQL"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection_enabled {
            properties.insert(
                "DeletionProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_encryption_key {
            properties.insert(
                "KmsEncryptionKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_region_properties {
            properties.insert(
                "MultiRegionProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.policy_document {
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
