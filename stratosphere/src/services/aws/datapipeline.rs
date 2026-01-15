pub mod pipeline {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-field.html
    pub struct Field_ {
        pub key: crate::value::ExpString,
        pub ref_value: Option<crate::value::ExpString>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datapipeline_Pipeline_Field {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DataPipeline::Pipeline.Field"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datapipeline_Pipeline_Field as Field;
    impl crate::value::ToValue for Field_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.ref_value {
                properties.insert(
                    "RefValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_value {
                properties.insert(
                    "StringValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterattribute.html
    pub struct ParameterAttribute_ {
        pub key: crate::value::ExpString,
        pub string_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datapipeline_Pipeline_ParameterAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DataPipeline::Pipeline.ParameterAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datapipeline_Pipeline_ParameterAttribute as ParameterAttribute;
    impl crate::value::ToValue for ParameterAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "StringValue".to_string(),
                crate::value::ToValue::to_value(&self.string_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parameterobject.html
    pub struct ParameterObject_ {
        pub attributes: Vec<ParameterAttribute_>,
        pub id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datapipeline_Pipeline_ParameterObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DataPipeline::Pipeline.ParameterObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datapipeline_Pipeline_ParameterObject as ParameterObject;
    impl crate::value::ToValue for ParameterObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-parametervalue.html
    pub struct ParameterValue_ {
        pub id: crate::value::ExpString,
        pub string_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datapipeline_Pipeline_ParameterValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DataPipeline::Pipeline.ParameterValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datapipeline_Pipeline_ParameterValue as ParameterValue;
    impl crate::value::ToValue for ParameterValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "StringValue".to_string(),
                crate::value::ToValue::to_value(&self.string_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelineobject.html
    pub struct PipelineObject_ {
        pub fields: Vec<Field_>,
        pub id: crate::value::ExpString,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datapipeline_Pipeline_PipelineObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DataPipeline::Pipeline.PipelineObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datapipeline_Pipeline_PipelineObject as PipelineObject;
    impl crate::value::ToValue for PipelineObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Fields".to_string(),
                crate::value::ToValue::to_value(&self.fields),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-datapipeline-pipeline-pipelinetag.html
    pub struct PipelineTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_datapipeline_Pipeline_PipelineTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::DataPipeline::Pipeline.PipelineTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_datapipeline_Pipeline_PipelineTag as PipelineTag;
    impl crate::value::ToValue for PipelineTag_ {
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-datapipeline-pipeline.html
pub struct Pipeline_ {
    pub activate: Option<crate::value::ExpBool>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub parameter_objects: Option<Vec<super::datapipeline::pipeline::ParameterObject_>>,
    pub parameter_values: Option<Vec<super::datapipeline::pipeline::ParameterValue_>>,
    pub pipeline_objects: Option<Vec<super::datapipeline::pipeline::PipelineObject_>>,
    pub pipeline_tags: Option<Vec<super::datapipeline::pipeline::PipelineTag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_datapipeline_Pipeline {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::DataPipeline::Pipeline"
        $($field $value)*)
    };
}
pub use crate::__aws_datapipeline_Pipeline as Pipeline;
impl crate::template::ToResource for Pipeline_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DataPipeline"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Pipeline"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.activate {
            properties.insert(
                "Activate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.parameter_objects {
            properties.insert(
                "ParameterObjects".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameter_values {
            properties.insert(
                "ParameterValues".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pipeline_objects {
            properties.insert(
                "PipelineObjects".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pipeline_tags {
            properties.insert(
                "PipelineTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
