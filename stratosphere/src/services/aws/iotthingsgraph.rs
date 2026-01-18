pub mod flowtemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iotthingsgraph-flowtemplate-definitiondocument.html
    pub struct DefinitionDocument_ {
        pub language: crate::value::ExpString,
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iotthingsgraph_FlowTemplate_DefinitionDocument {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTThingsGraph::FlowTemplate.DefinitionDocument"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iotthingsgraph_FlowTemplate_DefinitionDocument as DefinitionDocument;
    impl crate::value::ToValue for DefinitionDocument_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Language".to_string(),
                crate::value::ToValue::to_value(&self.language),
            );
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iotthingsgraph-flowtemplate.html
pub struct FlowTemplate_ {
    pub compatible_namespace_version: Option<f64>,
    pub definition: super::iotthingsgraph::flowtemplate::DefinitionDocument_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iotthingsgraph_FlowTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTThingsGraph::FlowTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_iotthingsgraph_FlowTemplate as FlowTemplate;
impl crate::template::ToResource for FlowTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTThingsGraph"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.compatible_namespace_version {
            properties.insert(
                "CompatibleNamespaceVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Definition".to_string(),
            crate::value::ToValue::to_value(&self.definition),
        );
        properties
    }
}
