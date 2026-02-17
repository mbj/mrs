pub mod annotationstore {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-referenceitem.html>
    pub struct ReferenceItem_ {
        pub reference_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_AnnotationStore_ReferenceItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::AnnotationStore.ReferenceItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_AnnotationStore_ReferenceItem as ReferenceItem;
    impl crate::value::ToValue for ReferenceItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ReferenceArn".to_string(),
                crate::value::ToValue::to_value(&self.reference_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-sseconfig.html>
    pub struct SseConfig_ {
        pub key_arn: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_AnnotationStore_SseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::AnnotationStore.SseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_AnnotationStore_SseConfig as SseConfig;
    impl crate::value::ToValue for SseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_arn {
                properties.insert("KeyArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-storeoptions.html>
    pub struct StoreOptions_ {
        pub tsv_store_options: Box<TsvStoreOptions_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_AnnotationStore_StoreOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::AnnotationStore.StoreOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_AnnotationStore_StoreOptions as StoreOptions;
    impl crate::value::ToValue for StoreOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TsvStoreOptions".to_string(),
                crate::value::ToValue::to_value(&self.tsv_store_options),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-annotationstore-tsvstoreoptions.html>
    pub struct TsvStoreOptions_ {
        pub annotation_type: Option<crate::value::ExpString>,
        pub format_to_header: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub schema: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_AnnotationStore_TsvStoreOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::AnnotationStore.TsvStoreOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_AnnotationStore_TsvStoreOptions as TsvStoreOptions;
    impl crate::value::ToValue for TsvStoreOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.annotation_type {
                properties.insert(
                    "AnnotationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.format_to_header {
                properties.insert(
                    "FormatToHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schema {
                properties.insert("Schema".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod referencestore {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-referencestore-sseconfig.html>
    pub struct SseConfig_ {
        pub key_arn: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_ReferenceStore_SseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::ReferenceStore.SseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_ReferenceStore_SseConfig as SseConfig;
    impl crate::value::ToValue for SseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_arn {
                properties.insert("KeyArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod sequencestore {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-sequencestore-sseconfig.html>
    pub struct SseConfig_ {
        pub key_arn: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_SequenceStore_SseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::SequenceStore.SseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_SequenceStore_SseConfig as SseConfig;
    impl crate::value::ToValue for SseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_arn {
                properties.insert("KeyArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod variantstore {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-variantstore-referenceitem.html>
    pub struct ReferenceItem_ {
        pub reference_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_VariantStore_ReferenceItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::VariantStore.ReferenceItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_VariantStore_ReferenceItem as ReferenceItem;
    impl crate::value::ToValue for ReferenceItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ReferenceArn".to_string(),
                crate::value::ToValue::to_value(&self.reference_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-variantstore-sseconfig.html>
    pub struct SseConfig_ {
        pub key_arn: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_VariantStore_SseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::VariantStore.SseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_VariantStore_SseConfig as SseConfig;
    impl crate::value::ToValue for SseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key_arn {
                properties.insert("KeyArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod workflow {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-containerregistrymap.html>
    pub struct ContainerRegistryMap_ {
        pub image_mappings: Option<Vec<ImageMapping_>>,
        pub registry_mappings: Option<Vec<RegistryMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_Workflow_ContainerRegistryMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::Workflow.ContainerRegistryMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_Workflow_ContainerRegistryMap as ContainerRegistryMap;
    impl crate::value::ToValue for ContainerRegistryMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_mappings {
                properties.insert(
                    "ImageMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.registry_mappings {
                properties.insert(
                    "RegistryMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-definitionrepository.html>
    pub struct DefinitionRepository_ {
        pub connection_arn: Option<crate::value::ExpString>,
        pub exclude_file_patterns: Option<Vec<crate::value::ExpString>>,
        pub full_repository_id: Option<crate::value::ExpString>,
        pub source_reference: Option<Box<SourceReference_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_Workflow_DefinitionRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::Workflow.DefinitionRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_Workflow_DefinitionRepository as DefinitionRepository;
    impl crate::value::ToValue for DefinitionRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_arn {
                properties.insert(
                    "connectionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_file_patterns {
                properties.insert(
                    "excludeFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.full_repository_id {
                properties.insert(
                    "fullRepositoryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_reference {
                properties.insert(
                    "sourceReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-imagemapping.html>
    pub struct ImageMapping_ {
        pub destination_image: Option<crate::value::ExpString>,
        pub source_image: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_Workflow_ImageMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::Workflow.ImageMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_Workflow_ImageMapping as ImageMapping;
    impl crate::value::ToValue for ImageMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_image {
                properties.insert(
                    "DestinationImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_image {
                properties.insert(
                    "SourceImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-registrymapping.html>
    pub struct RegistryMapping_ {
        pub ecr_account_id: Option<crate::value::ExpString>,
        pub ecr_repository_prefix: Option<crate::value::ExpString>,
        pub upstream_registry_url: Option<crate::value::ExpString>,
        pub upstream_repository_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_Workflow_RegistryMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::Workflow.RegistryMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_Workflow_RegistryMapping as RegistryMapping;
    impl crate::value::ToValue for RegistryMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ecr_account_id {
                properties.insert(
                    "EcrAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_repository_prefix {
                properties.insert(
                    "EcrRepositoryPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upstream_registry_url {
                properties.insert(
                    "UpstreamRegistryUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upstream_repository_prefix {
                properties.insert(
                    "UpstreamRepositoryPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-sourcereference.html>
    pub struct SourceReference_ {
        pub r#type: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_Workflow_SourceReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::Workflow.SourceReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_Workflow_SourceReference as SourceReference;
    impl crate::value::ToValue for SourceReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflow-workflowparameter.html>
    pub struct WorkflowParameter_ {
        pub description: Option<crate::value::ExpString>,
        pub optional: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_Workflow_WorkflowParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::Workflow.WorkflowParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_Workflow_WorkflowParameter as WorkflowParameter;
    impl crate::value::ToValue for WorkflowParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.optional {
                properties.insert(
                    "Optional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod workflowversion {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflowversion-containerregistrymap.html>
    pub struct ContainerRegistryMap_ {
        pub image_mappings: Option<Vec<ImageMapping_>>,
        pub registry_mappings: Option<Vec<RegistryMapping_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_WorkflowVersion_ContainerRegistryMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::WorkflowVersion.ContainerRegistryMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_WorkflowVersion_ContainerRegistryMap as ContainerRegistryMap;
    impl crate::value::ToValue for ContainerRegistryMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.image_mappings {
                properties.insert(
                    "ImageMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.registry_mappings {
                properties.insert(
                    "RegistryMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflowversion-definitionrepository.html>
    pub struct DefinitionRepository_ {
        pub connection_arn: Option<crate::value::ExpString>,
        pub exclude_file_patterns: Option<Vec<crate::value::ExpString>>,
        pub full_repository_id: Option<crate::value::ExpString>,
        pub source_reference: Option<Box<SourceReference_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_WorkflowVersion_DefinitionRepository {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::WorkflowVersion.DefinitionRepository"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_WorkflowVersion_DefinitionRepository as DefinitionRepository;
    impl crate::value::ToValue for DefinitionRepository_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connection_arn {
                properties.insert(
                    "connectionArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_file_patterns {
                properties.insert(
                    "excludeFilePatterns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.full_repository_id {
                properties.insert(
                    "fullRepositoryId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_reference {
                properties.insert(
                    "sourceReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflowversion-imagemapping.html>
    pub struct ImageMapping_ {
        pub destination_image: Option<crate::value::ExpString>,
        pub source_image: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_WorkflowVersion_ImageMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::WorkflowVersion.ImageMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_WorkflowVersion_ImageMapping as ImageMapping;
    impl crate::value::ToValue for ImageMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_image {
                properties.insert(
                    "DestinationImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_image {
                properties.insert(
                    "SourceImage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflowversion-registrymapping.html>
    pub struct RegistryMapping_ {
        pub ecr_account_id: Option<crate::value::ExpString>,
        pub ecr_repository_prefix: Option<crate::value::ExpString>,
        pub upstream_registry_url: Option<crate::value::ExpString>,
        pub upstream_repository_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_WorkflowVersion_RegistryMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::WorkflowVersion.RegistryMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_WorkflowVersion_RegistryMapping as RegistryMapping;
    impl crate::value::ToValue for RegistryMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ecr_account_id {
                properties.insert(
                    "EcrAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ecr_repository_prefix {
                properties.insert(
                    "EcrRepositoryPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upstream_registry_url {
                properties.insert(
                    "UpstreamRegistryUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.upstream_repository_prefix {
                properties.insert(
                    "UpstreamRepositoryPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflowversion-sourcereference.html>
    pub struct SourceReference_ {
        pub r#type: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_WorkflowVersion_SourceReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::WorkflowVersion.SourceReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_WorkflowVersion_SourceReference as SourceReference;
    impl crate::value::ToValue for SourceReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#type {
                properties.insert("type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-omics-workflowversion-workflowparameter.html>
    pub struct WorkflowParameter_ {
        pub description: Option<crate::value::ExpString>,
        pub optional: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_omics_WorkflowVersion_WorkflowParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Omics::WorkflowVersion.WorkflowParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_omics_WorkflowVersion_WorkflowParameter as WorkflowParameter;
    impl crate::value::ToValue for WorkflowParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.optional {
                properties.insert(
                    "Optional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-annotationstore.html>
pub struct AnnotationStore_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub reference: Option<super::omics::annotationstore::ReferenceItem_>,
    pub sse_config: Option<super::omics::annotationstore::SseConfig_>,
    pub store_format: crate::value::ExpString,
    pub store_options: Option<super::omics::annotationstore::StoreOptions_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_omics_AnnotationStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Omics::AnnotationStore"
        $($field $value)*)
    };
}
pub use crate::__aws_omics_AnnotationStore as AnnotationStore;
impl crate::template::ToResource for AnnotationStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Omics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AnnotationStore"),
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
        if let Some(ref value) = self.reference {
            properties.insert(
                "Reference".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sse_config {
            properties.insert(
                "SseConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StoreFormat".to_string(),
            crate::value::ToValue::to_value(&self.store_format),
        );
        if let Some(ref value) = self.store_options {
            properties.insert(
                "StoreOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-referencestore.html>
pub struct ReferenceStore_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub sse_config: Option<super::omics::referencestore::SseConfig_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_omics_ReferenceStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Omics::ReferenceStore"
        $($field $value)*)
    };
}
pub use crate::__aws_omics_ReferenceStore as ReferenceStore;
impl crate::template::ToResource for ReferenceStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Omics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReferenceStore"),
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
        if let Some(ref value) = self.sse_config {
            properties.insert(
                "SseConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-rungroup.html>
pub struct RunGroup_ {
    pub max_cpus: Option<f64>,
    pub max_duration: Option<f64>,
    pub max_gpus: Option<f64>,
    pub max_runs: Option<f64>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_omics_RunGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Omics::RunGroup" $($field
        $value)*)
    };
}
pub use crate::__aws_omics_RunGroup as RunGroup;
impl crate::template::ToResource for RunGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Omics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RunGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.max_cpus {
            properties.insert(
                "MaxCpus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_duration {
            properties.insert(
                "MaxDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_gpus {
            properties.insert(
                "MaxGpus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_runs {
            properties.insert(
                "MaxRuns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-sequencestore.html>
pub struct SequenceStore_ {
    pub access_log_location: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub e_tag_algorithm_family: Option<crate::value::ExpString>,
    pub fallback_location: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub propagated_set_level_tags: Option<Vec<crate::value::ExpString>>,
    pub s3_access_policy: Option<serde_json::Value>,
    pub sse_config: Option<super::omics::sequencestore::SseConfig_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_omics_SequenceStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Omics::SequenceStore"
        $($field $value)*)
    };
}
pub use crate::__aws_omics_SequenceStore as SequenceStore;
impl crate::template::ToResource for SequenceStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Omics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SequenceStore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_log_location {
            properties.insert(
                "AccessLogLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.e_tag_algorithm_family {
            properties.insert(
                "ETagAlgorithmFamily".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fallback_location {
            properties.insert(
                "FallbackLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.propagated_set_level_tags {
            properties.insert(
                "PropagatedSetLevelTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_access_policy {
            properties.insert(
                "S3AccessPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sse_config {
            properties.insert(
                "SseConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-variantstore.html>
pub struct VariantStore_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub reference: super::omics::variantstore::ReferenceItem_,
    pub sse_config: Option<super::omics::variantstore::SseConfig_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_omics_VariantStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Omics::VariantStore"
        $($field $value)*)
    };
}
pub use crate::__aws_omics_VariantStore as VariantStore;
impl crate::template::ToResource for VariantStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Omics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VariantStore"),
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
            "Reference".to_string(),
            crate::value::ToValue::to_value(&self.reference),
        );
        if let Some(ref value) = self.sse_config {
            properties.insert(
                "SseConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflow.html>
pub struct Workflow_ {
    pub accelerators: Option<crate::value::ExpString>,
    pub container_registry_map: Option<super::omics::workflow::ContainerRegistryMap_>,
    pub container_registry_map_uri: Option<crate::value::ExpString>,
    pub definition_repository: Option<super::omics::workflow::DefinitionRepository_>,
    pub definition_uri: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub engine: Option<crate::value::ExpString>,
    pub main: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub parameter_template:
        Option<std::collections::BTreeMap<String, super::omics::workflow::WorkflowParameter_>>,
    pub parameter_template_path: Option<crate::value::ExpString>,
    pub storage_capacity: Option<f64>,
    pub storage_type: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workflow_bucket_owner_id: Option<crate::value::ExpString>,
    pub readme_markdown: Option<crate::value::ExpString>,
    pub readme_path: Option<crate::value::ExpString>,
    pub readme_uri: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_omics_Workflow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Omics::Workflow" $($field
        $value)*)
    };
}
pub use crate::__aws_omics_Workflow as Workflow;
impl crate::template::ToResource for Workflow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Omics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workflow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accelerators {
            properties.insert(
                "Accelerators".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.container_registry_map {
            properties.insert(
                "ContainerRegistryMap".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.container_registry_map_uri {
            properties.insert(
                "ContainerRegistryMapUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_repository {
            properties.insert(
                "DefinitionRepository".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_uri {
            properties.insert(
                "DefinitionUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.main {
            properties.insert("Main".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.parameter_template {
            properties.insert(
                "ParameterTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameter_template_path {
            properties.insert(
                "ParameterTemplatePath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_capacity {
            properties.insert(
                "StorageCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_type {
            properties.insert(
                "StorageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.workflow_bucket_owner_id {
            properties.insert(
                "WorkflowBucketOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.readme_markdown {
            properties.insert(
                "readmeMarkdown".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.readme_path {
            properties.insert(
                "readmePath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.readme_uri {
            properties.insert(
                "readmeUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-omics-workflowversion.html>
pub struct WorkflowVersion_ {
    pub accelerators: Option<crate::value::ExpString>,
    pub container_registry_map: Option<super::omics::workflowversion::ContainerRegistryMap_>,
    pub container_registry_map_uri: Option<crate::value::ExpString>,
    pub definition_repository: Option<super::omics::workflowversion::DefinitionRepository_>,
    pub definition_uri: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub engine: Option<crate::value::ExpString>,
    pub main: Option<crate::value::ExpString>,
    pub parameter_template: Option<
        std::collections::BTreeMap<String, super::omics::workflowversion::WorkflowParameter_>,
    >,
    pub parameter_template_path: Option<crate::value::ExpString>,
    pub storage_capacity: Option<f64>,
    pub storage_type: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub version_name: crate::value::ExpString,
    pub workflow_bucket_owner_id: Option<crate::value::ExpString>,
    pub workflow_id: crate::value::ExpString,
    pub readme_markdown: Option<crate::value::ExpString>,
    pub readme_path: Option<crate::value::ExpString>,
    pub readme_uri: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_omics_WorkflowVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Omics::WorkflowVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_omics_WorkflowVersion as WorkflowVersion;
impl crate::template::ToResource for WorkflowVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Omics"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WorkflowVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.accelerators {
            properties.insert(
                "Accelerators".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.container_registry_map {
            properties.insert(
                "ContainerRegistryMap".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.container_registry_map_uri {
            properties.insert(
                "ContainerRegistryMapUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_repository {
            properties.insert(
                "DefinitionRepository".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_uri {
            properties.insert(
                "DefinitionUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.engine {
            properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.main {
            properties.insert("Main".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.parameter_template {
            properties.insert(
                "ParameterTemplate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameter_template_path {
            properties.insert(
                "ParameterTemplatePath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_capacity {
            properties.insert(
                "StorageCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_type {
            properties.insert(
                "StorageType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VersionName".to_string(),
            crate::value::ToValue::to_value(&self.version_name),
        );
        if let Some(ref value) = self.workflow_bucket_owner_id {
            properties.insert(
                "WorkflowBucketOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "WorkflowId".to_string(),
            crate::value::ToValue::to_value(&self.workflow_id),
        );
        if let Some(ref value) = self.readme_markdown {
            properties.insert(
                "readmeMarkdown".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.readme_path {
            properties.insert(
                "readmePath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.readme_uri {
            properties.insert(
                "readmeUri".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
