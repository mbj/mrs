pub mod dataset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-attributesitems.html>
    pub struct AttributesItems_ {
        pub attribute_name: Option<crate::value::ExpString>,
        pub attribute_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_forecast_Dataset_AttributesItems {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Forecast::Dataset.AttributesItems"
            $($field $value)*)
        };
    }
    pub use crate::__aws_forecast_Dataset_AttributesItems as AttributesItems;
    impl crate::value::ToValue for AttributesItems_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_name {
                properties.insert(
                    "AttributeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attribute_type {
                properties.insert(
                    "AttributeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-encryptionconfig.html>
    pub struct EncryptionConfig_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub role_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_forecast_Dataset_EncryptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Forecast::Dataset.EncryptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_forecast_Dataset_EncryptionConfig as EncryptionConfig;
    impl crate::value::ToValue for EncryptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.role_arn {
                properties.insert(
                    "RoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-schema.html>
    pub struct Schema_ {
        pub attributes: Option<Vec<AttributesItems_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_forecast_Dataset_Schema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Forecast::Dataset.Schema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_forecast_Dataset_Schema as Schema;
    impl crate::value::ToValue for Schema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-forecast-dataset-tagsitems.html>
    pub struct TagsItems_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_forecast_Dataset_TagsItems {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Forecast::Dataset.TagsItems"
            $($field $value)*)
        };
    }
    pub use crate::__aws_forecast_Dataset_TagsItems as TagsItems;
    impl crate::value::ToValue for TagsItems_ {
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
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-dataset.html>
pub struct Dataset_ {
    pub data_frequency: Option<crate::value::ExpString>,
    pub dataset_name: crate::value::ExpString,
    pub dataset_type: crate::value::ExpString,
    pub domain: crate::value::ExpString,
    pub encryption_config: Option<super::forecast::dataset::EncryptionConfig_>,
    pub schema: super::forecast::dataset::Schema_,
    pub tags: Option<Vec<super::forecast::dataset::TagsItems_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_forecast_Dataset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Forecast::Dataset"
        $($field $value)*)
    };
}
pub use crate::__aws_forecast_Dataset as Dataset;
impl crate::template::ToResource for Dataset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Forecast"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dataset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_frequency {
            properties.insert(
                "DataFrequency".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DatasetName".to_string(),
            crate::value::ToValue::to_value(&self.dataset_name),
        );
        properties.insert(
            "DatasetType".to_string(),
            crate::value::ToValue::to_value(&self.dataset_type),
        );
        properties.insert(
            "Domain".to_string(),
            crate::value::ToValue::to_value(&self.domain),
        );
        if let Some(ref value) = self.encryption_config {
            properties.insert(
                "EncryptionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Schema".to_string(),
            crate::value::ToValue::to_value(&self.schema),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-forecast-datasetgroup.html>
pub struct DatasetGroup_ {
    pub dataset_arns: Option<Vec<crate::value::ExpString>>,
    pub dataset_group_name: crate::value::ExpString,
    pub domain: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_forecast_DatasetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Forecast::DatasetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_forecast_DatasetGroup as DatasetGroup;
impl crate::template::ToResource for DatasetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Forecast"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DatasetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dataset_arns {
            properties.insert(
                "DatasetArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DatasetGroupName".to_string(),
            crate::value::ToValue::to_value(&self.dataset_group_name),
        );
        properties.insert(
            "Domain".to_string(),
            crate::value::ToValue::to_value(&self.domain),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
