pub mod agent {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-apischema.html>
    pub struct APISchema_ {
        pub payload: Option<crate::value::ExpString>,
        pub s3: Option<Box<S3Identifier_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_APISchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.APISchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_APISchema as APISchema;
    impl crate::value::ToValue for APISchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.payload {
                properties.insert(
                    "Payload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-actiongroupexecutor.html>
    pub struct ActionGroupExecutor_ {
        pub custom_control: Option<crate::value::ExpString>,
        pub lambda: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_ActionGroupExecutor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.ActionGroupExecutor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_ActionGroupExecutor as ActionGroupExecutor;
    impl crate::value::ToValue for ActionGroupExecutor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_control {
                properties.insert(
                    "CustomControl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-agentactiongroup.html>
    pub struct AgentActionGroup_ {
        pub action_group_executor: Option<Box<ActionGroupExecutor_>>,
        pub action_group_name: crate::value::ExpString,
        pub action_group_state: Option<crate::value::ExpString>,
        pub api_schema: Option<Box<APISchema_>>,
        pub description: Option<crate::value::ExpString>,
        pub function_schema: Option<Box<FunctionSchema_>>,
        pub parent_action_group_signature: Option<crate::value::ExpString>,
        pub skip_resource_in_use_check_on_delete: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_AgentActionGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.AgentActionGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_AgentActionGroup as AgentActionGroup;
    impl crate::value::ToValue for AgentActionGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_group_executor {
                properties.insert(
                    "ActionGroupExecutor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ActionGroupName".to_string(),
                crate::value::ToValue::to_value(&self.action_group_name),
            );
            if let Some(ref value) = self.action_group_state {
                properties.insert(
                    "ActionGroupState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.api_schema {
                properties.insert(
                    "ApiSchema".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.function_schema {
                properties.insert(
                    "FunctionSchema".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parent_action_group_signature {
                properties.insert(
                    "ParentActionGroupSignature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.skip_resource_in_use_check_on_delete {
                properties.insert(
                    "SkipResourceInUseCheckOnDelete".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-agentcollaborator.html>
    pub struct AgentCollaborator_ {
        pub agent_descriptor: Box<AgentDescriptor_>,
        pub collaboration_instruction: crate::value::ExpString,
        pub collaborator_name: crate::value::ExpString,
        pub relay_conversation_history: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_AgentCollaborator {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.AgentCollaborator"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_AgentCollaborator as AgentCollaborator;
    impl crate::value::ToValue for AgentCollaborator_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentDescriptor".to_string(),
                crate::value::ToValue::to_value(&self.agent_descriptor),
            );
            properties.insert(
                "CollaborationInstruction".to_string(),
                crate::value::ToValue::to_value(&self.collaboration_instruction),
            );
            properties.insert(
                "CollaboratorName".to_string(),
                crate::value::ToValue::to_value(&self.collaborator_name),
            );
            if let Some(ref value) = self.relay_conversation_history {
                properties.insert(
                    "RelayConversationHistory".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-agentdescriptor.html>
    pub struct AgentDescriptor_ {
        pub alias_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_AgentDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.AgentDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_AgentDescriptor as AgentDescriptor;
    impl crate::value::ToValue for AgentDescriptor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alias_arn {
                properties.insert(
                    "AliasArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-agentknowledgebase.html>
    pub struct AgentKnowledgeBase_ {
        pub description: crate::value::ExpString,
        pub knowledge_base_id: crate::value::ExpString,
        pub knowledge_base_state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_AgentKnowledgeBase {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.AgentKnowledgeBase"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_AgentKnowledgeBase as AgentKnowledgeBase;
    impl crate::value::ToValue for AgentKnowledgeBase_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(&self.description),
            );
            properties.insert(
                "KnowledgeBaseId".to_string(),
                crate::value::ToValue::to_value(&self.knowledge_base_id),
            );
            if let Some(ref value) = self.knowledge_base_state {
                properties.insert(
                    "KnowledgeBaseState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-customorchestration.html>
    pub struct CustomOrchestration_ {
        pub executor: Option<Box<OrchestrationExecutor_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_CustomOrchestration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.CustomOrchestration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_CustomOrchestration as CustomOrchestration;
    impl crate::value::ToValue for CustomOrchestration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.executor {
                properties.insert(
                    "Executor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-function.html>
    pub struct Function_ {
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub parameters: Option<std::collections::BTreeMap<String, ParameterDetail_>>,
        pub require_confirmation: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_Function {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.Function"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_Function as Function;
    impl crate::value::ToValue for Function_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.require_confirmation {
                properties.insert(
                    "RequireConfirmation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-functionschema.html>
    pub struct FunctionSchema_ {
        pub functions: Vec<Function_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_FunctionSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.FunctionSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_FunctionSchema as FunctionSchema;
    impl crate::value::ToValue for FunctionSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Functions".to_string(),
                crate::value::ToValue::to_value(&self.functions),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-guardrailconfiguration.html>
    pub struct GuardrailConfiguration_ {
        pub guardrail_identifier: Option<crate::value::ExpString>,
        pub guardrail_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_GuardrailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.GuardrailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_GuardrailConfiguration as GuardrailConfiguration;
    impl crate::value::ToValue for GuardrailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.guardrail_identifier {
                properties.insert(
                    "GuardrailIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.guardrail_version {
                properties.insert(
                    "GuardrailVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-inferenceconfiguration.html>
    pub struct InferenceConfiguration_ {
        pub maximum_length: Option<f64>,
        pub stop_sequences: Option<Vec<crate::value::ExpString>>,
        pub temperature: Option<f64>,
        pub top_k: Option<f64>,
        pub top_p: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_InferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.InferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_InferenceConfiguration as InferenceConfiguration;
    impl crate::value::ToValue for InferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.maximum_length {
                properties.insert(
                    "MaximumLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_sequences {
                properties.insert(
                    "StopSequences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temperature {
                properties.insert(
                    "Temperature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.top_k {
                properties.insert("TopK".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.top_p {
                properties.insert("TopP".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-memoryconfiguration.html>
    pub struct MemoryConfiguration_ {
        pub enabled_memory_types: Option<Vec<crate::value::ExpString>>,
        pub session_summary_configuration: Option<Box<SessionSummaryConfiguration_>>,
        pub storage_days: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_MemoryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.MemoryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_MemoryConfiguration as MemoryConfiguration;
    impl crate::value::ToValue for MemoryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled_memory_types {
                properties.insert(
                    "EnabledMemoryTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_summary_configuration {
                properties.insert(
                    "SessionSummaryConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage_days {
                properties.insert(
                    "StorageDays".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-orchestrationexecutor.html>
    pub struct OrchestrationExecutor_ {
        pub lambda: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_OrchestrationExecutor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.OrchestrationExecutor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_OrchestrationExecutor as OrchestrationExecutor;
    impl crate::value::ToValue for OrchestrationExecutor_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Lambda".to_string(),
                crate::value::ToValue::to_value(&self.lambda),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-parameterdetail.html>
    pub struct ParameterDetail_ {
        pub description: Option<crate::value::ExpString>,
        pub required: Option<crate::value::ExpBool>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_ParameterDetail {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.ParameterDetail"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_ParameterDetail as ParameterDetail;
    impl crate::value::ToValue for ParameterDetail_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.required {
                properties.insert(
                    "Required".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-promptconfiguration.html>
    pub struct PromptConfiguration_ {
        pub additional_model_request_fields: Option<serde_json::Value>,
        pub base_prompt_template: Option<crate::value::ExpString>,
        pub foundation_model: Option<crate::value::ExpString>,
        pub inference_configuration: Option<Box<InferenceConfiguration_>>,
        pub parser_mode: Option<crate::value::ExpString>,
        pub prompt_creation_mode: Option<crate::value::ExpString>,
        pub prompt_state: Option<crate::value::ExpString>,
        pub prompt_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_PromptConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.PromptConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_PromptConfiguration as PromptConfiguration;
    impl crate::value::ToValue for PromptConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_model_request_fields {
                properties.insert(
                    "AdditionalModelRequestFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.base_prompt_template {
                properties.insert(
                    "BasePromptTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.foundation_model {
                properties.insert(
                    "FoundationModel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_configuration {
                properties.insert(
                    "InferenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parser_mode {
                properties.insert(
                    "ParserMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_creation_mode {
                properties.insert(
                    "PromptCreationMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_state {
                properties.insert(
                    "PromptState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_type {
                properties.insert(
                    "PromptType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-promptoverrideconfiguration.html>
    pub struct PromptOverrideConfiguration_ {
        pub override_lambda: Option<crate::value::ExpString>,
        pub prompt_configurations: Vec<PromptConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_PromptOverrideConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.PromptOverrideConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_PromptOverrideConfiguration as PromptOverrideConfiguration;
    impl crate::value::ToValue for PromptOverrideConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.override_lambda {
                properties.insert(
                    "OverrideLambda".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PromptConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.prompt_configurations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-s3identifier.html>
    pub struct S3Identifier_ {
        pub s3_bucket_name: Option<crate::value::ExpString>,
        pub s3_object_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_S3Identifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.S3Identifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_S3Identifier as S3Identifier;
    impl crate::value::ToValue for S3Identifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_bucket_name {
                properties.insert(
                    "S3BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_object_key {
                properties.insert(
                    "S3ObjectKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agent-sessionsummaryconfiguration.html>
    pub struct SessionSummaryConfiguration_ {
        pub max_recent_sessions: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Agent_SessionSummaryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Agent.SessionSummaryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Agent_SessionSummaryConfiguration as SessionSummaryConfiguration;
    impl crate::value::ToValue for SessionSummaryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_recent_sessions {
                properties.insert(
                    "MaxRecentSessions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod agentalias {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agentalias-agentaliashistoryevent.html>
    pub struct AgentAliasHistoryEvent_ {
        pub end_date: Option<crate::value::ExpString>,
        pub routing_configuration: Option<Vec<AgentAliasRoutingConfigurationListItem_>>,
        pub start_date: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_AgentAlias_AgentAliasHistoryEvent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::AgentAlias.AgentAliasHistoryEvent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_AgentAlias_AgentAliasHistoryEvent as AgentAliasHistoryEvent;
    impl crate::value::ToValue for AgentAliasHistoryEvent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_date {
                properties.insert(
                    "EndDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.routing_configuration {
                properties.insert(
                    "RoutingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_date {
                properties.insert(
                    "StartDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-agentalias-agentaliasroutingconfigurationlistitem.html>
    pub struct AgentAliasRoutingConfigurationListItem_ {
        pub agent_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_AgentAlias_AgentAliasRoutingConfigurationListItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::AgentAlias.AgentAliasRoutingConfigurationListItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_AgentAlias_AgentAliasRoutingConfigurationListItem as AgentAliasRoutingConfigurationListItem;
    impl crate::value::ToValue for AgentAliasRoutingConfigurationListItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentVersion".to_string(),
                crate::value::ToValue::to_value(&self.agent_version),
            );
            properties.into()
        }
    }
}
pub mod applicationinferenceprofile {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-applicationinferenceprofile-inferenceprofilemodel.html>
    pub struct InferenceProfileModel_ {
        pub model_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_ApplicationInferenceProfile_InferenceProfileModel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::ApplicationInferenceProfile.InferenceProfileModel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_ApplicationInferenceProfile_InferenceProfileModel as InferenceProfileModel;
    impl crate::value::ToValue for InferenceProfileModel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.model_arn {
                properties.insert(
                    "ModelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-applicationinferenceprofile-inferenceprofilemodelsource.html>
    pub struct InferenceProfileModelSource_ {
        pub copy_from: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_ApplicationInferenceProfile_InferenceProfileModelSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::ApplicationInferenceProfile.InferenceProfileModelSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_ApplicationInferenceProfile_InferenceProfileModelSource as InferenceProfileModelSource;
    impl crate::value::ToValue for InferenceProfileModelSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CopyFrom".to_string(),
                crate::value::ToValue::to_value(&self.copy_from),
            );
            properties.into()
        }
    }
}
pub mod automatedreasoningpolicy {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-automatedreasoningpolicy-policydefinition.html>
    pub struct PolicyDefinition_ {
        pub rules: Option<Vec<PolicyDefinitionRule_>>,
        pub types: Option<Vec<PolicyDefinitionType_>>,
        pub variables: Option<Vec<PolicyDefinitionVariable_>>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_AutomatedReasoningPolicy_PolicyDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::AutomatedReasoningPolicy.PolicyDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_AutomatedReasoningPolicy_PolicyDefinition as PolicyDefinition;
    impl crate::value::ToValue for PolicyDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.rules {
                properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.variables {
                properties.insert(
                    "Variables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-automatedreasoningpolicy-policydefinitionrule.html>
    pub struct PolicyDefinitionRule_ {
        pub alternate_expression: Option<crate::value::ExpString>,
        pub expression: crate::value::ExpString,
        pub id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::AutomatedReasoningPolicy.PolicyDefinitionRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionRule as PolicyDefinitionRule;
    impl crate::value::ToValue for PolicyDefinitionRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alternate_expression {
                properties.insert(
                    "AlternateExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-automatedreasoningpolicy-policydefinitiontype.html>
    pub struct PolicyDefinitionType_ {
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub values: Vec<PolicyDefinitionTypeValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::AutomatedReasoningPolicy.PolicyDefinitionType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionType as PolicyDefinitionType;
    impl crate::value::ToValue for PolicyDefinitionType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-automatedreasoningpolicy-policydefinitiontypevalue.html>
    pub struct PolicyDefinitionTypeValue_ {
        pub description: Option<crate::value::ExpString>,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionTypeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::AutomatedReasoningPolicy.PolicyDefinitionTypeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionTypeValue as PolicyDefinitionTypeValue;
    impl crate::value::ToValue for PolicyDefinitionTypeValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-automatedreasoningpolicy-policydefinitionvariable.html>
    pub struct PolicyDefinitionVariable_ {
        pub description: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::AutomatedReasoningPolicy.PolicyDefinitionVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_AutomatedReasoningPolicy_PolicyDefinitionVariable as PolicyDefinitionVariable;
    impl crate::value::ToValue for PolicyDefinitionVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(&self.description),
            );
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
}
pub mod dataautomationproject {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-audioextractioncategory.html>
    pub struct AudioExtractionCategory_ {
        pub state: crate::value::ExpString,
        pub type_configuration: Option<Box<AudioExtractionCategoryTypeConfiguration_>>,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_AudioExtractionCategory {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.AudioExtractionCategory"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_AudioExtractionCategory as AudioExtractionCategory;
    impl crate::value::ToValue for AudioExtractionCategory_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            if let Some(ref value) = self.type_configuration {
                properties.insert(
                    "TypeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-audioextractioncategorytypeconfiguration.html>
    pub struct AudioExtractionCategoryTypeConfiguration_ {
        pub transcript: Option<Box<TranscriptConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_AudioExtractionCategoryTypeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.AudioExtractionCategoryTypeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_AudioExtractionCategoryTypeConfiguration as AudioExtractionCategoryTypeConfiguration;
    impl crate::value::ToValue for AudioExtractionCategoryTypeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.transcript {
                properties.insert(
                    "Transcript".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-audiolanguageconfiguration.html>
    pub struct AudioLanguageConfiguration_ {
        pub generative_output_language: Option<crate::value::ExpString>,
        pub identify_multiple_languages: Option<crate::value::ExpBool>,
        pub input_languages: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_AudioLanguageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.AudioLanguageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_AudioLanguageConfiguration as AudioLanguageConfiguration;
    impl crate::value::ToValue for AudioLanguageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.generative_output_language {
                properties.insert(
                    "GenerativeOutputLanguage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identify_multiple_languages {
                properties.insert(
                    "IdentifyMultipleLanguages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_languages {
                properties.insert(
                    "InputLanguages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-audiooverrideconfiguration.html>
    pub struct AudioOverrideConfiguration_ {
        pub language_configuration: Option<Box<AudioLanguageConfiguration_>>,
        pub modality_processing: Option<Box<ModalityProcessingConfiguration_>>,
        pub sensitive_data_configuration: Option<Box<SensitiveDataConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_AudioOverrideConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.AudioOverrideConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_AudioOverrideConfiguration as AudioOverrideConfiguration;
    impl crate::value::ToValue for AudioOverrideConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.language_configuration {
                properties.insert(
                    "LanguageConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.modality_processing {
                properties.insert(
                    "ModalityProcessing".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sensitive_data_configuration {
                properties.insert(
                    "SensitiveDataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-audiostandardextraction.html>
    pub struct AudioStandardExtraction_ {
        pub category: Box<AudioExtractionCategory_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_AudioStandardExtraction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.AudioStandardExtraction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_AudioStandardExtraction as AudioStandardExtraction;
    impl crate::value::ToValue for AudioStandardExtraction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(&self.category),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-audiostandardgenerativefield.html>
    pub struct AudioStandardGenerativeField_ {
        pub state: crate::value::ExpString,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_AudioStandardGenerativeField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.AudioStandardGenerativeField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_AudioStandardGenerativeField as AudioStandardGenerativeField;
    impl crate::value::ToValue for AudioStandardGenerativeField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-audiostandardoutputconfiguration.html>
    pub struct AudioStandardOutputConfiguration_ {
        pub extraction: Option<Box<AudioStandardExtraction_>>,
        pub generative_field: Option<Box<AudioStandardGenerativeField_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_AudioStandardOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.AudioStandardOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_AudioStandardOutputConfiguration as AudioStandardOutputConfiguration;
    impl crate::value::ToValue for AudioStandardOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.extraction {
                properties.insert(
                    "Extraction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generative_field {
                properties.insert(
                    "GenerativeField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-blueprintitem.html>
    pub struct BlueprintItem_ {
        pub blueprint_arn: crate::value::ExpString,
        pub blueprint_stage: Option<crate::value::ExpString>,
        pub blueprint_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_BlueprintItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.BlueprintItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_BlueprintItem as BlueprintItem;
    impl crate::value::ToValue for BlueprintItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BlueprintArn".to_string(),
                crate::value::ToValue::to_value(&self.blueprint_arn),
            );
            if let Some(ref value) = self.blueprint_stage {
                properties.insert(
                    "BlueprintStage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.blueprint_version {
                properties.insert(
                    "BlueprintVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-channellabelingconfiguration.html>
    pub struct ChannelLabelingConfiguration_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ChannelLabelingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ChannelLabelingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ChannelLabelingConfiguration as ChannelLabelingConfiguration;
    impl crate::value::ToValue for ChannelLabelingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-customoutputconfiguration.html>
    pub struct CustomOutputConfiguration_ {
        pub blueprints: Option<Vec<BlueprintItem_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_CustomOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.CustomOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_CustomOutputConfiguration as CustomOutputConfiguration;
    impl crate::value::ToValue for CustomOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.blueprints {
                properties.insert(
                    "Blueprints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentboundingbox.html>
    pub struct DocumentBoundingBox_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentBoundingBox {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentBoundingBox"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentBoundingBox as DocumentBoundingBox;
    impl crate::value::ToValue for DocumentBoundingBox_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentextractiongranularity.html>
    pub struct DocumentExtractionGranularity_ {
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentExtractionGranularity {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentExtractionGranularity"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentExtractionGranularity as DocumentExtractionGranularity;
    impl crate::value::ToValue for DocumentExtractionGranularity_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentoutputadditionalfileformat.html>
    pub struct DocumentOutputAdditionalFileFormat_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentOutputAdditionalFileFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentOutputAdditionalFileFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentOutputAdditionalFileFormat as DocumentOutputAdditionalFileFormat;
    impl crate::value::ToValue for DocumentOutputAdditionalFileFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentoutputformat.html>
    pub struct DocumentOutputFormat_ {
        pub additional_file_format: Box<DocumentOutputAdditionalFileFormat_>,
        pub text_format: Box<DocumentOutputTextFormat_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentOutputFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentOutputFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentOutputFormat as DocumentOutputFormat;
    impl crate::value::ToValue for DocumentOutputFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AdditionalFileFormat".to_string(),
                crate::value::ToValue::to_value(&self.additional_file_format),
            );
            properties.insert(
                "TextFormat".to_string(),
                crate::value::ToValue::to_value(&self.text_format),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentoutputtextformat.html>
    pub struct DocumentOutputTextFormat_ {
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentOutputTextFormat {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentOutputTextFormat"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentOutputTextFormat as DocumentOutputTextFormat;
    impl crate::value::ToValue for DocumentOutputTextFormat_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentoverrideconfiguration.html>
    pub struct DocumentOverrideConfiguration_ {
        pub modality_processing: Option<Box<ModalityProcessingConfiguration_>>,
        pub sensitive_data_configuration: Option<Box<SensitiveDataConfiguration_>>,
        pub splitter: Option<Box<SplitterConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentOverrideConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentOverrideConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentOverrideConfiguration as DocumentOverrideConfiguration;
    impl crate::value::ToValue for DocumentOverrideConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.modality_processing {
                properties.insert(
                    "ModalityProcessing".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sensitive_data_configuration {
                properties.insert(
                    "SensitiveDataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.splitter {
                properties.insert(
                    "Splitter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentstandardextraction.html>
    pub struct DocumentStandardExtraction_ {
        pub bounding_box: Box<DocumentBoundingBox_>,
        pub granularity: Box<DocumentExtractionGranularity_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentStandardExtraction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentStandardExtraction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentStandardExtraction as DocumentStandardExtraction;
    impl crate::value::ToValue for DocumentStandardExtraction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BoundingBox".to_string(),
                crate::value::ToValue::to_value(&self.bounding_box),
            );
            properties.insert(
                "Granularity".to_string(),
                crate::value::ToValue::to_value(&self.granularity),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentstandardgenerativefield.html>
    pub struct DocumentStandardGenerativeField_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentStandardGenerativeField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentStandardGenerativeField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentStandardGenerativeField as DocumentStandardGenerativeField;
    impl crate::value::ToValue for DocumentStandardGenerativeField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-documentstandardoutputconfiguration.html>
    pub struct DocumentStandardOutputConfiguration_ {
        pub extraction: Option<Box<DocumentStandardExtraction_>>,
        pub generative_field: Option<Box<DocumentStandardGenerativeField_>>,
        pub output_format: Option<Box<DocumentOutputFormat_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_DocumentStandardOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.DocumentStandardOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_DocumentStandardOutputConfiguration as DocumentStandardOutputConfiguration;
    impl crate::value::ToValue for DocumentStandardOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.extraction {
                properties.insert(
                    "Extraction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generative_field {
                properties.insert(
                    "GenerativeField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_format {
                properties.insert(
                    "OutputFormat".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-imageboundingbox.html>
    pub struct ImageBoundingBox_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ImageBoundingBox {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ImageBoundingBox"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ImageBoundingBox as ImageBoundingBox;
    impl crate::value::ToValue for ImageBoundingBox_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-imageextractioncategory.html>
    pub struct ImageExtractionCategory_ {
        pub state: crate::value::ExpString,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ImageExtractionCategory {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ImageExtractionCategory"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ImageExtractionCategory as ImageExtractionCategory;
    impl crate::value::ToValue for ImageExtractionCategory_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-imageoverrideconfiguration.html>
    pub struct ImageOverrideConfiguration_ {
        pub modality_processing: Option<Box<ModalityProcessingConfiguration_>>,
        pub sensitive_data_configuration: Option<Box<SensitiveDataConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ImageOverrideConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ImageOverrideConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ImageOverrideConfiguration as ImageOverrideConfiguration;
    impl crate::value::ToValue for ImageOverrideConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.modality_processing {
                properties.insert(
                    "ModalityProcessing".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sensitive_data_configuration {
                properties.insert(
                    "SensitiveDataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-imagestandardextraction.html>
    pub struct ImageStandardExtraction_ {
        pub bounding_box: Box<ImageBoundingBox_>,
        pub category: Box<ImageExtractionCategory_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ImageStandardExtraction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ImageStandardExtraction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ImageStandardExtraction as ImageStandardExtraction;
    impl crate::value::ToValue for ImageStandardExtraction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BoundingBox".to_string(),
                crate::value::ToValue::to_value(&self.bounding_box),
            );
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(&self.category),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-imagestandardgenerativefield.html>
    pub struct ImageStandardGenerativeField_ {
        pub state: crate::value::ExpString,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ImageStandardGenerativeField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ImageStandardGenerativeField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ImageStandardGenerativeField as ImageStandardGenerativeField;
    impl crate::value::ToValue for ImageStandardGenerativeField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-imagestandardoutputconfiguration.html>
    pub struct ImageStandardOutputConfiguration_ {
        pub extraction: Option<Box<ImageStandardExtraction_>>,
        pub generative_field: Option<Box<ImageStandardGenerativeField_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ImageStandardOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ImageStandardOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ImageStandardOutputConfiguration as ImageStandardOutputConfiguration;
    impl crate::value::ToValue for ImageStandardOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.extraction {
                properties.insert(
                    "Extraction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generative_field {
                properties.insert(
                    "GenerativeField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-modalityprocessingconfiguration.html>
    pub struct ModalityProcessingConfiguration_ {
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ModalityProcessingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ModalityProcessingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ModalityProcessingConfiguration as ModalityProcessingConfiguration;
    impl crate::value::ToValue for ModalityProcessingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-modalityroutingconfiguration.html>
    pub struct ModalityRoutingConfiguration_ {
        pub jpeg: Option<crate::value::ExpString>,
        pub mov: Option<crate::value::ExpString>,
        pub mp4: Option<crate::value::ExpString>,
        pub png: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_ModalityRoutingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.ModalityRoutingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_ModalityRoutingConfiguration as ModalityRoutingConfiguration;
    impl crate::value::ToValue for ModalityRoutingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.jpeg {
                properties.insert("jpeg".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mov {
                properties.insert("mov".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.mp4 {
                properties.insert("mp4".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.png {
                properties.insert("png".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-overrideconfiguration.html>
    pub struct OverrideConfiguration_ {
        pub audio: Option<Box<AudioOverrideConfiguration_>>,
        pub document: Option<Box<DocumentOverrideConfiguration_>>,
        pub image: Option<Box<ImageOverrideConfiguration_>>,
        pub modality_routing: Option<Box<ModalityRoutingConfiguration_>>,
        pub video: Option<Box<VideoOverrideConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_OverrideConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.OverrideConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_OverrideConfiguration as OverrideConfiguration;
    impl crate::value::ToValue for OverrideConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio {
                properties.insert("Audio".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.document {
                properties.insert(
                    "Document".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image {
                properties.insert("Image".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.modality_routing {
                properties.insert(
                    "ModalityRouting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video {
                properties.insert("Video".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-piientitiesconfiguration.html>
    pub struct PIIEntitiesConfiguration_ {
        pub pii_entity_types: Option<Vec<crate::value::ExpString>>,
        pub redaction_mask_mode: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_PIIEntitiesConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.PIIEntitiesConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_PIIEntitiesConfiguration as PIIEntitiesConfiguration;
    impl crate::value::ToValue for PIIEntitiesConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pii_entity_types {
                properties.insert(
                    "PiiEntityTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redaction_mask_mode {
                properties.insert(
                    "RedactionMaskMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-sensitivedataconfiguration.html>
    pub struct SensitiveDataConfiguration_ {
        pub detection_mode: Option<crate::value::ExpString>,
        pub detection_scope: Option<Vec<crate::value::ExpString>>,
        pub pii_entities_configuration: Option<Box<PIIEntitiesConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_SensitiveDataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.SensitiveDataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_SensitiveDataConfiguration as SensitiveDataConfiguration;
    impl crate::value::ToValue for SensitiveDataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.detection_mode {
                properties.insert(
                    "DetectionMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.detection_scope {
                properties.insert(
                    "DetectionScope".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pii_entities_configuration {
                properties.insert(
                    "PiiEntitiesConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-speakerlabelingconfiguration.html>
    pub struct SpeakerLabelingConfiguration_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_SpeakerLabelingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.SpeakerLabelingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_SpeakerLabelingConfiguration as SpeakerLabelingConfiguration;
    impl crate::value::ToValue for SpeakerLabelingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-splitterconfiguration.html>
    pub struct SplitterConfiguration_ {
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_SplitterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.SplitterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_SplitterConfiguration as SplitterConfiguration;
    impl crate::value::ToValue for SplitterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-standardoutputconfiguration.html>
    pub struct StandardOutputConfiguration_ {
        pub audio: Option<Box<AudioStandardOutputConfiguration_>>,
        pub document: Option<Box<DocumentStandardOutputConfiguration_>>,
        pub image: Option<Box<ImageStandardOutputConfiguration_>>,
        pub video: Option<Box<VideoStandardOutputConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_StandardOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.StandardOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_StandardOutputConfiguration as StandardOutputConfiguration;
    impl crate::value::ToValue for StandardOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio {
                properties.insert("Audio".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.document {
                properties.insert(
                    "Document".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image {
                properties.insert("Image".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.video {
                properties.insert("Video".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-transcriptconfiguration.html>
    pub struct TranscriptConfiguration_ {
        pub channel_labeling: Option<Box<ChannelLabelingConfiguration_>>,
        pub speaker_labeling: Option<Box<SpeakerLabelingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_TranscriptConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.TranscriptConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_TranscriptConfiguration as TranscriptConfiguration;
    impl crate::value::ToValue for TranscriptConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_labeling {
                properties.insert(
                    "ChannelLabeling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.speaker_labeling {
                properties.insert(
                    "SpeakerLabeling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-videoboundingbox.html>
    pub struct VideoBoundingBox_ {
        pub state: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_VideoBoundingBox {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.VideoBoundingBox"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_VideoBoundingBox as VideoBoundingBox;
    impl crate::value::ToValue for VideoBoundingBox_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-videoextractioncategory.html>
    pub struct VideoExtractionCategory_ {
        pub state: crate::value::ExpString,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_VideoExtractionCategory {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.VideoExtractionCategory"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_VideoExtractionCategory as VideoExtractionCategory;
    impl crate::value::ToValue for VideoExtractionCategory_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-videooverrideconfiguration.html>
    pub struct VideoOverrideConfiguration_ {
        pub modality_processing: Option<Box<ModalityProcessingConfiguration_>>,
        pub sensitive_data_configuration: Option<Box<SensitiveDataConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_VideoOverrideConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.VideoOverrideConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_VideoOverrideConfiguration as VideoOverrideConfiguration;
    impl crate::value::ToValue for VideoOverrideConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.modality_processing {
                properties.insert(
                    "ModalityProcessing".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sensitive_data_configuration {
                properties.insert(
                    "SensitiveDataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-videostandardextraction.html>
    pub struct VideoStandardExtraction_ {
        pub bounding_box: Box<VideoBoundingBox_>,
        pub category: Box<VideoExtractionCategory_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_VideoStandardExtraction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.VideoStandardExtraction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_VideoStandardExtraction as VideoStandardExtraction;
    impl crate::value::ToValue for VideoStandardExtraction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BoundingBox".to_string(),
                crate::value::ToValue::to_value(&self.bounding_box),
            );
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(&self.category),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-videostandardgenerativefield.html>
    pub struct VideoStandardGenerativeField_ {
        pub state: crate::value::ExpString,
        pub types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_VideoStandardGenerativeField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.VideoStandardGenerativeField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_VideoStandardGenerativeField as VideoStandardGenerativeField;
    impl crate::value::ToValue for VideoStandardGenerativeField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "State".to_string(),
                crate::value::ToValue::to_value(&self.state),
            );
            if let Some(ref value) = self.types {
                properties.insert("Types".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-dataautomationproject-videostandardoutputconfiguration.html>
    pub struct VideoStandardOutputConfiguration_ {
        pub extraction: Option<Box<VideoStandardExtraction_>>,
        pub generative_field: Option<Box<VideoStandardGenerativeField_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataAutomationProject_VideoStandardOutputConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataAutomationProject.VideoStandardOutputConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataAutomationProject_VideoStandardOutputConfiguration as VideoStandardOutputConfiguration;
    impl crate::value::ToValue for VideoStandardOutputConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.extraction {
                properties.insert(
                    "Extraction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generative_field {
                properties.insert(
                    "GenerativeField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod datasource {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-bedrockdataautomationconfiguration.html>
    pub struct BedrockDataAutomationConfiguration_ {
        pub parsing_modality: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_BedrockDataAutomationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.BedrockDataAutomationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_BedrockDataAutomationConfiguration as BedrockDataAutomationConfiguration;
    impl crate::value::ToValue for BedrockDataAutomationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parsing_modality {
                properties.insert(
                    "ParsingModality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-bedrockfoundationmodelconfiguration.html>
    pub struct BedrockFoundationModelConfiguration_ {
        pub model_arn: crate::value::ExpString,
        pub parsing_modality: Option<crate::value::ExpString>,
        pub parsing_prompt: Option<Box<ParsingPrompt_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_BedrockFoundationModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.BedrockFoundationModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_BedrockFoundationModelConfiguration as BedrockFoundationModelConfiguration;
    impl crate::value::ToValue for BedrockFoundationModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ModelArn".to_string(),
                crate::value::ToValue::to_value(&self.model_arn),
            );
            if let Some(ref value) = self.parsing_modality {
                properties.insert(
                    "ParsingModality".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parsing_prompt {
                properties.insert(
                    "ParsingPrompt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-bedrockfoundationmodelcontextenrichmentconfiguration.html>
    pub struct BedrockFoundationModelContextEnrichmentConfiguration_ {
        pub enrichment_strategy_configuration: Box<EnrichmentStrategyConfiguration_>,
        pub model_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_BedrockFoundationModelContextEnrichmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.BedrockFoundationModelContextEnrichmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_BedrockFoundationModelContextEnrichmentConfiguration as BedrockFoundationModelContextEnrichmentConfiguration;
    impl crate::value::ToValue for BedrockFoundationModelContextEnrichmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnrichmentStrategyConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.enrichment_strategy_configuration),
            );
            properties.insert(
                "ModelArn".to_string(),
                crate::value::ToValue::to_value(&self.model_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-chunkingconfiguration.html>
    pub struct ChunkingConfiguration_ {
        pub chunking_strategy: crate::value::ExpString,
        pub fixed_size_chunking_configuration: Option<Box<FixedSizeChunkingConfiguration_>>,
        pub hierarchical_chunking_configuration: Option<Box<HierarchicalChunkingConfiguration_>>,
        pub semantic_chunking_configuration: Option<Box<SemanticChunkingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ChunkingConfiguration as ChunkingConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-confluencecrawlerconfiguration.html>
    pub struct ConfluenceCrawlerConfiguration_ {
        pub filter_configuration: Option<Box<CrawlFilterConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ConfluenceCrawlerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ConfluenceCrawlerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ConfluenceCrawlerConfiguration as ConfluenceCrawlerConfiguration;
    impl crate::value::ToValue for ConfluenceCrawlerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-confluencedatasourceconfiguration.html>
    pub struct ConfluenceDataSourceConfiguration_ {
        pub crawler_configuration: Option<Box<ConfluenceCrawlerConfiguration_>>,
        pub source_configuration: Box<ConfluenceSourceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ConfluenceDataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ConfluenceDataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ConfluenceDataSourceConfiguration as ConfluenceDataSourceConfiguration;
    impl crate::value::ToValue for ConfluenceDataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawler_configuration {
                properties.insert(
                    "CrawlerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.source_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-confluencesourceconfiguration.html>
    pub struct ConfluenceSourceConfiguration_ {
        pub auth_type: crate::value::ExpString,
        pub credentials_secret_arn: crate::value::ExpString,
        pub host_type: crate::value::ExpString,
        pub host_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ConfluenceSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ConfluenceSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ConfluenceSourceConfiguration as ConfluenceSourceConfiguration;
    impl crate::value::ToValue for ConfluenceSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthType".to_string(),
                crate::value::ToValue::to_value(&self.auth_type),
            );
            properties.insert(
                "CredentialsSecretArn".to_string(),
                crate::value::ToValue::to_value(&self.credentials_secret_arn),
            );
            properties.insert(
                "HostType".to_string(),
                crate::value::ToValue::to_value(&self.host_type),
            );
            properties.insert(
                "HostUrl".to_string(),
                crate::value::ToValue::to_value(&self.host_url),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-contextenrichmentconfiguration.html>
    pub struct ContextEnrichmentConfiguration_ {
        pub bedrock_foundation_model_configuration:
            Option<Box<BedrockFoundationModelContextEnrichmentConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ContextEnrichmentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ContextEnrichmentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ContextEnrichmentConfiguration as ContextEnrichmentConfiguration;
    impl crate::value::ToValue for ContextEnrichmentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_foundation_model_configuration {
                properties.insert(
                    "BedrockFoundationModelConfiguration".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-crawlfilterconfiguration.html>
    pub struct CrawlFilterConfiguration_ {
        pub pattern_object_filter: Option<Box<PatternObjectFilterConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_CrawlFilterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.CrawlFilterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_CrawlFilterConfiguration as CrawlFilterConfiguration;
    impl crate::value::ToValue for CrawlFilterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pattern_object_filter {
                properties.insert(
                    "PatternObjectFilter".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-customtransformationconfiguration.html>
    pub struct CustomTransformationConfiguration_ {
        pub intermediate_storage: Box<IntermediateStorage_>,
        pub transformations: Vec<Transformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_CustomTransformationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.CustomTransformationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_CustomTransformationConfiguration as CustomTransformationConfiguration;
    impl crate::value::ToValue for CustomTransformationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IntermediateStorage".to_string(),
                crate::value::ToValue::to_value(&self.intermediate_storage),
            );
            properties.insert(
                "Transformations".to_string(),
                crate::value::ToValue::to_value(&self.transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-datasourceconfiguration.html>
    pub struct DataSourceConfiguration_ {
        pub confluence_configuration: Option<Box<ConfluenceDataSourceConfiguration_>>,
        pub s3_configuration: Option<Box<S3DataSourceConfiguration_>>,
        pub salesforce_configuration: Option<Box<SalesforceDataSourceConfiguration_>>,
        pub share_point_configuration: Option<Box<SharePointDataSourceConfiguration_>>,
        pub r#type: crate::value::ExpString,
        pub web_configuration: Option<Box<WebDataSourceConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_DataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.DataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_DataSourceConfiguration as DataSourceConfiguration;
    impl crate::value::ToValue for DataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.confluence_configuration {
                properties.insert(
                    "ConfluenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_configuration {
                properties.insert(
                    "S3Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.salesforce_configuration {
                properties.insert(
                    "SalesforceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.share_point_configuration {
                properties.insert(
                    "SharePointConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.web_configuration {
                properties.insert(
                    "WebConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-enrichmentstrategyconfiguration.html>
    pub struct EnrichmentStrategyConfiguration_ {
        pub method: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_EnrichmentStrategyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.EnrichmentStrategyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_EnrichmentStrategyConfiguration as EnrichmentStrategyConfiguration;
    impl crate::value::ToValue for EnrichmentStrategyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Method".to_string(),
                crate::value::ToValue::to_value(&self.method),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-fixedsizechunkingconfiguration.html>
    pub struct FixedSizeChunkingConfiguration_ {
        pub max_tokens: i32,
        pub overlap_percentage: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_FixedSizeChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.FixedSizeChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_FixedSizeChunkingConfiguration as FixedSizeChunkingConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-hierarchicalchunkingconfiguration.html>
    pub struct HierarchicalChunkingConfiguration_ {
        pub level_configurations: Vec<HierarchicalChunkingLevelConfiguration_>,
        pub overlap_tokens: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_HierarchicalChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.HierarchicalChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_HierarchicalChunkingConfiguration as HierarchicalChunkingConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-hierarchicalchunkinglevelconfiguration.html>
    pub struct HierarchicalChunkingLevelConfiguration_ {
        pub max_tokens: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_HierarchicalChunkingLevelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.HierarchicalChunkingLevelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_HierarchicalChunkingLevelConfiguration as HierarchicalChunkingLevelConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-intermediatestorage.html>
    pub struct IntermediateStorage_ {
        pub s3_location: Box<S3Location_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_IntermediateStorage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.IntermediateStorage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_IntermediateStorage as IntermediateStorage;
    impl crate::value::ToValue for IntermediateStorage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Location".to_string(),
                crate::value::ToValue::to_value(&self.s3_location),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-parsingconfiguration.html>
    pub struct ParsingConfiguration_ {
        pub bedrock_data_automation_configuration: Option<Box<BedrockDataAutomationConfiguration_>>,
        pub bedrock_foundation_model_configuration:
            Option<Box<BedrockFoundationModelConfiguration_>>,
        pub parsing_strategy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ParsingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ParsingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ParsingConfiguration as ParsingConfiguration;
    impl crate::value::ToValue for ParsingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_data_automation_configuration {
                properties.insert(
                    "BedrockDataAutomationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-parsingprompt.html>
    pub struct ParsingPrompt_ {
        pub parsing_prompt_text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ParsingPrompt {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ParsingPrompt"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ParsingPrompt as ParsingPrompt;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-patternobjectfilter.html>
    pub struct PatternObjectFilter_ {
        pub exclusion_filters: Option<Vec<crate::value::ExpString>>,
        pub inclusion_filters: Option<Vec<crate::value::ExpString>>,
        pub object_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_PatternObjectFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.PatternObjectFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_PatternObjectFilter as PatternObjectFilter;
    impl crate::value::ToValue for PatternObjectFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
            properties.insert(
                "ObjectType".to_string(),
                crate::value::ToValue::to_value(&self.object_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-patternobjectfilterconfiguration.html>
    pub struct PatternObjectFilterConfiguration_ {
        pub filters: Vec<PatternObjectFilter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_PatternObjectFilterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.PatternObjectFilterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_PatternObjectFilterConfiguration as PatternObjectFilterConfiguration;
    impl crate::value::ToValue for PatternObjectFilterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Filters".to_string(),
                crate::value::ToValue::to_value(&self.filters),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-s3datasourceconfiguration.html>
    pub struct S3DataSourceConfiguration_ {
        pub bucket_arn: crate::value::ExpString,
        pub bucket_owner_account_id: Option<crate::value::ExpString>,
        pub inclusion_prefixes: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_S3DataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.S3DataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_S3DataSourceConfiguration as S3DataSourceConfiguration;
    impl crate::value::ToValue for S3DataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketArn".to_string(),
                crate::value::ToValue::to_value(&self.bucket_arn),
            );
            if let Some(ref value) = self.bucket_owner_account_id {
                properties.insert(
                    "BucketOwnerAccountId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion_prefixes {
                properties.insert(
                    "InclusionPrefixes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-s3location.html>
    pub struct S3Location_ {
        pub uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "URI".to_string(),
                crate::value::ToValue::to_value(&self.uri),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-salesforcecrawlerconfiguration.html>
    pub struct SalesforceCrawlerConfiguration_ {
        pub filter_configuration: Option<Box<CrawlFilterConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SalesforceCrawlerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SalesforceCrawlerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SalesforceCrawlerConfiguration as SalesforceCrawlerConfiguration;
    impl crate::value::ToValue for SalesforceCrawlerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-salesforcedatasourceconfiguration.html>
    pub struct SalesforceDataSourceConfiguration_ {
        pub crawler_configuration: Option<Box<SalesforceCrawlerConfiguration_>>,
        pub source_configuration: Box<SalesforceSourceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SalesforceDataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SalesforceDataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SalesforceDataSourceConfiguration as SalesforceDataSourceConfiguration;
    impl crate::value::ToValue for SalesforceDataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawler_configuration {
                properties.insert(
                    "CrawlerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.source_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-salesforcesourceconfiguration.html>
    pub struct SalesforceSourceConfiguration_ {
        pub auth_type: crate::value::ExpString,
        pub credentials_secret_arn: crate::value::ExpString,
        pub host_url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SalesforceSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SalesforceSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SalesforceSourceConfiguration as SalesforceSourceConfiguration;
    impl crate::value::ToValue for SalesforceSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthType".to_string(),
                crate::value::ToValue::to_value(&self.auth_type),
            );
            properties.insert(
                "CredentialsSecretArn".to_string(),
                crate::value::ToValue::to_value(&self.credentials_secret_arn),
            );
            properties.insert(
                "HostUrl".to_string(),
                crate::value::ToValue::to_value(&self.host_url),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-seedurl.html>
    pub struct SeedUrl_ {
        pub url: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SeedUrl {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SeedUrl"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SeedUrl as SeedUrl;
    impl crate::value::ToValue for SeedUrl_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Url".to_string(),
                crate::value::ToValue::to_value(&self.url),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-semanticchunkingconfiguration.html>
    pub struct SemanticChunkingConfiguration_ {
        pub breakpoint_percentile_threshold: i32,
        pub buffer_size: i32,
        pub max_tokens: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SemanticChunkingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SemanticChunkingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SemanticChunkingConfiguration as SemanticChunkingConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-serversideencryptionconfiguration.html>
    pub struct ServerSideEncryptionConfiguration_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_ServerSideEncryptionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.ServerSideEncryptionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_ServerSideEncryptionConfiguration as ServerSideEncryptionConfiguration;
    impl crate::value::ToValue for ServerSideEncryptionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-sharepointcrawlerconfiguration.html>
    pub struct SharePointCrawlerConfiguration_ {
        pub filter_configuration: Option<Box<CrawlFilterConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SharePointCrawlerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SharePointCrawlerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SharePointCrawlerConfiguration as SharePointCrawlerConfiguration;
    impl crate::value::ToValue for SharePointCrawlerConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filter_configuration {
                properties.insert(
                    "FilterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-sharepointdatasourceconfiguration.html>
    pub struct SharePointDataSourceConfiguration_ {
        pub crawler_configuration: Option<Box<SharePointCrawlerConfiguration_>>,
        pub source_configuration: Box<SharePointSourceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SharePointDataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SharePointDataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SharePointDataSourceConfiguration as SharePointDataSourceConfiguration;
    impl crate::value::ToValue for SharePointDataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawler_configuration {
                properties.insert(
                    "CrawlerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.source_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-sharepointsourceconfiguration.html>
    pub struct SharePointSourceConfiguration_ {
        pub auth_type: crate::value::ExpString,
        pub credentials_secret_arn: crate::value::ExpString,
        pub domain: crate::value::ExpString,
        pub host_type: crate::value::ExpString,
        pub site_urls: Vec<crate::value::ExpString>,
        pub tenant_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_SharePointSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.SharePointSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_SharePointSourceConfiguration as SharePointSourceConfiguration;
    impl crate::value::ToValue for SharePointSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthType".to_string(),
                crate::value::ToValue::to_value(&self.auth_type),
            );
            properties.insert(
                "CredentialsSecretArn".to_string(),
                crate::value::ToValue::to_value(&self.credentials_secret_arn),
            );
            properties.insert(
                "Domain".to_string(),
                crate::value::ToValue::to_value(&self.domain),
            );
            properties.insert(
                "HostType".to_string(),
                crate::value::ToValue::to_value(&self.host_type),
            );
            properties.insert(
                "SiteUrls".to_string(),
                crate::value::ToValue::to_value(&self.site_urls),
            );
            if let Some(ref value) = self.tenant_id {
                properties.insert(
                    "TenantId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-transformation.html>
    pub struct Transformation_ {
        pub step_to_apply: crate::value::ExpString,
        pub transformation_function: Box<TransformationFunction_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_Transformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.Transformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_Transformation as Transformation;
    impl crate::value::ToValue for Transformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StepToApply".to_string(),
                crate::value::ToValue::to_value(&self.step_to_apply),
            );
            properties.insert(
                "TransformationFunction".to_string(),
                crate::value::ToValue::to_value(&self.transformation_function),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-transformationfunction.html>
    pub struct TransformationFunction_ {
        pub transformation_lambda_configuration: Box<TransformationLambdaConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_TransformationFunction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.TransformationFunction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_TransformationFunction as TransformationFunction;
    impl crate::value::ToValue for TransformationFunction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TransformationLambdaConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.transformation_lambda_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-transformationlambdaconfiguration.html>
    pub struct TransformationLambdaConfiguration_ {
        pub lambda_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_TransformationLambdaConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.TransformationLambdaConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_TransformationLambdaConfiguration as TransformationLambdaConfiguration;
    impl crate::value::ToValue for TransformationLambdaConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-urlconfiguration.html>
    pub struct UrlConfiguration_ {
        pub seed_urls: Vec<SeedUrl_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_UrlConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.UrlConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_UrlConfiguration as UrlConfiguration;
    impl crate::value::ToValue for UrlConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SeedUrls".to_string(),
                crate::value::ToValue::to_value(&self.seed_urls),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-vectoringestionconfiguration.html>
    pub struct VectorIngestionConfiguration_ {
        pub chunking_configuration: Option<Box<ChunkingConfiguration_>>,
        pub context_enrichment_configuration: Option<Box<ContextEnrichmentConfiguration_>>,
        pub custom_transformation_configuration: Option<Box<CustomTransformationConfiguration_>>,
        pub parsing_configuration: Option<Box<ParsingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_VectorIngestionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.VectorIngestionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_VectorIngestionConfiguration as VectorIngestionConfiguration;
    impl crate::value::ToValue for VectorIngestionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.chunking_configuration {
                properties.insert(
                    "ChunkingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.context_enrichment_configuration {
                properties.insert(
                    "ContextEnrichmentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_transformation_configuration {
                properties.insert(
                    "CustomTransformationConfiguration".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-webcrawlerconfiguration.html>
    pub struct WebCrawlerConfiguration_ {
        pub crawler_limits: Option<Box<WebCrawlerLimits_>>,
        pub exclusion_filters: Option<Vec<crate::value::ExpString>>,
        pub inclusion_filters: Option<Vec<crate::value::ExpString>>,
        pub scope: Option<crate::value::ExpString>,
        pub user_agent: Option<crate::value::ExpString>,
        pub user_agent_header: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_WebCrawlerConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.WebCrawlerConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_WebCrawlerConfiguration as WebCrawlerConfiguration;
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
            if let Some(ref value) = self.user_agent {
                properties.insert(
                    "UserAgent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_agent_header {
                properties.insert(
                    "UserAgentHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-webcrawlerlimits.html>
    pub struct WebCrawlerLimits_ {
        pub max_pages: Option<i32>,
        pub rate_limit: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_WebCrawlerLimits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.WebCrawlerLimits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_WebCrawlerLimits as WebCrawlerLimits;
    impl crate::value::ToValue for WebCrawlerLimits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_pages {
                properties.insert(
                    "MaxPages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_limit {
                properties.insert(
                    "RateLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-webdatasourceconfiguration.html>
    pub struct WebDataSourceConfiguration_ {
        pub crawler_configuration: Option<Box<WebCrawlerConfiguration_>>,
        pub source_configuration: Box<WebSourceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_WebDataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.WebDataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_WebDataSourceConfiguration as WebDataSourceConfiguration;
    impl crate::value::ToValue for WebDataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.crawler_configuration {
                properties.insert(
                    "CrawlerConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.source_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-datasource-websourceconfiguration.html>
    pub struct WebSourceConfiguration_ {
        pub url_configuration: Box<UrlConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_DataSource_WebSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::DataSource.WebSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_DataSource_WebSourceConfiguration as WebSourceConfiguration;
    impl crate::value::ToValue for WebSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "UrlConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.url_configuration),
            );
            properties.into()
        }
    }
}
pub mod flow {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-agentflownodeconfiguration.html>
    pub struct AgentFlowNodeConfiguration_ {
        pub agent_alias_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_AgentFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.AgentFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_AgentFlowNodeConfiguration as AgentFlowNodeConfiguration;
    impl crate::value::ToValue for AgentFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentAliasArn".to_string(),
                crate::value::ToValue::to_value(&self.agent_alias_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-conditionflownodeconfiguration.html>
    pub struct ConditionFlowNodeConfiguration_ {
        pub conditions: Vec<FlowCondition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_ConditionFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.ConditionFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_ConditionFlowNodeConfiguration as ConditionFlowNodeConfiguration;
    impl crate::value::ToValue for ConditionFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(&self.conditions),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-fieldforreranking.html>
    pub struct FieldForReranking_ {
        pub field_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FieldForReranking {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FieldForReranking"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FieldForReranking as FieldForReranking;
    impl crate::value::ToValue for FieldForReranking_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flowcondition.html>
    pub struct FlowCondition_ {
        pub expression: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowCondition as FlowCondition;
    impl crate::value::ToValue for FlowCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flowconditionalconnectionconfiguration.html>
    pub struct FlowConditionalConnectionConfiguration_ {
        pub condition: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowConditionalConnectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowConditionalConnectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowConditionalConnectionConfiguration as FlowConditionalConnectionConfiguration;
    impl crate::value::ToValue for FlowConditionalConnectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flowconnection.html>
    pub struct FlowConnection_ {
        pub configuration: Option<Box<FlowConnectionConfiguration_>>,
        pub name: crate::value::ExpString,
        pub source: crate::value::ExpString,
        pub target: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowConnection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowConnection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowConnection as FlowConnection;
    impl crate::value::ToValue for FlowConnection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flowconnectionconfiguration.html>
    pub struct FlowConnectionConfiguration_ {
        pub conditional: Option<Box<FlowConditionalConnectionConfiguration_>>,
        pub data: Option<Box<FlowDataConnectionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowConnectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowConnectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowConnectionConfiguration as FlowConnectionConfiguration;
    impl crate::value::ToValue for FlowConnectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conditional {
                properties.insert(
                    "Conditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data {
                properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flowdataconnectionconfiguration.html>
    pub struct FlowDataConnectionConfiguration_ {
        pub source_output: crate::value::ExpString,
        pub target_input: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowDataConnectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowDataConnectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowDataConnectionConfiguration as FlowDataConnectionConfiguration;
    impl crate::value::ToValue for FlowDataConnectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceOutput".to_string(),
                crate::value::ToValue::to_value(&self.source_output),
            );
            properties.insert(
                "TargetInput".to_string(),
                crate::value::ToValue::to_value(&self.target_input),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flowdefinition.html>
    pub struct FlowDefinition_ {
        pub connections: Option<Vec<FlowConnection_>>,
        pub nodes: Option<Vec<FlowNode_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowDefinition as FlowDefinition;
    impl crate::value::ToValue for FlowDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connections {
                properties.insert(
                    "Connections".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nodes {
                properties.insert("Nodes".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flownode.html>
    pub struct FlowNode_ {
        pub configuration: Option<Box<FlowNodeConfiguration_>>,
        pub inputs: Option<Vec<FlowNodeInput_>>,
        pub name: crate::value::ExpString,
        pub outputs: Option<Vec<FlowNodeOutput_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowNode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowNode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowNode as FlowNode;
    impl crate::value::ToValue for FlowNode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inputs {
                properties.insert("Inputs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.outputs {
                properties.insert(
                    "Outputs".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flownodeconfiguration.html>
    pub struct FlowNodeConfiguration_ {
        pub agent: Option<Box<AgentFlowNodeConfiguration_>>,
        pub collector: Option<serde_json::Value>,
        pub condition: Option<Box<ConditionFlowNodeConfiguration_>>,
        pub inline_code: Option<Box<InlineCodeFlowNodeConfiguration_>>,
        pub input: Option<serde_json::Value>,
        pub iterator: Option<serde_json::Value>,
        pub knowledge_base: Option<Box<KnowledgeBaseFlowNodeConfiguration_>>,
        pub lambda_function: Option<Box<LambdaFunctionFlowNodeConfiguration_>>,
        pub lex: Option<Box<LexFlowNodeConfiguration_>>,
        pub r#loop: Option<Box<LoopFlowNodeConfiguration_>>,
        pub loop_controller: Option<Box<LoopControllerFlowNodeConfiguration_>>,
        pub loop_input: Option<serde_json::Value>,
        pub output: Option<serde_json::Value>,
        pub prompt: Option<Box<PromptFlowNodeConfiguration_>>,
        pub retrieval: Option<Box<RetrievalFlowNodeConfiguration_>>,
        pub storage: Option<Box<StorageFlowNodeConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowNodeConfiguration as FlowNodeConfiguration;
    impl crate::value::ToValue for FlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agent {
                properties.insert("Agent".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.collector {
                properties.insert(
                    "Collector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline_code {
                properties.insert(
                    "InlineCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input {
                properties.insert("Input".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.iterator {
                properties.insert(
                    "Iterator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.knowledge_base {
                properties.insert(
                    "KnowledgeBase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function {
                properties.insert(
                    "LambdaFunction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lex {
                properties.insert("Lex".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#loop {
                properties.insert("Loop".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.loop_controller {
                properties.insert(
                    "LoopController".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.loop_input {
                properties.insert(
                    "LoopInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output {
                properties.insert("Output".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prompt {
                properties.insert("Prompt".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.retrieval {
                properties.insert(
                    "Retrieval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage {
                properties.insert(
                    "Storage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flownodeinput.html>
    pub struct FlowNodeInput_ {
        pub category: Option<crate::value::ExpString>,
        pub expression: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowNodeInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowNodeInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowNodeInput as FlowNodeInput;
    impl crate::value::ToValue for FlowNodeInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.category {
                properties.insert(
                    "Category".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flownodeoutput.html>
    pub struct FlowNodeOutput_ {
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowNodeOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowNodeOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowNodeOutput as FlowNodeOutput;
    impl crate::value::ToValue for FlowNodeOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-flowvalidation.html>
    pub struct FlowValidation_ {
        pub message: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_FlowValidation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.FlowValidation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_FlowValidation as FlowValidation;
    impl crate::value::ToValue for FlowValidation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(&self.message),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-guardrailconfiguration.html>
    pub struct GuardrailConfiguration_ {
        pub guardrail_identifier: Option<crate::value::ExpString>,
        pub guardrail_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_GuardrailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.GuardrailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_GuardrailConfiguration as GuardrailConfiguration;
    impl crate::value::ToValue for GuardrailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.guardrail_identifier {
                properties.insert(
                    "GuardrailIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.guardrail_version {
                properties.insert(
                    "GuardrailVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-inlinecodeflownodeconfiguration.html>
    pub struct InlineCodeFlowNodeConfiguration_ {
        pub code: crate::value::ExpString,
        pub language: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_InlineCodeFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.InlineCodeFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_InlineCodeFlowNodeConfiguration as InlineCodeFlowNodeConfiguration;
    impl crate::value::ToValue for InlineCodeFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Code".to_string(),
                crate::value::ToValue::to_value(&self.code),
            );
            properties.insert(
                "Language".to_string(),
                crate::value::ToValue::to_value(&self.language),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-knowledgebaseflownodeconfiguration.html>
    pub struct KnowledgeBaseFlowNodeConfiguration_ {
        pub guardrail_configuration: Option<Box<GuardrailConfiguration_>>,
        pub inference_configuration: Option<Box<PromptInferenceConfiguration_>>,
        pub knowledge_base_id: crate::value::ExpString,
        pub model_id: Option<crate::value::ExpString>,
        pub number_of_results: Option<f64>,
        pub orchestration_configuration: Option<Box<KnowledgeBaseOrchestrationConfiguration_>>,
        pub prompt_template: Option<Box<KnowledgeBasePromptTemplate_>>,
        pub reranking_configuration: Option<Box<VectorSearchRerankingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_KnowledgeBaseFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.KnowledgeBaseFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_KnowledgeBaseFlowNodeConfiguration as KnowledgeBaseFlowNodeConfiguration;
    impl crate::value::ToValue for KnowledgeBaseFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.guardrail_configuration {
                properties.insert(
                    "GuardrailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_configuration {
                properties.insert(
                    "InferenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KnowledgeBaseId".to_string(),
                crate::value::ToValue::to_value(&self.knowledge_base_id),
            );
            if let Some(ref value) = self.model_id {
                properties.insert(
                    "ModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_results {
                properties.insert(
                    "NumberOfResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.orchestration_configuration {
                properties.insert(
                    "OrchestrationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_template {
                properties.insert(
                    "PromptTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reranking_configuration {
                properties.insert(
                    "RerankingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-knowledgebaseorchestrationconfiguration.html>
    pub struct KnowledgeBaseOrchestrationConfiguration_ {
        pub additional_model_request_fields: Option<serde_json::Value>,
        pub inference_config: Option<Box<PromptInferenceConfiguration_>>,
        pub performance_config: Option<Box<PerformanceConfiguration_>>,
        pub prompt_template: Option<Box<KnowledgeBasePromptTemplate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_KnowledgeBaseOrchestrationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.KnowledgeBaseOrchestrationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_KnowledgeBaseOrchestrationConfiguration as KnowledgeBaseOrchestrationConfiguration;
    impl crate::value::ToValue for KnowledgeBaseOrchestrationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_model_request_fields {
                properties.insert(
                    "AdditionalModelRequestFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_config {
                properties.insert(
                    "InferenceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.performance_config {
                properties.insert(
                    "PerformanceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_template {
                properties.insert(
                    "PromptTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-knowledgebaseprompttemplate.html>
    pub struct KnowledgeBasePromptTemplate_ {
        pub text_prompt_template: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_KnowledgeBasePromptTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.KnowledgeBasePromptTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_KnowledgeBasePromptTemplate as KnowledgeBasePromptTemplate;
    impl crate::value::ToValue for KnowledgeBasePromptTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TextPromptTemplate".to_string(),
                crate::value::ToValue::to_value(&self.text_prompt_template),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-lambdafunctionflownodeconfiguration.html>
    pub struct LambdaFunctionFlowNodeConfiguration_ {
        pub lambda_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_LambdaFunctionFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.LambdaFunctionFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_LambdaFunctionFlowNodeConfiguration as LambdaFunctionFlowNodeConfiguration;
    impl crate::value::ToValue for LambdaFunctionFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-lexflownodeconfiguration.html>
    pub struct LexFlowNodeConfiguration_ {
        pub bot_alias_arn: crate::value::ExpString,
        pub locale_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_LexFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.LexFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_LexFlowNodeConfiguration as LexFlowNodeConfiguration;
    impl crate::value::ToValue for LexFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BotAliasArn".to_string(),
                crate::value::ToValue::to_value(&self.bot_alias_arn),
            );
            properties.insert(
                "LocaleId".to_string(),
                crate::value::ToValue::to_value(&self.locale_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-loopcontrollerflownodeconfiguration.html>
    pub struct LoopControllerFlowNodeConfiguration_ {
        pub continue_condition: Box<FlowCondition_>,
        pub max_iterations: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_LoopControllerFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.LoopControllerFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_LoopControllerFlowNodeConfiguration as LoopControllerFlowNodeConfiguration;
    impl crate::value::ToValue for LoopControllerFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContinueCondition".to_string(),
                crate::value::ToValue::to_value(&self.continue_condition),
            );
            if let Some(ref value) = self.max_iterations {
                properties.insert(
                    "MaxIterations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-loopflownodeconfiguration.html>
    pub struct LoopFlowNodeConfiguration_ {
        pub definition: Box<FlowDefinition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_LoopFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.LoopFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_LoopFlowNodeConfiguration as LoopFlowNodeConfiguration;
    impl crate::value::ToValue for LoopFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Definition".to_string(),
                crate::value::ToValue::to_value(&self.definition),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-metadataconfigurationforreranking.html>
    pub struct MetadataConfigurationForReranking_ {
        pub selection_mode: crate::value::ExpString,
        pub selective_mode_configuration: Option<Box<RerankingMetadataSelectiveModeConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_MetadataConfigurationForReranking {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.MetadataConfigurationForReranking"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_MetadataConfigurationForReranking as MetadataConfigurationForReranking;
    impl crate::value::ToValue for MetadataConfigurationForReranking_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SelectionMode".to_string(),
                crate::value::ToValue::to_value(&self.selection_mode),
            );
            if let Some(ref value) = self.selective_mode_configuration {
                properties.insert(
                    "SelectiveModeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-performanceconfiguration.html>
    pub struct PerformanceConfiguration_ {
        pub latency: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PerformanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PerformanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PerformanceConfiguration as PerformanceConfiguration;
    impl crate::value::ToValue for PerformanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.latency {
                properties.insert(
                    "Latency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-promptflownodeconfiguration.html>
    pub struct PromptFlowNodeConfiguration_ {
        pub guardrail_configuration: Option<Box<GuardrailConfiguration_>>,
        pub source_configuration: Box<PromptFlowNodeSourceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptFlowNodeConfiguration as PromptFlowNodeConfiguration;
    impl crate::value::ToValue for PromptFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.guardrail_configuration {
                properties.insert(
                    "GuardrailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.source_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-promptflownodeinlineconfiguration.html>
    pub struct PromptFlowNodeInlineConfiguration_ {
        pub inference_configuration: Option<Box<PromptInferenceConfiguration_>>,
        pub model_id: crate::value::ExpString,
        pub template_configuration: Box<PromptTemplateConfiguration_>,
        pub template_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptFlowNodeInlineConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptFlowNodeInlineConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptFlowNodeInlineConfiguration as PromptFlowNodeInlineConfiguration;
    impl crate::value::ToValue for PromptFlowNodeInlineConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inference_configuration {
                properties.insert(
                    "InferenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.insert(
                "TemplateConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.template_configuration),
            );
            properties.insert(
                "TemplateType".to_string(),
                crate::value::ToValue::to_value(&self.template_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-promptflownoderesourceconfiguration.html>
    pub struct PromptFlowNodeResourceConfiguration_ {
        pub prompt_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptFlowNodeResourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptFlowNodeResourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptFlowNodeResourceConfiguration as PromptFlowNodeResourceConfiguration;
    impl crate::value::ToValue for PromptFlowNodeResourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PromptArn".to_string(),
                crate::value::ToValue::to_value(&self.prompt_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-promptflownodesourceconfiguration.html>
    pub struct PromptFlowNodeSourceConfiguration_ {
        pub inline: Option<Box<PromptFlowNodeInlineConfiguration_>>,
        pub resource: Option<Box<PromptFlowNodeResourceConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptFlowNodeSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptFlowNodeSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptFlowNodeSourceConfiguration as PromptFlowNodeSourceConfiguration;
    impl crate::value::ToValue for PromptFlowNodeSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource {
                properties.insert(
                    "Resource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-promptinferenceconfiguration.html>
    pub struct PromptInferenceConfiguration_ {
        pub text: Box<PromptModelInferenceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptInferenceConfiguration as PromptInferenceConfiguration;
    impl crate::value::ToValue for PromptInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-promptinputvariable.html>
    pub struct PromptInputVariable_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptInputVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptInputVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptInputVariable as PromptInputVariable;
    impl crate::value::ToValue for PromptInputVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-promptmodelinferenceconfiguration.html>
    pub struct PromptModelInferenceConfiguration_ {
        pub max_tokens: Option<f64>,
        pub stop_sequences: Option<Vec<crate::value::ExpString>>,
        pub temperature: Option<f64>,
        pub top_p: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptModelInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptModelInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptModelInferenceConfiguration as PromptModelInferenceConfiguration;
    impl crate::value::ToValue for PromptModelInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_tokens {
                properties.insert(
                    "MaxTokens".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_sequences {
                properties.insert(
                    "StopSequences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temperature {
                properties.insert(
                    "Temperature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.top_p {
                properties.insert("TopP".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-prompttemplateconfiguration.html>
    pub struct PromptTemplateConfiguration_ {
        pub text: Box<TextPromptTemplateConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_PromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.PromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_PromptTemplateConfiguration as PromptTemplateConfiguration;
    impl crate::value::ToValue for PromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-rerankingmetadataselectivemodeconfiguration.html>
    pub struct RerankingMetadataSelectiveModeConfiguration_ {
        pub fields_to_exclude: Option<Vec<FieldForReranking_>>,
        pub fields_to_include: Option<Vec<FieldForReranking_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_RerankingMetadataSelectiveModeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.RerankingMetadataSelectiveModeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_RerankingMetadataSelectiveModeConfiguration as RerankingMetadataSelectiveModeConfiguration;
    impl crate::value::ToValue for RerankingMetadataSelectiveModeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fields_to_exclude {
                properties.insert(
                    "FieldsToExclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fields_to_include {
                properties.insert(
                    "FieldsToInclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-retrievalflownodeconfiguration.html>
    pub struct RetrievalFlowNodeConfiguration_ {
        pub service_configuration: Box<RetrievalFlowNodeServiceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_RetrievalFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.RetrievalFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_RetrievalFlowNodeConfiguration as RetrievalFlowNodeConfiguration;
    impl crate::value::ToValue for RetrievalFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ServiceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.service_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-retrievalflownodes3configuration.html>
    pub struct RetrievalFlowNodeS3Configuration_ {
        pub bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_RetrievalFlowNodeS3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.RetrievalFlowNodeS3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_RetrievalFlowNodeS3Configuration as RetrievalFlowNodeS3Configuration;
    impl crate::value::ToValue for RetrievalFlowNodeS3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-retrievalflownodeserviceconfiguration.html>
    pub struct RetrievalFlowNodeServiceConfiguration_ {
        pub s3: Option<Box<RetrievalFlowNodeS3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_RetrievalFlowNodeServiceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.RetrievalFlowNodeServiceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_RetrievalFlowNodeServiceConfiguration as RetrievalFlowNodeServiceConfiguration;
    impl crate::value::ToValue for RetrievalFlowNodeServiceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-s3location.html>
    pub struct S3Location_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-storageflownodeconfiguration.html>
    pub struct StorageFlowNodeConfiguration_ {
        pub service_configuration: Box<StorageFlowNodeServiceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_StorageFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.StorageFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_StorageFlowNodeConfiguration as StorageFlowNodeConfiguration;
    impl crate::value::ToValue for StorageFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ServiceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.service_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-storageflownodes3configuration.html>
    pub struct StorageFlowNodeS3Configuration_ {
        pub bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_StorageFlowNodeS3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.StorageFlowNodeS3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_StorageFlowNodeS3Configuration as StorageFlowNodeS3Configuration;
    impl crate::value::ToValue for StorageFlowNodeS3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-storageflownodeserviceconfiguration.html>
    pub struct StorageFlowNodeServiceConfiguration_ {
        pub s3: Option<Box<StorageFlowNodeS3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_StorageFlowNodeServiceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.StorageFlowNodeServiceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_StorageFlowNodeServiceConfiguration as StorageFlowNodeServiceConfiguration;
    impl crate::value::ToValue for StorageFlowNodeServiceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-textprompttemplateconfiguration.html>
    pub struct TextPromptTemplateConfiguration_ {
        pub input_variables: Option<Vec<PromptInputVariable_>>,
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_TextPromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.TextPromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_TextPromptTemplateConfiguration as TextPromptTemplateConfiguration;
    impl crate::value::ToValue for TextPromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_variables {
                properties.insert(
                    "InputVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-vectorsearchbedrockrerankingconfiguration.html>
    pub struct VectorSearchBedrockRerankingConfiguration_ {
        pub metadata_configuration: Option<Box<MetadataConfigurationForReranking_>>,
        pub model_configuration: Box<VectorSearchBedrockRerankingModelConfiguration_>,
        pub number_of_reranked_results: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_VectorSearchBedrockRerankingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.VectorSearchBedrockRerankingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_VectorSearchBedrockRerankingConfiguration as VectorSearchBedrockRerankingConfiguration;
    impl crate::value::ToValue for VectorSearchBedrockRerankingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metadata_configuration {
                properties.insert(
                    "MetadataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ModelConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.model_configuration),
            );
            if let Some(ref value) = self.number_of_reranked_results {
                properties.insert(
                    "NumberOfRerankedResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-vectorsearchbedrockrerankingmodelconfiguration.html>
    pub struct VectorSearchBedrockRerankingModelConfiguration_ {
        pub additional_model_request_fields: Option<serde_json::Value>,
        pub model_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_VectorSearchBedrockRerankingModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.VectorSearchBedrockRerankingModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_VectorSearchBedrockRerankingModelConfiguration as VectorSearchBedrockRerankingModelConfiguration;
    impl crate::value::ToValue for VectorSearchBedrockRerankingModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_model_request_fields {
                properties.insert(
                    "AdditionalModelRequestFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ModelArn".to_string(),
                crate::value::ToValue::to_value(&self.model_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flow-vectorsearchrerankingconfiguration.html>
    pub struct VectorSearchRerankingConfiguration_ {
        pub bedrock_reranking_configuration:
            Option<Box<VectorSearchBedrockRerankingConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Flow_VectorSearchRerankingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Flow.VectorSearchRerankingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Flow_VectorSearchRerankingConfiguration as VectorSearchRerankingConfiguration;
    impl crate::value::ToValue for VectorSearchRerankingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_reranking_configuration {
                properties.insert(
                    "BedrockRerankingConfiguration".to_string(),
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
pub mod flowalias {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowalias-flowaliasconcurrencyconfiguration.html>
    pub struct FlowAliasConcurrencyConfiguration_ {
        pub max_concurrency: Option<f64>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowAlias_FlowAliasConcurrencyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowAlias.FlowAliasConcurrencyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowAlias_FlowAliasConcurrencyConfiguration as FlowAliasConcurrencyConfiguration;
    impl crate::value::ToValue for FlowAliasConcurrencyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_concurrency {
                properties.insert(
                    "MaxConcurrency".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowalias-flowaliasroutingconfigurationlistitem.html>
    pub struct FlowAliasRoutingConfigurationListItem_ {
        pub flow_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowAlias_FlowAliasRoutingConfigurationListItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowAlias.FlowAliasRoutingConfigurationListItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowAlias_FlowAliasRoutingConfigurationListItem as FlowAliasRoutingConfigurationListItem;
    impl crate::value::ToValue for FlowAliasRoutingConfigurationListItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.flow_version {
                properties.insert(
                    "FlowVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod flowversion {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-agentflownodeconfiguration.html>
    pub struct AgentFlowNodeConfiguration_ {
        pub agent_alias_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_AgentFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.AgentFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_AgentFlowNodeConfiguration as AgentFlowNodeConfiguration;
    impl crate::value::ToValue for AgentFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentAliasArn".to_string(),
                crate::value::ToValue::to_value(&self.agent_alias_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-conditionflownodeconfiguration.html>
    pub struct ConditionFlowNodeConfiguration_ {
        pub conditions: Vec<FlowCondition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_ConditionFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.ConditionFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_ConditionFlowNodeConfiguration as ConditionFlowNodeConfiguration;
    impl crate::value::ToValue for ConditionFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(&self.conditions),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-fieldforreranking.html>
    pub struct FieldForReranking_ {
        pub field_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FieldForReranking {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FieldForReranking"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FieldForReranking as FieldForReranking;
    impl crate::value::ToValue for FieldForReranking_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldName".to_string(),
                crate::value::ToValue::to_value(&self.field_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flowcondition.html>
    pub struct FlowCondition_ {
        pub expression: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowCondition as FlowCondition;
    impl crate::value::ToValue for FlowCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flowconditionalconnectionconfiguration.html>
    pub struct FlowConditionalConnectionConfiguration_ {
        pub condition: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowConditionalConnectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowConditionalConnectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowConditionalConnectionConfiguration as FlowConditionalConnectionConfiguration;
    impl crate::value::ToValue for FlowConditionalConnectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flowconnection.html>
    pub struct FlowConnection_ {
        pub configuration: Option<Box<FlowConnectionConfiguration_>>,
        pub name: crate::value::ExpString,
        pub source: crate::value::ExpString,
        pub target: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowConnection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowConnection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowConnection as FlowConnection;
    impl crate::value::ToValue for FlowConnection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Source".to_string(),
                crate::value::ToValue::to_value(&self.source),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flowconnectionconfiguration.html>
    pub struct FlowConnectionConfiguration_ {
        pub conditional: Option<Box<FlowConditionalConnectionConfiguration_>>,
        pub data: Option<Box<FlowDataConnectionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowConnectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowConnectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowConnectionConfiguration as FlowConnectionConfiguration;
    impl crate::value::ToValue for FlowConnectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.conditional {
                properties.insert(
                    "Conditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data {
                properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flowdataconnectionconfiguration.html>
    pub struct FlowDataConnectionConfiguration_ {
        pub source_output: crate::value::ExpString,
        pub target_input: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowDataConnectionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowDataConnectionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowDataConnectionConfiguration as FlowDataConnectionConfiguration;
    impl crate::value::ToValue for FlowDataConnectionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceOutput".to_string(),
                crate::value::ToValue::to_value(&self.source_output),
            );
            properties.insert(
                "TargetInput".to_string(),
                crate::value::ToValue::to_value(&self.target_input),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flowdefinition.html>
    pub struct FlowDefinition_ {
        pub connections: Option<Vec<FlowConnection_>>,
        pub nodes: Option<Vec<FlowNode_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowDefinition as FlowDefinition;
    impl crate::value::ToValue for FlowDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.connections {
                properties.insert(
                    "Connections".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nodes {
                properties.insert("Nodes".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flownode.html>
    pub struct FlowNode_ {
        pub configuration: Option<Box<FlowNodeConfiguration_>>,
        pub inputs: Option<Vec<FlowNodeInput_>>,
        pub name: crate::value::ExpString,
        pub outputs: Option<Vec<FlowNodeOutput_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowNode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowNode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowNode as FlowNode;
    impl crate::value::ToValue for FlowNode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inputs {
                properties.insert("Inputs".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.outputs {
                properties.insert(
                    "Outputs".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flownodeconfiguration.html>
    pub struct FlowNodeConfiguration_ {
        pub agent: Option<Box<AgentFlowNodeConfiguration_>>,
        pub collector: Option<serde_json::Value>,
        pub condition: Option<Box<ConditionFlowNodeConfiguration_>>,
        pub inline_code: Option<Box<InlineCodeFlowNodeConfiguration_>>,
        pub input: Option<serde_json::Value>,
        pub iterator: Option<serde_json::Value>,
        pub knowledge_base: Option<Box<KnowledgeBaseFlowNodeConfiguration_>>,
        pub lambda_function: Option<Box<LambdaFunctionFlowNodeConfiguration_>>,
        pub lex: Option<Box<LexFlowNodeConfiguration_>>,
        pub r#loop: Option<Box<LoopFlowNodeConfiguration_>>,
        pub loop_controller: Option<Box<LoopControllerFlowNodeConfiguration_>>,
        pub loop_input: Option<serde_json::Value>,
        pub output: Option<serde_json::Value>,
        pub prompt: Option<Box<PromptFlowNodeConfiguration_>>,
        pub retrieval: Option<Box<RetrievalFlowNodeConfiguration_>>,
        pub storage: Option<Box<StorageFlowNodeConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowNodeConfiguration as FlowNodeConfiguration;
    impl crate::value::ToValue for FlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agent {
                properties.insert("Agent".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.collector {
                properties.insert(
                    "Collector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline_code {
                properties.insert(
                    "InlineCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input {
                properties.insert("Input".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.iterator {
                properties.insert(
                    "Iterator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.knowledge_base {
                properties.insert(
                    "KnowledgeBase".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_function {
                properties.insert(
                    "LambdaFunction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lex {
                properties.insert("Lex".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#loop {
                properties.insert("Loop".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.loop_controller {
                properties.insert(
                    "LoopController".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.loop_input {
                properties.insert(
                    "LoopInput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output {
                properties.insert("Output".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.prompt {
                properties.insert("Prompt".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.retrieval {
                properties.insert(
                    "Retrieval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.storage {
                properties.insert(
                    "Storage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flownodeinput.html>
    pub struct FlowNodeInput_ {
        pub expression: crate::value::ExpString,
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowNodeInput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowNodeInput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowNodeInput as FlowNodeInput;
    impl crate::value::ToValue for FlowNodeInput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Expression".to_string(),
                crate::value::ToValue::to_value(&self.expression),
            );
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-flownodeoutput.html>
    pub struct FlowNodeOutput_ {
        pub name: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_FlowNodeOutput {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.FlowNodeOutput"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_FlowNodeOutput as FlowNodeOutput;
    impl crate::value::ToValue for FlowNodeOutput_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-guardrailconfiguration.html>
    pub struct GuardrailConfiguration_ {
        pub guardrail_identifier: Option<crate::value::ExpString>,
        pub guardrail_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_GuardrailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.GuardrailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_GuardrailConfiguration as GuardrailConfiguration;
    impl crate::value::ToValue for GuardrailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.guardrail_identifier {
                properties.insert(
                    "GuardrailIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.guardrail_version {
                properties.insert(
                    "GuardrailVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-inlinecodeflownodeconfiguration.html>
    pub struct InlineCodeFlowNodeConfiguration_ {
        pub code: crate::value::ExpString,
        pub language: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_InlineCodeFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.InlineCodeFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_InlineCodeFlowNodeConfiguration as InlineCodeFlowNodeConfiguration;
    impl crate::value::ToValue for InlineCodeFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Code".to_string(),
                crate::value::ToValue::to_value(&self.code),
            );
            properties.insert(
                "Language".to_string(),
                crate::value::ToValue::to_value(&self.language),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-knowledgebaseflownodeconfiguration.html>
    pub struct KnowledgeBaseFlowNodeConfiguration_ {
        pub guardrail_configuration: Option<Box<GuardrailConfiguration_>>,
        pub inference_configuration: Option<Box<PromptInferenceConfiguration_>>,
        pub knowledge_base_id: crate::value::ExpString,
        pub model_id: Option<crate::value::ExpString>,
        pub number_of_results: Option<f64>,
        pub orchestration_configuration: Option<Box<KnowledgeBaseOrchestrationConfiguration_>>,
        pub prompt_template: Option<Box<KnowledgeBasePromptTemplate_>>,
        pub reranking_configuration: Option<Box<VectorSearchRerankingConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_KnowledgeBaseFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.KnowledgeBaseFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_KnowledgeBaseFlowNodeConfiguration as KnowledgeBaseFlowNodeConfiguration;
    impl crate::value::ToValue for KnowledgeBaseFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.guardrail_configuration {
                properties.insert(
                    "GuardrailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_configuration {
                properties.insert(
                    "InferenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "KnowledgeBaseId".to_string(),
                crate::value::ToValue::to_value(&self.knowledge_base_id),
            );
            if let Some(ref value) = self.model_id {
                properties.insert(
                    "ModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_results {
                properties.insert(
                    "NumberOfResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.orchestration_configuration {
                properties.insert(
                    "OrchestrationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_template {
                properties.insert(
                    "PromptTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reranking_configuration {
                properties.insert(
                    "RerankingConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-knowledgebaseorchestrationconfiguration.html>
    pub struct KnowledgeBaseOrchestrationConfiguration_ {
        pub additional_model_request_fields: Option<serde_json::Value>,
        pub inference_config: Option<Box<PromptInferenceConfiguration_>>,
        pub performance_config: Option<Box<PerformanceConfiguration_>>,
        pub prompt_template: Option<Box<KnowledgeBasePromptTemplate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_KnowledgeBaseOrchestrationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.KnowledgeBaseOrchestrationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_KnowledgeBaseOrchestrationConfiguration as KnowledgeBaseOrchestrationConfiguration;
    impl crate::value::ToValue for KnowledgeBaseOrchestrationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_model_request_fields {
                properties.insert(
                    "AdditionalModelRequestFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_config {
                properties.insert(
                    "InferenceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.performance_config {
                properties.insert(
                    "PerformanceConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_template {
                properties.insert(
                    "PromptTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-knowledgebaseprompttemplate.html>
    pub struct KnowledgeBasePromptTemplate_ {
        pub text_prompt_template: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_KnowledgeBasePromptTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.KnowledgeBasePromptTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_KnowledgeBasePromptTemplate as KnowledgeBasePromptTemplate;
    impl crate::value::ToValue for KnowledgeBasePromptTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TextPromptTemplate".to_string(),
                crate::value::ToValue::to_value(&self.text_prompt_template),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-lambdafunctionflownodeconfiguration.html>
    pub struct LambdaFunctionFlowNodeConfiguration_ {
        pub lambda_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_LambdaFunctionFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.LambdaFunctionFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_LambdaFunctionFlowNodeConfiguration as LambdaFunctionFlowNodeConfiguration;
    impl crate::value::ToValue for LambdaFunctionFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-lexflownodeconfiguration.html>
    pub struct LexFlowNodeConfiguration_ {
        pub bot_alias_arn: crate::value::ExpString,
        pub locale_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_LexFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.LexFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_LexFlowNodeConfiguration as LexFlowNodeConfiguration;
    impl crate::value::ToValue for LexFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BotAliasArn".to_string(),
                crate::value::ToValue::to_value(&self.bot_alias_arn),
            );
            properties.insert(
                "LocaleId".to_string(),
                crate::value::ToValue::to_value(&self.locale_id),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-loopcontrollerflownodeconfiguration.html>
    pub struct LoopControllerFlowNodeConfiguration_ {
        pub continue_condition: Box<FlowCondition_>,
        pub max_iterations: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_LoopControllerFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.LoopControllerFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_LoopControllerFlowNodeConfiguration as LoopControllerFlowNodeConfiguration;
    impl crate::value::ToValue for LoopControllerFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContinueCondition".to_string(),
                crate::value::ToValue::to_value(&self.continue_condition),
            );
            if let Some(ref value) = self.max_iterations {
                properties.insert(
                    "MaxIterations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-loopflownodeconfiguration.html>
    pub struct LoopFlowNodeConfiguration_ {
        pub definition: Box<FlowDefinition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_LoopFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.LoopFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_LoopFlowNodeConfiguration as LoopFlowNodeConfiguration;
    impl crate::value::ToValue for LoopFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Definition".to_string(),
                crate::value::ToValue::to_value(&self.definition),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-metadataconfigurationforreranking.html>
    pub struct MetadataConfigurationForReranking_ {
        pub selection_mode: crate::value::ExpString,
        pub selective_mode_configuration: Option<Box<RerankingMetadataSelectiveModeConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_MetadataConfigurationForReranking {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.MetadataConfigurationForReranking"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_MetadataConfigurationForReranking as MetadataConfigurationForReranking;
    impl crate::value::ToValue for MetadataConfigurationForReranking_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SelectionMode".to_string(),
                crate::value::ToValue::to_value(&self.selection_mode),
            );
            if let Some(ref value) = self.selective_mode_configuration {
                properties.insert(
                    "SelectiveModeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-performanceconfiguration.html>
    pub struct PerformanceConfiguration_ {
        pub latency: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PerformanceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PerformanceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PerformanceConfiguration as PerformanceConfiguration;
    impl crate::value::ToValue for PerformanceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.latency {
                properties.insert(
                    "Latency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-promptflownodeconfiguration.html>
    pub struct PromptFlowNodeConfiguration_ {
        pub guardrail_configuration: Option<Box<GuardrailConfiguration_>>,
        pub source_configuration: Box<PromptFlowNodeSourceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptFlowNodeConfiguration as PromptFlowNodeConfiguration;
    impl crate::value::ToValue for PromptFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.guardrail_configuration {
                properties.insert(
                    "GuardrailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.source_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-promptflownodeinlineconfiguration.html>
    pub struct PromptFlowNodeInlineConfiguration_ {
        pub inference_configuration: Option<Box<PromptInferenceConfiguration_>>,
        pub model_id: crate::value::ExpString,
        pub template_configuration: Box<PromptTemplateConfiguration_>,
        pub template_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptFlowNodeInlineConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptFlowNodeInlineConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptFlowNodeInlineConfiguration as PromptFlowNodeInlineConfiguration;
    impl crate::value::ToValue for PromptFlowNodeInlineConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inference_configuration {
                properties.insert(
                    "InferenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ModelId".to_string(),
                crate::value::ToValue::to_value(&self.model_id),
            );
            properties.insert(
                "TemplateConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.template_configuration),
            );
            properties.insert(
                "TemplateType".to_string(),
                crate::value::ToValue::to_value(&self.template_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-promptflownoderesourceconfiguration.html>
    pub struct PromptFlowNodeResourceConfiguration_ {
        pub prompt_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptFlowNodeResourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptFlowNodeResourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptFlowNodeResourceConfiguration as PromptFlowNodeResourceConfiguration;
    impl crate::value::ToValue for PromptFlowNodeResourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PromptArn".to_string(),
                crate::value::ToValue::to_value(&self.prompt_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-promptflownodesourceconfiguration.html>
    pub struct PromptFlowNodeSourceConfiguration_ {
        pub inline: Option<Box<PromptFlowNodeInlineConfiguration_>>,
        pub resource: Option<Box<PromptFlowNodeResourceConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptFlowNodeSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptFlowNodeSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptFlowNodeSourceConfiguration as PromptFlowNodeSourceConfiguration;
    impl crate::value::ToValue for PromptFlowNodeSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource {
                properties.insert(
                    "Resource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-promptinferenceconfiguration.html>
    pub struct PromptInferenceConfiguration_ {
        pub text: Box<PromptModelInferenceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptInferenceConfiguration as PromptInferenceConfiguration;
    impl crate::value::ToValue for PromptInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-promptinputvariable.html>
    pub struct PromptInputVariable_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptInputVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptInputVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptInputVariable as PromptInputVariable;
    impl crate::value::ToValue for PromptInputVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-promptmodelinferenceconfiguration.html>
    pub struct PromptModelInferenceConfiguration_ {
        pub max_tokens: Option<f64>,
        pub stop_sequences: Option<Vec<crate::value::ExpString>>,
        pub temperature: Option<f64>,
        pub top_p: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptModelInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptModelInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptModelInferenceConfiguration as PromptModelInferenceConfiguration;
    impl crate::value::ToValue for PromptModelInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_tokens {
                properties.insert(
                    "MaxTokens".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_sequences {
                properties.insert(
                    "StopSequences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temperature {
                properties.insert(
                    "Temperature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.top_p {
                properties.insert("TopP".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-prompttemplateconfiguration.html>
    pub struct PromptTemplateConfiguration_ {
        pub text: Box<TextPromptTemplateConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_PromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.PromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_PromptTemplateConfiguration as PromptTemplateConfiguration;
    impl crate::value::ToValue for PromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-rerankingmetadataselectivemodeconfiguration.html>
    pub struct RerankingMetadataSelectiveModeConfiguration_ {
        pub fields_to_exclude: Option<Vec<FieldForReranking_>>,
        pub fields_to_include: Option<Vec<FieldForReranking_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_RerankingMetadataSelectiveModeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.RerankingMetadataSelectiveModeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_RerankingMetadataSelectiveModeConfiguration as RerankingMetadataSelectiveModeConfiguration;
    impl crate::value::ToValue for RerankingMetadataSelectiveModeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fields_to_exclude {
                properties.insert(
                    "FieldsToExclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fields_to_include {
                properties.insert(
                    "FieldsToInclude".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-retrievalflownodeconfiguration.html>
    pub struct RetrievalFlowNodeConfiguration_ {
        pub service_configuration: Box<RetrievalFlowNodeServiceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_RetrievalFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.RetrievalFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_RetrievalFlowNodeConfiguration as RetrievalFlowNodeConfiguration;
    impl crate::value::ToValue for RetrievalFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ServiceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.service_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-retrievalflownodes3configuration.html>
    pub struct RetrievalFlowNodeS3Configuration_ {
        pub bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_RetrievalFlowNodeS3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.RetrievalFlowNodeS3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_RetrievalFlowNodeS3Configuration as RetrievalFlowNodeS3Configuration;
    impl crate::value::ToValue for RetrievalFlowNodeS3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-retrievalflownodeserviceconfiguration.html>
    pub struct RetrievalFlowNodeServiceConfiguration_ {
        pub s3: Option<Box<RetrievalFlowNodeS3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_RetrievalFlowNodeServiceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.RetrievalFlowNodeServiceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_RetrievalFlowNodeServiceConfiguration as RetrievalFlowNodeServiceConfiguration;
    impl crate::value::ToValue for RetrievalFlowNodeServiceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-storageflownodeconfiguration.html>
    pub struct StorageFlowNodeConfiguration_ {
        pub service_configuration: Box<StorageFlowNodeServiceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_StorageFlowNodeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.StorageFlowNodeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_StorageFlowNodeConfiguration as StorageFlowNodeConfiguration;
    impl crate::value::ToValue for StorageFlowNodeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ServiceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.service_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-storageflownodes3configuration.html>
    pub struct StorageFlowNodeS3Configuration_ {
        pub bucket_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_StorageFlowNodeS3Configuration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.StorageFlowNodeS3Configuration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_StorageFlowNodeS3Configuration as StorageFlowNodeS3Configuration;
    impl crate::value::ToValue for StorageFlowNodeS3Configuration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-storageflownodeserviceconfiguration.html>
    pub struct StorageFlowNodeServiceConfiguration_ {
        pub s3: Option<Box<StorageFlowNodeS3Configuration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_StorageFlowNodeServiceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.StorageFlowNodeServiceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_StorageFlowNodeServiceConfiguration as StorageFlowNodeServiceConfiguration;
    impl crate::value::ToValue for StorageFlowNodeServiceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3 {
                properties.insert("S3".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-textprompttemplateconfiguration.html>
    pub struct TextPromptTemplateConfiguration_ {
        pub input_variables: Option<Vec<PromptInputVariable_>>,
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_TextPromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.TextPromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_TextPromptTemplateConfiguration as TextPromptTemplateConfiguration;
    impl crate::value::ToValue for TextPromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_variables {
                properties.insert(
                    "InputVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-vectorsearchbedrockrerankingconfiguration.html>
    pub struct VectorSearchBedrockRerankingConfiguration_ {
        pub metadata_configuration: Option<Box<MetadataConfigurationForReranking_>>,
        pub model_configuration: Box<VectorSearchBedrockRerankingModelConfiguration_>,
        pub number_of_reranked_results: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_VectorSearchBedrockRerankingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.VectorSearchBedrockRerankingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_VectorSearchBedrockRerankingConfiguration as VectorSearchBedrockRerankingConfiguration;
    impl crate::value::ToValue for VectorSearchBedrockRerankingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.metadata_configuration {
                properties.insert(
                    "MetadataConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ModelConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.model_configuration),
            );
            if let Some(ref value) = self.number_of_reranked_results {
                properties.insert(
                    "NumberOfRerankedResults".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-vectorsearchbedrockrerankingmodelconfiguration.html>
    pub struct VectorSearchBedrockRerankingModelConfiguration_ {
        pub additional_model_request_fields: Option<serde_json::Value>,
        pub model_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_VectorSearchBedrockRerankingModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.VectorSearchBedrockRerankingModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_VectorSearchBedrockRerankingModelConfiguration as VectorSearchBedrockRerankingModelConfiguration;
    impl crate::value::ToValue for VectorSearchBedrockRerankingModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_model_request_fields {
                properties.insert(
                    "AdditionalModelRequestFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ModelArn".to_string(),
                crate::value::ToValue::to_value(&self.model_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-flowversion-vectorsearchrerankingconfiguration.html>
    pub struct VectorSearchRerankingConfiguration_ {
        pub bedrock_reranking_configuration:
            Option<Box<VectorSearchBedrockRerankingConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_FlowVersion_VectorSearchRerankingConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::FlowVersion.VectorSearchRerankingConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_FlowVersion_VectorSearchRerankingConfiguration as VectorSearchRerankingConfiguration;
    impl crate::value::ToValue for VectorSearchRerankingConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_reranking_configuration {
                properties.insert(
                    "BedrockRerankingConfiguration".to_string(),
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
pub mod guardrail {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-automatedreasoningpolicyconfig.html>
    pub struct AutomatedReasoningPolicyConfig_ {
        pub confidence_threshold: Option<f64>,
        pub policies: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_AutomatedReasoningPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.AutomatedReasoningPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_AutomatedReasoningPolicyConfig as AutomatedReasoningPolicyConfig;
    impl crate::value::ToValue for AutomatedReasoningPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.confidence_threshold {
                properties.insert(
                    "ConfidenceThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Policies".to_string(),
                crate::value::ToValue::to_value(&self.policies),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-contentfilterconfig.html>
    pub struct ContentFilterConfig_ {
        pub input_action: Option<crate::value::ExpString>,
        pub input_enabled: Option<crate::value::ExpBool>,
        pub input_modalities: Option<Vec<crate::value::ExpString>>,
        pub input_strength: crate::value::ExpString,
        pub output_action: Option<crate::value::ExpString>,
        pub output_enabled: Option<crate::value::ExpBool>,
        pub output_modalities: Option<Vec<crate::value::ExpString>>,
        pub output_strength: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_ContentFilterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.ContentFilterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_ContentFilterConfig as ContentFilterConfig;
    impl crate::value::ToValue for ContentFilterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_action {
                properties.insert(
                    "InputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_enabled {
                properties.insert(
                    "InputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_modalities {
                properties.insert(
                    "InputModalities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputStrength".to_string(),
                crate::value::ToValue::to_value(&self.input_strength),
            );
            if let Some(ref value) = self.output_action {
                properties.insert(
                    "OutputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_enabled {
                properties.insert(
                    "OutputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_modalities {
                properties.insert(
                    "OutputModalities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-contentfilterstierconfig.html>
    pub struct ContentFiltersTierConfig_ {
        pub tier_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_ContentFiltersTierConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.ContentFiltersTierConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_ContentFiltersTierConfig as ContentFiltersTierConfig;
    impl crate::value::ToValue for ContentFiltersTierConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TierName".to_string(),
                crate::value::ToValue::to_value(&self.tier_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-contentpolicyconfig.html>
    pub struct ContentPolicyConfig_ {
        pub content_filters_tier_config: Option<Box<ContentFiltersTierConfig_>>,
        pub filters_config: Vec<ContentFilterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_ContentPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.ContentPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_ContentPolicyConfig as ContentPolicyConfig;
    impl crate::value::ToValue for ContentPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content_filters_tier_config {
                properties.insert(
                    "ContentFiltersTierConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FiltersConfig".to_string(),
                crate::value::ToValue::to_value(&self.filters_config),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-contextualgroundingfilterconfig.html>
    pub struct ContextualGroundingFilterConfig_ {
        pub action: Option<crate::value::ExpString>,
        pub enabled: Option<crate::value::ExpBool>,
        pub threshold: f64,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_ContextualGroundingFilterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.ContextualGroundingFilterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_ContextualGroundingFilterConfig as ContextualGroundingFilterConfig;
    impl crate::value::ToValue for ContextualGroundingFilterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-contextualgroundingpolicyconfig.html>
    pub struct ContextualGroundingPolicyConfig_ {
        pub filters_config: Vec<ContextualGroundingFilterConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_ContextualGroundingPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.ContextualGroundingPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_ContextualGroundingPolicyConfig as ContextualGroundingPolicyConfig;
    impl crate::value::ToValue for ContextualGroundingPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FiltersConfig".to_string(),
                crate::value::ToValue::to_value(&self.filters_config),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-guardrailcrossregionconfig.html>
    pub struct GuardrailCrossRegionConfig_ {
        pub guardrail_profile_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_GuardrailCrossRegionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.GuardrailCrossRegionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_GuardrailCrossRegionConfig as GuardrailCrossRegionConfig;
    impl crate::value::ToValue for GuardrailCrossRegionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "GuardrailProfileArn".to_string(),
                crate::value::ToValue::to_value(&self.guardrail_profile_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-managedwordsconfig.html>
    pub struct ManagedWordsConfig_ {
        pub input_action: Option<crate::value::ExpString>,
        pub input_enabled: Option<crate::value::ExpBool>,
        pub output_action: Option<crate::value::ExpString>,
        pub output_enabled: Option<crate::value::ExpBool>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_ManagedWordsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.ManagedWordsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_ManagedWordsConfig as ManagedWordsConfig;
    impl crate::value::ToValue for ManagedWordsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_action {
                properties.insert(
                    "InputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_enabled {
                properties.insert(
                    "InputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_action {
                properties.insert(
                    "OutputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_enabled {
                properties.insert(
                    "OutputEnabled".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-piientityconfig.html>
    pub struct PiiEntityConfig_ {
        pub action: crate::value::ExpString,
        pub input_action: Option<crate::value::ExpString>,
        pub input_enabled: Option<crate::value::ExpBool>,
        pub output_action: Option<crate::value::ExpString>,
        pub output_enabled: Option<crate::value::ExpBool>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_PiiEntityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.PiiEntityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_PiiEntityConfig as PiiEntityConfig;
    impl crate::value::ToValue for PiiEntityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.input_action {
                properties.insert(
                    "InputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_enabled {
                properties.insert(
                    "InputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_action {
                properties.insert(
                    "OutputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_enabled {
                properties.insert(
                    "OutputEnabled".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-regexconfig.html>
    pub struct RegexConfig_ {
        pub action: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub input_action: Option<crate::value::ExpString>,
        pub input_enabled: Option<crate::value::ExpBool>,
        pub name: crate::value::ExpString,
        pub output_action: Option<crate::value::ExpString>,
        pub output_enabled: Option<crate::value::ExpBool>,
        pub pattern: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_RegexConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.RegexConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_RegexConfig as RegexConfig;
    impl crate::value::ToValue for RegexConfig_ {
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
            if let Some(ref value) = self.input_action {
                properties.insert(
                    "InputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_enabled {
                properties.insert(
                    "InputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.output_action {
                properties.insert(
                    "OutputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_enabled {
                properties.insert(
                    "OutputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Pattern".to_string(),
                crate::value::ToValue::to_value(&self.pattern),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-sensitiveinformationpolicyconfig.html>
    pub struct SensitiveInformationPolicyConfig_ {
        pub pii_entities_config: Option<Vec<PiiEntityConfig_>>,
        pub regexes_config: Option<Vec<RegexConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_SensitiveInformationPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.SensitiveInformationPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_SensitiveInformationPolicyConfig as SensitiveInformationPolicyConfig;
    impl crate::value::ToValue for SensitiveInformationPolicyConfig_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-topicconfig.html>
    pub struct TopicConfig_ {
        pub definition: crate::value::ExpString,
        pub examples: Option<Vec<crate::value::ExpString>>,
        pub input_action: Option<crate::value::ExpString>,
        pub input_enabled: Option<crate::value::ExpBool>,
        pub name: crate::value::ExpString,
        pub output_action: Option<crate::value::ExpString>,
        pub output_enabled: Option<crate::value::ExpBool>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_TopicConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.TopicConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_TopicConfig as TopicConfig;
    impl crate::value::ToValue for TopicConfig_ {
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
            if let Some(ref value) = self.input_action {
                properties.insert(
                    "InputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_enabled {
                properties.insert(
                    "InputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.output_action {
                properties.insert(
                    "OutputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_enabled {
                properties.insert(
                    "OutputEnabled".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-topicpolicyconfig.html>
    pub struct TopicPolicyConfig_ {
        pub topics_config: Vec<TopicConfig_>,
        pub topics_tier_config: Option<Box<TopicsTierConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_TopicPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.TopicPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_TopicPolicyConfig as TopicPolicyConfig;
    impl crate::value::ToValue for TopicPolicyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TopicsConfig".to_string(),
                crate::value::ToValue::to_value(&self.topics_config),
            );
            if let Some(ref value) = self.topics_tier_config {
                properties.insert(
                    "TopicsTierConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-topicstierconfig.html>
    pub struct TopicsTierConfig_ {
        pub tier_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_TopicsTierConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.TopicsTierConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_TopicsTierConfig as TopicsTierConfig;
    impl crate::value::ToValue for TopicsTierConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TierName".to_string(),
                crate::value::ToValue::to_value(&self.tier_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-wordconfig.html>
    pub struct WordConfig_ {
        pub input_action: Option<crate::value::ExpString>,
        pub input_enabled: Option<crate::value::ExpBool>,
        pub output_action: Option<crate::value::ExpString>,
        pub output_enabled: Option<crate::value::ExpBool>,
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_WordConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.WordConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_WordConfig as WordConfig;
    impl crate::value::ToValue for WordConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_action {
                properties.insert(
                    "InputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_enabled {
                properties.insert(
                    "InputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_action {
                properties.insert(
                    "OutputAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.output_enabled {
                properties.insert(
                    "OutputEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-guardrail-wordpolicyconfig.html>
    pub struct WordPolicyConfig_ {
        pub managed_word_lists_config: Option<Vec<ManagedWordsConfig_>>,
        pub words_config: Option<Vec<WordConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Guardrail_WordPolicyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Guardrail.WordPolicyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Guardrail_WordPolicyConfig as WordPolicyConfig;
    impl crate::value::ToValue for WordPolicyConfig_ {
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
}
pub mod intelligentpromptrouter {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-intelligentpromptrouter-promptroutertargetmodel.html>
    pub struct PromptRouterTargetModel_ {
        pub model_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_IntelligentPromptRouter_PromptRouterTargetModel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::IntelligentPromptRouter.PromptRouterTargetModel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_IntelligentPromptRouter_PromptRouterTargetModel as PromptRouterTargetModel;
    impl crate::value::ToValue for PromptRouterTargetModel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ModelArn".to_string(),
                crate::value::ToValue::to_value(&self.model_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-intelligentpromptrouter-routingcriteria.html>
    pub struct RoutingCriteria_ {
        pub response_quality_difference: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_IntelligentPromptRouter_RoutingCriteria {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::IntelligentPromptRouter.RoutingCriteria"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_IntelligentPromptRouter_RoutingCriteria as RoutingCriteria;
    impl crate::value::ToValue for RoutingCriteria_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ResponseQualityDifference".to_string(),
                crate::value::ToValue::to_value(&self.response_quality_difference),
            );
            properties.into()
        }
    }
}
pub mod knowledgebase {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-audioconfiguration.html>
    pub struct AudioConfiguration_ {
        pub segmentation_configuration: Box<AudioSegmentationConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_AudioConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.AudioConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_AudioConfiguration as AudioConfiguration;
    impl crate::value::ToValue for AudioConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SegmentationConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.segmentation_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-audiosegmentationconfiguration.html>
    pub struct AudioSegmentationConfiguration_ {
        pub fixed_length_duration: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_AudioSegmentationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.AudioSegmentationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_AudioSegmentationConfiguration as AudioSegmentationConfiguration;
    impl crate::value::ToValue for AudioSegmentationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FixedLengthDuration".to_string(),
                crate::value::ToValue::to_value(&self.fixed_length_duration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-bedrockembeddingmodelconfiguration.html>
    pub struct BedrockEmbeddingModelConfiguration_ {
        pub audio: Option<Vec<AudioConfiguration_>>,
        pub dimensions: Option<i32>,
        pub embedding_data_type: Option<crate::value::ExpString>,
        pub video: Option<Vec<VideoConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_BedrockEmbeddingModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.BedrockEmbeddingModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_BedrockEmbeddingModelConfiguration as BedrockEmbeddingModelConfiguration;
    impl crate::value::ToValue for BedrockEmbeddingModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio {
                properties.insert("Audio".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.embedding_data_type {
                properties.insert(
                    "EmbeddingDataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.video {
                properties.insert("Video".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-curatedquery.html>
    pub struct CuratedQuery_ {
        pub natural_language: crate::value::ExpString,
        pub sql: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_CuratedQuery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.CuratedQuery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_CuratedQuery as CuratedQuery;
    impl crate::value::ToValue for CuratedQuery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NaturalLanguage".to_string(),
                crate::value::ToValue::to_value(&self.natural_language),
            );
            properties.insert(
                "Sql".to_string(),
                crate::value::ToValue::to_value(&self.sql),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-embeddingmodelconfiguration.html>
    pub struct EmbeddingModelConfiguration_ {
        pub bedrock_embedding_model_configuration: Option<Box<BedrockEmbeddingModelConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_EmbeddingModelConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.EmbeddingModelConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_EmbeddingModelConfiguration as EmbeddingModelConfiguration;
    impl crate::value::ToValue for EmbeddingModelConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_embedding_model_configuration {
                properties.insert(
                    "BedrockEmbeddingModelConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-kendraknowledgebaseconfiguration.html>
    pub struct KendraKnowledgeBaseConfiguration_ {
        pub kendra_index_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_KendraKnowledgeBaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.KendraKnowledgeBaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_KendraKnowledgeBaseConfiguration as KendraKnowledgeBaseConfiguration;
    impl crate::value::ToValue for KendraKnowledgeBaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KendraIndexArn".to_string(),
                crate::value::ToValue::to_value(&self.kendra_index_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-knowledgebaseconfiguration.html>
    pub struct KnowledgeBaseConfiguration_ {
        pub kendra_knowledge_base_configuration: Option<Box<KendraKnowledgeBaseConfiguration_>>,
        pub sql_knowledge_base_configuration: Option<Box<SqlKnowledgeBaseConfiguration_>>,
        pub r#type: crate::value::ExpString,
        pub vector_knowledge_base_configuration: Option<Box<VectorKnowledgeBaseConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_KnowledgeBaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.KnowledgeBaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_KnowledgeBaseConfiguration as KnowledgeBaseConfiguration;
    impl crate::value::ToValue for KnowledgeBaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kendra_knowledge_base_configuration {
                properties.insert(
                    "KendraKnowledgeBaseConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sql_knowledge_base_configuration {
                properties.insert(
                    "SqlKnowledgeBaseConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.vector_knowledge_base_configuration {
                properties.insert(
                    "VectorKnowledgeBaseConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-mongodbatlasconfiguration.html>
    pub struct MongoDbAtlasConfiguration_ {
        pub collection_name: crate::value::ExpString,
        pub credentials_secret_arn: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub endpoint: crate::value::ExpString,
        pub endpoint_service_name: Option<crate::value::ExpString>,
        pub field_mapping: Box<MongoDbAtlasFieldMapping_>,
        pub text_index_name: Option<crate::value::ExpString>,
        pub vector_index_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_MongoDbAtlasConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.MongoDbAtlasConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_MongoDbAtlasConfiguration as MongoDbAtlasConfiguration;
    impl crate::value::ToValue for MongoDbAtlasConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CollectionName".to_string(),
                crate::value::ToValue::to_value(&self.collection_name),
            );
            properties.insert(
                "CredentialsSecretArn".to_string(),
                crate::value::ToValue::to_value(&self.credentials_secret_arn),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            if let Some(ref value) = self.endpoint_service_name {
                properties.insert(
                    "EndpointServiceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FieldMapping".to_string(),
                crate::value::ToValue::to_value(&self.field_mapping),
            );
            if let Some(ref value) = self.text_index_name {
                properties.insert(
                    "TextIndexName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VectorIndexName".to_string(),
                crate::value::ToValue::to_value(&self.vector_index_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-mongodbatlasfieldmapping.html>
    pub struct MongoDbAtlasFieldMapping_ {
        pub metadata_field: crate::value::ExpString,
        pub text_field: crate::value::ExpString,
        pub vector_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_MongoDbAtlasFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.MongoDbAtlasFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_MongoDbAtlasFieldMapping as MongoDbAtlasFieldMapping;
    impl crate::value::ToValue for MongoDbAtlasFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetadataField".to_string(),
                crate::value::ToValue::to_value(&self.metadata_field),
            );
            properties.insert(
                "TextField".to_string(),
                crate::value::ToValue::to_value(&self.text_field),
            );
            properties.insert(
                "VectorField".to_string(),
                crate::value::ToValue::to_value(&self.vector_field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-neptuneanalyticsconfiguration.html>
    pub struct NeptuneAnalyticsConfiguration_ {
        pub field_mapping: Box<NeptuneAnalyticsFieldMapping_>,
        pub graph_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_NeptuneAnalyticsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.NeptuneAnalyticsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_NeptuneAnalyticsConfiguration as NeptuneAnalyticsConfiguration;
    impl crate::value::ToValue for NeptuneAnalyticsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldMapping".to_string(),
                crate::value::ToValue::to_value(&self.field_mapping),
            );
            properties.insert(
                "GraphArn".to_string(),
                crate::value::ToValue::to_value(&self.graph_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-neptuneanalyticsfieldmapping.html>
    pub struct NeptuneAnalyticsFieldMapping_ {
        pub metadata_field: crate::value::ExpString,
        pub text_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_NeptuneAnalyticsFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.NeptuneAnalyticsFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_NeptuneAnalyticsFieldMapping as NeptuneAnalyticsFieldMapping;
    impl crate::value::ToValue for NeptuneAnalyticsFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetadataField".to_string(),
                crate::value::ToValue::to_value(&self.metadata_field),
            );
            properties.insert(
                "TextField".to_string(),
                crate::value::ToValue::to_value(&self.text_field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-opensearchmanagedclusterconfiguration.html>
    pub struct OpenSearchManagedClusterConfiguration_ {
        pub domain_arn: crate::value::ExpString,
        pub domain_endpoint: crate::value::ExpString,
        pub field_mapping: Box<OpenSearchManagedClusterFieldMapping_>,
        pub vector_index_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_OpenSearchManagedClusterConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.OpenSearchManagedClusterConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_OpenSearchManagedClusterConfiguration as OpenSearchManagedClusterConfiguration;
    impl crate::value::ToValue for OpenSearchManagedClusterConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DomainArn".to_string(),
                crate::value::ToValue::to_value(&self.domain_arn),
            );
            properties.insert(
                "DomainEndpoint".to_string(),
                crate::value::ToValue::to_value(&self.domain_endpoint),
            );
            properties.insert(
                "FieldMapping".to_string(),
                crate::value::ToValue::to_value(&self.field_mapping),
            );
            properties.insert(
                "VectorIndexName".to_string(),
                crate::value::ToValue::to_value(&self.vector_index_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-opensearchmanagedclusterfieldmapping.html>
    pub struct OpenSearchManagedClusterFieldMapping_ {
        pub metadata_field: crate::value::ExpString,
        pub text_field: crate::value::ExpString,
        pub vector_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_OpenSearchManagedClusterFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.OpenSearchManagedClusterFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_OpenSearchManagedClusterFieldMapping as OpenSearchManagedClusterFieldMapping;
    impl crate::value::ToValue for OpenSearchManagedClusterFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetadataField".to_string(),
                crate::value::ToValue::to_value(&self.metadata_field),
            );
            properties.insert(
                "TextField".to_string(),
                crate::value::ToValue::to_value(&self.text_field),
            );
            properties.insert(
                "VectorField".to_string(),
                crate::value::ToValue::to_value(&self.vector_field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-opensearchserverlessconfiguration.html>
    pub struct OpenSearchServerlessConfiguration_ {
        pub collection_arn: crate::value::ExpString,
        pub field_mapping: Box<OpenSearchServerlessFieldMapping_>,
        pub vector_index_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_OpenSearchServerlessConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.OpenSearchServerlessConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_OpenSearchServerlessConfiguration as OpenSearchServerlessConfiguration;
    impl crate::value::ToValue for OpenSearchServerlessConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CollectionArn".to_string(),
                crate::value::ToValue::to_value(&self.collection_arn),
            );
            properties.insert(
                "FieldMapping".to_string(),
                crate::value::ToValue::to_value(&self.field_mapping),
            );
            properties.insert(
                "VectorIndexName".to_string(),
                crate::value::ToValue::to_value(&self.vector_index_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-opensearchserverlessfieldmapping.html>
    pub struct OpenSearchServerlessFieldMapping_ {
        pub metadata_field: crate::value::ExpString,
        pub text_field: crate::value::ExpString,
        pub vector_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_OpenSearchServerlessFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.OpenSearchServerlessFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_OpenSearchServerlessFieldMapping as OpenSearchServerlessFieldMapping;
    impl crate::value::ToValue for OpenSearchServerlessFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetadataField".to_string(),
                crate::value::ToValue::to_value(&self.metadata_field),
            );
            properties.insert(
                "TextField".to_string(),
                crate::value::ToValue::to_value(&self.text_field),
            );
            properties.insert(
                "VectorField".to_string(),
                crate::value::ToValue::to_value(&self.vector_field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-pineconeconfiguration.html>
    pub struct PineconeConfiguration_ {
        pub connection_string: crate::value::ExpString,
        pub credentials_secret_arn: crate::value::ExpString,
        pub field_mapping: Box<PineconeFieldMapping_>,
        pub namespace: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_PineconeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.PineconeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_PineconeConfiguration as PineconeConfiguration;
    impl crate::value::ToValue for PineconeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConnectionString".to_string(),
                crate::value::ToValue::to_value(&self.connection_string),
            );
            properties.insert(
                "CredentialsSecretArn".to_string(),
                crate::value::ToValue::to_value(&self.credentials_secret_arn),
            );
            properties.insert(
                "FieldMapping".to_string(),
                crate::value::ToValue::to_value(&self.field_mapping),
            );
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-pineconefieldmapping.html>
    pub struct PineconeFieldMapping_ {
        pub metadata_field: crate::value::ExpString,
        pub text_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_PineconeFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.PineconeFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_PineconeFieldMapping as PineconeFieldMapping;
    impl crate::value::ToValue for PineconeFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MetadataField".to_string(),
                crate::value::ToValue::to_value(&self.metadata_field),
            );
            properties.insert(
                "TextField".to_string(),
                crate::value::ToValue::to_value(&self.text_field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-querygenerationcolumn.html>
    pub struct QueryGenerationColumn_ {
        pub description: Option<crate::value::ExpString>,
        pub inclusion: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_QueryGenerationColumn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.QueryGenerationColumn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_QueryGenerationColumn as QueryGenerationColumn;
    impl crate::value::ToValue for QueryGenerationColumn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion {
                properties.insert(
                    "Inclusion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-querygenerationconfiguration.html>
    pub struct QueryGenerationConfiguration_ {
        pub execution_timeout_seconds: Option<i32>,
        pub generation_context: Option<Box<QueryGenerationContext_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_QueryGenerationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.QueryGenerationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_QueryGenerationConfiguration as QueryGenerationConfiguration;
    impl crate::value::ToValue for QueryGenerationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.execution_timeout_seconds {
                properties.insert(
                    "ExecutionTimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generation_context {
                properties.insert(
                    "GenerationContext".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-querygenerationcontext.html>
    pub struct QueryGenerationContext_ {
        pub curated_queries: Option<Vec<CuratedQuery_>>,
        pub tables: Option<Vec<QueryGenerationTable_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_QueryGenerationContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.QueryGenerationContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_QueryGenerationContext as QueryGenerationContext;
    impl crate::value::ToValue for QueryGenerationContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.curated_queries {
                properties.insert(
                    "CuratedQueries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tables {
                properties.insert("Tables".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-querygenerationtable.html>
    pub struct QueryGenerationTable_ {
        pub columns: Option<Vec<QueryGenerationColumn_>>,
        pub description: Option<crate::value::ExpString>,
        pub inclusion: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_QueryGenerationTable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.QueryGenerationTable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_QueryGenerationTable as QueryGenerationTable;
    impl crate::value::ToValue for QueryGenerationTable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.columns {
                properties.insert(
                    "Columns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inclusion {
                properties.insert(
                    "Inclusion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-rdsconfiguration.html>
    pub struct RdsConfiguration_ {
        pub credentials_secret_arn: crate::value::ExpString,
        pub database_name: crate::value::ExpString,
        pub field_mapping: Box<RdsFieldMapping_>,
        pub resource_arn: crate::value::ExpString,
        pub table_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RdsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RdsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RdsConfiguration as RdsConfiguration;
    impl crate::value::ToValue for RdsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CredentialsSecretArn".to_string(),
                crate::value::ToValue::to_value(&self.credentials_secret_arn),
            );
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.insert(
                "FieldMapping".to_string(),
                crate::value::ToValue::to_value(&self.field_mapping),
            );
            properties.insert(
                "ResourceArn".to_string(),
                crate::value::ToValue::to_value(&self.resource_arn),
            );
            properties.insert(
                "TableName".to_string(),
                crate::value::ToValue::to_value(&self.table_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-rdsfieldmapping.html>
    pub struct RdsFieldMapping_ {
        pub custom_metadata_field: Option<crate::value::ExpString>,
        pub metadata_field: crate::value::ExpString,
        pub primary_key_field: crate::value::ExpString,
        pub text_field: crate::value::ExpString,
        pub vector_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RdsFieldMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RdsFieldMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RdsFieldMapping as RdsFieldMapping;
    impl crate::value::ToValue for RdsFieldMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_metadata_field {
                properties.insert(
                    "CustomMetadataField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MetadataField".to_string(),
                crate::value::ToValue::to_value(&self.metadata_field),
            );
            properties.insert(
                "PrimaryKeyField".to_string(),
                crate::value::ToValue::to_value(&self.primary_key_field),
            );
            properties.insert(
                "TextField".to_string(),
                crate::value::ToValue::to_value(&self.text_field),
            );
            properties.insert(
                "VectorField".to_string(),
                crate::value::ToValue::to_value(&self.vector_field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftconfiguration.html>
    pub struct RedshiftConfiguration_ {
        pub query_engine_configuration: Box<RedshiftQueryEngineConfiguration_>,
        pub query_generation_configuration: Option<Box<QueryGenerationConfiguration_>>,
        pub storage_configurations: Vec<RedshiftQueryEngineStorageConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftConfiguration as RedshiftConfiguration;
    impl crate::value::ToValue for RedshiftConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QueryEngineConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.query_engine_configuration),
            );
            if let Some(ref value) = self.query_generation_configuration {
                properties.insert(
                    "QueryGenerationConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StorageConfigurations".to_string(),
                crate::value::ToValue::to_value(&self.storage_configurations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftprovisionedauthconfiguration.html>
    pub struct RedshiftProvisionedAuthConfiguration_ {
        pub database_user: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
        pub username_password_secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftProvisionedAuthConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftProvisionedAuthConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftProvisionedAuthConfiguration as RedshiftProvisionedAuthConfiguration;
    impl crate::value::ToValue for RedshiftProvisionedAuthConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.database_user {
                properties.insert(
                    "DatabaseUser".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.username_password_secret_arn {
                properties.insert(
                    "UsernamePasswordSecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftprovisionedconfiguration.html>
    pub struct RedshiftProvisionedConfiguration_ {
        pub auth_configuration: Box<RedshiftProvisionedAuthConfiguration_>,
        pub cluster_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftProvisionedConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftProvisionedConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftProvisionedConfiguration as RedshiftProvisionedConfiguration;
    impl crate::value::ToValue for RedshiftProvisionedConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.auth_configuration),
            );
            properties.insert(
                "ClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.cluster_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftqueryengineawsdatacatalogstorageconfiguration.html>
    pub struct RedshiftQueryEngineAwsDataCatalogStorageConfiguration_ {
        pub table_names: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftQueryEngineAwsDataCatalogStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftQueryEngineAwsDataCatalogStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftQueryEngineAwsDataCatalogStorageConfiguration as RedshiftQueryEngineAwsDataCatalogStorageConfiguration;
    impl crate::value::ToValue for RedshiftQueryEngineAwsDataCatalogStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TableNames".to_string(),
                crate::value::ToValue::to_value(&self.table_names),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftqueryengineconfiguration.html>
    pub struct RedshiftQueryEngineConfiguration_ {
        pub provisioned_configuration: Option<Box<RedshiftProvisionedConfiguration_>>,
        pub serverless_configuration: Option<Box<RedshiftServerlessConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftQueryEngineConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftQueryEngineConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftQueryEngineConfiguration as RedshiftQueryEngineConfiguration;
    impl crate::value::ToValue for RedshiftQueryEngineConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.provisioned_configuration {
                properties.insert(
                    "ProvisionedConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.serverless_configuration {
                properties.insert(
                    "ServerlessConfiguration".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftqueryengineredshiftstorageconfiguration.html>
    pub struct RedshiftQueryEngineRedshiftStorageConfiguration_ {
        pub database_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftQueryEngineRedshiftStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftQueryEngineRedshiftStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftQueryEngineRedshiftStorageConfiguration as RedshiftQueryEngineRedshiftStorageConfiguration;
    impl crate::value::ToValue for RedshiftQueryEngineRedshiftStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DatabaseName".to_string(),
                crate::value::ToValue::to_value(&self.database_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftqueryenginestorageconfiguration.html>
    pub struct RedshiftQueryEngineStorageConfiguration_ {
        pub aws_data_catalog_configuration:
            Option<Box<RedshiftQueryEngineAwsDataCatalogStorageConfiguration_>>,
        pub redshift_configuration: Option<Box<RedshiftQueryEngineRedshiftStorageConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftQueryEngineStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftQueryEngineStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftQueryEngineStorageConfiguration as RedshiftQueryEngineStorageConfiguration;
    impl crate::value::ToValue for RedshiftQueryEngineStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_data_catalog_configuration {
                properties.insert(
                    "AwsDataCatalogConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.redshift_configuration {
                properties.insert(
                    "RedshiftConfiguration".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftserverlessauthconfiguration.html>
    pub struct RedshiftServerlessAuthConfiguration_ {
        pub r#type: crate::value::ExpString,
        pub username_password_secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftServerlessAuthConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftServerlessAuthConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftServerlessAuthConfiguration as RedshiftServerlessAuthConfiguration;
    impl crate::value::ToValue for RedshiftServerlessAuthConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.username_password_secret_arn {
                properties.insert(
                    "UsernamePasswordSecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-redshiftserverlessconfiguration.html>
    pub struct RedshiftServerlessConfiguration_ {
        pub auth_configuration: Box<RedshiftServerlessAuthConfiguration_>,
        pub workgroup_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_RedshiftServerlessConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.RedshiftServerlessConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_RedshiftServerlessConfiguration as RedshiftServerlessConfiguration;
    impl crate::value::ToValue for RedshiftServerlessConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AuthConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.auth_configuration),
            );
            properties.insert(
                "WorkgroupArn".to_string(),
                crate::value::ToValue::to_value(&self.workgroup_arn),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-s3location.html>
    pub struct S3Location_ {
        pub uri: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "URI".to_string(),
                crate::value::ToValue::to_value(&self.uri),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-s3vectorsconfiguration.html>
    pub struct S3VectorsConfiguration_ {
        pub index_arn: Option<crate::value::ExpString>,
        pub index_name: Option<crate::value::ExpString>,
        pub vector_bucket_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_S3VectorsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.S3VectorsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_S3VectorsConfiguration as S3VectorsConfiguration;
    impl crate::value::ToValue for S3VectorsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.index_arn {
                properties.insert(
                    "IndexArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.index_name {
                properties.insert(
                    "IndexName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vector_bucket_arn {
                properties.insert(
                    "VectorBucketArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-sqlknowledgebaseconfiguration.html>
    pub struct SqlKnowledgeBaseConfiguration_ {
        pub redshift_configuration: Option<Box<RedshiftConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_SqlKnowledgeBaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.SqlKnowledgeBaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_SqlKnowledgeBaseConfiguration as SqlKnowledgeBaseConfiguration;
    impl crate::value::ToValue for SqlKnowledgeBaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.redshift_configuration {
                properties.insert(
                    "RedshiftConfiguration".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-storageconfiguration.html>
    pub struct StorageConfiguration_ {
        pub mongo_db_atlas_configuration: Option<Box<MongoDbAtlasConfiguration_>>,
        pub neptune_analytics_configuration: Option<Box<NeptuneAnalyticsConfiguration_>>,
        pub opensearch_managed_cluster_configuration:
            Option<Box<OpenSearchManagedClusterConfiguration_>>,
        pub opensearch_serverless_configuration: Option<Box<OpenSearchServerlessConfiguration_>>,
        pub pinecone_configuration: Option<Box<PineconeConfiguration_>>,
        pub rds_configuration: Option<Box<RdsConfiguration_>>,
        pub s3_vectors_configuration: Option<Box<S3VectorsConfiguration_>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_StorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.StorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_StorageConfiguration as StorageConfiguration;
    impl crate::value::ToValue for StorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mongo_db_atlas_configuration {
                properties.insert(
                    "MongoDbAtlasConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.neptune_analytics_configuration {
                properties.insert(
                    "NeptuneAnalyticsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.opensearch_managed_cluster_configuration {
                properties.insert(
                    "OpensearchManagedClusterConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.opensearch_serverless_configuration {
                properties.insert(
                    "OpensearchServerlessConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.pinecone_configuration {
                properties.insert(
                    "PineconeConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rds_configuration {
                properties.insert(
                    "RdsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_vectors_configuration {
                properties.insert(
                    "S3VectorsConfiguration".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-supplementaldatastorageconfiguration.html>
    pub struct SupplementalDataStorageConfiguration_ {
        pub supplemental_data_storage_locations: Vec<SupplementalDataStorageLocation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_SupplementalDataStorageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.SupplementalDataStorageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_SupplementalDataStorageConfiguration as SupplementalDataStorageConfiguration;
    impl crate::value::ToValue for SupplementalDataStorageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SupplementalDataStorageLocations".to_string(),
                crate::value::ToValue::to_value(&self.supplemental_data_storage_locations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-supplementaldatastoragelocation.html>
    pub struct SupplementalDataStorageLocation_ {
        pub s3_location: Option<Box<S3Location_>>,
        pub supplemental_data_storage_location_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_SupplementalDataStorageLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.SupplementalDataStorageLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_SupplementalDataStorageLocation as SupplementalDataStorageLocation;
    impl crate::value::ToValue for SupplementalDataStorageLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_location {
                properties.insert(
                    "S3Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SupplementalDataStorageLocationType".to_string(),
                crate::value::ToValue::to_value(&self.supplemental_data_storage_location_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-vectorknowledgebaseconfiguration.html>
    pub struct VectorKnowledgeBaseConfiguration_ {
        pub embedding_model_arn: crate::value::ExpString,
        pub embedding_model_configuration: Option<Box<EmbeddingModelConfiguration_>>,
        pub supplemental_data_storage_configuration:
            Option<Box<SupplementalDataStorageConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_VectorKnowledgeBaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.VectorKnowledgeBaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_VectorKnowledgeBaseConfiguration as VectorKnowledgeBaseConfiguration;
    impl crate::value::ToValue for VectorKnowledgeBaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EmbeddingModelArn".to_string(),
                crate::value::ToValue::to_value(&self.embedding_model_arn),
            );
            if let Some(ref value) = self.embedding_model_configuration {
                properties.insert(
                    "EmbeddingModelConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.supplemental_data_storage_configuration {
                properties.insert(
                    "SupplementalDataStorageConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-videoconfiguration.html>
    pub struct VideoConfiguration_ {
        pub segmentation_configuration: Box<VideoSegmentationConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_VideoConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.VideoConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_VideoConfiguration as VideoConfiguration;
    impl crate::value::ToValue for VideoConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SegmentationConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.segmentation_configuration),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-knowledgebase-videosegmentationconfiguration.html>
    pub struct VideoSegmentationConfiguration_ {
        pub fixed_length_duration: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_KnowledgeBase_VideoSegmentationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::KnowledgeBase.VideoSegmentationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_KnowledgeBase_VideoSegmentationConfiguration as VideoSegmentationConfiguration;
    impl crate::value::ToValue for VideoSegmentationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FixedLengthDuration".to_string(),
                crate::value::ToValue::to_value(&self.fixed_length_duration),
            );
            properties.into()
        }
    }
}
pub mod prompt {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-cachepointblock.html>
    pub struct CachePointBlock_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_CachePointBlock {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.CachePointBlock"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_CachePointBlock as CachePointBlock;
    impl crate::value::ToValue for CachePointBlock_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-chatprompttemplateconfiguration.html>
    pub struct ChatPromptTemplateConfiguration_ {
        pub input_variables: Option<Vec<PromptInputVariable_>>,
        pub messages: Vec<Message_>,
        pub system: Option<Vec<SystemContentBlock_>>,
        pub tool_configuration: Option<Box<ToolConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_ChatPromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.ChatPromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_ChatPromptTemplateConfiguration as ChatPromptTemplateConfiguration;
    impl crate::value::ToValue for ChatPromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_variables {
                properties.insert(
                    "InputVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Messages".to_string(),
                crate::value::ToValue::to_value(&self.messages),
            );
            if let Some(ref value) = self.system {
                properties.insert("System".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tool_configuration {
                properties.insert(
                    "ToolConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-contentblock.html>
    pub struct ContentBlock_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_ContentBlock {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.ContentBlock"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_ContentBlock as ContentBlock;
    impl crate::value::ToValue for ContentBlock_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-message.html>
    pub struct Message_ {
        pub content: Vec<ContentBlock_>,
        pub role: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_Message {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.Message"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_Message as Message;
    impl crate::value::ToValue for Message_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Content".to_string(),
                crate::value::ToValue::to_value(&self.content),
            );
            properties.insert(
                "Role".to_string(),
                crate::value::ToValue::to_value(&self.role),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-promptagentresource.html>
    pub struct PromptAgentResource_ {
        pub agent_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptAgentResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptAgentResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptAgentResource as PromptAgentResource;
    impl crate::value::ToValue for PromptAgentResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.agent_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-promptgenairesource.html>
    pub struct PromptGenAiResource_ {
        pub agent: Box<PromptAgentResource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptGenAiResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptGenAiResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptGenAiResource as PromptGenAiResource;
    impl crate::value::ToValue for PromptGenAiResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Agent".to_string(),
                crate::value::ToValue::to_value(&self.agent),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-promptinferenceconfiguration.html>
    pub struct PromptInferenceConfiguration_ {
        pub text: Box<PromptModelInferenceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptInferenceConfiguration as PromptInferenceConfiguration;
    impl crate::value::ToValue for PromptInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-promptinputvariable.html>
    pub struct PromptInputVariable_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptInputVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptInputVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptInputVariable as PromptInputVariable;
    impl crate::value::ToValue for PromptInputVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-promptmetadataentry.html>
    pub struct PromptMetadataEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptMetadataEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptMetadataEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptMetadataEntry as PromptMetadataEntry;
    impl crate::value::ToValue for PromptMetadataEntry_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-promptmodelinferenceconfiguration.html>
    pub struct PromptModelInferenceConfiguration_ {
        pub max_tokens: Option<f64>,
        pub stop_sequences: Option<Vec<crate::value::ExpString>>,
        pub temperature: Option<f64>,
        pub top_p: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptModelInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptModelInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptModelInferenceConfiguration as PromptModelInferenceConfiguration;
    impl crate::value::ToValue for PromptModelInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_tokens {
                properties.insert(
                    "MaxTokens".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_sequences {
                properties.insert(
                    "StopSequences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temperature {
                properties.insert(
                    "Temperature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.top_p {
                properties.insert("TopP".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-prompttemplateconfiguration.html>
    pub struct PromptTemplateConfiguration_ {
        pub chat: Option<Box<ChatPromptTemplateConfiguration_>>,
        pub text: Option<Box<TextPromptTemplateConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptTemplateConfiguration as PromptTemplateConfiguration;
    impl crate::value::ToValue for PromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.chat {
                properties.insert("Chat".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-promptvariant.html>
    pub struct PromptVariant_ {
        pub additional_model_request_fields: Option<serde_json::Value>,
        pub gen_ai_resource: Option<Box<PromptGenAiResource_>>,
        pub inference_configuration: Option<Box<PromptInferenceConfiguration_>>,
        pub metadata: Option<Vec<PromptMetadataEntry_>>,
        pub model_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub template_configuration: Box<PromptTemplateConfiguration_>,
        pub template_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_PromptVariant {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.PromptVariant"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_PromptVariant as PromptVariant;
    impl crate::value::ToValue for PromptVariant_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_model_request_fields {
                properties.insert(
                    "AdditionalModelRequestFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gen_ai_resource {
                properties.insert(
                    "GenAiResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_configuration {
                properties.insert(
                    "InferenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_id {
                properties.insert(
                    "ModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TemplateConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.template_configuration),
            );
            properties.insert(
                "TemplateType".to_string(),
                crate::value::ToValue::to_value(&self.template_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-specifictoolchoice.html>
    pub struct SpecificToolChoice_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_SpecificToolChoice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.SpecificToolChoice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_SpecificToolChoice as SpecificToolChoice;
    impl crate::value::ToValue for SpecificToolChoice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-systemcontentblock.html>
    pub struct SystemContentBlock_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_SystemContentBlock {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.SystemContentBlock"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_SystemContentBlock as SystemContentBlock;
    impl crate::value::ToValue for SystemContentBlock_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-textprompttemplateconfiguration.html>
    pub struct TextPromptTemplateConfiguration_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub input_variables: Option<Vec<PromptInputVariable_>>,
        pub text: Option<crate::value::ExpString>,
        pub text_s3_location: Option<Box<TextS3Location_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_TextPromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.TextPromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_TextPromptTemplateConfiguration as TextPromptTemplateConfiguration;
    impl crate::value::ToValue for TextPromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_variables {
                properties.insert(
                    "InputVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text_s3_location {
                properties.insert(
                    "TextS3Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-texts3location.html>
    pub struct TextS3Location_ {
        pub bucket: crate::value::ExpString,
        pub key: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_TextS3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.TextS3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_TextS3Location as TextS3Location;
    impl crate::value::ToValue for TextS3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Bucket".to_string(),
                crate::value::ToValue::to_value(&self.bucket),
            );
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-tool.html>
    pub struct Tool_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub tool_spec: Option<Box<ToolSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_Tool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.Tool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_Tool as Tool;
    impl crate::value::ToValue for Tool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tool_spec {
                properties.insert(
                    "ToolSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-toolchoice.html>
    pub struct ToolChoice_ {
        pub any: Option<serde_json::Value>,
        pub auto: Option<serde_json::Value>,
        pub tool: Option<Box<SpecificToolChoice_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_ToolChoice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.ToolChoice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_ToolChoice as ToolChoice;
    impl crate::value::ToValue for ToolChoice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.any {
                properties.insert("Any".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.auto {
                properties.insert("Auto".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tool {
                properties.insert("Tool".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-toolconfiguration.html>
    pub struct ToolConfiguration_ {
        pub tool_choice: Option<Box<ToolChoice_>>,
        pub tools: Vec<Tool_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_ToolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.ToolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_ToolConfiguration as ToolConfiguration;
    impl crate::value::ToValue for ToolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tool_choice {
                properties.insert(
                    "ToolChoice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Tools".to_string(),
                crate::value::ToValue::to_value(&self.tools),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-toolinputschema.html>
    pub struct ToolInputSchema_ {
        pub json: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_ToolInputSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.ToolInputSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_ToolInputSchema as ToolInputSchema;
    impl crate::value::ToValue for ToolInputSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Json".to_string(),
                crate::value::ToValue::to_value(&self.json),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-prompt-toolspecification.html>
    pub struct ToolSpecification_ {
        pub description: Option<crate::value::ExpString>,
        pub input_schema: Box<ToolInputSchema_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_Prompt_ToolSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::Prompt.ToolSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_Prompt_ToolSpecification as ToolSpecification;
    impl crate::value::ToValue for ToolSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputSchema".to_string(),
                crate::value::ToValue::to_value(&self.input_schema),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod promptversion {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-cachepointblock.html>
    pub struct CachePointBlock_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_CachePointBlock {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.CachePointBlock"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_CachePointBlock as CachePointBlock;
    impl crate::value::ToValue for CachePointBlock_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-chatprompttemplateconfiguration.html>
    pub struct ChatPromptTemplateConfiguration_ {
        pub input_variables: Option<Vec<PromptInputVariable_>>,
        pub messages: Vec<Message_>,
        pub system: Option<Vec<SystemContentBlock_>>,
        pub tool_configuration: Option<Box<ToolConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_ChatPromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.ChatPromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_ChatPromptTemplateConfiguration as ChatPromptTemplateConfiguration;
    impl crate::value::ToValue for ChatPromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.input_variables {
                properties.insert(
                    "InputVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Messages".to_string(),
                crate::value::ToValue::to_value(&self.messages),
            );
            if let Some(ref value) = self.system {
                properties.insert("System".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tool_configuration {
                properties.insert(
                    "ToolConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-contentblock.html>
    pub struct ContentBlock_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_ContentBlock {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.ContentBlock"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_ContentBlock as ContentBlock;
    impl crate::value::ToValue for ContentBlock_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-message.html>
    pub struct Message_ {
        pub content: Vec<ContentBlock_>,
        pub role: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_Message {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.Message"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_Message as Message;
    impl crate::value::ToValue for Message_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Content".to_string(),
                crate::value::ToValue::to_value(&self.content),
            );
            properties.insert(
                "Role".to_string(),
                crate::value::ToValue::to_value(&self.role),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-promptagentresource.html>
    pub struct PromptAgentResource_ {
        pub agent_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptAgentResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptAgentResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptAgentResource as PromptAgentResource;
    impl crate::value::ToValue for PromptAgentResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AgentIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.agent_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-promptgenairesource.html>
    pub struct PromptGenAiResource_ {
        pub agent: Box<PromptAgentResource_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptGenAiResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptGenAiResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptGenAiResource as PromptGenAiResource;
    impl crate::value::ToValue for PromptGenAiResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Agent".to_string(),
                crate::value::ToValue::to_value(&self.agent),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-promptinferenceconfiguration.html>
    pub struct PromptInferenceConfiguration_ {
        pub text: Box<PromptModelInferenceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptInferenceConfiguration as PromptInferenceConfiguration;
    impl crate::value::ToValue for PromptInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-promptinputvariable.html>
    pub struct PromptInputVariable_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptInputVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptInputVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptInputVariable as PromptInputVariable;
    impl crate::value::ToValue for PromptInputVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-promptmetadataentry.html>
    pub struct PromptMetadataEntry_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptMetadataEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptMetadataEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptMetadataEntry as PromptMetadataEntry;
    impl crate::value::ToValue for PromptMetadataEntry_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-promptmodelinferenceconfiguration.html>
    pub struct PromptModelInferenceConfiguration_ {
        pub max_tokens: Option<f64>,
        pub stop_sequences: Option<Vec<crate::value::ExpString>>,
        pub temperature: Option<f64>,
        pub top_p: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptModelInferenceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptModelInferenceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptModelInferenceConfiguration as PromptModelInferenceConfiguration;
    impl crate::value::ToValue for PromptModelInferenceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_tokens {
                properties.insert(
                    "MaxTokens".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_sequences {
                properties.insert(
                    "StopSequences".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.temperature {
                properties.insert(
                    "Temperature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.top_p {
                properties.insert("TopP".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-prompttemplateconfiguration.html>
    pub struct PromptTemplateConfiguration_ {
        pub chat: Option<Box<ChatPromptTemplateConfiguration_>>,
        pub text: Option<Box<TextPromptTemplateConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptTemplateConfiguration as PromptTemplateConfiguration;
    impl crate::value::ToValue for PromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.chat {
                properties.insert("Chat".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-promptvariant.html>
    pub struct PromptVariant_ {
        pub additional_model_request_fields: Option<serde_json::Value>,
        pub gen_ai_resource: Option<Box<PromptGenAiResource_>>,
        pub inference_configuration: Option<Box<PromptInferenceConfiguration_>>,
        pub metadata: Option<Vec<PromptMetadataEntry_>>,
        pub model_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub template_configuration: Box<PromptTemplateConfiguration_>,
        pub template_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_PromptVariant {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.PromptVariant"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_PromptVariant as PromptVariant;
    impl crate::value::ToValue for PromptVariant_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.additional_model_request_fields {
                properties.insert(
                    "AdditionalModelRequestFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gen_ai_resource {
                properties.insert(
                    "GenAiResource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inference_configuration {
                properties.insert(
                    "InferenceConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata {
                properties.insert(
                    "Metadata".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model_id {
                properties.insert(
                    "ModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TemplateConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.template_configuration),
            );
            properties.insert(
                "TemplateType".to_string(),
                crate::value::ToValue::to_value(&self.template_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-specifictoolchoice.html>
    pub struct SpecificToolChoice_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_SpecificToolChoice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.SpecificToolChoice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_SpecificToolChoice as SpecificToolChoice;
    impl crate::value::ToValue for SpecificToolChoice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-systemcontentblock.html>
    pub struct SystemContentBlock_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub text: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_SystemContentBlock {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.SystemContentBlock"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_SystemContentBlock as SystemContentBlock;
    impl crate::value::ToValue for SystemContentBlock_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-textprompttemplateconfiguration.html>
    pub struct TextPromptTemplateConfiguration_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub input_variables: Option<Vec<PromptInputVariable_>>,
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_TextPromptTemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.TextPromptTemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_TextPromptTemplateConfiguration as TextPromptTemplateConfiguration;
    impl crate::value::ToValue for TextPromptTemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_variables {
                properties.insert(
                    "InputVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-tool.html>
    pub struct Tool_ {
        pub cache_point: Option<Box<CachePointBlock_>>,
        pub tool_spec: Option<Box<ToolSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_Tool {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.Tool"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_Tool as Tool;
    impl crate::value::ToValue for Tool_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cache_point {
                properties.insert(
                    "CachePoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tool_spec {
                properties.insert(
                    "ToolSpec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-toolchoice.html>
    pub struct ToolChoice_ {
        pub any: Option<serde_json::Value>,
        pub auto: Option<serde_json::Value>,
        pub tool: Option<Box<SpecificToolChoice_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_ToolChoice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.ToolChoice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_ToolChoice as ToolChoice;
    impl crate::value::ToValue for ToolChoice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.any {
                properties.insert("Any".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.auto {
                properties.insert("Auto".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tool {
                properties.insert("Tool".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-toolconfiguration.html>
    pub struct ToolConfiguration_ {
        pub tool_choice: Option<Box<ToolChoice_>>,
        pub tools: Vec<Tool_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_ToolConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.ToolConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_ToolConfiguration as ToolConfiguration;
    impl crate::value::ToValue for ToolConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.tool_choice {
                properties.insert(
                    "ToolChoice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Tools".to_string(),
                crate::value::ToValue::to_value(&self.tools),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-toolinputschema.html>
    pub struct ToolInputSchema_ {
        pub json: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_ToolInputSchema {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.ToolInputSchema"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_ToolInputSchema as ToolInputSchema;
    impl crate::value::ToValue for ToolInputSchema_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Json".to_string(),
                crate::value::ToValue::to_value(&self.json),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-bedrock-promptversion-toolspecification.html>
    pub struct ToolSpecification_ {
        pub description: Option<crate::value::ExpString>,
        pub input_schema: Box<ToolInputSchema_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_bedrock_PromptVersion_ToolSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Bedrock::PromptVersion.ToolSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_bedrock_PromptVersion_ToolSpecification as ToolSpecification;
    impl crate::value::ToValue for ToolSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InputSchema".to_string(),
                crate::value::ToValue::to_value(&self.input_schema),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-agent.html>
pub struct Agent_ {
    pub action_groups: Option<Vec<super::bedrock::agent::AgentActionGroup_>>,
    pub agent_collaboration: Option<crate::value::ExpString>,
    pub agent_collaborators: Option<Vec<super::bedrock::agent::AgentCollaborator_>>,
    pub agent_name: crate::value::ExpString,
    pub agent_resource_role_arn: Option<crate::value::ExpString>,
    pub auto_prepare: Option<crate::value::ExpBool>,
    pub custom_orchestration: Option<super::bedrock::agent::CustomOrchestration_>,
    pub customer_encryption_key_arn: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub foundation_model: Option<crate::value::ExpString>,
    pub guardrail_configuration: Option<super::bedrock::agent::GuardrailConfiguration_>,
    pub idle_session_ttl_in_seconds: Option<f64>,
    pub instruction: Option<crate::value::ExpString>,
    pub knowledge_bases: Option<Vec<super::bedrock::agent::AgentKnowledgeBase_>>,
    pub memory_configuration: Option<super::bedrock::agent::MemoryConfiguration_>,
    pub orchestration_type: Option<crate::value::ExpString>,
    pub prompt_override_configuration: Option<super::bedrock::agent::PromptOverrideConfiguration_>,
    pub skip_resource_in_use_check_on_delete: Option<crate::value::ExpBool>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub test_alias_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_Agent {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::Agent" $($field
        $value)*)
    };
}
pub use crate::__aws_bedrock_Agent as Agent;
impl crate::template::ToResource for Agent_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Agent"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.action_groups {
            properties.insert(
                "ActionGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.agent_collaboration {
            properties.insert(
                "AgentCollaboration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.agent_collaborators {
            properties.insert(
                "AgentCollaborators".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "AgentName".to_string(),
            crate::value::ToValue::to_value(&self.agent_name),
        );
        if let Some(ref value) = self.agent_resource_role_arn {
            properties.insert(
                "AgentResourceRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.auto_prepare {
            properties.insert(
                "AutoPrepare".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_orchestration {
            properties.insert(
                "CustomOrchestration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_encryption_key_arn {
            properties.insert(
                "CustomerEncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.foundation_model {
            properties.insert(
                "FoundationModel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.guardrail_configuration {
            properties.insert(
                "GuardrailConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.idle_session_ttl_in_seconds {
            properties.insert(
                "IdleSessionTTLInSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.instruction {
            properties.insert(
                "Instruction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.knowledge_bases {
            properties.insert(
                "KnowledgeBases".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.memory_configuration {
            properties.insert(
                "MemoryConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.orchestration_type {
            properties.insert(
                "OrchestrationType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.prompt_override_configuration {
            properties.insert(
                "PromptOverrideConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.skip_resource_in_use_check_on_delete {
            properties.insert(
                "SkipResourceInUseCheckOnDelete".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.test_alias_tags {
            properties.insert(
                "TestAliasTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-agentalias.html>
pub struct AgentAlias_ {
    pub agent_alias_name: crate::value::ExpString,
    pub agent_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub routing_configuration:
        Option<Vec<super::bedrock::agentalias::AgentAliasRoutingConfigurationListItem_>>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_AgentAlias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::AgentAlias"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_AgentAlias as AgentAlias;
impl crate::template::ToResource for AgentAlias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AgentAlias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AgentAliasName".to_string(),
            crate::value::ToValue::to_value(&self.agent_alias_name),
        );
        properties.insert(
            "AgentId".to_string(),
            crate::value::ToValue::to_value(&self.agent_id),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.routing_configuration {
            properties.insert(
                "RoutingConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-applicationinferenceprofile.html>
pub struct ApplicationInferenceProfile_ {
    pub description: Option<crate::value::ExpString>,
    pub inference_profile_name: crate::value::ExpString,
    pub model_source:
        Option<super::bedrock::applicationinferenceprofile::InferenceProfileModelSource_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_ApplicationInferenceProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::ApplicationInferenceProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_ApplicationInferenceProfile as ApplicationInferenceProfile;
impl crate::template::ToResource for ApplicationInferenceProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ApplicationInferenceProfile",
            ),
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
            "InferenceProfileName".to_string(),
            crate::value::ToValue::to_value(&self.inference_profile_name),
        );
        if let Some(ref value) = self.model_source {
            properties.insert(
                "ModelSource".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-automatedreasoningpolicy.html>
pub struct AutomatedReasoningPolicy_ {
    pub description: Option<crate::value::ExpString>,
    pub force_delete: Option<crate::value::ExpBool>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub policy_definition: Option<super::bedrock::automatedreasoningpolicy::PolicyDefinition_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_AutomatedReasoningPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::AutomatedReasoningPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_AutomatedReasoningPolicy as AutomatedReasoningPolicy;
impl crate::template::ToResource for AutomatedReasoningPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AutomatedReasoningPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force_delete {
            properties.insert(
                "ForceDelete".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.policy_definition {
            properties.insert(
                "PolicyDefinition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-automatedreasoningpolicyversion.html>
pub struct AutomatedReasoningPolicyVersion_ {
    pub last_updated_definition_hash: Option<crate::value::ExpString>,
    pub policy_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_AutomatedReasoningPolicyVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::AutomatedReasoningPolicyVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_AutomatedReasoningPolicyVersion as AutomatedReasoningPolicyVersion;
impl crate::template::ToResource for AutomatedReasoningPolicyVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "AutomatedReasoningPolicyVersion",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.last_updated_definition_hash {
            properties.insert(
                "LastUpdatedDefinitionHash".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyArn".to_string(),
            crate::value::ToValue::to_value(&self.policy_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-blueprint.html>
pub struct Blueprint_ {
    pub blueprint_name: crate::value::ExpString,
    pub kms_encryption_context: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub schema: serde_json::Value,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_Blueprint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::Blueprint"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_Blueprint as Blueprint;
impl crate::template::ToResource for Blueprint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Blueprint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BlueprintName".to_string(),
            crate::value::ToValue::to_value(&self.blueprint_name),
        );
        if let Some(ref value) = self.kms_encryption_context {
            properties.insert(
                "KmsEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
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
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-dataautomationproject.html>
pub struct DataAutomationProject_ {
    pub custom_output_configuration:
        Option<super::bedrock::dataautomationproject::CustomOutputConfiguration_>,
    pub kms_encryption_context: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub override_configuration:
        Option<super::bedrock::dataautomationproject::OverrideConfiguration_>,
    pub project_description: Option<crate::value::ExpString>,
    pub project_name: crate::value::ExpString,
    pub project_type: Option<crate::value::ExpString>,
    pub standard_output_configuration:
        Option<super::bedrock::dataautomationproject::StandardOutputConfiguration_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_DataAutomationProject {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::DataAutomationProject"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_DataAutomationProject as DataAutomationProject;
impl crate::template::ToResource for DataAutomationProject_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataAutomationProject"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.custom_output_configuration {
            properties.insert(
                "CustomOutputConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_encryption_context {
            properties.insert(
                "KmsEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.override_configuration {
            properties.insert(
                "OverrideConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.project_description {
            properties.insert(
                "ProjectDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ProjectName".to_string(),
            crate::value::ToValue::to_value(&self.project_name),
        );
        if let Some(ref value) = self.project_type {
            properties.insert(
                "ProjectType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.standard_output_configuration {
            properties.insert(
                "StandardOutputConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-datasource.html>
pub struct DataSource_ {
    pub data_deletion_policy: Option<crate::value::ExpString>,
    pub data_source_configuration: super::bedrock::datasource::DataSourceConfiguration_,
    pub description: Option<crate::value::ExpString>,
    pub knowledge_base_id: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub server_side_encryption_configuration:
        Option<super::bedrock::datasource::ServerSideEncryptionConfiguration_>,
    pub vector_ingestion_configuration:
        Option<super::bedrock::datasource::VectorIngestionConfiguration_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_DataSource {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::DataSource"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_DataSource as DataSource;
impl crate::template::ToResource for DataSource_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DataSource"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.data_deletion_policy {
            properties.insert(
                "DataDeletionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataSourceConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.data_source_configuration),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KnowledgeBaseId".to_string(),
            crate::value::ToValue::to_value(&self.knowledge_base_id),
        );
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
        if let Some(ref value) = self.vector_ingestion_configuration {
            properties.insert(
                "VectorIngestionConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-flow.html>
pub struct Flow_ {
    pub customer_encryption_key_arn: Option<crate::value::ExpString>,
    pub definition: Option<super::bedrock::flow::FlowDefinition_>,
    pub definition_s3_location: Option<super::bedrock::flow::S3Location_>,
    pub definition_string: Option<crate::value::ExpString>,
    pub definition_substitutions: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    pub description: Option<crate::value::ExpString>,
    pub execution_role_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub test_alias_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_Flow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::Flow" $($field
        $value)*)
    };
}
pub use crate::__aws_bedrock_Flow as Flow;
impl crate::template::ToResource for Flow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Flow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.customer_encryption_key_arn {
            properties.insert(
                "CustomerEncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition {
            properties.insert(
                "Definition".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_s3_location {
            properties.insert(
                "DefinitionS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_string {
            properties.insert(
                "DefinitionString".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.definition_substitutions {
            properties.insert(
                "DefinitionSubstitutions".to_string(),
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
            "ExecutionRoleArn".to_string(),
            crate::value::ToValue::to_value(&self.execution_role_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.test_alias_tags {
            properties.insert(
                "TestAliasTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-flowalias.html>
pub struct FlowAlias_ {
    pub concurrency_configuration:
        Option<super::bedrock::flowalias::FlowAliasConcurrencyConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub flow_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub routing_configuration:
        Vec<super::bedrock::flowalias::FlowAliasRoutingConfigurationListItem_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_FlowAlias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::FlowAlias"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_FlowAlias as FlowAlias;
impl crate::template::ToResource for FlowAlias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowAlias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.concurrency_configuration {
            properties.insert(
                "ConcurrencyConfiguration".to_string(),
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
            "FlowArn".to_string(),
            crate::value::ToValue::to_value(&self.flow_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoutingConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.routing_configuration),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-flowversion.html>
pub struct FlowVersion_ {
    pub description: Option<crate::value::ExpString>,
    pub flow_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_FlowVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::FlowVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_FlowVersion as FlowVersion;
impl crate::template::ToResource for FlowVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("FlowVersion"),
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
            "FlowArn".to_string(),
            crate::value::ToValue::to_value(&self.flow_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-guardrail.html>
pub struct Guardrail_ {
    pub automated_reasoning_policy_config:
        Option<super::bedrock::guardrail::AutomatedReasoningPolicyConfig_>,
    pub blocked_input_messaging: crate::value::ExpString,
    pub blocked_outputs_messaging: crate::value::ExpString,
    pub content_policy_config: Option<super::bedrock::guardrail::ContentPolicyConfig_>,
    pub contextual_grounding_policy_config:
        Option<super::bedrock::guardrail::ContextualGroundingPolicyConfig_>,
    pub cross_region_config: Option<super::bedrock::guardrail::GuardrailCrossRegionConfig_>,
    pub description: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub sensitive_information_policy_config:
        Option<super::bedrock::guardrail::SensitiveInformationPolicyConfig_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub topic_policy_config: Option<super::bedrock::guardrail::TopicPolicyConfig_>,
    pub word_policy_config: Option<super::bedrock::guardrail::WordPolicyConfig_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_Guardrail {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::Guardrail"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_Guardrail as Guardrail;
impl crate::template::ToResource for Guardrail_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Guardrail"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.automated_reasoning_policy_config {
            properties.insert(
                "AutomatedReasoningPolicyConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.cross_region_config {
            properties.insert(
                "CrossRegionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
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
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-guardrailversion.html>
pub struct GuardrailVersion_ {
    pub description: Option<crate::value::ExpString>,
    pub guardrail_identifier: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_GuardrailVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::GuardrailVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_GuardrailVersion as GuardrailVersion;
impl crate::template::ToResource for GuardrailVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GuardrailVersion"),
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
            "GuardrailIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.guardrail_identifier),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-intelligentpromptrouter.html>
pub struct IntelligentPromptRouter_ {
    pub description: Option<crate::value::ExpString>,
    pub fallback_model: super::bedrock::intelligentpromptrouter::PromptRouterTargetModel_,
    pub models: Vec<super::bedrock::intelligentpromptrouter::PromptRouterTargetModel_>,
    pub prompt_router_name: crate::value::ExpString,
    pub routing_criteria: super::bedrock::intelligentpromptrouter::RoutingCriteria_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_IntelligentPromptRouter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::IntelligentPromptRouter"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_IntelligentPromptRouter as IntelligentPromptRouter;
impl crate::template::ToResource for IntelligentPromptRouter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IntelligentPromptRouter"),
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
            "FallbackModel".to_string(),
            crate::value::ToValue::to_value(&self.fallback_model),
        );
        properties.insert(
            "Models".to_string(),
            crate::value::ToValue::to_value(&self.models),
        );
        properties.insert(
            "PromptRouterName".to_string(),
            crate::value::ToValue::to_value(&self.prompt_router_name),
        );
        properties.insert(
            "RoutingCriteria".to_string(),
            crate::value::ToValue::to_value(&self.routing_criteria),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-knowledgebase.html>
pub struct KnowledgeBase_ {
    pub description: Option<crate::value::ExpString>,
    pub knowledge_base_configuration: super::bedrock::knowledgebase::KnowledgeBaseConfiguration_,
    pub name: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
    pub storage_configuration: Option<super::bedrock::knowledgebase::StorageConfiguration_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_KnowledgeBase {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::KnowledgeBase"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_KnowledgeBase as KnowledgeBase;
impl crate::template::ToResource for KnowledgeBase_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
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
            "KnowledgeBaseConfiguration".to_string(),
            crate::value::ToValue::to_value(&self.knowledge_base_configuration),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.storage_configuration {
            properties.insert(
                "StorageConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-prompt.html>
pub struct Prompt_ {
    pub customer_encryption_key_arn: Option<crate::value::ExpString>,
    pub default_variant: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub variants: Option<Vec<super::bedrock::prompt::PromptVariant_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_Prompt {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::Prompt" $($field
        $value)*)
    };
}
pub use crate::__aws_bedrock_Prompt as Prompt;
impl crate::template::ToResource for Prompt_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Prompt"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.customer_encryption_key_arn {
            properties.insert(
                "CustomerEncryptionKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_variant {
            properties.insert(
                "DefaultVariant".to_string(),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.variants {
            properties.insert(
                "Variants".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-bedrock-promptversion.html>
pub struct PromptVersion_ {
    pub description: Option<crate::value::ExpString>,
    pub prompt_arn: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_bedrock_PromptVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Bedrock::PromptVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_bedrock_PromptVersion as PromptVersion;
impl crate::template::ToResource for PromptVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Bedrock"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PromptVersion"),
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
            "PromptArn".to_string(),
            crate::value::ToValue::to_value(&self.prompt_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
