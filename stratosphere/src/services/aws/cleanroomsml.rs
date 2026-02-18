pub mod trainingdataset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanroomsml-trainingdataset-columnschema.html>
    pub struct ColumnSchema_ {
        pub column_name: crate::value::ExpString,
        pub column_types: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanroomsml_TrainingDataset_ColumnSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRoomsML::TrainingDataset.ColumnSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanroomsml_TrainingDataset_ColumnSchema as ColumnSchema;
    impl crate::value::ToValue for ColumnSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ColumnName".to_string(),
                crate::value::ToValue::to_value(&self.column_name),
            );
            properties.insert(
                "ColumnTypes".to_string(),
                crate::value::ToValue::to_value(&self.column_types),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanroomsml-trainingdataset-datasource.html>
    pub struct DataSource_ {
        pub glue_data_source: Box<GlueDataSource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanroomsml_TrainingDataset_DataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRoomsML::TrainingDataset.DataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanroomsml_TrainingDataset_DataSource as DataSource;
    impl crate::value::ToValue for DataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GlueDataSource".to_string(),
                crate::value::ToValue::to_value(&self.glue_data_source),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanroomsml-trainingdataset-dataset.html>
    pub struct Dataset_ {
        pub input_config: Box<DatasetInputConfig_>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanroomsml_TrainingDataset_Dataset {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRoomsML::TrainingDataset.Dataset"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanroomsml_TrainingDataset_Dataset as Dataset;
    impl crate::value::ToValue for Dataset_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputConfig".to_string(),
                crate::value::ToValue::to_value(&self.input_config),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanroomsml-trainingdataset-datasetinputconfig.html>
    pub struct DatasetInputConfig_ {
        pub data_source: Box<DataSource_>,
        pub schema: Vec<ColumnSchema_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanroomsml_TrainingDataset_DatasetInputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRoomsML::TrainingDataset.DatasetInputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanroomsml_TrainingDataset_DatasetInputConfig as DatasetInputConfig;
    impl crate::value::ToValue for DatasetInputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSource".to_string(),
                crate::value::ToValue::to_value(&self.data_source),
            );
            properties.insert(
                "Schema".to_string(),
                crate::value::ToValue::to_value(&self.schema),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cleanroomsml-trainingdataset-gluedatasource.html>
    pub struct GlueDataSource_ {
        pub catalog_id: Option<crate::value::ExpString>,
        pub database_name: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cleanroomsml_TrainingDataset_GlueDataSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CleanRoomsML::TrainingDataset.GlueDataSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cleanroomsml_TrainingDataset_GlueDataSource as GlueDataSource;
    impl crate::value::ToValue for GlueDataSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.catalog_id {
                properties.insert(
                    "CatalogId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cleanroomsml-trainingdataset.html>
pub struct TrainingDataset_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub training_data: Vec<super::cleanroomsml::trainingdataset::Dataset_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cleanroomsml_TrainingDataset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CleanRoomsML::TrainingDataset"
        $($field $value)*)
    };
}
pub use crate::__aws_cleanroomsml_TrainingDataset as TrainingDataset;
impl crate::template::ToResource for TrainingDataset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CleanRoomsML"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrainingDataset"),
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
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TrainingData".to_string(),
            crate::value::ToValue::to_value(&self.training_data),
        );
        properties
    }
}
