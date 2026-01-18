pub mod export {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bcmdataexports-export-dataquery.html
    pub struct DataQuery_ {
        pub query_statement: crate::value::ExpString,
        pub table_configurations: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bcmdataexports_Export_DataQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BCMDataExports::Export.DataQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bcmdataexports_Export_DataQuery as DataQuery;
    impl crate::value::ToValue for DataQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QueryStatement".to_string(),
                crate::value::ToValue::to_value(&self.query_statement),
            );
            if let Some(ref value) = self.table_configurations {
                properties.insert(
                    "TableConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bcmdataexports-export-destinationconfigurations.html
    pub struct DestinationConfigurations_ {
        pub s3_destination: Box<S3Destination_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bcmdataexports_Export_DestinationConfigurations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BCMDataExports::Export.DestinationConfigurations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bcmdataexports_Export_DestinationConfigurations as DestinationConfigurations;
    impl crate::value::ToValue for DestinationConfigurations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Destination".to_string(),
                crate::value::ToValue::to_value(&self.s3_destination),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bcmdataexports-export-export.html
    pub struct Export_ {
        pub data_query: Box<DataQuery_>,
        pub description: Option<crate::value::ExpString>,
        pub destination_configurations: Box<DestinationConfigurations_>,
        pub export_arn: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub refresh_cadence: Box<RefreshCadence_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bcmdataexports_Export_Export {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BCMDataExports::Export.Export"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bcmdataexports_Export_Export as Export;
    impl crate::value::ToValue for Export_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataQuery".to_string(),
                crate::value::ToValue::to_value(&self.data_query),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DestinationConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.destination_configurations),
            );
            if let Some(ref value) = self.export_arn {
                properties.insert(
                    "ExportArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "RefreshCadence".to_string(),
                crate::value::ToValue::to_value(&self.refresh_cadence),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bcmdataexports-export-refreshcadence.html
    pub struct RefreshCadence_ {
        pub frequency: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bcmdataexports_Export_RefreshCadence {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BCMDataExports::Export.RefreshCadence"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bcmdataexports_Export_RefreshCadence as RefreshCadence;
    impl crate::value::ToValue for RefreshCadence_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Frequency".to_string(),
                crate::value::ToValue::to_value(&self.frequency),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bcmdataexports-export-resourcetag.html
    pub struct ResourceTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bcmdataexports_Export_ResourceTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BCMDataExports::Export.ResourceTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bcmdataexports_Export_ResourceTag as ResourceTag;
    impl crate::value::ToValue for ResourceTag_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bcmdataexports-export-s3destination.html
    pub struct S3Destination_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_output_configurations: Box<S3OutputConfigurations_>,
        pub s3_prefix: crate::value::ExpString,
        pub s3_region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bcmdataexports_Export_S3Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BCMDataExports::Export.S3Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bcmdataexports_Export_S3Destination as S3Destination;
    impl crate::value::ToValue for S3Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.insert(
                "S3OutputConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.s3_output_configurations),
            );
            properties.insert(
                "S3Prefix".to_string(),
                crate::value::ToValue::to_value(&self.s3_prefix),
            );
            properties.insert(
                "S3Region".to_string(),
                crate::value::ToValue::to_value(&self.s3_region),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bcmdataexports-export-s3outputconfigurations.html
    pub struct S3OutputConfigurations_ {
        pub compression: crate::value::ExpString,
        pub format: crate::value::ExpString,
        pub output_type: crate::value::ExpString,
        pub overwrite: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bcmdataexports_Export_S3OutputConfigurations {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::BCMDataExports::Export.S3OutputConfigurations"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bcmdataexports_Export_S3OutputConfigurations as S3OutputConfigurations;
    impl crate::value::ToValue for S3OutputConfigurations_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Compression".to_string(),
                crate::value::ToValue::to_value(&self.compression),
            );
            properties.insert(
                "Format".to_string(),
                crate::value::ToValue::to_value(&self.format),
            );
            properties.insert(
                "OutputType".to_string(),
                crate::value::ToValue::to_value(&self.output_type),
            );
            properties.insert(
                "Overwrite".to_string(),
                crate::value::ToValue::to_value(&self.overwrite),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bcmdataexports-export.html
pub struct Export_ {
    pub export: super::bcmdataexports::export::Export_,
    pub tags: Option<Vec<super::bcmdataexports::export::ResourceTag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bcmdataexports_Export {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::BCMDataExports::Export"
        $($field $value)*)
    };
}
pub use crate::__aws_bcmdataexports_Export as Export;
impl crate::template::ToResource for Export_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("BCMDataExports"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Export"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Export".to_string(),
            crate::value::ToValue::to_value(&self.export),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
