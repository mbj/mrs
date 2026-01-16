pub mod aiagent {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-aiagentconfiguration.html
    pub struct AIAgentConfiguration_ {
        pub answer_recommendation_ai_agent_configuration:
            Option<Box<AnswerRecommendationAIAgentConfiguration_>>,
        pub manual_search_ai_agent_configuration: Option<Box<ManualSearchAIAgentConfiguration_>>,
        pub self_service_ai_agent_configuration: Option<Box<SelfServiceAIAgentConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_AIAgentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.AIAgentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_AIAgentConfiguration as AIAgentConfiguration;
    impl crate::value::ToValue for AIAgentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_recommendation_ai_agent_configuration {
                properties.insert(
                    "AnswerRecommendationAIAgentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.manual_search_ai_agent_configuration {
                properties.insert(
                    "ManualSearchAIAgentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_service_ai_agent_configuration {
                properties.insert(
                    "SelfServiceAIAgentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-answerrecommendationaiagentconfiguration.html
    pub struct AnswerRecommendationAIAgentConfiguration_ {
        pub answer_generation_ai_guardrail_id: Option<crate::value::ExpString>,
        pub answer_generation_ai_prompt_id: Option<crate::value::ExpString>,
        pub association_configurations: Option<Vec<AssociationConfiguration_>>,
        pub intent_labeling_generation_ai_prompt_id: Option<crate::value::ExpString>,
        pub locale: Option<crate::value::ExpString>,
        pub query_reformulation_ai_prompt_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_AnswerRecommendationAIAgentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.AnswerRecommendationAIAgentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_AnswerRecommendationAIAgentConfiguration as AnswerRecommendationAIAgentConfiguration;
    impl crate::value::ToValue for AnswerRecommendationAIAgentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_generation_ai_guardrail_id {
                properties.insert(
                    "AnswerGenerationAIGuardrailId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.answer_generation_ai_prompt_id {
                properties.insert(
                    "AnswerGenerationAIPromptId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.association_configurations {
                properties.insert(
                    "AssociationConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intent_labeling_generation_ai_prompt_id {
                properties.insert(
                    "IntentLabelingGenerationAIPromptId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.locale {
                properties.insert("Locale".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_reformulation_ai_prompt_id {
                properties.insert(
                    "QueryReformulationAIPromptId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-associationconfiguration.html
    pub struct AssociationConfiguration_ {
        pub association_configuration_data: Option<Box<AssociationConfigurationData_>>,
        pub association_id: Option<crate::value::ExpString>,
        pub association_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_AssociationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.AssociationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_AssociationConfiguration as AssociationConfiguration;
    impl crate::value::ToValue for AssociationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.association_configuration_data {
                properties.insert(
                    "AssociationConfigurationData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.association_id {
                properties.insert(
                    "AssociationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.association_type {
                properties.insert(
                    "AssociationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-associationconfigurationdata.html
    pub struct AssociationConfigurationData_ {
        pub knowledge_base_association_configuration_data:
            Box<KnowledgeBaseAssociationConfigurationData_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_AssociationConfigurationData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.AssociationConfigurationData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_AssociationConfigurationData as AssociationConfigurationData;
    impl crate::value::ToValue for AssociationConfigurationData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KnowledgeBaseAssociationConfigurationData".to_string(),
                crate::value::ToValue::to_value(
                    &self.knowledge_base_association_configuration_data,
                ),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-knowledgebaseassociationconfigurationdata.html
    pub struct KnowledgeBaseAssociationConfigurationData_ {
        pub content_tag_filter: Option<Box<TagFilter_>>,
        pub max_results: Option<f64>,
        pub override_knowledge_base_search_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_KnowledgeBaseAssociationConfigurationData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.KnowledgeBaseAssociationConfigurationData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_KnowledgeBaseAssociationConfigurationData as KnowledgeBaseAssociationConfigurationData;
    impl crate::value::ToValue for KnowledgeBaseAssociationConfigurationData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_tag_filter {
                properties.insert(
                    "ContentTagFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_results {
                properties.insert(
                    "MaxResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.override_knowledge_base_search_type {
                properties.insert(
                    "OverrideKnowledgeBaseSearchType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-manualsearchaiagentconfiguration.html
    pub struct ManualSearchAIAgentConfiguration_ {
        pub answer_generation_ai_guardrail_id: Option<crate::value::ExpString>,
        pub answer_generation_ai_prompt_id: Option<crate::value::ExpString>,
        pub association_configurations: Option<Vec<AssociationConfiguration_>>,
        pub locale: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_ManualSearchAIAgentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.ManualSearchAIAgentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_ManualSearchAIAgentConfiguration as ManualSearchAIAgentConfiguration;
    impl crate::value::ToValue for ManualSearchAIAgentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_generation_ai_guardrail_id {
                properties.insert(
                    "AnswerGenerationAIGuardrailId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.answer_generation_ai_prompt_id {
                properties.insert(
                    "AnswerGenerationAIPromptId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.association_configurations {
                properties.insert(
                    "AssociationConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.locale {
                properties.insert("Locale".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-orcondition.html
    pub struct OrCondition_ {
        pub and_conditions: Option<Vec<TagCondition_>>,
        pub tag_condition: Option<Box<TagCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_OrCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.OrCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_OrCondition as OrCondition;
    impl crate::value::ToValue for OrCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and_conditions {
                properties.insert(
                    "AndConditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_condition {
                properties.insert(
                    "TagCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-selfserviceaiagentconfiguration.html
    pub struct SelfServiceAIAgentConfiguration_ {
        pub association_configurations: Option<Vec<AssociationConfiguration_>>,
        pub self_service_ai_guardrail_id: Option<crate::value::ExpString>,
        pub self_service_answer_generation_ai_prompt_id: Option<crate::value::ExpString>,
        pub self_service_pre_processing_ai_prompt_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_SelfServiceAIAgentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.SelfServiceAIAgentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_SelfServiceAIAgentConfiguration as SelfServiceAIAgentConfiguration;
    impl crate::value::ToValue for SelfServiceAIAgentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.association_configurations {
                properties.insert(
                    "AssociationConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_service_ai_guardrail_id {
                properties.insert(
                    "SelfServiceAIGuardrailId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_service_answer_generation_ai_prompt_id {
                properties.insert(
                    "SelfServiceAnswerGenerationAIPromptId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.self_service_pre_processing_ai_prompt_id {
                properties.insert(
                    "SelfServicePreProcessingAIPromptId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-tagcondition.html
    pub struct TagCondition_ {
        pub key: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_TagCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.TagCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_TagCondition as TagCondition;
    impl crate::value::ToValue for TagCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiagent-tagfilter.html
    pub struct TagFilter_ {
        pub and_conditions: Option<Vec<TagCondition_>>,
        pub or_conditions: Option<Vec<OrCondition_>>,
        pub tag_condition: Option<Box<TagCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIAgent_TagFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIAgent.TagFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIAgent_TagFilter as TagFilter;
    impl crate::value::ToValue for TagFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and_conditions {
                properties.insert(
                    "AndConditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.or_conditions {
                properties.insert(
                    "OrConditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_condition {
                properties.insert(
                    "TagCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod aiguardrail {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-aiguardrailcontentpolicyconfig.html
    pub struct AIGuardrailContentPolicyConfig_ {
        pub filters_config: Vec<GuardrailContentFilterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_AIGuardrailContentPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.AIGuardrailContentPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_AIGuardrailContentPolicyConfig as AIGuardrailContentPolicyConfig;
    impl crate::value::ToValue for AIGuardrailContentPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FiltersConfig".to_string(),
                crate::value::ToValue::to_value(&self.filters_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-aiguardrailcontextualgroundingpolicyconfig.html
    pub struct AIGuardrailContextualGroundingPolicyConfig_ {
        pub filters_config: Vec<GuardrailContextualGroundingFilterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_AIGuardrailContextualGroundingPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.AIGuardrailContextualGroundingPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_AIGuardrailContextualGroundingPolicyConfig as AIGuardrailContextualGroundingPolicyConfig;
    impl crate::value::ToValue for AIGuardrailContextualGroundingPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FiltersConfig".to_string(),
                crate::value::ToValue::to_value(&self.filters_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-aiguardrailsensitiveinformationpolicyconfig.html
    pub struct AIGuardrailSensitiveInformationPolicyConfig_ {
        pub pii_entities_config: Option<Vec<GuardrailPiiEntityConfig_>>,
        pub regexes_config: Option<Vec<GuardrailRegexConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_AIGuardrailSensitiveInformationPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.AIGuardrailSensitiveInformationPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_AIGuardrailSensitiveInformationPolicyConfig as AIGuardrailSensitiveInformationPolicyConfig;
    impl crate::value::ToValue for AIGuardrailSensitiveInformationPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pii_entities_config {
                properties.insert(
                    "PiiEntitiesConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regexes_config {
                properties.insert(
                    "RegexesConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-aiguardrailtopicpolicyconfig.html
    pub struct AIGuardrailTopicPolicyConfig_ {
        pub topics_config: Vec<GuardrailTopicConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_AIGuardrailTopicPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.AIGuardrailTopicPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_AIGuardrailTopicPolicyConfig as AIGuardrailTopicPolicyConfig;
    impl crate::value::ToValue for AIGuardrailTopicPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TopicsConfig".to_string(),
                crate::value::ToValue::to_value(&self.topics_config),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-aiguardrailwordpolicyconfig.html
    pub struct AIGuardrailWordPolicyConfig_ {
        pub managed_word_lists_config: Option<Vec<GuardrailManagedWordsConfig_>>,
        pub words_config: Option<Vec<GuardrailWordConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_AIGuardrailWordPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.AIGuardrailWordPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_AIGuardrailWordPolicyConfig as AIGuardrailWordPolicyConfig;
    impl crate::value::ToValue for AIGuardrailWordPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.managed_word_lists_config {
                properties.insert(
                    "ManagedWordListsConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.words_config {
                properties.insert(
                    "WordsConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-guardrailcontentfilterconfig.html
    pub struct GuardrailContentFilterConfig_ {
        pub input_strength: crate::value::ExpString,
        pub output_strength: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_GuardrailContentFilterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.GuardrailContentFilterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_GuardrailContentFilterConfig as GuardrailContentFilterConfig;
    impl crate::value::ToValue for GuardrailContentFilterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InputStrength".to_string(),
                crate::value::ToValue::to_value(&self.input_strength),
            );
            properties.insert(
                "OutputStrength".to_string(),
                crate::value::ToValue::to_value(&self.output_strength),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-guardrailcontextualgroundingfilterconfig.html
    pub struct GuardrailContextualGroundingFilterConfig_ {
        pub threshold: f64,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_GuardrailContextualGroundingFilterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.GuardrailContextualGroundingFilterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_GuardrailContextualGroundingFilterConfig as GuardrailContextualGroundingFilterConfig;
    impl crate::value::ToValue for GuardrailContextualGroundingFilterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Threshold".to_string(),
                crate::value::ToValue::to_value(&self.threshold),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-guardrailmanagedwordsconfig.html
    pub struct GuardrailManagedWordsConfig_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_GuardrailManagedWordsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.GuardrailManagedWordsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_GuardrailManagedWordsConfig as GuardrailManagedWordsConfig;
    impl crate::value::ToValue for GuardrailManagedWordsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-guardrailpiientityconfig.html
    pub struct GuardrailPiiEntityConfig_ {
        pub action: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_GuardrailPiiEntityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.GuardrailPiiEntityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_GuardrailPiiEntityConfig as GuardrailPiiEntityConfig;
    impl crate::value::ToValue for GuardrailPiiEntityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-guardrailregexconfig.html
    pub struct GuardrailRegexConfig_ {
        pub action: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub pattern: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_GuardrailRegexConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.GuardrailRegexConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_GuardrailRegexConfig as GuardrailRegexConfig;
    impl crate::value::ToValue for GuardrailRegexConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
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
                "Pattern".to_string(),
                crate::value::ToValue::to_value(&self.pattern),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-guardrailtopicconfig.html
    pub struct GuardrailTopicConfig_ {
        pub definition: crate::value::ExpString,
        pub examples: Option<Vec<crate::value::ExpString>>,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_GuardrailTopicConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.GuardrailTopicConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_GuardrailTopicConfig as GuardrailTopicConfig;
    impl crate::value::ToValue for GuardrailTopicConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Definition".to_string(),
                crate::value::ToValue::to_value(&self.definition),
            );
            if let Some(ref value) = self.examples {
                properties.insert(
                    "Examples".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiguardrail-guardrailwordconfig.html
    pub struct GuardrailWordConfig_ {
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIGuardrail_GuardrailWordConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIGuardrail.GuardrailWordConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIGuardrail_GuardrailWordConfig as GuardrailWordConfig;
    impl crate::value::ToValue for GuardrailWordConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
}
pub mod aiprompt {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiprompt-aiprompttemplateconfiguration.html
    pub struct AIPromptTemplateConfiguration_ {
        pub text_full_ai_prompt_edit_template_configuration:
            Box<TextFullAIPromptEditTemplateConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIPrompt_AIPromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIPrompt.AIPromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIPrompt_AIPromptTemplateConfiguration as AIPromptTemplateConfiguration;
    impl crate::value::ToValue for AIPromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TextFullAIPromptEditTemplateConfiguration".to_string(),
                crate::value::ToValue::to_value(
                    &self.text_full_ai_prompt_edit_template_configuration,
                ),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-aiprompt-textfullaipromptedittemplateconfiguration.html
    pub struct TextFullAIPromptEditTemplateConfiguration_ {
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AIPrompt_TextFullAIPromptEditTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AIPrompt.TextFullAIPromptEditTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AIPrompt_TextFullAIPromptEditTemplateConfiguration as TextFullAIPromptEditTemplateConfiguration;
    impl crate::value::ToValue for TextFullAIPromptEditTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
}
pub mod assistant {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-assistant-serversideencryptionconfiguration.html
    pub struct ServerSideEncryptionConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_Assistant_ServerSideEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::Assistant.ServerSideEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_Assistant_ServerSideEncryptionConfiguration as ServerSideEncryptionConfiguration;
    impl crate::value::ToValue for ServerSideEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod assistantassociation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-assistantassociation-associationdata.html
    pub struct AssociationData_ {
        pub knowledge_base_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_AssistantAssociation_AssociationData {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::AssistantAssociation.AssociationData"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_AssistantAssociation_AssociationData as AssociationData;
    impl crate::value::ToValue for AssociationData_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KnowledgeBaseId".to_string(),
                crate::value::ToValue::to_value(&self.knowledge_base_id),
            );
            properties.into()
        }
    }
}
pub mod knowledgebase {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-appintegrationsconfiguration.html
    pub struct AppIntegrationsConfiguration_ {
        pub app_integration_arn: crate::value::ExpString,
        pub object_fields: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_AppIntegrationsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.AppIntegrationsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_AppIntegrationsConfiguration as AppIntegrationsConfiguration;
    impl crate::value::ToValue for AppIntegrationsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AppIntegrationArn".to_string(),
                crate::value::ToValue::to_value(&self.app_integration_arn),
            );
            if let Some(ref value) = self.object_fields {
                properties.insert(
                    "ObjectFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-bedrockfoundationmodelconfiguration.html
    pub struct BedrockFoundationModelConfiguration_ {
        pub model_arn: crate::value::ExpString,
        pub parsing_prompt: Option<Box<ParsingPrompt_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_BedrockFoundationModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.BedrockFoundationModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_BedrockFoundationModelConfiguration as BedrockFoundationModelConfiguration;
    impl crate::value::ToValue for BedrockFoundationModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ModelArn".to_string(),
                crate::value::ToValue::to_value(&self.model_arn),
            );
            if let Some(ref value) = self.parsing_prompt {
                properties.insert(
                    "ParsingPrompt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-chunkingconfiguration.html
    pub struct ChunkingConfiguration_ {
        pub chunking_strategy: crate::value::ExpString,
        pub fixed_size_chunking_configuration: Option<Box<FixedSizeChunkingConfiguration_>>,
        pub hierarchical_chunking_configuration: Option<Box<HierarchicalChunkingConfiguration_>>,
        pub semantic_chunking_configuration: Option<Box<SemanticChunkingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_ChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.ChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_ChunkingConfiguration as ChunkingConfiguration;
    impl crate::value::ToValue for ChunkingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChunkingStrategy".to_string(),
                crate::value::ToValue::to_value(&self.chunking_strategy),
            );
            if let Some(ref value) = self.fixed_size_chunking_configuration {
                properties.insert(
                    "FixedSizeChunkingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchical_chunking_configuration {
                properties.insert(
                    "HierarchicalChunkingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.semantic_chunking_configuration {
                properties.insert(
                    "SemanticChunkingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-crawlerlimits.html
    pub struct CrawlerLimits_ {
        pub rate_limit: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_CrawlerLimits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.CrawlerLimits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_CrawlerLimits as CrawlerLimits;
    impl crate::value::ToValue for CrawlerLimits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rate_limit {
                properties.insert(
                    "RateLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-fixedsizechunkingconfiguration.html
    pub struct FixedSizeChunkingConfiguration_ {
        pub max_tokens: f64,
        pub overlap_percentage: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_FixedSizeChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.FixedSizeChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_FixedSizeChunkingConfiguration as FixedSizeChunkingConfiguration;
    impl crate::value::ToValue for FixedSizeChunkingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxTokens".to_string(),
                crate::value::ToValue::to_value(&self.max_tokens),
            );
            properties.insert(
                "OverlapPercentage".to_string(),
                crate::value::ToValue::to_value(&self.overlap_percentage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-hierarchicalchunkingconfiguration.html
    pub struct HierarchicalChunkingConfiguration_ {
        pub level_configurations: Vec<HierarchicalChunkingLevelConfiguration_>,
        pub overlap_tokens: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_HierarchicalChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.HierarchicalChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_HierarchicalChunkingConfiguration as HierarchicalChunkingConfiguration;
    impl crate::value::ToValue for HierarchicalChunkingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LevelConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.level_configurations),
            );
            properties.insert(
                "OverlapTokens".to_string(),
                crate::value::ToValue::to_value(&self.overlap_tokens),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-hierarchicalchunkinglevelconfiguration.html
    pub struct HierarchicalChunkingLevelConfiguration_ {
        pub max_tokens: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_HierarchicalChunkingLevelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.HierarchicalChunkingLevelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_HierarchicalChunkingLevelConfiguration as HierarchicalChunkingLevelConfiguration;
    impl crate::value::ToValue for HierarchicalChunkingLevelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MaxTokens".to_string(),
                crate::value::ToValue::to_value(&self.max_tokens),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-managedsourceconfiguration.html
    pub struct ManagedSourceConfiguration_ {
        pub web_crawler_configuration: Box<WebCrawlerConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_ManagedSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.ManagedSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_ManagedSourceConfiguration as ManagedSourceConfiguration;
    impl crate::value::ToValue for ManagedSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "WebCrawlerConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.web_crawler_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-parsingconfiguration.html
    pub struct ParsingConfiguration_ {
        pub bedrock_foundation_model_configuration:
            Option<Box<BedrockFoundationModelConfiguration_>>,
        pub parsing_strategy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_ParsingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.ParsingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_ParsingConfiguration as ParsingConfiguration;
    impl crate::value::ToValue for ParsingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_foundation_model_configuration {
                properties.insert(
                    "BedrockFoundationModelConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ParsingStrategy".to_string(),
                crate::value::ToValue::to_value(&self.parsing_strategy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-parsingprompt.html
    pub struct ParsingPrompt_ {
        pub parsing_prompt_text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_ParsingPrompt {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.ParsingPrompt"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_ParsingPrompt as ParsingPrompt;
    impl crate::value::ToValue for ParsingPrompt_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParsingPromptText".to_string(),
                crate::value::ToValue::to_value(&self.parsing_prompt_text),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-renderingconfiguration.html
    pub struct RenderingConfiguration_ {
        pub template_uri: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_RenderingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.RenderingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_RenderingConfiguration as RenderingConfiguration;
    impl crate::value::ToValue for RenderingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.template_uri {
                properties.insert(
                    "TemplateUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-seedurl.html
    pub struct SeedUrl_ {
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_SeedUrl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.SeedUrl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_SeedUrl as SeedUrl;
    impl crate::value::ToValue for SeedUrl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-semanticchunkingconfiguration.html
    pub struct SemanticChunkingConfiguration_ {
        pub breakpoint_percentile_threshold: f64,
        pub buffer_size: f64,
        pub max_tokens: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_SemanticChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.SemanticChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_SemanticChunkingConfiguration as SemanticChunkingConfiguration;
    impl crate::value::ToValue for SemanticChunkingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BreakpointPercentileThreshold".to_string(),
                crate::value::ToValue::to_value(&self.breakpoint_percentile_threshold),
            );
            properties.insert(
                "BufferSize".to_string(),
                crate::value::ToValue::to_value(&self.buffer_size),
            );
            properties.insert(
                "MaxTokens".to_string(),
                crate::value::ToValue::to_value(&self.max_tokens),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-serversideencryptionconfiguration.html
    pub struct ServerSideEncryptionConfiguration_ {
        pub kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_ServerSideEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.ServerSideEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_ServerSideEncryptionConfiguration as ServerSideEncryptionConfiguration;
    impl crate::value::ToValue for ServerSideEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-sourceconfiguration.html
    pub struct SourceConfiguration_ {
        pub app_integrations: Option<Box<AppIntegrationsConfiguration_>>,
        pub managed_source_configuration: Option<Box<ManagedSourceConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_SourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.SourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_SourceConfiguration as SourceConfiguration;
    impl crate::value::ToValue for SourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_integrations {
                properties.insert(
                    "AppIntegrations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_source_configuration {
                properties.insert(
                    "ManagedSourceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-urlconfiguration.html
    pub struct UrlConfiguration_ {
        pub seed_urls: Option<Vec<SeedUrl_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_UrlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.UrlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_UrlConfiguration as UrlConfiguration;
    impl crate::value::ToValue for UrlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.seed_urls {
                properties.insert(
                    "SeedUrls".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-vectoringestionconfiguration.html
    pub struct VectorIngestionConfiguration_ {
        pub chunking_configuration: Option<Box<ChunkingConfiguration_>>,
        pub parsing_configuration: Option<Box<ParsingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_VectorIngestionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.VectorIngestionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_VectorIngestionConfiguration as VectorIngestionConfiguration;
    impl crate::value::ToValue for VectorIngestionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.chunking_configuration {
                properties.insert(
                    "ChunkingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parsing_configuration {
                properties.insert(
                    "ParsingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-knowledgebase-webcrawlerconfiguration.html
    pub struct WebCrawlerConfiguration_ {
        pub crawler_limits: Option<Box<CrawlerLimits_>>,
        pub exclusion_filters: Option<Vec<crate::value::ExpString>>,
        pub inclusion_filters: Option<Vec<crate::value::ExpString>>,
        pub scope: Option<crate::value::ExpString>,
        pub url_configuration: Box<UrlConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_KnowledgeBase_WebCrawlerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::KnowledgeBase.WebCrawlerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_KnowledgeBase_WebCrawlerConfiguration as WebCrawlerConfiguration;
    impl crate::value::ToValue for WebCrawlerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawler_limits {
                properties.insert(
                    "CrawlerLimits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclusion_filters {
                properties.insert(
                    "ExclusionFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_filters {
                properties.insert(
                    "InclusionFilters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "UrlConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.url_configuration),
            );
            properties.into()
        }
    }
}
pub mod messagetemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-agentattributes.html
    pub struct AgentAttributes_ {
        pub first_name: Option<crate::value::ExpString>,
        pub last_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_AgentAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.AgentAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_AgentAttributes as AgentAttributes;
    impl crate::value::ToValue for AgentAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.first_name {
                properties.insert(
                    "FirstName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_name {
                properties.insert(
                    "LastName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-content.html
    pub struct Content_ {
        pub email_message_template_content: Option<Box<EmailMessageTemplateContent_>>,
        pub sms_message_template_content: Option<Box<SmsMessageTemplateContent_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_Content {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.Content"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_Content as Content;
    impl crate::value::ToValue for Content_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email_message_template_content {
                properties.insert(
                    "EmailMessageTemplateContent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sms_message_template_content {
                properties.insert(
                    "SmsMessageTemplateContent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-customerprofileattributes.html
    pub struct CustomerProfileAttributes_ {
        pub account_number: Option<crate::value::ExpString>,
        pub additional_information: Option<crate::value::ExpString>,
        pub address1: Option<crate::value::ExpString>,
        pub address2: Option<crate::value::ExpString>,
        pub address3: Option<crate::value::ExpString>,
        pub address4: Option<crate::value::ExpString>,
        pub billing_address1: Option<crate::value::ExpString>,
        pub billing_address2: Option<crate::value::ExpString>,
        pub billing_address3: Option<crate::value::ExpString>,
        pub billing_address4: Option<crate::value::ExpString>,
        pub billing_city: Option<crate::value::ExpString>,
        pub billing_country: Option<crate::value::ExpString>,
        pub billing_county: Option<crate::value::ExpString>,
        pub billing_postal_code: Option<crate::value::ExpString>,
        pub billing_province: Option<crate::value::ExpString>,
        pub billing_state: Option<crate::value::ExpString>,
        pub birth_date: Option<crate::value::ExpString>,
        pub business_email_address: Option<crate::value::ExpString>,
        pub business_name: Option<crate::value::ExpString>,
        pub business_phone_number: Option<crate::value::ExpString>,
        pub city: Option<crate::value::ExpString>,
        pub country: Option<crate::value::ExpString>,
        pub county: Option<crate::value::ExpString>,
        pub custom: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub email_address: Option<crate::value::ExpString>,
        pub first_name: Option<crate::value::ExpString>,
        pub gender: Option<crate::value::ExpString>,
        pub home_phone_number: Option<crate::value::ExpString>,
        pub last_name: Option<crate::value::ExpString>,
        pub mailing_address1: Option<crate::value::ExpString>,
        pub mailing_address2: Option<crate::value::ExpString>,
        pub mailing_address3: Option<crate::value::ExpString>,
        pub mailing_address4: Option<crate::value::ExpString>,
        pub mailing_city: Option<crate::value::ExpString>,
        pub mailing_country: Option<crate::value::ExpString>,
        pub mailing_county: Option<crate::value::ExpString>,
        pub mailing_postal_code: Option<crate::value::ExpString>,
        pub mailing_province: Option<crate::value::ExpString>,
        pub mailing_state: Option<crate::value::ExpString>,
        pub middle_name: Option<crate::value::ExpString>,
        pub mobile_phone_number: Option<crate::value::ExpString>,
        pub party_type: Option<crate::value::ExpString>,
        pub phone_number: Option<crate::value::ExpString>,
        pub postal_code: Option<crate::value::ExpString>,
        pub profile_arn: Option<crate::value::ExpString>,
        pub profile_id: Option<crate::value::ExpString>,
        pub province: Option<crate::value::ExpString>,
        pub shipping_address1: Option<crate::value::ExpString>,
        pub shipping_address2: Option<crate::value::ExpString>,
        pub shipping_address3: Option<crate::value::ExpString>,
        pub shipping_address4: Option<crate::value::ExpString>,
        pub shipping_city: Option<crate::value::ExpString>,
        pub shipping_country: Option<crate::value::ExpString>,
        pub shipping_county: Option<crate::value::ExpString>,
        pub shipping_postal_code: Option<crate::value::ExpString>,
        pub shipping_province: Option<crate::value::ExpString>,
        pub shipping_state: Option<crate::value::ExpString>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_CustomerProfileAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.CustomerProfileAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_CustomerProfileAttributes as CustomerProfileAttributes;
    impl crate::value::ToValue for CustomerProfileAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.account_number {
                properties.insert(
                    "AccountNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.additional_information {
                properties.insert(
                    "AdditionalInformation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.address1 {
                properties.insert(
                    "Address1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.address2 {
                properties.insert(
                    "Address2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.address3 {
                properties.insert(
                    "Address3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.address4 {
                properties.insert(
                    "Address4".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_address1 {
                properties.insert(
                    "BillingAddress1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_address2 {
                properties.insert(
                    "BillingAddress2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_address3 {
                properties.insert(
                    "BillingAddress3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_address4 {
                properties.insert(
                    "BillingAddress4".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_city {
                properties.insert(
                    "BillingCity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_country {
                properties.insert(
                    "BillingCountry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_county {
                properties.insert(
                    "BillingCounty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_postal_code {
                properties.insert(
                    "BillingPostalCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_province {
                properties.insert(
                    "BillingProvince".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.billing_state {
                properties.insert(
                    "BillingState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.birth_date {
                properties.insert(
                    "BirthDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_email_address {
                properties.insert(
                    "BusinessEmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_name {
                properties.insert(
                    "BusinessName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.business_phone_number {
                properties.insert(
                    "BusinessPhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.city {
                properties.insert("City".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.country {
                properties.insert(
                    "Country".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.county {
                properties.insert("County".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.custom {
                properties.insert("Custom".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.email_address {
                properties.insert(
                    "EmailAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.first_name {
                properties.insert(
                    "FirstName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gender {
                properties.insert("Gender".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.home_phone_number {
                properties.insert(
                    "HomePhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_name {
                properties.insert(
                    "LastName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_address1 {
                properties.insert(
                    "MailingAddress1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_address2 {
                properties.insert(
                    "MailingAddress2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_address3 {
                properties.insert(
                    "MailingAddress3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_address4 {
                properties.insert(
                    "MailingAddress4".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_city {
                properties.insert(
                    "MailingCity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_country {
                properties.insert(
                    "MailingCountry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_county {
                properties.insert(
                    "MailingCounty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_postal_code {
                properties.insert(
                    "MailingPostalCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_province {
                properties.insert(
                    "MailingProvince".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mailing_state {
                properties.insert(
                    "MailingState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.middle_name {
                properties.insert(
                    "MiddleName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mobile_phone_number {
                properties.insert(
                    "MobilePhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.party_type {
                properties.insert(
                    "PartyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.phone_number {
                properties.insert(
                    "PhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.postal_code {
                properties.insert(
                    "PostalCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_arn {
                properties.insert(
                    "ProfileARN".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.profile_id {
                properties.insert(
                    "ProfileId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.province {
                properties.insert(
                    "Province".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_address1 {
                properties.insert(
                    "ShippingAddress1".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_address2 {
                properties.insert(
                    "ShippingAddress2".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_address3 {
                properties.insert(
                    "ShippingAddress3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_address4 {
                properties.insert(
                    "ShippingAddress4".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_city {
                properties.insert(
                    "ShippingCity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_country {
                properties.insert(
                    "ShippingCountry".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_county {
                properties.insert(
                    "ShippingCounty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_postal_code {
                properties.insert(
                    "ShippingPostalCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_province {
                properties.insert(
                    "ShippingProvince".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.shipping_state {
                properties.insert(
                    "ShippingState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-emailmessagetemplatecontent.html
    pub struct EmailMessageTemplateContent_ {
        pub body: Box<EmailMessageTemplateContentBody_>,
        pub headers: Vec<EmailMessageTemplateHeader_>,
        pub subject: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_EmailMessageTemplateContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.EmailMessageTemplateContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_EmailMessageTemplateContent as EmailMessageTemplateContent;
    impl crate::value::ToValue for EmailMessageTemplateContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Body".to_string(),
                crate::value::ToValue::to_value(&self.body),
            );
            properties.insert(
                "Headers".to_string(),
                crate::value::ToValue::to_value(&self.headers),
            );
            properties.insert(
                "Subject".to_string(),
                crate::value::ToValue::to_value(&self.subject),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-emailmessagetemplatecontentbody.html
    pub struct EmailMessageTemplateContentBody_ {
        pub html: Option<Box<MessageTemplateBodyContentProvider_>>,
        pub plain_text: Option<Box<MessageTemplateBodyContentProvider_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_EmailMessageTemplateContentBody {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.EmailMessageTemplateContentBody"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_EmailMessageTemplateContentBody as EmailMessageTemplateContentBody;
    impl crate::value::ToValue for EmailMessageTemplateContentBody_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.html {
                properties.insert("Html".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.plain_text {
                properties.insert(
                    "PlainText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-emailmessagetemplateheader.html
    pub struct EmailMessageTemplateHeader_ {
        pub name: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_EmailMessageTemplateHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.EmailMessageTemplateHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_EmailMessageTemplateHeader as EmailMessageTemplateHeader;
    impl crate::value::ToValue for EmailMessageTemplateHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-groupingconfiguration.html
    pub struct GroupingConfiguration_ {
        pub criteria: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_GroupingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.GroupingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_GroupingConfiguration as GroupingConfiguration;
    impl crate::value::ToValue for GroupingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Criteria".to_string(),
                crate::value::ToValue::to_value(&self.criteria),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-messagetemplateattachment.html
    pub struct MessageTemplateAttachment_ {
        pub attachment_id: Option<crate::value::ExpString>,
        pub attachment_name: crate::value::ExpString,
        pub s3_presigned_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_MessageTemplateAttachment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.MessageTemplateAttachment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_MessageTemplateAttachment as MessageTemplateAttachment;
    impl crate::value::ToValue for MessageTemplateAttachment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attachment_id {
                properties.insert(
                    "AttachmentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AttachmentName".to_string(),
                crate::value::ToValue::to_value(&self.attachment_name),
            );
            properties.insert(
                "S3PresignedUrl".to_string(),
                crate::value::ToValue::to_value(&self.s3_presigned_url),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-messagetemplateattributes.html
    pub struct MessageTemplateAttributes_ {
        pub agent_attributes: Option<Box<AgentAttributes_>>,
        pub custom_attributes: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub customer_profile_attributes: Option<Box<CustomerProfileAttributes_>>,
        pub system_attributes: Option<Box<SystemAttributes_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_MessageTemplateAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.MessageTemplateAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_MessageTemplateAttributes as MessageTemplateAttributes;
    impl crate::value::ToValue for MessageTemplateAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agent_attributes {
                properties.insert(
                    "AgentAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_attributes {
                properties.insert(
                    "CustomAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.customer_profile_attributes {
                properties.insert(
                    "CustomerProfileAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.system_attributes {
                properties.insert(
                    "SystemAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-messagetemplatebodycontentprovider.html
    pub struct MessageTemplateBodyContentProvider_ {
        pub content: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_MessageTemplateBodyContentProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.MessageTemplateBodyContentProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_MessageTemplateBodyContentProvider as MessageTemplateBodyContentProvider;
    impl crate::value::ToValue for MessageTemplateBodyContentProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-smsmessagetemplatecontent.html
    pub struct SmsMessageTemplateContent_ {
        pub body: Box<SmsMessageTemplateContentBody_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_SmsMessageTemplateContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.SmsMessageTemplateContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_SmsMessageTemplateContent as SmsMessageTemplateContent;
    impl crate::value::ToValue for SmsMessageTemplateContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Body".to_string(),
                crate::value::ToValue::to_value(&self.body),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-smsmessagetemplatecontentbody.html
    pub struct SmsMessageTemplateContentBody_ {
        pub plain_text: Option<Box<MessageTemplateBodyContentProvider_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_SmsMessageTemplateContentBody {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.SmsMessageTemplateContentBody"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_SmsMessageTemplateContentBody as SmsMessageTemplateContentBody;
    impl crate::value::ToValue for SmsMessageTemplateContentBody_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.plain_text {
                properties.insert(
                    "PlainText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-systemattributes.html
    pub struct SystemAttributes_ {
        pub customer_endpoint: Option<Box<SystemEndpointAttributes_>>,
        pub name: Option<crate::value::ExpString>,
        pub system_endpoint: Option<Box<SystemEndpointAttributes_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_SystemAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.SystemAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_SystemAttributes as SystemAttributes;
    impl crate::value::ToValue for SystemAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_endpoint {
                properties.insert(
                    "CustomerEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.system_endpoint {
                properties.insert(
                    "SystemEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-messagetemplate-systemendpointattributes.html
    pub struct SystemEndpointAttributes_ {
        pub address: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_MessageTemplate_SystemEndpointAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::MessageTemplate.SystemEndpointAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_MessageTemplate_SystemEndpointAttributes as SystemEndpointAttributes;
    impl crate::value::ToValue for SystemEndpointAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod quickresponse {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-quickresponse-groupingconfiguration.html
    pub struct GroupingConfiguration_ {
        pub criteria: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_QuickResponse_GroupingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::QuickResponse.GroupingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_QuickResponse_GroupingConfiguration as GroupingConfiguration;
    impl crate::value::ToValue for GroupingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Criteria".to_string(),
                crate::value::ToValue::to_value(&self.criteria),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-quickresponse-quickresponsecontentprovider.html
    pub struct QuickResponseContentProvider_ {
        pub content: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_QuickResponse_QuickResponseContentProvider {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::QuickResponse.QuickResponseContentProvider"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_QuickResponse_QuickResponseContentProvider as QuickResponseContentProvider;
    impl crate::value::ToValue for QuickResponseContentProvider_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wisdom-quickresponse-quickresponsecontents.html
    pub struct QuickResponseContents_ {
        pub markdown: Option<Box<QuickResponseContentProvider_>>,
        pub plain_text: Option<Box<QuickResponseContentProvider_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wisdom_QuickResponse_QuickResponseContents {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Wisdom::QuickResponse.QuickResponseContents"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wisdom_QuickResponse_QuickResponseContents as QuickResponseContents;
    impl crate::value::ToValue for QuickResponseContents_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.markdown {
                properties.insert(
                    "Markdown".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.plain_text {
                properties.insert(
                    "PlainText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-aiagent.html
pub struct AIAgent_ {
    pub assistant_id: crate::value::ExpString,
    pub configuration: super::wisdom::aiagent::AIAgentConfiguration_,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_AIAgent {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::AIAgent" $($field
        $value)*)
    };
}
pub use crate::__aws_wisdom_AIAgent as AIAgent;
impl crate::template::ToResource for AIAgent_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AIAgent"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AssistantId".to_string(),
            crate::value::ToValue::to_value(&self.assistant_id),
        );
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-aiagentversion.html
pub struct AIAgentVersion_ {
    pub ai_agent_id: crate::value::ExpString,
    pub assistant_id: crate::value::ExpString,
    pub modified_time_seconds: Option<f64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_AIAgentVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::AIAgentVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_AIAgentVersion as AIAgentVersion;
impl crate::template::ToResource for AIAgentVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AIAgentVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AIAgentId".to_string(),
            crate::value::ToValue::to_value(&self.ai_agent_id),
        );
        properties.insert(
            "AssistantId".to_string(),
            crate::value::ToValue::to_value(&self.assistant_id),
        );
        if let Some(ref value) = self.modified_time_seconds {
            properties.insert(
                "ModifiedTimeSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-aiguardrail.html
pub struct AIGuardrail_ {
    pub assistant_id: crate::value::ExpString,
    pub blocked_input_messaging: crate::value::ExpString,
    pub blocked_outputs_messaging: crate::value::ExpString,
    pub content_policy_config: Option<super::wisdom::aiguardrail::AIGuardrailContentPolicyConfig_>,
    pub contextual_grounding_policy_config:
        Option<super::wisdom::aiguardrail::AIGuardrailContextualGroundingPolicyConfig_>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub sensitive_information_policy_config:
        Option<super::wisdom::aiguardrail::AIGuardrailSensitiveInformationPolicyConfig_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub topic_policy_config: Option<super::wisdom::aiguardrail::AIGuardrailTopicPolicyConfig_>,
    pub word_policy_config: Option<super::wisdom::aiguardrail::AIGuardrailWordPolicyConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_AIGuardrail {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::AIGuardrail"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_AIGuardrail as AIGuardrail;
impl crate::template::ToResource for AIGuardrail_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AIGuardrail"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AssistantId".to_string(),
            crate::value::ToValue::to_value(&self.assistant_id),
        );
        properties.insert(
            "BlockedInputMessaging".to_string(),
            crate::value::ToValue::to_value(&self.blocked_input_messaging),
        );
        properties.insert(
            "BlockedOutputsMessaging".to_string(),
            crate::value::ToValue::to_value(&self.blocked_outputs_messaging),
        );
        if let Some(ref value) = self.content_policy_config {
            properties.insert(
                "ContentPolicyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.contextual_grounding_policy_config {
            properties.insert(
                "ContextualGroundingPolicyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.sensitive_information_policy_config {
            properties.insert(
                "SensitiveInformationPolicyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.topic_policy_config {
            properties.insert(
                "TopicPolicyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.word_policy_config {
            properties.insert(
                "WordPolicyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-aiguardrailversion.html
pub struct AIGuardrailVersion_ {
    pub ai_guardrail_id: crate::value::ExpString,
    pub assistant_id: crate::value::ExpString,
    pub modified_time_seconds: Option<f64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_AIGuardrailVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::AIGuardrailVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_AIGuardrailVersion as AIGuardrailVersion;
impl crate::template::ToResource for AIGuardrailVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AIGuardrailVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AIGuardrailId".to_string(),
            crate::value::ToValue::to_value(&self.ai_guardrail_id),
        );
        properties.insert(
            "AssistantId".to_string(),
            crate::value::ToValue::to_value(&self.assistant_id),
        );
        if let Some(ref value) = self.modified_time_seconds {
            properties.insert(
                "ModifiedTimeSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-aiprompt.html
pub struct AIPrompt_ {
    pub api_format: crate::value::ExpString,
    pub assistant_id: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub model_id: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub template_configuration: super::wisdom::aiprompt::AIPromptTemplateConfiguration_,
    pub template_type: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_AIPrompt {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::AIPrompt" $($field
        $value)*)
    };
}
pub use crate::__aws_wisdom_AIPrompt as AIPrompt;
impl crate::template::ToResource for AIPrompt_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AIPrompt"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiFormat".to_string(),
            crate::value::ToValue::to_value(&self.api_format),
        );
        if let Some(ref value) = self.assistant_id {
            properties.insert(
                "AssistantId".to_string(),
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
            "ModelId".to_string(),
            crate::value::ToValue::to_value(&self.model_id),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TemplateConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.template_configuration),
        );
        properties.insert(
            "TemplateType".to_string(),
            crate::value::ToValue::to_value(&self.template_type),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-aipromptversion.html
pub struct AIPromptVersion_ {
    pub ai_prompt_id: crate::value::ExpString,
    pub assistant_id: crate::value::ExpString,
    pub modified_time_seconds: Option<f64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_AIPromptVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::AIPromptVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_AIPromptVersion as AIPromptVersion;
impl crate::template::ToResource for AIPromptVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AIPromptVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AIPromptId".to_string(),
            crate::value::ToValue::to_value(&self.ai_prompt_id),
        );
        properties.insert(
            "AssistantId".to_string(),
            crate::value::ToValue::to_value(&self.assistant_id),
        );
        if let Some(ref value) = self.modified_time_seconds {
            properties.insert(
                "ModifiedTimeSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistant.html
pub struct Assistant_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub server_side_encryption_configuration:
        Option<super::wisdom::assistant::ServerSideEncryptionConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_Assistant {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::Assistant"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_Assistant as Assistant;
impl crate::template::ToResource for Assistant_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Assistant"),
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
        if let Some(ref value) = self.server_side_encryption_configuration {
            properties.insert(
                "ServerSideEncryptionConfiguration".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-assistantassociation.html
pub struct AssistantAssociation_ {
    pub assistant_id: crate::value::ExpString,
    pub association: super::wisdom::assistantassociation::AssociationData_,
    pub association_type: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_AssistantAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::AssistantAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_AssistantAssociation as AssistantAssociation;
impl crate::template::ToResource for AssistantAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AssistantAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AssistantId".to_string(),
            crate::value::ToValue::to_value(&self.assistant_id),
        );
        properties.insert(
            "Association".to_string(),
            crate::value::ToValue::to_value(&self.association),
        );
        properties.insert(
            "AssociationType".to_string(),
            crate::value::ToValue::to_value(&self.association_type),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-knowledgebase.html
pub struct KnowledgeBase_ {
    pub description: Option<crate::value::ExpString>,
    pub knowledge_base_type: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub rendering_configuration: Option<super::wisdom::knowledgebase::RenderingConfiguration_>,
    pub server_side_encryption_configuration:
        Option<super::wisdom::knowledgebase::ServerSideEncryptionConfiguration_>,
    pub source_configuration: Option<super::wisdom::knowledgebase::SourceConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vector_ingestion_configuration:
        Option<super::wisdom::knowledgebase::VectorIngestionConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_KnowledgeBase {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::KnowledgeBase"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_KnowledgeBase as KnowledgeBase;
impl crate::template::ToResource for KnowledgeBase_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("KnowledgeBase"),
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
            "KnowledgeBaseType".to_string(),
            crate::value::ToValue::to_value(&self.knowledge_base_type),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.rendering_configuration {
            properties.insert(
                "RenderingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.server_side_encryption_configuration {
            properties.insert(
                "ServerSideEncryptionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_configuration {
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vector_ingestion_configuration {
            properties.insert(
                "VectorIngestionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-messagetemplate.html
pub struct MessageTemplate_ {
    pub channel_subtype: crate::value::ExpString,
    pub content: super::wisdom::messagetemplate::Content_,
    pub default_attributes: Option<super::wisdom::messagetemplate::MessageTemplateAttributes_>,
    pub description: Option<crate::value::ExpString>,
    pub grouping_configuration: Option<super::wisdom::messagetemplate::GroupingConfiguration_>,
    pub knowledge_base_arn: crate::value::ExpString,
    pub language: Option<crate::value::ExpString>,
    pub message_template_attachments:
        Option<Vec<super::wisdom::messagetemplate::MessageTemplateAttachment_>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_MessageTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::MessageTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_MessageTemplate as MessageTemplate;
impl crate::template::ToResource for MessageTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MessageTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelSubtype".to_string(),
            crate::value::ToValue::to_value(&self.channel_subtype),
        );
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.default_attributes {
            properties.insert(
                "DefaultAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.grouping_configuration {
            properties.insert(
                "GroupingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KnowledgeBaseArn".to_string(),
            crate::value::ToValue::to_value(&self.knowledge_base_arn),
        );
        if let Some(ref value) = self.language {
            properties.insert(
                "Language".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.message_template_attachments {
            properties.insert(
                "MessageTemplateAttachments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-messagetemplateversion.html
pub struct MessageTemplateVersion_ {
    pub message_template_arn: crate::value::ExpString,
    pub message_template_content_sha256: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_MessageTemplateVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::MessageTemplateVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_MessageTemplateVersion as MessageTemplateVersion;
impl crate::template::ToResource for MessageTemplateVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MessageTemplateVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MessageTemplateArn".to_string(),
            crate::value::ToValue::to_value(&self.message_template_arn),
        );
        if let Some(ref value) = self.message_template_content_sha256 {
            properties.insert(
                "MessageTemplateContentSha256".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wisdom-quickresponse.html
pub struct QuickResponse_ {
    pub channels: Option<Vec<crate::value::ExpString>>,
    pub content: super::wisdom::quickresponse::QuickResponseContentProvider_,
    pub content_type: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub grouping_configuration: Option<super::wisdom::quickresponse::GroupingConfiguration_>,
    pub is_active: Option<crate::value::ExpBool>,
    pub knowledge_base_arn: crate::value::ExpString,
    pub language: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub shortcut_key: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wisdom_QuickResponse {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Wisdom::QuickResponse"
        $($field $value)*)
    };
}
pub use crate::__aws_wisdom_QuickResponse as QuickResponse;
impl crate::template::ToResource for QuickResponse_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Wisdom"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QuickResponse"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.channels {
            properties.insert(
                "Channels".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.content_type {
            properties.insert(
                "ContentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.grouping_configuration {
            properties.insert(
                "GroupingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_active {
            properties.insert(
                "IsActive".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KnowledgeBaseArn".to_string(),
            crate::value::ToValue::to_value(&self.knowledge_base_arn),
        );
        if let Some(ref value) = self.language {
            properties.insert(
                "Language".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.shortcut_key {
            properties.insert(
                "ShortcutKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
