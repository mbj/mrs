pub mod idmappingworkflow {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingincrementalrunconfig.html
    pub struct IdMappingIncrementalRunConfig_ {
        pub incremental_run_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_IdMappingIncrementalRunConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.IdMappingIncrementalRunConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_IdMappingIncrementalRunConfig as IdMappingIncrementalRunConfig;
    impl crate::value::ToValue for IdMappingIncrementalRunConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IncrementalRunType".to_string(),
                crate::value::ToValue::to_value(&self.incremental_run_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingrulebasedproperties.html
    pub struct IdMappingRuleBasedProperties_ {
        pub attribute_matching_model: crate::value::ExpString,
        pub record_matching_model: crate::value::ExpString,
        pub rule_definition_type: Option<crate::value::ExpString>,
        pub rules: Option<Vec<Rule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_IdMappingRuleBasedProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.IdMappingRuleBasedProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_IdMappingRuleBasedProperties as IdMappingRuleBasedProperties;
    impl crate::value::ToValue for IdMappingRuleBasedProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeMatchingModel".to_string(),
                crate::value::ToValue::to_value(&self.attribute_matching_model),
            );
            properties.insert(
                "RecordMatchingModel".to_string(),
                crate::value::ToValue::to_value(&self.record_matching_model),
            );
            if let Some(ref value) = self.rule_definition_type {
                properties.insert(
                    "RuleDefinitionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rules {
                properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingtechniques.html
    pub struct IdMappingTechniques_ {
        pub id_mapping_type: Option<crate::value::ExpString>,
        pub normalization_version: Option<crate::value::ExpString>,
        pub provider_properties: Option<Box<ProviderProperties_>>,
        pub rule_based_properties: Option<Box<IdMappingRuleBasedProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_IdMappingTechniques {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.IdMappingTechniques"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_IdMappingTechniques as IdMappingTechniques;
    impl crate::value::ToValue for IdMappingTechniques_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id_mapping_type {
                properties.insert(
                    "IdMappingType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.normalization_version {
                properties.insert(
                    "NormalizationVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provider_properties {
                properties.insert(
                    "ProviderProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_based_properties {
                properties.insert(
                    "RuleBasedProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowinputsource.html
    pub struct IdMappingWorkflowInputSource_ {
        pub input_source_arn: crate::value::ExpString,
        pub schema_arn: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_IdMappingWorkflowInputSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.IdMappingWorkflowInputSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_IdMappingWorkflowInputSource as IdMappingWorkflowInputSource;
    impl crate::value::ToValue for IdMappingWorkflowInputSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputSourceARN".to_string(),
                crate::value::ToValue::to_value(&self.input_source_arn),
            );
            if let Some(ref value) = self.schema_arn {
                properties.insert(
                    "SchemaArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-idmappingworkflowoutputsource.html
    pub struct IdMappingWorkflowOutputSource_ {
        pub kms_arn: Option<crate::value::ExpString>,
        pub output_s3_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_IdMappingWorkflowOutputSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.IdMappingWorkflowOutputSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_IdMappingWorkflowOutputSource as IdMappingWorkflowOutputSource;
    impl crate::value::ToValue for IdMappingWorkflowOutputSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_arn {
                properties.insert("KMSArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "OutputS3Path".to_string(),
                crate::value::ToValue::to_value(&self.output_s3_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-intermediatesourceconfiguration.html
    pub struct IntermediateSourceConfiguration_ {
        pub intermediate_s3_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_IntermediateSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.IntermediateSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_IntermediateSourceConfiguration as IntermediateSourceConfiguration;
    impl crate::value::ToValue for IntermediateSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IntermediateS3Path".to_string(),
                crate::value::ToValue::to_value(&self.intermediate_s3_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-providerproperties.html
    pub struct ProviderProperties_ {
        pub intermediate_source_configuration: Option<Box<IntermediateSourceConfiguration_>>,
        pub provider_configuration:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub provider_service_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_ProviderProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.ProviderProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_ProviderProperties as ProviderProperties;
    impl crate::value::ToValue for ProviderProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.intermediate_source_configuration {
                properties.insert(
                    "IntermediateSourceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provider_configuration {
                properties.insert(
                    "ProviderConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProviderServiceArn".to_string(),
                crate::value::ToValue::to_value(&self.provider_service_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idmappingworkflow-rule.html
    pub struct Rule_ {
        pub matching_keys: Vec<crate::value::ExpString>,
        pub rule_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdMappingWorkflow_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdMappingWorkflow.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdMappingWorkflow_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchingKeys".to_string(),
                crate::value::ToValue::to_value(&self.matching_keys),
            );
            properties.insert(
                "RuleName".to_string(),
                crate::value::ToValue::to_value(&self.rule_name),
            );
            properties.into()
        }
    }
}
pub mod idnamespace {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idnamespace-idnamespaceidmappingworkflowproperties.html
    pub struct IdNamespaceIdMappingWorkflowProperties_ {
        pub id_mapping_type: crate::value::ExpString,
        pub provider_properties: Option<Box<NamespaceProviderProperties_>>,
        pub rule_based_properties: Option<Box<NamespaceRuleBasedProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdNamespace_IdNamespaceIdMappingWorkflowProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdNamespace.IdNamespaceIdMappingWorkflowProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdNamespace_IdNamespaceIdMappingWorkflowProperties as IdNamespaceIdMappingWorkflowProperties;
    impl crate::value::ToValue for IdNamespaceIdMappingWorkflowProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IdMappingType".to_string(),
                crate::value::ToValue::to_value(&self.id_mapping_type),
            );
            if let Some(ref value) = self.provider_properties {
                properties.insert(
                    "ProviderProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_based_properties {
                properties.insert(
                    "RuleBasedProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idnamespace-idnamespaceinputsource.html
    pub struct IdNamespaceInputSource_ {
        pub input_source_arn: crate::value::ExpString,
        pub schema_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdNamespace_IdNamespaceInputSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdNamespace.IdNamespaceInputSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdNamespace_IdNamespaceInputSource as IdNamespaceInputSource;
    impl crate::value::ToValue for IdNamespaceInputSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputSourceARN".to_string(),
                crate::value::ToValue::to_value(&self.input_source_arn),
            );
            if let Some(ref value) = self.schema_name {
                properties.insert(
                    "SchemaName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idnamespace-namespaceproviderproperties.html
    pub struct NamespaceProviderProperties_ {
        pub provider_configuration:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub provider_service_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdNamespace_NamespaceProviderProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdNamespace.NamespaceProviderProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdNamespace_NamespaceProviderProperties as NamespaceProviderProperties;
    impl crate::value::ToValue for NamespaceProviderProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provider_configuration {
                properties.insert(
                    "ProviderConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProviderServiceArn".to_string(),
                crate::value::ToValue::to_value(&self.provider_service_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idnamespace-namespacerulebasedproperties.html
    pub struct NamespaceRuleBasedProperties_ {
        pub attribute_matching_model: Option<crate::value::ExpString>,
        pub record_matching_models: Option<Vec<crate::value::ExpString>>,
        pub rule_definition_types: Option<Vec<crate::value::ExpString>>,
        pub rules: Option<Vec<Rule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdNamespace_NamespaceRuleBasedProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdNamespace.NamespaceRuleBasedProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdNamespace_NamespaceRuleBasedProperties as NamespaceRuleBasedProperties;
    impl crate::value::ToValue for NamespaceRuleBasedProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_matching_model {
                properties.insert(
                    "AttributeMatchingModel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.record_matching_models {
                properties.insert(
                    "RecordMatchingModels".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_definition_types {
                properties.insert(
                    "RuleDefinitionTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rules {
                properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-idnamespace-rule.html
    pub struct Rule_ {
        pub matching_keys: Vec<crate::value::ExpString>,
        pub rule_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_IdNamespace_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::IdNamespace.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_IdNamespace_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchingKeys".to_string(),
                crate::value::ToValue::to_value(&self.matching_keys),
            );
            properties.insert(
                "RuleName".to_string(),
                crate::value::ToValue::to_value(&self.rule_name),
            );
            properties.into()
        }
    }
}
pub mod matchingworkflow {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-customerprofilesintegrationconfig.html
    pub struct CustomerProfilesIntegrationConfig_ {
        pub domain_arn: crate::value::ExpString,
        pub object_type_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_CustomerProfilesIntegrationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.CustomerProfilesIntegrationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_CustomerProfilesIntegrationConfig as CustomerProfilesIntegrationConfig;
    impl crate::value::ToValue for CustomerProfilesIntegrationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DomainArn".to_string(),
                crate::value::ToValue::to_value(&self.domain_arn),
            );
            properties.insert(
                "ObjectTypeArn".to_string(),
                crate::value::ToValue::to_value(&self.object_type_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-incrementalrunconfig.html
    pub struct IncrementalRunConfig_ {
        pub incremental_run_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_IncrementalRunConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.IncrementalRunConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_IncrementalRunConfig as IncrementalRunConfig;
    impl crate::value::ToValue for IncrementalRunConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IncrementalRunType".to_string(),
                crate::value::ToValue::to_value(&self.incremental_run_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-inputsource.html
    pub struct InputSource_ {
        pub apply_normalization: Option<crate::value::ExpBool>,
        pub input_source_arn: crate::value::ExpString,
        pub schema_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_InputSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.InputSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_InputSource as InputSource;
    impl crate::value::ToValue for InputSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.apply_normalization {
                properties.insert(
                    "ApplyNormalization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputSourceARN".to_string(),
                crate::value::ToValue::to_value(&self.input_source_arn),
            );
            properties.insert(
                "SchemaArn".to_string(),
                crate::value::ToValue::to_value(&self.schema_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-intermediatesourceconfiguration.html
    pub struct IntermediateSourceConfiguration_ {
        pub intermediate_s3_path: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_IntermediateSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.IntermediateSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_IntermediateSourceConfiguration as IntermediateSourceConfiguration;
    impl crate::value::ToValue for IntermediateSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IntermediateS3Path".to_string(),
                crate::value::ToValue::to_value(&self.intermediate_s3_path),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputattribute.html
    pub struct OutputAttribute_ {
        pub hashed: Option<crate::value::ExpBool>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_OutputAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.OutputAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_OutputAttribute as OutputAttribute;
    impl crate::value::ToValue for OutputAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hashed {
                properties.insert("Hashed".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-outputsource.html
    pub struct OutputSource_ {
        pub apply_normalization: Option<crate::value::ExpBool>,
        pub customer_profiles_integration_config: Option<Box<CustomerProfilesIntegrationConfig_>>,
        pub kms_arn: Option<crate::value::ExpString>,
        pub output: Vec<OutputAttribute_>,
        pub output_s3_path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_OutputSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.OutputSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_OutputSource as OutputSource;
    impl crate::value::ToValue for OutputSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.apply_normalization {
                properties.insert(
                    "ApplyNormalization".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customer_profiles_integration_config {
                properties.insert(
                    "CustomerProfilesIntegrationConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_arn {
                properties.insert("KMSArn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Output".to_string(),
                crate::value::ToValue::to_value(&self.output),
            );
            if let Some(ref value) = self.output_s3_path {
                properties.insert(
                    "OutputS3Path".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-providerproperties.html
    pub struct ProviderProperties_ {
        pub intermediate_source_configuration: Option<Box<IntermediateSourceConfiguration_>>,
        pub provider_configuration:
            Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub provider_service_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_ProviderProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.ProviderProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_ProviderProperties as ProviderProperties;
    impl crate::value::ToValue for ProviderProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.intermediate_source_configuration {
                properties.insert(
                    "IntermediateSourceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.provider_configuration {
                properties.insert(
                    "ProviderConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ProviderServiceArn".to_string(),
                crate::value::ToValue::to_value(&self.provider_service_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-resolutiontechniques.html
    pub struct ResolutionTechniques_ {
        pub provider_properties: Option<Box<ProviderProperties_>>,
        pub resolution_type: Option<crate::value::ExpString>,
        pub rule_based_properties: Option<Box<RuleBasedProperties_>>,
        pub rule_condition_properties: Option<Box<RuleConditionProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_ResolutionTechniques {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.ResolutionTechniques"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_ResolutionTechniques as ResolutionTechniques;
    impl crate::value::ToValue for ResolutionTechniques_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provider_properties {
                properties.insert(
                    "ProviderProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resolution_type {
                properties.insert(
                    "ResolutionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_based_properties {
                properties.insert(
                    "RuleBasedProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_condition_properties {
                properties.insert(
                    "RuleConditionProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rule.html
    pub struct Rule_ {
        pub matching_keys: Vec<crate::value::ExpString>,
        pub rule_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchingKeys".to_string(),
                crate::value::ToValue::to_value(&self.matching_keys),
            );
            properties.insert(
                "RuleName".to_string(),
                crate::value::ToValue::to_value(&self.rule_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rulebasedproperties.html
    pub struct RuleBasedProperties_ {
        pub attribute_matching_model: crate::value::ExpString,
        pub match_purpose: Option<crate::value::ExpString>,
        pub rules: Vec<Rule_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_RuleBasedProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.RuleBasedProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_RuleBasedProperties as RuleBasedProperties;
    impl crate::value::ToValue for RuleBasedProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeMatchingModel".to_string(),
                crate::value::ToValue::to_value(&self.attribute_matching_model),
            );
            if let Some(ref value) = self.match_purpose {
                properties.insert(
                    "MatchPurpose".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-rulecondition.html
    pub struct RuleCondition_ {
        pub condition: Option<crate::value::ExpString>,
        pub rule_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_RuleCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.RuleCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_RuleCondition as RuleCondition;
    impl crate::value::ToValue for RuleCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_name {
                properties.insert(
                    "RuleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-matchingworkflow-ruleconditionproperties.html
    pub struct RuleConditionProperties_ {
        pub rules: Vec<RuleCondition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_MatchingWorkflow_RuleConditionProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::MatchingWorkflow.RuleConditionProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_MatchingWorkflow_RuleConditionProperties as RuleConditionProperties;
    impl crate::value::ToValue for RuleConditionProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Rules".to_string(),
                crate::value::ToValue::to_value(&self.rules),
            );
            properties.into()
        }
    }
}
pub mod schemamapping {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-entityresolution-schemamapping-schemainputattribute.html
    pub struct SchemaInputAttribute_ {
        pub field_name: crate::value::ExpString,
        pub group_name: Option<crate::value::ExpString>,
        pub hashed: Option<crate::value::ExpBool>,
        pub match_key: Option<crate::value::ExpString>,
        pub sub_type: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_entityresolution_SchemaMapping_SchemaInputAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::EntityResolution::SchemaMapping.SchemaInputAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_entityresolution_SchemaMapping_SchemaInputAttribute as SchemaInputAttribute;
    impl crate::value::ToValue for SchemaInputAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            if let Some(ref value) = self.group_name {
                properties.insert(
                    "GroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hashed {
                properties.insert("Hashed".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.match_key {
                properties.insert(
                    "MatchKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sub_type {
                properties.insert(
                    "SubType".to_string(),
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
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idmappingworkflow.html
pub struct IdMappingWorkflow_ {
    pub description: Option<crate::value::ExpString>,
    pub id_mapping_incremental_run_config:
        Option<super::entityresolution::idmappingworkflow::IdMappingIncrementalRunConfig_>,
    pub id_mapping_techniques: super::entityresolution::idmappingworkflow::IdMappingTechniques_,
    pub input_source_config:
        Vec<super::entityresolution::idmappingworkflow::IdMappingWorkflowInputSource_>,
    pub output_source_config:
        Option<Vec<super::entityresolution::idmappingworkflow::IdMappingWorkflowOutputSource_>>,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub workflow_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_entityresolution_IdMappingWorkflow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EntityResolution::IdMappingWorkflow"
        $($field $value)*)
    };
}
pub use crate::__aws_entityresolution_IdMappingWorkflow as IdMappingWorkflow;
impl crate::template::ToResource for IdMappingWorkflow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EntityResolution"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdMappingWorkflow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.id_mapping_incremental_run_config {
            properties.insert(
                "IdMappingIncrementalRunConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IdMappingTechniques".to_string(),
            crate::value::ToValue::to_value(&self.id_mapping_techniques),
        );
        properties.insert(
            "InputSourceConfig".to_string(),
            crate::value::ToValue::to_value(&self.input_source_config),
        );
        if let Some(ref value) = self.output_source_config {
            properties.insert(
                "OutputSourceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkflowName".to_string(),
            crate::value::ToValue::to_value(&self.workflow_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-idnamespace.html
pub struct IdNamespace_ {
    pub description: Option<crate::value::ExpString>,
    pub id_mapping_workflow_properties:
        Option<Vec<super::entityresolution::idnamespace::IdNamespaceIdMappingWorkflowProperties_>>,
    pub id_namespace_name: crate::value::ExpString,
    pub input_source_config:
        Option<Vec<super::entityresolution::idnamespace::IdNamespaceInputSource_>>,
    pub role_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_entityresolution_IdNamespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EntityResolution::IdNamespace"
        $($field $value)*)
    };
}
pub use crate::__aws_entityresolution_IdNamespace as IdNamespace;
impl crate::template::ToResource for IdNamespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EntityResolution"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IdNamespace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.id_mapping_workflow_properties {
            properties.insert(
                "IdMappingWorkflowProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IdNamespaceName".to_string(),
            crate::value::ToValue::to_value(&self.id_namespace_name),
        );
        if let Some(ref value) = self.input_source_config {
            properties.insert(
                "InputSourceConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-matchingworkflow.html
pub struct MatchingWorkflow_ {
    pub description: Option<crate::value::ExpString>,
    pub incremental_run_config:
        Option<super::entityresolution::matchingworkflow::IncrementalRunConfig_>,
    pub input_source_config: Vec<super::entityresolution::matchingworkflow::InputSource_>,
    pub output_source_config: Vec<super::entityresolution::matchingworkflow::OutputSource_>,
    pub resolution_techniques: super::entityresolution::matchingworkflow::ResolutionTechniques_,
    pub role_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub workflow_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_entityresolution_MatchingWorkflow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EntityResolution::MatchingWorkflow"
        $($field $value)*)
    };
}
pub use crate::__aws_entityresolution_MatchingWorkflow as MatchingWorkflow;
impl crate::template::ToResource for MatchingWorkflow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EntityResolution"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MatchingWorkflow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.incremental_run_config {
            properties.insert(
                "IncrementalRunConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InputSourceConfig".to_string(),
            crate::value::ToValue::to_value(&self.input_source_config),
        );
        properties.insert(
            "OutputSourceConfig".to_string(),
            crate::value::ToValue::to_value(&self.output_source_config),
        );
        properties.insert(
            "ResolutionTechniques".to_string(),
            crate::value::ToValue::to_value(&self.resolution_techniques),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkflowName".to_string(),
            crate::value::ToValue::to_value(&self.workflow_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-policystatement.html
pub struct PolicyStatement_ {
    pub action: Option<Vec<crate::value::ExpString>>,
    pub arn: crate::value::ExpString,
    pub condition: Option<crate::value::ExpString>,
    pub effect: Option<crate::value::ExpString>,
    pub principal: Option<Vec<crate::value::ExpString>>,
    pub statement_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_entityresolution_PolicyStatement {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EntityResolution::PolicyStatement"
        $($field $value)*)
    };
}
pub use crate::__aws_entityresolution_PolicyStatement as PolicyStatement;
impl crate::template::ToResource for PolicyStatement_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EntityResolution"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PolicyStatement"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.action {
            properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Arn".to_string(),
            crate::value::ToValue::to_value(&self.arn),
        );
        if let Some(ref value) = self.condition {
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.effect {
            properties.insert("Effect".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.principal {
            properties.insert(
                "Principal".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StatementId".to_string(),
            crate::value::ToValue::to_value(&self.statement_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-entityresolution-schemamapping.html
pub struct SchemaMapping_ {
    pub description: Option<crate::value::ExpString>,
    pub mapped_input_fields: Vec<super::entityresolution::schemamapping::SchemaInputAttribute_>,
    pub schema_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_entityresolution_SchemaMapping {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::EntityResolution::SchemaMapping"
        $($field $value)*)
    };
}
pub use crate::__aws_entityresolution_SchemaMapping as SchemaMapping;
impl crate::template::ToResource for SchemaMapping_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("EntityResolution"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SchemaMapping"),
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
            "MappedInputFields".to_string(),
            crate::value::ToValue::to_value(&self.mapped_input_fields),
        );
        properties.insert(
            "SchemaName".to_string(),
            crate::value::ToValue::to_value(&self.schema_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
