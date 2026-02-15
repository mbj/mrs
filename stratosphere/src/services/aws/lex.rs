pub mod bot {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-advancedrecognitionsetting.html
    pub struct AdvancedRecognitionSetting_ {
        pub audio_recognition_strategy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_AdvancedRecognitionSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.AdvancedRecognitionSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_AdvancedRecognitionSetting as AdvancedRecognitionSetting;
    impl crate::value::ToValue for AdvancedRecognitionSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_recognition_strategy {
                properties.insert(
                    "AudioRecognitionStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-allowedinputtypes.html
    pub struct AllowedInputTypes_ {
        pub allow_audio_input: crate::value::ExpBool,
        pub allow_dtmf_input: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_AllowedInputTypes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.AllowedInputTypes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_AllowedInputTypes as AllowedInputTypes;
    impl crate::value::ToValue for AllowedInputTypes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllowAudioInput".to_string(),
                crate::value::ToValue::to_value(&self.allow_audio_input),
            );
            properties.insert(
                "AllowDTMFInput".to_string(),
                crate::value::ToValue::to_value(&self.allow_dtmf_input),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audioanddtmfinputspecification.html
    pub struct AudioAndDTMFInputSpecification_ {
        pub audio_specification: Option<Box<AudioSpecification_>>,
        pub dtmf_specification: Option<Box<DTMFSpecification_>>,
        pub start_timeout_ms: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_AudioAndDTMFInputSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.AudioAndDTMFInputSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_AudioAndDTMFInputSpecification as AudioAndDTMFInputSpecification;
    impl crate::value::ToValue for AudioAndDTMFInputSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_specification {
                properties.insert(
                    "AudioSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dtmf_specification {
                properties.insert(
                    "DTMFSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StartTimeoutMs".to_string(),
                crate::value::ToValue::to_value(&self.start_timeout_ms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiologdestination.html
    pub struct AudioLogDestination_ {
        pub s3_bucket: Box<S3BucketLogDestination_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_AudioLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.AudioLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_AudioLogDestination as AudioLogDestination;
    impl crate::value::ToValue for AudioLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiologsetting.html
    pub struct AudioLogSetting_ {
        pub destination: Box<AudioLogDestination_>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_AudioLogSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.AudioLogSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_AudioLogSetting as AudioLogSetting;
    impl crate::value::ToValue for AudioLogSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-audiospecification.html
    pub struct AudioSpecification_ {
        pub end_timeout_ms: i32,
        pub max_length_ms: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_AudioSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.AudioSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_AudioSpecification as AudioSpecification;
    impl crate::value::ToValue for AudioSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndTimeoutMs".to_string(),
                crate::value::ToValue::to_value(&self.end_timeout_ms),
            );
            properties.insert(
                "MaxLengthMs".to_string(),
                crate::value::ToValue::to_value(&self.max_length_ms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-bkbexactresponsefields.html
    pub struct BKBExactResponseFields_ {
        pub answer_field: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BKBExactResponseFields {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BKBExactResponseFields"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BKBExactResponseFields as BKBExactResponseFields;
    impl crate::value::ToValue for BKBExactResponseFields_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_field {
                properties.insert(
                    "AnswerField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-bedrockagentconfiguration.html
    pub struct BedrockAgentConfiguration_ {
        pub bedrock_agent_alias_id: Option<crate::value::ExpString>,
        pub bedrock_agent_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BedrockAgentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BedrockAgentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BedrockAgentConfiguration as BedrockAgentConfiguration;
    impl crate::value::ToValue for BedrockAgentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_agent_alias_id {
                properties.insert(
                    "BedrockAgentAliasId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bedrock_agent_id {
                properties.insert(
                    "BedrockAgentId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-bedrockagentintentconfiguration.html
    pub struct BedrockAgentIntentConfiguration_ {
        pub bedrock_agent_configuration: Option<Box<BedrockAgentConfiguration_>>,
        pub bedrock_agent_intent_knowledge_base_configuration:
            Option<Box<BedrockAgentIntentKnowledgeBaseConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BedrockAgentIntentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BedrockAgentIntentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BedrockAgentIntentConfiguration as BedrockAgentIntentConfiguration;
    impl crate::value::ToValue for BedrockAgentIntentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_agent_configuration {
                properties.insert(
                    "BedrockAgentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bedrock_agent_intent_knowledge_base_configuration {
                properties.insert(
                    "BedrockAgentIntentKnowledgeBaseConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-bedrockagentintentknowledgebaseconfiguration.html
    pub struct BedrockAgentIntentKnowledgeBaseConfiguration_ {
        pub bedrock_knowledge_base_arn: crate::value::ExpString,
        pub bedrock_model_configuration: Box<BedrockModelSpecification_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BedrockAgentIntentKnowledgeBaseConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BedrockAgentIntentKnowledgeBaseConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BedrockAgentIntentKnowledgeBaseConfiguration as BedrockAgentIntentKnowledgeBaseConfiguration;
    impl crate::value::ToValue for BedrockAgentIntentKnowledgeBaseConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BedrockKnowledgeBaseArn".to_string(),
                crate::value::ToValue::to_value(&self.bedrock_knowledge_base_arn),
            );
            properties.insert(
                "BedrockModelConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.bedrock_model_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-bedrockguardrailconfiguration.html
    pub struct BedrockGuardrailConfiguration_ {
        pub bedrock_guardrail_identifier: Option<crate::value::ExpString>,
        pub bedrock_guardrail_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BedrockGuardrailConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BedrockGuardrailConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BedrockGuardrailConfiguration as BedrockGuardrailConfiguration;
    impl crate::value::ToValue for BedrockGuardrailConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_guardrail_identifier {
                properties.insert(
                    "BedrockGuardrailIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bedrock_guardrail_version {
                properties.insert(
                    "BedrockGuardrailVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-bedrockknowledgestoreconfiguration.html
    pub struct BedrockKnowledgeStoreConfiguration_ {
        pub bkb_exact_response_fields: Option<Box<BKBExactResponseFields_>>,
        pub bedrock_knowledge_base_arn: Option<crate::value::ExpString>,
        pub exact_response: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BedrockKnowledgeStoreConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BedrockKnowledgeStoreConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BedrockKnowledgeStoreConfiguration as BedrockKnowledgeStoreConfiguration;
    impl crate::value::ToValue for BedrockKnowledgeStoreConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bkb_exact_response_fields {
                properties.insert(
                    "BKBExactResponseFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bedrock_knowledge_base_arn {
                properties.insert(
                    "BedrockKnowledgeBaseArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exact_response {
                properties.insert(
                    "ExactResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-bedrockmodelspecification.html
    pub struct BedrockModelSpecification_ {
        pub bedrock_guardrail_configuration: Option<Box<BedrockGuardrailConfiguration_>>,
        pub bedrock_model_custom_prompt: Option<crate::value::ExpString>,
        pub bedrock_trace_status: Option<crate::value::ExpString>,
        pub model_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BedrockModelSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BedrockModelSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BedrockModelSpecification as BedrockModelSpecification;
    impl crate::value::ToValue for BedrockModelSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_guardrail_configuration {
                properties.insert(
                    "BedrockGuardrailConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bedrock_model_custom_prompt {
                properties.insert(
                    "BedrockModelCustomPrompt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bedrock_trace_status {
                properties.insert(
                    "BedrockTraceStatus".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettings.html
    pub struct BotAliasLocaleSettings_ {
        pub code_hook_specification: Option<Box<CodeHookSpecification_>>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BotAliasLocaleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BotAliasLocaleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BotAliasLocaleSettings as BotAliasLocaleSettings;
    impl crate::value::ToValue for BotAliasLocaleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_hook_specification {
                properties.insert(
                    "CodeHookSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botaliaslocalesettingsitem.html
    pub struct BotAliasLocaleSettingsItem_ {
        pub bot_alias_locale_setting: Box<BotAliasLocaleSettings_>,
        pub locale_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BotAliasLocaleSettingsItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BotAliasLocaleSettingsItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BotAliasLocaleSettingsItem as BotAliasLocaleSettingsItem;
    impl crate::value::ToValue for BotAliasLocaleSettingsItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BotAliasLocaleSetting".to_string(),
                crate::value::ToValue::to_value(&self.bot_alias_locale_setting),
            );
            properties.insert(
                "LocaleId".to_string(),
                crate::value::ToValue::to_value(&self.locale_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-botlocale.html
    pub struct BotLocale_ {
        pub custom_vocabulary: Option<Box<CustomVocabulary_>>,
        pub description: Option<crate::value::ExpString>,
        pub generative_ai_settings: Option<Box<GenerativeAISettings_>>,
        pub intents: Option<Vec<Intent_>>,
        pub locale_id: crate::value::ExpString,
        pub nlu_confidence_threshold: f64,
        pub slot_types: Option<Vec<SlotType_>>,
        pub speech_detection_sensitivity: Option<crate::value::ExpString>,
        pub speech_recognition_settings: Option<Box<SpeechRecognitionSettings_>>,
        pub unified_speech_settings: Option<Box<UnifiedSpeechSettings_>>,
        pub voice_settings: Option<Box<VoiceSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BotLocale {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BotLocale"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BotLocale as BotLocale;
    impl crate::value::ToValue for BotLocale_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_vocabulary {
                properties.insert(
                    "CustomVocabulary".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.generative_ai_settings {
                properties.insert(
                    "GenerativeAISettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intents {
                properties.insert(
                    "Intents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LocaleId".to_string(),
                crate::value::ToValue::to_value(&self.locale_id),
            );
            properties.insert(
                "NluConfidenceThreshold".to_string(),
                crate::value::ToValue::to_value(&self.nlu_confidence_threshold),
            );
            if let Some(ref value) = self.slot_types {
                properties.insert(
                    "SlotTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.speech_detection_sensitivity {
                properties.insert(
                    "SpeechDetectionSensitivity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.speech_recognition_settings {
                properties.insert(
                    "SpeechRecognitionSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unified_speech_settings {
                properties.insert(
                    "UnifiedSpeechSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.voice_settings {
                properties.insert(
                    "VoiceSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-buildtimesettings.html
    pub struct BuildtimeSettings_ {
        pub descriptive_bot_builder_specification: Option<Box<DescriptiveBotBuilderSpecification_>>,
        pub sample_utterance_generation_specification:
            Option<Box<SampleUtteranceGenerationSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_BuildtimeSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.BuildtimeSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_BuildtimeSettings as BuildtimeSettings;
    impl crate::value::ToValue for BuildtimeSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.descriptive_bot_builder_specification {
                properties.insert(
                    "DescriptiveBotBuilderSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_utterance_generation_specification {
                properties.insert(
                    "SampleUtteranceGenerationSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-button.html
    pub struct Button_ {
        pub text: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_Button {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.Button"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_Button as Button;
    impl crate::value::ToValue for Button_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-cloudwatchloggrouplogdestination.html
    pub struct CloudWatchLogGroupLogDestination_ {
        pub cloud_watch_log_group_arn: crate::value::ExpString,
        pub log_prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_CloudWatchLogGroupLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.CloudWatchLogGroupLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_CloudWatchLogGroupLogDestination as CloudWatchLogGroupLogDestination;
    impl crate::value::ToValue for CloudWatchLogGroupLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchLogGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_log_group_arn),
            );
            properties.insert(
                "LogPrefix".to_string(),
                crate::value::ToValue::to_value(&self.log_prefix),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-codehookspecification.html
    pub struct CodeHookSpecification_ {
        pub lambda_code_hook: Box<LambdaCodeHook_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_CodeHookSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.CodeHookSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_CodeHookSpecification as CodeHookSpecification;
    impl crate::value::ToValue for CodeHookSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaCodeHook".to_string(),
                crate::value::ToValue::to_value(&self.lambda_code_hook),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-compositeslottypesetting.html
    pub struct CompositeSlotTypeSetting_ {
        pub sub_slots: Option<Vec<SubSlotTypeComposition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_CompositeSlotTypeSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.CompositeSlotTypeSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_CompositeSlotTypeSetting as CompositeSlotTypeSetting;
    impl crate::value::ToValue for CompositeSlotTypeSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sub_slots {
                properties.insert(
                    "SubSlots".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-condition.html
    pub struct Condition_ {
        pub expression_string: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExpressionString".to_string(),
                crate::value::ToValue::to_value(&self.expression_string),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalbranch.html
    pub struct ConditionalBranch_ {
        pub condition: Box<Condition_>,
        pub name: crate::value::ExpString,
        pub next_step: Box<DialogState_>,
        pub response: Option<Box<ResponseSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ConditionalBranch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ConditionalBranch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ConditionalBranch as ConditionalBranch;
    impl crate::value::ToValue for ConditionalBranch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "NextStep".to_string(),
                crate::value::ToValue::to_value(&self.next_step),
            );
            if let Some(ref value) = self.response {
                properties.insert(
                    "Response".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conditionalspecification.html
    pub struct ConditionalSpecification_ {
        pub conditional_branches: Vec<ConditionalBranch_>,
        pub default_branch: Box<DefaultConditionalBranch_>,
        pub is_active: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ConditionalSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ConditionalSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ConditionalSpecification as ConditionalSpecification;
    impl crate::value::ToValue for ConditionalSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConditionalBranches".to_string(),
                crate::value::ToValue::to_value(&self.conditional_branches),
            );
            properties.insert(
                "DefaultBranch".to_string(),
                crate::value::ToValue::to_value(&self.default_branch),
            );
            properties.insert(
                "IsActive".to_string(),
                crate::value::ToValue::to_value(&self.is_active),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-conversationlogsettings.html
    pub struct ConversationLogSettings_ {
        pub audio_log_settings: Option<Vec<AudioLogSetting_>>,
        pub text_log_settings: Option<Vec<TextLogSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ConversationLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ConversationLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ConversationLogSettings as ConversationLogSettings;
    impl crate::value::ToValue for ConversationLogSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_log_settings {
                properties.insert(
                    "AudioLogSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text_log_settings {
                properties.insert(
                    "TextLogSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-custompayload.html
    pub struct CustomPayload_ {
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_CustomPayload {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.CustomPayload"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_CustomPayload as CustomPayload;
    impl crate::value::ToValue for CustomPayload_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabulary.html
    pub struct CustomVocabulary_ {
        pub custom_vocabulary_items: Vec<CustomVocabularyItem_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_CustomVocabulary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.CustomVocabulary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_CustomVocabulary as CustomVocabulary;
    impl crate::value::ToValue for CustomVocabulary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CustomVocabularyItems".to_string(),
                crate::value::ToValue::to_value(&self.custom_vocabulary_items),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-customvocabularyitem.html
    pub struct CustomVocabularyItem_ {
        pub display_as: Option<crate::value::ExpString>,
        pub phrase: crate::value::ExpString,
        pub weight: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_CustomVocabularyItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.CustomVocabularyItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_CustomVocabularyItem as CustomVocabularyItem;
    impl crate::value::ToValue for CustomVocabularyItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.display_as {
                properties.insert(
                    "DisplayAs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Phrase".to_string(),
                crate::value::ToValue::to_value(&self.phrase),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dtmfspecification.html
    pub struct DTMFSpecification_ {
        pub deletion_character: crate::value::ExpString,
        pub end_character: crate::value::ExpString,
        pub end_timeout_ms: i32,
        pub max_length: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DTMFSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DTMFSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DTMFSpecification as DTMFSpecification;
    impl crate::value::ToValue for DTMFSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeletionCharacter".to_string(),
                crate::value::ToValue::to_value(&self.deletion_character),
            );
            properties.insert(
                "EndCharacter".to_string(),
                crate::value::ToValue::to_value(&self.end_character),
            );
            properties.insert(
                "EndTimeoutMs".to_string(),
                crate::value::ToValue::to_value(&self.end_timeout_ms),
            );
            properties.insert(
                "MaxLength".to_string(),
                crate::value::ToValue::to_value(&self.max_length),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dataprivacy.html
    pub struct DataPrivacy_ {
        pub child_directed: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DataPrivacy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DataPrivacy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DataPrivacy as DataPrivacy;
    impl crate::value::ToValue for DataPrivacy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ChildDirected".to_string(),
                crate::value::ToValue::to_value(&self.child_directed),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-datasourceconfiguration.html
    pub struct DataSourceConfiguration_ {
        pub bedrock_knowledge_store_configuration: Option<Box<BedrockKnowledgeStoreConfiguration_>>,
        pub kendra_configuration: Option<Box<QnAKendraConfiguration_>>,
        pub opensearch_configuration: Option<Box<OpensearchConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DataSourceConfiguration as DataSourceConfiguration;
    impl crate::value::ToValue for DataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_knowledge_store_configuration {
                properties.insert(
                    "BedrockKnowledgeStoreConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kendra_configuration {
                properties.insert(
                    "KendraConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.opensearch_configuration {
                properties.insert(
                    "OpensearchConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-deepgramspeechmodelconfig.html
    pub struct DeepgramSpeechModelConfig_ {
        pub api_token_secret_arn: crate::value::ExpString,
        pub model_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DeepgramSpeechModelConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DeepgramSpeechModelConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DeepgramSpeechModelConfig as DeepgramSpeechModelConfig;
    impl crate::value::ToValue for DeepgramSpeechModelConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApiTokenSecretArn".to_string(),
                crate::value::ToValue::to_value(&self.api_token_secret_arn),
            );
            if let Some(ref value) = self.model_id {
                properties.insert(
                    "ModelId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-defaultconditionalbranch.html
    pub struct DefaultConditionalBranch_ {
        pub next_step: Option<Box<DialogState_>>,
        pub response: Option<Box<ResponseSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DefaultConditionalBranch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DefaultConditionalBranch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DefaultConditionalBranch as DefaultConditionalBranch;
    impl crate::value::ToValue for DefaultConditionalBranch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.next_step {
                properties.insert(
                    "NextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response {
                properties.insert(
                    "Response".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-descriptivebotbuilderspecification.html
    pub struct DescriptiveBotBuilderSpecification_ {
        pub bedrock_model_specification: Option<Box<BedrockModelSpecification_>>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DescriptiveBotBuilderSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DescriptiveBotBuilderSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DescriptiveBotBuilderSpecification as DescriptiveBotBuilderSpecification;
    impl crate::value::ToValue for DescriptiveBotBuilderSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_model_specification {
                properties.insert(
                    "BedrockModelSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogaction.html
    pub struct DialogAction_ {
        pub slot_to_elicit: Option<crate::value::ExpString>,
        pub suppress_next_message: Option<crate::value::ExpBool>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DialogAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DialogAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DialogAction as DialogAction;
    impl crate::value::ToValue for DialogAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.slot_to_elicit {
                properties.insert(
                    "SlotToElicit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.suppress_next_message {
                properties.insert(
                    "SuppressNextMessage".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehookinvocationsetting.html
    pub struct DialogCodeHookInvocationSetting_ {
        pub enable_code_hook_invocation: crate::value::ExpBool,
        pub invocation_label: Option<crate::value::ExpString>,
        pub is_active: crate::value::ExpBool,
        pub post_code_hook_specification: Box<PostDialogCodeHookInvocationSpecification_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DialogCodeHookInvocationSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DialogCodeHookInvocationSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DialogCodeHookInvocationSetting as DialogCodeHookInvocationSetting;
    impl crate::value::ToValue for DialogCodeHookInvocationSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnableCodeHookInvocation".to_string(),
                crate::value::ToValue::to_value(&self.enable_code_hook_invocation),
            );
            if let Some(ref value) = self.invocation_label {
                properties.insert(
                    "InvocationLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "IsActive".to_string(),
                crate::value::ToValue::to_value(&self.is_active),
            );
            properties.insert(
                "PostCodeHookSpecification".to_string(),
                crate::value::ToValue::to_value(&self.post_code_hook_specification),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogcodehooksetting.html
    pub struct DialogCodeHookSetting_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DialogCodeHookSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DialogCodeHookSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DialogCodeHookSetting as DialogCodeHookSetting;
    impl crate::value::ToValue for DialogCodeHookSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-dialogstate.html
    pub struct DialogState_ {
        pub dialog_action: Option<Box<DialogAction_>>,
        pub intent: Option<Box<IntentOverride_>>,
        pub session_attributes: Option<Vec<SessionAttribute_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_DialogState {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.DialogState"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_DialogState as DialogState;
    impl crate::value::ToValue for DialogState_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dialog_action {
                properties.insert(
                    "DialogAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intent {
                properties.insert("Intent".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.session_attributes {
                properties.insert(
                    "SessionAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-elicitationcodehookinvocationsetting.html
    pub struct ElicitationCodeHookInvocationSetting_ {
        pub enable_code_hook_invocation: crate::value::ExpBool,
        pub invocation_label: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ElicitationCodeHookInvocationSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ElicitationCodeHookInvocationSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ElicitationCodeHookInvocationSetting as ElicitationCodeHookInvocationSetting;
    impl crate::value::ToValue for ElicitationCodeHookInvocationSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EnableCodeHookInvocation".to_string(),
                crate::value::ToValue::to_value(&self.enable_code_hook_invocation),
            );
            if let Some(ref value) = self.invocation_label {
                properties.insert(
                    "InvocationLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-errorlogsettings.html
    pub struct ErrorLogSettings_ {
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ErrorLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ErrorLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ErrorLogSettings as ErrorLogSettings;
    impl crate::value::ToValue for ErrorLogSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-exactresponsefields.html
    pub struct ExactResponseFields_ {
        pub answer_field: Option<crate::value::ExpString>,
        pub question_field: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ExactResponseFields {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ExactResponseFields"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ExactResponseFields as ExactResponseFields;
    impl crate::value::ToValue for ExactResponseFields_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_field {
                properties.insert(
                    "AnswerField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.question_field {
                properties.insert(
                    "QuestionField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-externalsourcesetting.html
    pub struct ExternalSourceSetting_ {
        pub grammar_slot_type_setting: Option<Box<GrammarSlotTypeSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ExternalSourceSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ExternalSourceSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ExternalSourceSetting as ExternalSourceSetting;
    impl crate::value::ToValue for ExternalSourceSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.grammar_slot_type_setting {
                properties.insert(
                    "GrammarSlotTypeSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentcodehooksetting.html
    pub struct FulfillmentCodeHookSetting_ {
        pub enabled: crate::value::ExpBool,
        pub fulfillment_updates_specification: Option<Box<FulfillmentUpdatesSpecification_>>,
        pub is_active: Option<crate::value::ExpBool>,
        pub post_fulfillment_status_specification: Option<Box<PostFulfillmentStatusSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_FulfillmentCodeHookSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.FulfillmentCodeHookSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_FulfillmentCodeHookSetting as FulfillmentCodeHookSetting;
    impl crate::value::ToValue for FulfillmentCodeHookSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.fulfillment_updates_specification {
                properties.insert(
                    "FulfillmentUpdatesSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_active {
                properties.insert(
                    "IsActive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.post_fulfillment_status_specification {
                properties.insert(
                    "PostFulfillmentStatusSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentstartresponsespecification.html
    pub struct FulfillmentStartResponseSpecification_ {
        pub allow_interrupt: Option<crate::value::ExpBool>,
        pub delay_in_seconds: i32,
        pub message_groups: Vec<MessageGroup_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_FulfillmentStartResponseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.FulfillmentStartResponseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_FulfillmentStartResponseSpecification as FulfillmentStartResponseSpecification;
    impl crate::value::ToValue for FulfillmentStartResponseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_interrupt {
                properties.insert(
                    "AllowInterrupt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DelayInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.delay_in_seconds),
            );
            properties.insert(
                "MessageGroups".to_string(),
                crate::value::ToValue::to_value(&self.message_groups),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdateresponsespecification.html
    pub struct FulfillmentUpdateResponseSpecification_ {
        pub allow_interrupt: Option<crate::value::ExpBool>,
        pub frequency_in_seconds: i32,
        pub message_groups: Vec<MessageGroup_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_FulfillmentUpdateResponseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.FulfillmentUpdateResponseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_FulfillmentUpdateResponseSpecification as FulfillmentUpdateResponseSpecification;
    impl crate::value::ToValue for FulfillmentUpdateResponseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_interrupt {
                properties.insert(
                    "AllowInterrupt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FrequencyInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.frequency_in_seconds),
            );
            properties.insert(
                "MessageGroups".to_string(),
                crate::value::ToValue::to_value(&self.message_groups),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-fulfillmentupdatesspecification.html
    pub struct FulfillmentUpdatesSpecification_ {
        pub active: crate::value::ExpBool,
        pub start_response: Option<Box<FulfillmentStartResponseSpecification_>>,
        pub timeout_in_seconds: Option<i32>,
        pub update_response: Option<Box<FulfillmentUpdateResponseSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_FulfillmentUpdatesSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.FulfillmentUpdatesSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_FulfillmentUpdatesSpecification as FulfillmentUpdatesSpecification;
    impl crate::value::ToValue for FulfillmentUpdatesSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Active".to_string(),
                crate::value::ToValue::to_value(&self.active),
            );
            if let Some(ref value) = self.start_response {
                properties.insert(
                    "StartResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_in_seconds {
                properties.insert(
                    "TimeoutInSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_response {
                properties.insert(
                    "UpdateResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-generativeaisettings.html
    pub struct GenerativeAISettings_ {
        pub buildtime_settings: Option<Box<BuildtimeSettings_>>,
        pub runtime_settings: Option<Box<RuntimeSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_GenerativeAISettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.GenerativeAISettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_GenerativeAISettings as GenerativeAISettings;
    impl crate::value::ToValue for GenerativeAISettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buildtime_settings {
                properties.insert(
                    "BuildtimeSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.runtime_settings {
                properties.insert(
                    "RuntimeSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesetting.html
    pub struct GrammarSlotTypeSetting_ {
        pub source: Option<Box<GrammarSlotTypeSource_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_GrammarSlotTypeSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.GrammarSlotTypeSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_GrammarSlotTypeSetting as GrammarSlotTypeSetting;
    impl crate::value::ToValue for GrammarSlotTypeSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.source {
                properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-grammarslottypesource.html
    pub struct GrammarSlotTypeSource_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub s3_bucket_name: crate::value::ExpString,
        pub s3_object_key: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_GrammarSlotTypeSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.GrammarSlotTypeSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_GrammarSlotTypeSource as GrammarSlotTypeSource;
    impl crate::value::ToValue for GrammarSlotTypeSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "S3BucketName".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket_name),
            );
            properties.insert(
                "S3ObjectKey".to_string(),
                crate::value::ToValue::to_value(&self.s3_object_key),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-imageresponsecard.html
    pub struct ImageResponseCard_ {
        pub buttons: Option<Vec<Button_>>,
        pub image_url: Option<crate::value::ExpString>,
        pub subtitle: Option<crate::value::ExpString>,
        pub title: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ImageResponseCard {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ImageResponseCard"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ImageResponseCard as ImageResponseCard;
    impl crate::value::ToValue for ImageResponseCard_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.buttons {
                properties.insert(
                    "Buttons".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_url {
                properties.insert(
                    "ImageUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subtitle {
                properties.insert(
                    "Subtitle".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Title".to_string(),
                crate::value::ToValue::to_value(&self.title),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-initialresponsesetting.html
    pub struct InitialResponseSetting_ {
        pub code_hook: Option<Box<DialogCodeHookInvocationSetting_>>,
        pub conditional: Option<Box<ConditionalSpecification_>>,
        pub initial_response: Option<Box<ResponseSpecification_>>,
        pub next_step: Option<Box<DialogState_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_InitialResponseSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.InitialResponseSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_InitialResponseSetting as InitialResponseSetting;
    impl crate::value::ToValue for InitialResponseSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_hook {
                properties.insert(
                    "CodeHook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.conditional {
                properties.insert(
                    "Conditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.initial_response {
                properties.insert(
                    "InitialResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.next_step {
                properties.insert(
                    "NextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-inputcontext.html
    pub struct InputContext_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_InputContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.InputContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_InputContext as InputContext;
    impl crate::value::ToValue for InputContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intent.html
    pub struct Intent_ {
        pub bedrock_agent_intent_configuration: Option<Box<BedrockAgentIntentConfiguration_>>,
        pub description: Option<crate::value::ExpString>,
        pub dialog_code_hook: Option<Box<DialogCodeHookSetting_>>,
        pub display_name: Option<crate::value::ExpString>,
        pub fulfillment_code_hook: Option<Box<FulfillmentCodeHookSetting_>>,
        pub initial_response_setting: Option<Box<InitialResponseSetting_>>,
        pub input_contexts: Option<Vec<InputContext_>>,
        pub intent_closing_setting: Option<Box<IntentClosingSetting_>>,
        pub intent_confirmation_setting: Option<Box<IntentConfirmationSetting_>>,
        pub kendra_configuration: Option<Box<KendraConfiguration_>>,
        pub name: crate::value::ExpString,
        pub output_contexts: Option<Vec<OutputContext_>>,
        pub parent_intent_signature: Option<crate::value::ExpString>,
        pub q_in_connect_intent_configuration: Option<Box<QInConnectIntentConfiguration_>>,
        pub qn_a_intent_configuration: Option<Box<QnAIntentConfiguration_>>,
        pub sample_utterances: Option<Vec<SampleUtterance_>>,
        pub slot_priorities: Option<Vec<SlotPriority_>>,
        pub slots: Option<Vec<Slot_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_Intent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.Intent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_Intent as Intent;
    impl crate::value::ToValue for Intent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_agent_intent_configuration {
                properties.insert(
                    "BedrockAgentIntentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dialog_code_hook {
                properties.insert(
                    "DialogCodeHook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.display_name {
                properties.insert(
                    "DisplayName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fulfillment_code_hook {
                properties.insert(
                    "FulfillmentCodeHook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.initial_response_setting {
                properties.insert(
                    "InitialResponseSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_contexts {
                properties.insert(
                    "InputContexts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intent_closing_setting {
                properties.insert(
                    "IntentClosingSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.intent_confirmation_setting {
                properties.insert(
                    "IntentConfirmationSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kendra_configuration {
                properties.insert(
                    "KendraConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.output_contexts {
                properties.insert(
                    "OutputContexts".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parent_intent_signature {
                properties.insert(
                    "ParentIntentSignature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.q_in_connect_intent_configuration {
                properties.insert(
                    "QInConnectIntentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.qn_a_intent_configuration {
                properties.insert(
                    "QnAIntentConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_utterances {
                properties.insert(
                    "SampleUtterances".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_priorities {
                properties.insert(
                    "SlotPriorities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slots {
                properties.insert("Slots".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentclosingsetting.html
    pub struct IntentClosingSetting_ {
        pub closing_response: Option<Box<ResponseSpecification_>>,
        pub conditional: Option<Box<ConditionalSpecification_>>,
        pub is_active: Option<crate::value::ExpBool>,
        pub next_step: Option<Box<DialogState_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_IntentClosingSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.IntentClosingSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_IntentClosingSetting as IntentClosingSetting;
    impl crate::value::ToValue for IntentClosingSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.closing_response {
                properties.insert(
                    "ClosingResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.conditional {
                properties.insert(
                    "Conditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_active {
                properties.insert(
                    "IsActive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.next_step {
                properties.insert(
                    "NextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentconfirmationsetting.html
    pub struct IntentConfirmationSetting_ {
        pub code_hook: Option<Box<DialogCodeHookInvocationSetting_>>,
        pub confirmation_conditional: Option<Box<ConditionalSpecification_>>,
        pub confirmation_next_step: Option<Box<DialogState_>>,
        pub confirmation_response: Option<Box<ResponseSpecification_>>,
        pub declination_conditional: Option<Box<ConditionalSpecification_>>,
        pub declination_next_step: Option<Box<DialogState_>>,
        pub declination_response: Option<Box<ResponseSpecification_>>,
        pub elicitation_code_hook: Option<Box<ElicitationCodeHookInvocationSetting_>>,
        pub failure_conditional: Option<Box<ConditionalSpecification_>>,
        pub failure_next_step: Option<Box<DialogState_>>,
        pub failure_response: Option<Box<ResponseSpecification_>>,
        pub is_active: Option<crate::value::ExpBool>,
        pub prompt_specification: Box<PromptSpecification_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_IntentConfirmationSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.IntentConfirmationSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_IntentConfirmationSetting as IntentConfirmationSetting;
    impl crate::value::ToValue for IntentConfirmationSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_hook {
                properties.insert(
                    "CodeHook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.confirmation_conditional {
                properties.insert(
                    "ConfirmationConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.confirmation_next_step {
                properties.insert(
                    "ConfirmationNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.confirmation_response {
                properties.insert(
                    "ConfirmationResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.declination_conditional {
                properties.insert(
                    "DeclinationConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.declination_next_step {
                properties.insert(
                    "DeclinationNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.declination_response {
                properties.insert(
                    "DeclinationResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elicitation_code_hook {
                properties.insert(
                    "ElicitationCodeHook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_conditional {
                properties.insert(
                    "FailureConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_next_step {
                properties.insert(
                    "FailureNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_response {
                properties.insert(
                    "FailureResponse".to_string(),
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
                "PromptSpecification".to_string(),
                crate::value::ToValue::to_value(&self.prompt_specification),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentdisambiguationsettings.html
    pub struct IntentDisambiguationSettings_ {
        pub custom_disambiguation_message: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
        pub max_disambiguation_intents: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_IntentDisambiguationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.IntentDisambiguationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_IntentDisambiguationSettings as IntentDisambiguationSettings;
    impl crate::value::ToValue for IntentDisambiguationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_disambiguation_message {
                properties.insert(
                    "CustomDisambiguationMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.max_disambiguation_intents {
                properties.insert(
                    "MaxDisambiguationIntents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-intentoverride.html
    pub struct IntentOverride_ {
        pub name: Option<crate::value::ExpString>,
        pub slots: Option<Vec<SlotValueOverrideMap_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_IntentOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.IntentOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_IntentOverride as IntentOverride;
    impl crate::value::ToValue for IntentOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.slots {
                properties.insert("Slots".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-kendraconfiguration.html
    pub struct KendraConfiguration_ {
        pub kendra_index: crate::value::ExpString,
        pub query_filter_string: Option<crate::value::ExpString>,
        pub query_filter_string_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_KendraConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.KendraConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_KendraConfiguration as KendraConfiguration;
    impl crate::value::ToValue for KendraConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "KendraIndex".to_string(),
                crate::value::ToValue::to_value(&self.kendra_index),
            );
            if let Some(ref value) = self.query_filter_string {
                properties.insert(
                    "QueryFilterString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_filter_string_enabled {
                properties.insert(
                    "QueryFilterStringEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-lambdacodehook.html
    pub struct LambdaCodeHook_ {
        pub code_hook_interface_version: crate::value::ExpString,
        pub lambda_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_LambdaCodeHook {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.LambdaCodeHook"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_LambdaCodeHook as LambdaCodeHook;
    impl crate::value::ToValue for LambdaCodeHook_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CodeHookInterfaceVersion".to_string(),
                crate::value::ToValue::to_value(&self.code_hook_interface_version),
            );
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-message.html
    pub struct Message_ {
        pub custom_payload: Option<Box<CustomPayload_>>,
        pub image_response_card: Option<Box<ImageResponseCard_>>,
        pub plain_text_message: Option<Box<PlainTextMessage_>>,
        pub ssml_message: Option<Box<SSMLMessage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_Message {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.Message"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_Message as Message;
    impl crate::value::ToValue for Message_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_payload {
                properties.insert(
                    "CustomPayload".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_response_card {
                properties.insert(
                    "ImageResponseCard".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.plain_text_message {
                properties.insert(
                    "PlainTextMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ssml_message {
                properties.insert(
                    "SSMLMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-messagegroup.html
    pub struct MessageGroup_ {
        pub message: Box<Message_>,
        pub variations: Option<Vec<Message_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_MessageGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.MessageGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_MessageGroup as MessageGroup;
    impl crate::value::ToValue for MessageGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(&self.message),
            );
            if let Some(ref value) = self.variations {
                properties.insert(
                    "Variations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-multiplevaluessetting.html
    pub struct MultipleValuesSetting_ {
        pub allow_multiple_values: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_MultipleValuesSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.MultipleValuesSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_MultipleValuesSetting as MultipleValuesSetting;
    impl crate::value::ToValue for MultipleValuesSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_multiple_values {
                properties.insert(
                    "AllowMultipleValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-nluimprovementspecification.html
    pub struct NluImprovementSpecification_ {
        pub assisted_nlu_mode: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
        pub intent_disambiguation_settings: Option<Box<IntentDisambiguationSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_NluImprovementSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.NluImprovementSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_NluImprovementSpecification as NluImprovementSpecification;
    impl crate::value::ToValue for NluImprovementSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.assisted_nlu_mode {
                properties.insert(
                    "AssistedNluMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            if let Some(ref value) = self.intent_disambiguation_settings {
                properties.insert(
                    "IntentDisambiguationSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-obfuscationsetting.html
    pub struct ObfuscationSetting_ {
        pub obfuscation_setting_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ObfuscationSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ObfuscationSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ObfuscationSetting as ObfuscationSetting;
    impl crate::value::ToValue for ObfuscationSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ObfuscationSettingType".to_string(),
                crate::value::ToValue::to_value(&self.obfuscation_setting_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-opensearchconfiguration.html
    pub struct OpensearchConfiguration_ {
        pub domain_endpoint: Option<crate::value::ExpString>,
        pub exact_response: Option<crate::value::ExpBool>,
        pub exact_response_fields: Option<Box<ExactResponseFields_>>,
        pub include_fields: Option<Vec<crate::value::ExpString>>,
        pub index_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_OpensearchConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.OpensearchConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_OpensearchConfiguration as OpensearchConfiguration;
    impl crate::value::ToValue for OpensearchConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_endpoint {
                properties.insert(
                    "DomainEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exact_response {
                properties.insert(
                    "ExactResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exact_response_fields {
                properties.insert(
                    "ExactResponseFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_fields {
                properties.insert(
                    "IncludeFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.index_name {
                properties.insert(
                    "IndexName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-outputcontext.html
    pub struct OutputContext_ {
        pub name: crate::value::ExpString,
        pub time_to_live_in_seconds: i32,
        pub turns_to_live: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_OutputContext {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.OutputContext"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_OutputContext as OutputContext;
    impl crate::value::ToValue for OutputContext_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TimeToLiveInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.time_to_live_in_seconds),
            );
            properties.insert(
                "TurnsToLive".to_string(),
                crate::value::ToValue::to_value(&self.turns_to_live),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-plaintextmessage.html
    pub struct PlainTextMessage_ {
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_PlainTextMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.PlainTextMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_PlainTextMessage as PlainTextMessage;
    impl crate::value::ToValue for PlainTextMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postdialogcodehookinvocationspecification.html
    pub struct PostDialogCodeHookInvocationSpecification_ {
        pub failure_conditional: Option<Box<ConditionalSpecification_>>,
        pub failure_next_step: Option<Box<DialogState_>>,
        pub failure_response: Option<Box<ResponseSpecification_>>,
        pub success_conditional: Option<Box<ConditionalSpecification_>>,
        pub success_next_step: Option<Box<DialogState_>>,
        pub success_response: Option<Box<ResponseSpecification_>>,
        pub timeout_conditional: Option<Box<ConditionalSpecification_>>,
        pub timeout_next_step: Option<Box<DialogState_>>,
        pub timeout_response: Option<Box<ResponseSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_PostDialogCodeHookInvocationSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.PostDialogCodeHookInvocationSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_PostDialogCodeHookInvocationSpecification as PostDialogCodeHookInvocationSpecification;
    impl crate::value::ToValue for PostDialogCodeHookInvocationSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failure_conditional {
                properties.insert(
                    "FailureConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_next_step {
                properties.insert(
                    "FailureNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_response {
                properties.insert(
                    "FailureResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_conditional {
                properties.insert(
                    "SuccessConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_next_step {
                properties.insert(
                    "SuccessNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_response {
                properties.insert(
                    "SuccessResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_conditional {
                properties.insert(
                    "TimeoutConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_next_step {
                properties.insert(
                    "TimeoutNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_response {
                properties.insert(
                    "TimeoutResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-postfulfillmentstatusspecification.html
    pub struct PostFulfillmentStatusSpecification_ {
        pub failure_conditional: Option<Box<ConditionalSpecification_>>,
        pub failure_next_step: Option<Box<DialogState_>>,
        pub failure_response: Option<Box<ResponseSpecification_>>,
        pub success_conditional: Option<Box<ConditionalSpecification_>>,
        pub success_next_step: Option<Box<DialogState_>>,
        pub success_response: Option<Box<ResponseSpecification_>>,
        pub timeout_conditional: Option<Box<ConditionalSpecification_>>,
        pub timeout_next_step: Option<Box<DialogState_>>,
        pub timeout_response: Option<Box<ResponseSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_PostFulfillmentStatusSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.PostFulfillmentStatusSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_PostFulfillmentStatusSpecification as PostFulfillmentStatusSpecification;
    impl crate::value::ToValue for PostFulfillmentStatusSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failure_conditional {
                properties.insert(
                    "FailureConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_next_step {
                properties.insert(
                    "FailureNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_response {
                properties.insert(
                    "FailureResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_conditional {
                properties.insert(
                    "SuccessConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_next_step {
                properties.insert(
                    "SuccessNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_response {
                properties.insert(
                    "SuccessResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_conditional {
                properties.insert(
                    "TimeoutConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_next_step {
                properties.insert(
                    "TimeoutNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_response {
                properties.insert(
                    "TimeoutResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptattemptspecification.html
    pub struct PromptAttemptSpecification_ {
        pub allow_interrupt: Option<crate::value::ExpBool>,
        pub allowed_input_types: Box<AllowedInputTypes_>,
        pub audio_and_dtmf_input_specification: Option<Box<AudioAndDTMFInputSpecification_>>,
        pub text_input_specification: Option<Box<TextInputSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_PromptAttemptSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.PromptAttemptSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_PromptAttemptSpecification as PromptAttemptSpecification;
    impl crate::value::ToValue for PromptAttemptSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_interrupt {
                properties.insert(
                    "AllowInterrupt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AllowedInputTypes".to_string(),
                crate::value::ToValue::to_value(&self.allowed_input_types),
            );
            if let Some(ref value) = self.audio_and_dtmf_input_specification {
                properties.insert(
                    "AudioAndDTMFInputSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text_input_specification {
                properties.insert(
                    "TextInputSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-promptspecification.html
    pub struct PromptSpecification_ {
        pub allow_interrupt: Option<crate::value::ExpBool>,
        pub max_retries: i32,
        pub message_groups_list: Vec<MessageGroup_>,
        pub message_selection_strategy: Option<crate::value::ExpString>,
        pub prompt_attempts_specification:
            Option<std::collections::BTreeMap<String, PromptAttemptSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_PromptSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.PromptSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_PromptSpecification as PromptSpecification;
    impl crate::value::ToValue for PromptSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_interrupt {
                properties.insert(
                    "AllowInterrupt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxRetries".to_string(),
                crate::value::ToValue::to_value(&self.max_retries),
            );
            properties.insert(
                "MessageGroupsList".to_string(),
                crate::value::ToValue::to_value(&self.message_groups_list),
            );
            if let Some(ref value) = self.message_selection_strategy {
                properties.insert(
                    "MessageSelectionStrategy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_attempts_specification {
                properties.insert(
                    "PromptAttemptsSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-qinconnectassistantconfiguration.html
    pub struct QInConnectAssistantConfiguration_ {
        pub assistant_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_QInConnectAssistantConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.QInConnectAssistantConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_QInConnectAssistantConfiguration as QInConnectAssistantConfiguration;
    impl crate::value::ToValue for QInConnectAssistantConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AssistantArn".to_string(),
                crate::value::ToValue::to_value(&self.assistant_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-qinconnectintentconfiguration.html
    pub struct QInConnectIntentConfiguration_ {
        pub q_in_connect_assistant_configuration: Option<Box<QInConnectAssistantConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_QInConnectIntentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.QInConnectIntentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_QInConnectIntentConfiguration as QInConnectIntentConfiguration;
    impl crate::value::ToValue for QInConnectIntentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.q_in_connect_assistant_configuration {
                properties.insert(
                    "QInConnectAssistantConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-qnaintentconfiguration.html
    pub struct QnAIntentConfiguration_ {
        pub bedrock_model_configuration: Box<BedrockModelSpecification_>,
        pub data_source_configuration: Box<DataSourceConfiguration_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_QnAIntentConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.QnAIntentConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_QnAIntentConfiguration as QnAIntentConfiguration;
    impl crate::value::ToValue for QnAIntentConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BedrockModelConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.bedrock_model_configuration),
            );
            properties.insert(
                "DataSourceConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.data_source_configuration),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-qnakendraconfiguration.html
    pub struct QnAKendraConfiguration_ {
        pub exact_response: crate::value::ExpBool,
        pub kendra_index: crate::value::ExpString,
        pub query_filter_string: Option<crate::value::ExpString>,
        pub query_filter_string_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_QnAKendraConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.QnAKendraConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_QnAKendraConfiguration as QnAKendraConfiguration;
    impl crate::value::ToValue for QnAKendraConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ExactResponse".to_string(),
                crate::value::ToValue::to_value(&self.exact_response),
            );
            properties.insert(
                "KendraIndex".to_string(),
                crate::value::ToValue::to_value(&self.kendra_index),
            );
            if let Some(ref value) = self.query_filter_string {
                properties.insert(
                    "QueryFilterString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueryFilterStringEnabled".to_string(),
                crate::value::ToValue::to_value(&self.query_filter_string_enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-replication.html
    pub struct Replication_ {
        pub replica_regions: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_Replication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.Replication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_Replication as Replication;
    impl crate::value::ToValue for Replication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ReplicaRegions".to_string(),
                crate::value::ToValue::to_value(&self.replica_regions),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-responsespecification.html
    pub struct ResponseSpecification_ {
        pub allow_interrupt: Option<crate::value::ExpBool>,
        pub message_groups_list: Vec<MessageGroup_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_ResponseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.ResponseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_ResponseSpecification as ResponseSpecification;
    impl crate::value::ToValue for ResponseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_interrupt {
                properties.insert(
                    "AllowInterrupt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MessageGroupsList".to_string(),
                crate::value::ToValue::to_value(&self.message_groups_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-runtimesettings.html
    pub struct RuntimeSettings_ {
        pub nlu_improvement_specification: Option<Box<NluImprovementSpecification_>>,
        pub slot_resolution_improvement_specification:
            Option<Box<SlotResolutionImprovementSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_RuntimeSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.RuntimeSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_RuntimeSettings as RuntimeSettings;
    impl crate::value::ToValue for RuntimeSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.nlu_improvement_specification {
                properties.insert(
                    "NluImprovementSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_resolution_improvement_specification {
                properties.insert(
                    "SlotResolutionImprovementSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3bucketlogdestination.html
    pub struct S3BucketLogDestination_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub log_prefix: crate::value::ExpString,
        pub s3_bucket_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_S3BucketLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.S3BucketLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_S3BucketLogDestination as S3BucketLogDestination;
    impl crate::value::ToValue for S3BucketLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LogPrefix".to_string(),
                crate::value::ToValue::to_value(&self.log_prefix),
            );
            properties.insert(
                "S3BucketArn".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-s3location.html
    pub struct S3Location_ {
        pub s3_bucket: crate::value::ExpString,
        pub s3_object_key: crate::value::ExpString,
        pub s3_object_version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_S3Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.S3Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_S3Location as S3Location;
    impl crate::value::ToValue for S3Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.insert(
                "S3ObjectKey".to_string(),
                crate::value::ToValue::to_value(&self.s3_object_key),
            );
            if let Some(ref value) = self.s3_object_version {
                properties.insert(
                    "S3ObjectVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-ssmlmessage.html
    pub struct SSMLMessage_ {
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SSMLMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SSMLMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SSMLMessage as SSMLMessage;
    impl crate::value::ToValue for SSMLMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sampleutterance.html
    pub struct SampleUtterance_ {
        pub utterance: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SampleUtterance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SampleUtterance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SampleUtterance as SampleUtterance;
    impl crate::value::ToValue for SampleUtterance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Utterance".to_string(),
                crate::value::ToValue::to_value(&self.utterance),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sampleutterancegenerationspecification.html
    pub struct SampleUtteranceGenerationSpecification_ {
        pub bedrock_model_specification: Option<Box<BedrockModelSpecification_>>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SampleUtteranceGenerationSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SampleUtteranceGenerationSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SampleUtteranceGenerationSpecification as SampleUtteranceGenerationSpecification;
    impl crate::value::ToValue for SampleUtteranceGenerationSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_model_specification {
                properties.insert(
                    "BedrockModelSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-samplevalue.html
    pub struct SampleValue_ {
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SampleValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SampleValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SampleValue as SampleValue;
    impl crate::value::ToValue for SampleValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sentimentanalysissettings.html
    pub struct SentimentAnalysisSettings_ {
        pub detect_sentiment: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SentimentAnalysisSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SentimentAnalysisSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SentimentAnalysisSettings as SentimentAnalysisSettings;
    impl crate::value::ToValue for SentimentAnalysisSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DetectSentiment".to_string(),
                crate::value::ToValue::to_value(&self.detect_sentiment),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-sessionattribute.html
    pub struct SessionAttribute_ {
        pub key: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SessionAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SessionAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SessionAttribute as SessionAttribute;
    impl crate::value::ToValue for SessionAttribute_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slot.html
    pub struct Slot_ {
        pub description: Option<crate::value::ExpString>,
        pub multiple_values_setting: Option<Box<MultipleValuesSetting_>>,
        pub name: crate::value::ExpString,
        pub obfuscation_setting: Option<Box<ObfuscationSetting_>>,
        pub slot_type_name: crate::value::ExpString,
        pub sub_slot_setting: Option<Box<SubSlotSetting_>>,
        pub value_elicitation_setting: Box<SlotValueElicitationSetting_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_Slot {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.Slot"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_Slot as Slot;
    impl crate::value::ToValue for Slot_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multiple_values_setting {
                properties.insert(
                    "MultipleValuesSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.obfuscation_setting {
                properties.insert(
                    "ObfuscationSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SlotTypeName".to_string(),
                crate::value::ToValue::to_value(&self.slot_type_name),
            );
            if let Some(ref value) = self.sub_slot_setting {
                properties.insert(
                    "SubSlotSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ValueElicitationSetting".to_string(),
                crate::value::ToValue::to_value(&self.value_elicitation_setting),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotcapturesetting.html
    pub struct SlotCaptureSetting_ {
        pub capture_conditional: Option<Box<ConditionalSpecification_>>,
        pub capture_next_step: Option<Box<DialogState_>>,
        pub capture_response: Option<Box<ResponseSpecification_>>,
        pub code_hook: Option<Box<DialogCodeHookInvocationSetting_>>,
        pub elicitation_code_hook: Option<Box<ElicitationCodeHookInvocationSetting_>>,
        pub failure_conditional: Option<Box<ConditionalSpecification_>>,
        pub failure_next_step: Option<Box<DialogState_>>,
        pub failure_response: Option<Box<ResponseSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotCaptureSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotCaptureSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotCaptureSetting as SlotCaptureSetting;
    impl crate::value::ToValue for SlotCaptureSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capture_conditional {
                properties.insert(
                    "CaptureConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capture_next_step {
                properties.insert(
                    "CaptureNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capture_response {
                properties.insert(
                    "CaptureResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.code_hook {
                properties.insert(
                    "CodeHook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.elicitation_code_hook {
                properties.insert(
                    "ElicitationCodeHook".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_conditional {
                properties.insert(
                    "FailureConditional".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_next_step {
                properties.insert(
                    "FailureNextStep".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_response {
                properties.insert(
                    "FailureResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotdefaultvalue.html
    pub struct SlotDefaultValue_ {
        pub default_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotDefaultValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotDefaultValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotDefaultValue as SlotDefaultValue;
    impl crate::value::ToValue for SlotDefaultValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultValue".to_string(),
                crate::value::ToValue::to_value(&self.default_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotdefaultvaluespecification.html
    pub struct SlotDefaultValueSpecification_ {
        pub default_value_list: Vec<SlotDefaultValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotDefaultValueSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotDefaultValueSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotDefaultValueSpecification as SlotDefaultValueSpecification;
    impl crate::value::ToValue for SlotDefaultValueSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultValueList".to_string(),
                crate::value::ToValue::to_value(&self.default_value_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotpriority.html
    pub struct SlotPriority_ {
        pub priority: i32,
        pub slot_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotPriority {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotPriority"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotPriority as SlotPriority;
    impl crate::value::ToValue for SlotPriority_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "SlotName".to_string(),
                crate::value::ToValue::to_value(&self.slot_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotresolutionimprovementspecification.html
    pub struct SlotResolutionImprovementSpecification_ {
        pub bedrock_model_specification: Option<Box<BedrockModelSpecification_>>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotResolutionImprovementSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotResolutionImprovementSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotResolutionImprovementSpecification as SlotResolutionImprovementSpecification;
    impl crate::value::ToValue for SlotResolutionImprovementSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bedrock_model_specification {
                properties.insert(
                    "BedrockModelSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottype.html
    pub struct SlotType_ {
        pub composite_slot_type_setting: Option<Box<CompositeSlotTypeSetting_>>,
        pub description: Option<crate::value::ExpString>,
        pub external_source_setting: Option<Box<ExternalSourceSetting_>>,
        pub name: crate::value::ExpString,
        pub parent_slot_type_signature: Option<crate::value::ExpString>,
        pub slot_type_values: Option<Vec<SlotTypeValue_>>,
        pub value_selection_setting: Option<Box<SlotValueSelectionSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotType as SlotType;
    impl crate::value::ToValue for SlotType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.composite_slot_type_setting {
                properties.insert(
                    "CompositeSlotTypeSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.external_source_setting {
                properties.insert(
                    "ExternalSourceSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.parent_slot_type_signature {
                properties.insert(
                    "ParentSlotTypeSignature".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_type_values {
                properties.insert(
                    "SlotTypeValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value_selection_setting {
                properties.insert(
                    "ValueSelectionSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slottypevalue.html
    pub struct SlotTypeValue_ {
        pub sample_value: Box<SampleValue_>,
        pub synonyms: Option<Vec<SampleValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotTypeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotTypeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotTypeValue as SlotTypeValue;
    impl crate::value::ToValue for SlotTypeValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SampleValue".to_string(),
                crate::value::ToValue::to_value(&self.sample_value),
            );
            if let Some(ref value) = self.synonyms {
                properties.insert(
                    "Synonyms".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalue.html
    pub struct SlotValue_ {
        pub interpreted_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotValue as SlotValue;
    impl crate::value::ToValue for SlotValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.interpreted_value {
                properties.insert(
                    "InterpretedValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueelicitationsetting.html
    pub struct SlotValueElicitationSetting_ {
        pub default_value_specification: Option<Box<SlotDefaultValueSpecification_>>,
        pub prompt_specification: Option<Box<PromptSpecification_>>,
        pub sample_utterances: Option<Vec<SampleUtterance_>>,
        pub slot_capture_setting: Option<Box<SlotCaptureSetting_>>,
        pub slot_constraint: crate::value::ExpString,
        pub wait_and_continue_specification: Option<Box<WaitAndContinueSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotValueElicitationSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotValueElicitationSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotValueElicitationSetting as SlotValueElicitationSetting;
    impl crate::value::ToValue for SlotValueElicitationSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_value_specification {
                properties.insert(
                    "DefaultValueSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_specification {
                properties.insert(
                    "PromptSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_utterances {
                properties.insert(
                    "SampleUtterances".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_capture_setting {
                properties.insert(
                    "SlotCaptureSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SlotConstraint".to_string(),
                crate::value::ToValue::to_value(&self.slot_constraint),
            );
            if let Some(ref value) = self.wait_and_continue_specification {
                properties.insert(
                    "WaitAndContinueSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverride.html
    pub struct SlotValueOverride_ {
        pub shape: Option<crate::value::ExpString>,
        pub value: Option<Box<SlotValue_>>,
        pub values: Option<Vec<SlotValueOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotValueOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotValueOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotValueOverride as SlotValueOverride;
    impl crate::value::ToValue for SlotValueOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.shape {
                properties.insert("Shape".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueoverridemap.html
    pub struct SlotValueOverrideMap_ {
        pub slot_name: Option<crate::value::ExpString>,
        pub slot_value_override: Option<Box<SlotValueOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotValueOverrideMap {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotValueOverrideMap"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotValueOverrideMap as SlotValueOverrideMap;
    impl crate::value::ToValue for SlotValueOverrideMap_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.slot_name {
                properties.insert(
                    "SlotName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_value_override {
                properties.insert(
                    "SlotValueOverride".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueregexfilter.html
    pub struct SlotValueRegexFilter_ {
        pub pattern: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotValueRegexFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotValueRegexFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotValueRegexFilter as SlotValueRegexFilter;
    impl crate::value::ToValue for SlotValueRegexFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Pattern".to_string(),
                crate::value::ToValue::to_value(&self.pattern),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-slotvalueselectionsetting.html
    pub struct SlotValueSelectionSetting_ {
        pub advanced_recognition_setting: Option<Box<AdvancedRecognitionSetting_>>,
        pub regex_filter: Option<Box<SlotValueRegexFilter_>>,
        pub resolution_strategy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SlotValueSelectionSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SlotValueSelectionSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SlotValueSelectionSetting as SlotValueSelectionSetting;
    impl crate::value::ToValue for SlotValueSelectionSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.advanced_recognition_setting {
                properties.insert(
                    "AdvancedRecognitionSetting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regex_filter {
                properties.insert(
                    "RegexFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResolutionStrategy".to_string(),
                crate::value::ToValue::to_value(&self.resolution_strategy),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-specifications.html
    pub struct Specifications_ {
        pub slot_type_id: Option<crate::value::ExpString>,
        pub slot_type_name: Option<crate::value::ExpString>,
        pub value_elicitation_setting: Box<SubSlotValueElicitationSetting_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_Specifications {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.Specifications"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_Specifications as Specifications;
    impl crate::value::ToValue for Specifications_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.slot_type_id {
                properties.insert(
                    "SlotTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_type_name {
                properties.insert(
                    "SlotTypeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ValueElicitationSetting".to_string(),
                crate::value::ToValue::to_value(&self.value_elicitation_setting),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-speechfoundationmodel.html
    pub struct SpeechFoundationModel_ {
        pub model_arn: crate::value::ExpString,
        pub voice_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SpeechFoundationModel {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SpeechFoundationModel"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SpeechFoundationModel as SpeechFoundationModel;
    impl crate::value::ToValue for SpeechFoundationModel_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ModelArn".to_string(),
                crate::value::ToValue::to_value(&self.model_arn),
            );
            if let Some(ref value) = self.voice_id {
                properties.insert(
                    "VoiceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-speechmodelconfig.html
    pub struct SpeechModelConfig_ {
        pub deepgram_config: Option<Box<DeepgramSpeechModelConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SpeechModelConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SpeechModelConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SpeechModelConfig as SpeechModelConfig;
    impl crate::value::ToValue for SpeechModelConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.deepgram_config {
                properties.insert(
                    "DeepgramConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-speechrecognitionsettings.html
    pub struct SpeechRecognitionSettings_ {
        pub speech_model_config: Option<Box<SpeechModelConfig_>>,
        pub speech_model_preference: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SpeechRecognitionSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SpeechRecognitionSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SpeechRecognitionSettings as SpeechRecognitionSettings;
    impl crate::value::ToValue for SpeechRecognitionSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.speech_model_config {
                properties.insert(
                    "SpeechModelConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.speech_model_preference {
                properties.insert(
                    "SpeechModelPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-stillwaitingresponsespecification.html
    pub struct StillWaitingResponseSpecification_ {
        pub allow_interrupt: Option<crate::value::ExpBool>,
        pub frequency_in_seconds: i32,
        pub message_groups_list: Vec<MessageGroup_>,
        pub timeout_in_seconds: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_StillWaitingResponseSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.StillWaitingResponseSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_StillWaitingResponseSpecification as StillWaitingResponseSpecification;
    impl crate::value::ToValue for StillWaitingResponseSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_interrupt {
                properties.insert(
                    "AllowInterrupt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FrequencyInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.frequency_in_seconds),
            );
            properties.insert(
                "MessageGroupsList".to_string(),
                crate::value::ToValue::to_value(&self.message_groups_list),
            );
            properties.insert(
                "TimeoutInSeconds".to_string(),
                crate::value::ToValue::to_value(&self.timeout_in_seconds),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-subslotsetting.html
    pub struct SubSlotSetting_ {
        pub expression: Option<crate::value::ExpString>,
        pub slot_specifications: Option<std::collections::BTreeMap<String, Specifications_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SubSlotSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SubSlotSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SubSlotSetting as SubSlotSetting;
    impl crate::value::ToValue for SubSlotSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_specifications {
                properties.insert(
                    "SlotSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-subslottypecomposition.html
    pub struct SubSlotTypeComposition_ {
        pub name: crate::value::ExpString,
        pub slot_type_id: Option<crate::value::ExpString>,
        pub slot_type_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SubSlotTypeComposition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SubSlotTypeComposition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SubSlotTypeComposition as SubSlotTypeComposition;
    impl crate::value::ToValue for SubSlotTypeComposition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.slot_type_id {
                properties.insert(
                    "SlotTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_type_name {
                properties.insert(
                    "SlotTypeName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-subslotvalueelicitationsetting.html
    pub struct SubSlotValueElicitationSetting_ {
        pub default_value_specification: Option<Box<SlotDefaultValueSpecification_>>,
        pub prompt_specification: Option<Box<PromptSpecification_>>,
        pub sample_utterances: Option<Vec<SampleUtterance_>>,
        pub wait_and_continue_specification: Option<Box<WaitAndContinueSpecification_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_SubSlotValueElicitationSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.SubSlotValueElicitationSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_SubSlotValueElicitationSetting as SubSlotValueElicitationSetting;
    impl crate::value::ToValue for SubSlotValueElicitationSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_value_specification {
                properties.insert(
                    "DefaultValueSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.prompt_specification {
                properties.insert(
                    "PromptSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sample_utterances {
                properties.insert(
                    "SampleUtterances".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.wait_and_continue_specification {
                properties.insert(
                    "WaitAndContinueSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-testbotaliassettings.html
    pub struct TestBotAliasSettings_ {
        pub bot_alias_locale_settings: Option<Vec<BotAliasLocaleSettingsItem_>>,
        pub conversation_log_settings: Option<Box<ConversationLogSettings_>>,
        pub description: Option<crate::value::ExpString>,
        pub sentiment_analysis_settings: Option<Box<SentimentAnalysisSettings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_TestBotAliasSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.TestBotAliasSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_TestBotAliasSettings as TestBotAliasSettings;
    impl crate::value::ToValue for TestBotAliasSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bot_alias_locale_settings {
                properties.insert(
                    "BotAliasLocaleSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.conversation_log_settings {
                properties.insert(
                    "ConversationLogSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sentiment_analysis_settings {
                properties.insert(
                    "SentimentAnalysisSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textinputspecification.html
    pub struct TextInputSpecification_ {
        pub start_timeout_ms: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_TextInputSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.TextInputSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_TextInputSpecification as TextInputSpecification;
    impl crate::value::ToValue for TextInputSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StartTimeoutMs".to_string(),
                crate::value::ToValue::to_value(&self.start_timeout_ms),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textlogdestination.html
    pub struct TextLogDestination_ {
        pub cloud_watch: Box<CloudWatchLogGroupLogDestination_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_TextLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.TextLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_TextLogDestination as TextLogDestination;
    impl crate::value::ToValue for TextLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatch".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-textlogsetting.html
    pub struct TextLogSetting_ {
        pub destination: Box<TextLogDestination_>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_TextLogSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.TextLogSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_TextLogSetting as TextLogSetting;
    impl crate::value::ToValue for TextLogSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-unifiedspeechsettings.html
    pub struct UnifiedSpeechSettings_ {
        pub speech_foundation_model: Box<SpeechFoundationModel_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_UnifiedSpeechSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.UnifiedSpeechSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_UnifiedSpeechSettings as UnifiedSpeechSettings;
    impl crate::value::ToValue for UnifiedSpeechSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SpeechFoundationModel".to_string(),
                crate::value::ToValue::to_value(&self.speech_foundation_model),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-voicesettings.html
    pub struct VoiceSettings_ {
        pub engine: Option<crate::value::ExpString>,
        pub voice_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_VoiceSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.VoiceSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_VoiceSettings as VoiceSettings;
    impl crate::value::ToValue for VoiceSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.engine {
                properties.insert("Engine".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "VoiceId".to_string(),
                crate::value::ToValue::to_value(&self.voice_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-bot-waitandcontinuespecification.html
    pub struct WaitAndContinueSpecification_ {
        pub continue_response: Box<ResponseSpecification_>,
        pub is_active: Option<crate::value::ExpBool>,
        pub still_waiting_response: Option<Box<StillWaitingResponseSpecification_>>,
        pub waiting_response: Box<ResponseSpecification_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_Bot_WaitAndContinueSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::Bot.WaitAndContinueSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_Bot_WaitAndContinueSpecification as WaitAndContinueSpecification;
    impl crate::value::ToValue for WaitAndContinueSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContinueResponse".to_string(),
                crate::value::ToValue::to_value(&self.continue_response),
            );
            if let Some(ref value) = self.is_active {
                properties.insert(
                    "IsActive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.still_waiting_response {
                properties.insert(
                    "StillWaitingResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "WaitingResponse".to_string(),
                crate::value::ToValue::to_value(&self.waiting_response),
            );
            properties.into()
        }
    }
}
pub mod botalias {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-audiologdestination.html
    pub struct AudioLogDestination_ {
        pub s3_bucket: Box<S3BucketLogDestination_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_AudioLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.AudioLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_AudioLogDestination as AudioLogDestination;
    impl crate::value::ToValue for AudioLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-audiologsetting.html
    pub struct AudioLogSetting_ {
        pub destination: Box<AudioLogDestination_>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_AudioLogSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.AudioLogSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_AudioLogSetting as AudioLogSetting;
    impl crate::value::ToValue for AudioLogSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettings.html
    pub struct BotAliasLocaleSettings_ {
        pub code_hook_specification: Option<Box<CodeHookSpecification_>>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_BotAliasLocaleSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.BotAliasLocaleSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_BotAliasLocaleSettings as BotAliasLocaleSettings;
    impl crate::value::ToValue for BotAliasLocaleSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code_hook_specification {
                properties.insert(
                    "CodeHookSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-botaliaslocalesettingsitem.html
    pub struct BotAliasLocaleSettingsItem_ {
        pub bot_alias_locale_setting: Box<BotAliasLocaleSettings_>,
        pub locale_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_BotAliasLocaleSettingsItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.BotAliasLocaleSettingsItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_BotAliasLocaleSettingsItem as BotAliasLocaleSettingsItem;
    impl crate::value::ToValue for BotAliasLocaleSettingsItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BotAliasLocaleSetting".to_string(),
                crate::value::ToValue::to_value(&self.bot_alias_locale_setting),
            );
            properties.insert(
                "LocaleId".to_string(),
                crate::value::ToValue::to_value(&self.locale_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-cloudwatchloggrouplogdestination.html
    pub struct CloudWatchLogGroupLogDestination_ {
        pub cloud_watch_log_group_arn: crate::value::ExpString,
        pub log_prefix: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_CloudWatchLogGroupLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.CloudWatchLogGroupLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_CloudWatchLogGroupLogDestination as CloudWatchLogGroupLogDestination;
    impl crate::value::ToValue for CloudWatchLogGroupLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchLogGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_log_group_arn),
            );
            properties.insert(
                "LogPrefix".to_string(),
                crate::value::ToValue::to_value(&self.log_prefix),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-codehookspecification.html
    pub struct CodeHookSpecification_ {
        pub lambda_code_hook: Box<LambdaCodeHook_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_CodeHookSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.CodeHookSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_CodeHookSpecification as CodeHookSpecification;
    impl crate::value::ToValue for CodeHookSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LambdaCodeHook".to_string(),
                crate::value::ToValue::to_value(&self.lambda_code_hook),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-conversationlogsettings.html
    pub struct ConversationLogSettings_ {
        pub audio_log_settings: Option<Vec<AudioLogSetting_>>,
        pub text_log_settings: Option<Vec<TextLogSetting_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_ConversationLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.ConversationLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_ConversationLogSettings as ConversationLogSettings;
    impl crate::value::ToValue for ConversationLogSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.audio_log_settings {
                properties.insert(
                    "AudioLogSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text_log_settings {
                properties.insert(
                    "TextLogSettings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-lambdacodehook.html
    pub struct LambdaCodeHook_ {
        pub code_hook_interface_version: crate::value::ExpString,
        pub lambda_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_LambdaCodeHook {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.LambdaCodeHook"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_LambdaCodeHook as LambdaCodeHook;
    impl crate::value::ToValue for LambdaCodeHook_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CodeHookInterfaceVersion".to_string(),
                crate::value::ToValue::to_value(&self.code_hook_interface_version),
            );
            properties.insert(
                "LambdaArn".to_string(),
                crate::value::ToValue::to_value(&self.lambda_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-s3bucketlogdestination.html
    pub struct S3BucketLogDestination_ {
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub log_prefix: crate::value::ExpString,
        pub s3_bucket_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_S3BucketLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.S3BucketLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_S3BucketLogDestination as S3BucketLogDestination;
    impl crate::value::ToValue for S3BucketLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LogPrefix".to_string(),
                crate::value::ToValue::to_value(&self.log_prefix),
            );
            properties.insert(
                "S3BucketArn".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-sentimentanalysissettings.html
    pub struct SentimentAnalysisSettings_ {
        pub detect_sentiment: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_SentimentAnalysisSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.SentimentAnalysisSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_SentimentAnalysisSettings as SentimentAnalysisSettings;
    impl crate::value::ToValue for SentimentAnalysisSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DetectSentiment".to_string(),
                crate::value::ToValue::to_value(&self.detect_sentiment),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-textlogdestination.html
    pub struct TextLogDestination_ {
        pub cloud_watch: Box<CloudWatchLogGroupLogDestination_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_TextLogDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.TextLogDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_TextLogDestination as TextLogDestination;
    impl crate::value::ToValue for TextLogDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatch".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botalias-textlogsetting.html
    pub struct TextLogSetting_ {
        pub destination: Box<TextLogDestination_>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotAlias_TextLogSetting {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotAlias.TextLogSetting"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotAlias_TextLogSetting as TextLogSetting;
    impl crate::value::ToValue for TextLogSetting_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
}
pub mod botversion {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botversion-botversionlocaledetails.html
    pub struct BotVersionLocaleDetails_ {
        pub source_bot_version: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotVersion_BotVersionLocaleDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotVersion.BotVersionLocaleDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotVersion_BotVersionLocaleDetails as BotVersionLocaleDetails;
    impl crate::value::ToValue for BotVersionLocaleDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "SourceBotVersion".to_string(),
                crate::value::ToValue::to_value(&self.source_bot_version),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lex-botversion-botversionlocalespecification.html
    pub struct BotVersionLocaleSpecification_ {
        pub bot_version_locale_details: Box<BotVersionLocaleDetails_>,
        pub locale_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lex_BotVersion_BotVersionLocaleSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lex::BotVersion.BotVersionLocaleSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lex_BotVersion_BotVersionLocaleSpecification as BotVersionLocaleSpecification;
    impl crate::value::ToValue for BotVersionLocaleSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BotVersionLocaleDetails".to_string(),
                crate::value::ToValue::to_value(&self.bot_version_locale_details),
            );
            properties.insert(
                "LocaleId".to_string(),
                crate::value::ToValue::to_value(&self.locale_id),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-bot.html
pub struct Bot_ {
    pub auto_build_bot_locales: Option<crate::value::ExpBool>,
    pub bot_file_s3_location: Option<super::lex::bot::S3Location_>,
    pub bot_locales: Option<Vec<super::lex::bot::BotLocale_>>,
    pub bot_tags: Option<Vec<crate::Tag_>>,
    pub data_privacy: super::lex::bot::DataPrivacy_,
    pub description: Option<crate::value::ExpString>,
    pub error_log_settings: Option<super::lex::bot::ErrorLogSettings_>,
    pub idle_session_ttl_in_seconds: i32,
    pub name: crate::value::ExpString,
    pub replication: Option<super::lex::bot::Replication_>,
    pub role_arn: crate::value::ExpString,
    pub test_bot_alias_settings: Option<super::lex::bot::TestBotAliasSettings_>,
    pub test_bot_alias_tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lex_Bot {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lex::Bot" $($field
        $value)*)
    };
}
pub use crate::__aws_lex_Bot as Bot;
impl crate::template::ToResource for Bot_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lex"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Bot"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_build_bot_locales {
            properties.insert(
                "AutoBuildBotLocales".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bot_file_s3_location {
            properties.insert(
                "BotFileS3Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bot_locales {
            properties.insert(
                "BotLocales".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.bot_tags {
            properties.insert(
                "BotTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DataPrivacy".to_string(),
            crate::value::ToValue::to_value(&self.data_privacy),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.error_log_settings {
            properties.insert(
                "ErrorLogSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IdleSessionTTLInSeconds".to_string(),
            crate::value::ToValue::to_value(&self.idle_session_ttl_in_seconds),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.replication {
            properties.insert(
                "Replication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        if let Some(ref value) = self.test_bot_alias_settings {
            properties.insert(
                "TestBotAliasSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.test_bot_alias_tags {
            properties.insert(
                "TestBotAliasTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botalias.html
pub struct BotAlias_ {
    pub bot_alias_locale_settings: Option<Vec<super::lex::botalias::BotAliasLocaleSettingsItem_>>,
    pub bot_alias_name: crate::value::ExpString,
    pub bot_alias_tags: Option<Vec<crate::Tag_>>,
    pub bot_id: crate::value::ExpString,
    pub bot_version: Option<crate::value::ExpString>,
    pub conversation_log_settings: Option<super::lex::botalias::ConversationLogSettings_>,
    pub description: Option<crate::value::ExpString>,
    pub sentiment_analysis_settings: Option<super::lex::botalias::SentimentAnalysisSettings_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lex_BotAlias {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lex::BotAlias" $($field
        $value)*)
    };
}
pub use crate::__aws_lex_BotAlias as BotAlias;
impl crate::template::ToResource for BotAlias_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lex"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BotAlias"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.bot_alias_locale_settings {
            properties.insert(
                "BotAliasLocaleSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BotAliasName".to_string(),
            crate::value::ToValue::to_value(&self.bot_alias_name),
        );
        if let Some(ref value) = self.bot_alias_tags {
            properties.insert(
                "BotAliasTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BotId".to_string(),
            crate::value::ToValue::to_value(&self.bot_id),
        );
        if let Some(ref value) = self.bot_version {
            properties.insert(
                "BotVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.conversation_log_settings {
            properties.insert(
                "ConversationLogSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sentiment_analysis_settings {
            properties.insert(
                "SentimentAnalysisSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-botversion.html
pub struct BotVersion_ {
    pub bot_id: crate::value::ExpString,
    pub bot_version_locale_specification:
        Vec<super::lex::botversion::BotVersionLocaleSpecification_>,
    pub description: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lex_BotVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lex::BotVersion" $($field
        $value)*)
    };
}
pub use crate::__aws_lex_BotVersion as BotVersion;
impl crate::template::ToResource for BotVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lex"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BotVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BotId".to_string(),
            crate::value::ToValue::to_value(&self.bot_id),
        );
        properties.insert(
            "BotVersionLocaleSpecification".to_string(),
            crate::value::ToValue::to_value(&self.bot_version_locale_specification),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lex-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub policy: serde_json::Value,
    pub resource_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lex_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lex::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_lex_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lex"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Policy".to_string(),
            crate::value::ToValue::to_value(&self.policy),
        );
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties
    }
}
