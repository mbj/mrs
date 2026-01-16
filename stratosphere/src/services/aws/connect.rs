pub mod evaluationform {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-autoevaluationconfiguration.html
    pub struct AutoEvaluationConfiguration_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_AutoEvaluationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.AutoEvaluationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_AutoEvaluationConfiguration as AutoEvaluationConfiguration;
    impl crate::value::ToValue for AutoEvaluationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformbaseitem.html
    pub struct EvaluationFormBaseItem_ {
        pub section: Box<EvaluationFormSection_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormBaseItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormBaseItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormBaseItem as EvaluationFormBaseItem;
    impl crate::value::ToValue for EvaluationFormBaseItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Section".to_string(),
                crate::value::ToValue::to_value(&self.section),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformitem.html
    pub struct EvaluationFormItem_ {
        pub question: Option<Box<EvaluationFormQuestion_>>,
        pub section: Option<Box<EvaluationFormSection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormItem as EvaluationFormItem;
    impl crate::value::ToValue for EvaluationFormItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.question {
                properties.insert(
                    "Question".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.section {
                properties.insert(
                    "Section".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionautomation.html
    pub struct EvaluationFormNumericQuestionAutomation_ {
        pub answer_source: Option<serde_json::Value>,
        pub property_value: Option<Box<NumericQuestionPropertyValueAutomation_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormNumericQuestionAutomation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormNumericQuestionAutomation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormNumericQuestionAutomation as EvaluationFormNumericQuestionAutomation;
    impl crate::value::ToValue for EvaluationFormNumericQuestionAutomation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_source {
                properties.insert(
                    "AnswerSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_value {
                properties.insert(
                    "PropertyValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionoption.html
    pub struct EvaluationFormNumericQuestionOption_ {
        pub automatic_fail: Option<crate::value::ExpBool>,
        pub max_value: i64,
        pub min_value: i64,
        pub score: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormNumericQuestionOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormNumericQuestionOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormNumericQuestionOption as EvaluationFormNumericQuestionOption;
    impl crate::value::ToValue for EvaluationFormNumericQuestionOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic_fail {
                properties.insert(
                    "AutomaticFail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxValue".to_string(),
                crate::value::ToValue::to_value(&self.max_value),
            );
            properties.insert(
                "MinValue".to_string(),
                crate::value::ToValue::to_value(&self.min_value),
            );
            if let Some(ref value) = self.score {
                properties.insert("Score".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformnumericquestionproperties.html
    pub struct EvaluationFormNumericQuestionProperties_ {
        pub automation: Option<Box<EvaluationFormNumericQuestionAutomation_>>,
        pub max_value: i64,
        pub min_value: i64,
        pub options: Option<Vec<EvaluationFormNumericQuestionOption_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormNumericQuestionProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormNumericQuestionProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormNumericQuestionProperties as EvaluationFormNumericQuestionProperties;
    impl crate::value::ToValue for EvaluationFormNumericQuestionProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automation {
                properties.insert(
                    "Automation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MaxValue".to_string(),
                crate::value::ToValue::to_value(&self.max_value),
            );
            properties.insert(
                "MinValue".to_string(),
                crate::value::ToValue::to_value(&self.min_value),
            );
            if let Some(ref value) = self.options {
                properties.insert(
                    "Options".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestion.html
    pub struct EvaluationFormQuestion_ {
        pub instructions: Option<crate::value::ExpString>,
        pub not_applicable_enabled: Option<crate::value::ExpBool>,
        pub question_type: crate::value::ExpString,
        pub question_type_properties: Option<Box<EvaluationFormQuestionTypeProperties_>>,
        pub ref_id: crate::value::ExpString,
        pub title: crate::value::ExpString,
        pub weight: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormQuestion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormQuestion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormQuestion as EvaluationFormQuestion;
    impl crate::value::ToValue for EvaluationFormQuestion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instructions {
                properties.insert(
                    "Instructions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_applicable_enabled {
                properties.insert(
                    "NotApplicableEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QuestionType".to_string(),
                crate::value::ToValue::to_value(&self.question_type),
            );
            if let Some(ref value) = self.question_type_properties {
                properties.insert(
                    "QuestionTypeProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RefId".to_string(),
                crate::value::ToValue::to_value(&self.ref_id),
            );
            properties.insert(
                "Title".to_string(),
                crate::value::ToValue::to_value(&self.title),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformquestiontypeproperties.html
    pub struct EvaluationFormQuestionTypeProperties_ {
        pub numeric: Option<Box<EvaluationFormNumericQuestionProperties_>>,
        pub single_select: Option<Box<EvaluationFormSingleSelectQuestionProperties_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormQuestionTypeProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormQuestionTypeProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormQuestionTypeProperties as EvaluationFormQuestionTypeProperties;
    impl crate::value::ToValue for EvaluationFormQuestionTypeProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.numeric {
                properties.insert(
                    "Numeric".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_select {
                properties.insert(
                    "SingleSelect".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsection.html
    pub struct EvaluationFormSection_ {
        pub instructions: Option<crate::value::ExpString>,
        pub items: Option<Vec<EvaluationFormItem_>>,
        pub ref_id: crate::value::ExpString,
        pub title: crate::value::ExpString,
        pub weight: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormSection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormSection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormSection as EvaluationFormSection;
    impl crate::value::ToValue for EvaluationFormSection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instructions {
                properties.insert(
                    "Instructions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.items {
                properties.insert("Items".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "RefId".to_string(),
                crate::value::ToValue::to_value(&self.ref_id),
            );
            properties.insert(
                "Title".to_string(),
                crate::value::ToValue::to_value(&self.title),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionautomation.html
    pub struct EvaluationFormSingleSelectQuestionAutomation_ {
        pub default_option_ref_id: Option<crate::value::ExpString>,
        pub options: Vec<EvaluationFormSingleSelectQuestionAutomationOption_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionAutomation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionAutomation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionAutomation as EvaluationFormSingleSelectQuestionAutomation;
    impl crate::value::ToValue for EvaluationFormSingleSelectQuestionAutomation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_option_ref_id {
                properties.insert(
                    "DefaultOptionRefId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(&self.options),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionautomationoption.html
    pub struct EvaluationFormSingleSelectQuestionAutomationOption_ {
        pub rule_category: Box<SingleSelectQuestionRuleCategoryAutomation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionAutomationOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionAutomationOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionAutomationOption as EvaluationFormSingleSelectQuestionAutomationOption;
    impl crate::value::ToValue for EvaluationFormSingleSelectQuestionAutomationOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RuleCategory".to_string(),
                crate::value::ToValue::to_value(&self.rule_category),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionoption.html
    pub struct EvaluationFormSingleSelectQuestionOption_ {
        pub automatic_fail: Option<crate::value::ExpBool>,
        pub ref_id: crate::value::ExpString,
        pub score: Option<i64>,
        pub text: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionOption {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionOption"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionOption as EvaluationFormSingleSelectQuestionOption;
    impl crate::value::ToValue for EvaluationFormSingleSelectQuestionOption_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automatic_fail {
                properties.insert(
                    "AutomaticFail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RefId".to_string(),
                crate::value::ToValue::to_value(&self.ref_id),
            );
            if let Some(ref value) = self.score {
                properties.insert("Score".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Text".to_string(),
                crate::value::ToValue::to_value(&self.text),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-evaluationformsingleselectquestionproperties.html
    pub struct EvaluationFormSingleSelectQuestionProperties_ {
        pub automation: Option<Box<EvaluationFormSingleSelectQuestionAutomation_>>,
        pub display_as: Option<crate::value::ExpString>,
        pub options: Vec<EvaluationFormSingleSelectQuestionOption_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.EvaluationFormSingleSelectQuestionProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_EvaluationFormSingleSelectQuestionProperties as EvaluationFormSingleSelectQuestionProperties;
    impl crate::value::ToValue for EvaluationFormSingleSelectQuestionProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.automation {
                properties.insert(
                    "Automation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.display_as {
                properties.insert(
                    "DisplayAs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Options".to_string(),
                crate::value::ToValue::to_value(&self.options),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-numericquestionpropertyvalueautomation.html
    pub struct NumericQuestionPropertyValueAutomation_ {
        pub label: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_NumericQuestionPropertyValueAutomation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.NumericQuestionPropertyValueAutomation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_NumericQuestionPropertyValueAutomation as NumericQuestionPropertyValueAutomation;
    impl crate::value::ToValue for NumericQuestionPropertyValueAutomation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Label".to_string(),
                crate::value::ToValue::to_value(&self.label),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-scoringstrategy.html
    pub struct ScoringStrategy_ {
        pub mode: crate::value::ExpString,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_ScoringStrategy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.ScoringStrategy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_ScoringStrategy as ScoringStrategy;
    impl crate::value::ToValue for ScoringStrategy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-evaluationform-singleselectquestionrulecategoryautomation.html
    pub struct SingleSelectQuestionRuleCategoryAutomation_ {
        pub category: crate::value::ExpString,
        pub condition: crate::value::ExpString,
        pub option_ref_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_EvaluationForm_SingleSelectQuestionRuleCategoryAutomation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::EvaluationForm.SingleSelectQuestionRuleCategoryAutomation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_EvaluationForm_SingleSelectQuestionRuleCategoryAutomation as SingleSelectQuestionRuleCategoryAutomation;
    impl crate::value::ToValue for SingleSelectQuestionRuleCategoryAutomation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(&self.category),
            );
            properties.insert(
                "Condition".to_string(),
                crate::value::ToValue::to_value(&self.condition),
            );
            properties.insert(
                "OptionRefId".to_string(),
                crate::value::ToValue::to_value(&self.option_ref_id),
            );
            properties.into()
        }
    }
}
pub mod hoursofoperation {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationconfig.html
    pub struct HoursOfOperationConfig_ {
        pub day: crate::value::ExpString,
        pub end_time: Box<HoursOfOperationTimeSlice_>,
        pub start_time: Box<HoursOfOperationTimeSlice_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_HoursOfOperation_HoursOfOperationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::HoursOfOperation.HoursOfOperationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_HoursOfOperation_HoursOfOperationConfig as HoursOfOperationConfig;
    impl crate::value::ToValue for HoursOfOperationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Day".to_string(),
                crate::value::ToValue::to_value(&self.day),
            );
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(&self.end_time),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationoverride.html
    pub struct HoursOfOperationOverride_ {
        pub effective_from: crate::value::ExpString,
        pub effective_till: crate::value::ExpString,
        pub hours_of_operation_override_id: Option<crate::value::ExpString>,
        pub override_config: Vec<HoursOfOperationOverrideConfig_>,
        pub override_description: Option<crate::value::ExpString>,
        pub override_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_HoursOfOperation_HoursOfOperationOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::HoursOfOperation.HoursOfOperationOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_HoursOfOperation_HoursOfOperationOverride as HoursOfOperationOverride;
    impl crate::value::ToValue for HoursOfOperationOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EffectiveFrom".to_string(),
                crate::value::ToValue::to_value(&self.effective_from),
            );
            properties.insert(
                "EffectiveTill".to_string(),
                crate::value::ToValue::to_value(&self.effective_till),
            );
            if let Some(ref value) = self.hours_of_operation_override_id {
                properties.insert(
                    "HoursOfOperationOverrideId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OverrideConfig".to_string(),
                crate::value::ToValue::to_value(&self.override_config),
            );
            if let Some(ref value) = self.override_description {
                properties.insert(
                    "OverrideDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OverrideName".to_string(),
                crate::value::ToValue::to_value(&self.override_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationoverrideconfig.html
    pub struct HoursOfOperationOverrideConfig_ {
        pub day: crate::value::ExpString,
        pub end_time: Box<OverrideTimeSlice_>,
        pub start_time: Box<OverrideTimeSlice_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_HoursOfOperation_HoursOfOperationOverrideConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::HoursOfOperation.HoursOfOperationOverrideConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_HoursOfOperation_HoursOfOperationOverrideConfig as HoursOfOperationOverrideConfig;
    impl crate::value::ToValue for HoursOfOperationOverrideConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Day".to_string(),
                crate::value::ToValue::to_value(&self.day),
            );
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(&self.end_time),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-hoursofoperationtimeslice.html
    pub struct HoursOfOperationTimeSlice_ {
        pub hours: i64,
        pub minutes: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_HoursOfOperation_HoursOfOperationTimeSlice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::HoursOfOperation.HoursOfOperationTimeSlice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_HoursOfOperation_HoursOfOperationTimeSlice as HoursOfOperationTimeSlice;
    impl crate::value::ToValue for HoursOfOperationTimeSlice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Hours".to_string(),
                crate::value::ToValue::to_value(&self.hours),
            );
            properties.insert(
                "Minutes".to_string(),
                crate::value::ToValue::to_value(&self.minutes),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-hoursofoperation-overridetimeslice.html
    pub struct OverrideTimeSlice_ {
        pub hours: i64,
        pub minutes: i64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_HoursOfOperation_OverrideTimeSlice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::HoursOfOperation.OverrideTimeSlice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_HoursOfOperation_OverrideTimeSlice as OverrideTimeSlice;
    impl crate::value::ToValue for OverrideTimeSlice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Hours".to_string(),
                crate::value::ToValue::to_value(&self.hours),
            );
            properties.insert(
                "Minutes".to_string(),
                crate::value::ToValue::to_value(&self.minutes),
            );
            properties.into()
        }
    }
}
pub mod instance {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instance-attributes.html
    pub struct Attributes_ {
        pub auto_resolve_best_voices: Option<crate::value::ExpBool>,
        pub contact_lens: Option<crate::value::ExpBool>,
        pub contactflow_logs: Option<crate::value::ExpBool>,
        pub early_media: Option<crate::value::ExpBool>,
        pub enhanced_chat_monitoring: Option<crate::value::ExpBool>,
        pub enhanced_contact_monitoring: Option<crate::value::ExpBool>,
        pub high_volume_out_bound: Option<crate::value::ExpBool>,
        pub inbound_calls: crate::value::ExpBool,
        pub multi_party_chat_conference: Option<crate::value::ExpBool>,
        pub multi_party_conference: Option<crate::value::ExpBool>,
        pub outbound_calls: crate::value::ExpBool,
        pub use_custom_tts_voices: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Instance_Attributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Instance.Attributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Instance_Attributes as Attributes;
    impl crate::value::ToValue for Attributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_resolve_best_voices {
                properties.insert(
                    "AutoResolveBestVoices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contact_lens {
                properties.insert(
                    "ContactLens".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.contactflow_logs {
                properties.insert(
                    "ContactflowLogs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.early_media {
                properties.insert(
                    "EarlyMedia".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enhanced_chat_monitoring {
                properties.insert(
                    "EnhancedChatMonitoring".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enhanced_contact_monitoring {
                properties.insert(
                    "EnhancedContactMonitoring".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.high_volume_out_bound {
                properties.insert(
                    "HighVolumeOutBound".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InboundCalls".to_string(),
                crate::value::ToValue::to_value(&self.inbound_calls),
            );
            if let Some(ref value) = self.multi_party_chat_conference {
                properties.insert(
                    "MultiPartyChatConference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multi_party_conference {
                properties.insert(
                    "MultiPartyConference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "OutboundCalls".to_string(),
                crate::value::ToValue::to_value(&self.outbound_calls),
            );
            if let Some(ref value) = self.use_custom_tts_voices {
                properties.insert(
                    "UseCustomTTSVoices".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod instancestorageconfig {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-encryptionconfig.html
    pub struct EncryptionConfig_ {
        pub encryption_type: crate::value::ExpString,
        pub key_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_InstanceStorageConfig_EncryptionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::InstanceStorageConfig.EncryptionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_InstanceStorageConfig_EncryptionConfig as EncryptionConfig;
    impl crate::value::ToValue for EncryptionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionType".to_string(),
                crate::value::ToValue::to_value(&self.encryption_type),
            );
            properties.insert(
                "KeyId".to_string(),
                crate::value::ToValue::to_value(&self.key_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisfirehoseconfig.html
    pub struct KinesisFirehoseConfig_ {
        pub firehose_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_InstanceStorageConfig_KinesisFirehoseConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::InstanceStorageConfig.KinesisFirehoseConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_InstanceStorageConfig_KinesisFirehoseConfig as KinesisFirehoseConfig;
    impl crate::value::ToValue for KinesisFirehoseConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FirehoseArn".to_string(),
                crate::value::ToValue::to_value(&self.firehose_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisstreamconfig.html
    pub struct KinesisStreamConfig_ {
        pub stream_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_InstanceStorageConfig_KinesisStreamConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::InstanceStorageConfig.KinesisStreamConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_InstanceStorageConfig_KinesisStreamConfig as KinesisStreamConfig;
    impl crate::value::ToValue for KinesisStreamConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "StreamArn".to_string(),
                crate::value::ToValue::to_value(&self.stream_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-kinesisvideostreamconfig.html
    pub struct KinesisVideoStreamConfig_ {
        pub encryption_config: Box<EncryptionConfig_>,
        pub prefix: crate::value::ExpString,
        pub retention_period_hours: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_InstanceStorageConfig_KinesisVideoStreamConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::InstanceStorageConfig.KinesisVideoStreamConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_InstanceStorageConfig_KinesisVideoStreamConfig as KinesisVideoStreamConfig;
    impl crate::value::ToValue for KinesisVideoStreamConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EncryptionConfig".to_string(),
                crate::value::ToValue::to_value(&self.encryption_config),
            );
            properties.insert(
                "Prefix".to_string(),
                crate::value::ToValue::to_value(&self.prefix),
            );
            properties.insert(
                "RetentionPeriodHours".to_string(),
                crate::value::ToValue::to_value(&self.retention_period_hours),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-instancestorageconfig-s3config.html
    pub struct S3Config_ {
        pub bucket_name: crate::value::ExpString,
        pub bucket_prefix: crate::value::ExpString,
        pub encryption_config: Option<Box<EncryptionConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_InstanceStorageConfig_S3Config {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::InstanceStorageConfig.S3Config"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_InstanceStorageConfig_S3Config as S3Config;
    impl crate::value::ToValue for S3Config_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            properties.insert(
                "BucketPrefix".to_string(),
                crate::value::ToValue::to_value(&self.bucket_prefix),
            );
            if let Some(ref value) = self.encryption_config {
                properties.insert(
                    "EncryptionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod predefinedattribute {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-predefinedattribute-attributeconfiguration.html
    pub struct AttributeConfiguration_ {
        pub enable_value_validation_on_association: Option<crate::value::ExpBool>,
        pub is_read_only: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_PredefinedAttribute_AttributeConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::PredefinedAttribute.AttributeConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_PredefinedAttribute_AttributeConfiguration as AttributeConfiguration;
    impl crate::value::ToValue for AttributeConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_value_validation_on_association {
                properties.insert(
                    "EnableValueValidationOnAssociation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_read_only {
                properties.insert(
                    "IsReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-predefinedattribute-values.html
    pub struct Values_ {
        pub string_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_PredefinedAttribute_Values {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::PredefinedAttribute.Values"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_PredefinedAttribute_Values as Values;
    impl crate::value::ToValue for Values_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.string_list {
                properties.insert(
                    "StringList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod queue {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-queue-outboundcallerconfig.html
    pub struct OutboundCallerConfig_ {
        pub outbound_caller_id_name: Option<crate::value::ExpString>,
        pub outbound_caller_id_number_arn: Option<crate::value::ExpString>,
        pub outbound_flow_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Queue_OutboundCallerConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Queue.OutboundCallerConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Queue_OutboundCallerConfig as OutboundCallerConfig;
    impl crate::value::ToValue for OutboundCallerConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.outbound_caller_id_name {
                properties.insert(
                    "OutboundCallerIdName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outbound_caller_id_number_arn {
                properties.insert(
                    "OutboundCallerIdNumberArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outbound_flow_arn {
                properties.insert(
                    "OutboundFlowArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-queue-outboundemailconfig.html
    pub struct OutboundEmailConfig_ {
        pub outbound_email_address_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Queue_OutboundEmailConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Queue.OutboundEmailConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Queue_OutboundEmailConfig as OutboundEmailConfig;
    impl crate::value::ToValue for OutboundEmailConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.outbound_email_address_id {
                properties.insert(
                    "OutboundEmailAddressId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod quickconnect {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-phonenumberquickconnectconfig.html
    pub struct PhoneNumberQuickConnectConfig_ {
        pub phone_number: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_QuickConnect_PhoneNumberQuickConnectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::QuickConnect.PhoneNumberQuickConnectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_QuickConnect_PhoneNumberQuickConnectConfig as PhoneNumberQuickConnectConfig;
    impl crate::value::ToValue for PhoneNumberQuickConnectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PhoneNumber".to_string(),
                crate::value::ToValue::to_value(&self.phone_number),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-queuequickconnectconfig.html
    pub struct QueueQuickConnectConfig_ {
        pub contact_flow_arn: crate::value::ExpString,
        pub queue_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_QuickConnect_QueueQuickConnectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::QuickConnect.QueueQuickConnectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_QuickConnect_QueueQuickConnectConfig as QueueQuickConnectConfig;
    impl crate::value::ToValue for QueueQuickConnectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContactFlowArn".to_string(),
                crate::value::ToValue::to_value(&self.contact_flow_arn),
            );
            properties.insert(
                "QueueArn".to_string(),
                crate::value::ToValue::to_value(&self.queue_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-quickconnectconfig.html
    pub struct QuickConnectConfig_ {
        pub phone_config: Option<Box<PhoneNumberQuickConnectConfig_>>,
        pub queue_config: Option<Box<QueueQuickConnectConfig_>>,
        pub quick_connect_type: crate::value::ExpString,
        pub user_config: Option<Box<UserQuickConnectConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_QuickConnect_QuickConnectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::QuickConnect.QuickConnectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_QuickConnect_QuickConnectConfig as QuickConnectConfig;
    impl crate::value::ToValue for QuickConnectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.phone_config {
                properties.insert(
                    "PhoneConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.queue_config {
                properties.insert(
                    "QueueConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QuickConnectType".to_string(),
                crate::value::ToValue::to_value(&self.quick_connect_type),
            );
            if let Some(ref value) = self.user_config {
                properties.insert(
                    "UserConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-quickconnect-userquickconnectconfig.html
    pub struct UserQuickConnectConfig_ {
        pub contact_flow_arn: crate::value::ExpString,
        pub user_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_QuickConnect_UserQuickConnectConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::QuickConnect.UserQuickConnectConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_QuickConnect_UserQuickConnectConfig as UserQuickConnectConfig;
    impl crate::value::ToValue for UserQuickConnectConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContactFlowArn".to_string(),
                crate::value::ToValue::to_value(&self.contact_flow_arn),
            );
            properties.insert(
                "UserArn".to_string(),
                crate::value::ToValue::to_value(&self.user_arn),
            );
            properties.into()
        }
    }
}
pub mod routingprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-crosschannelbehavior.html
    pub struct CrossChannelBehavior_ {
        pub behavior_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_RoutingProfile_CrossChannelBehavior {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::RoutingProfile.CrossChannelBehavior"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_RoutingProfile_CrossChannelBehavior as CrossChannelBehavior;
    impl crate::value::ToValue for CrossChannelBehavior_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BehaviorType".to_string(),
                crate::value::ToValue::to_value(&self.behavior_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-mediaconcurrency.html
    pub struct MediaConcurrency_ {
        pub channel: crate::value::ExpString,
        pub concurrency: i64,
        pub cross_channel_behavior: Option<Box<CrossChannelBehavior_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_RoutingProfile_MediaConcurrency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::RoutingProfile.MediaConcurrency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_RoutingProfile_MediaConcurrency as MediaConcurrency;
    impl crate::value::ToValue for MediaConcurrency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Channel".to_string(),
                crate::value::ToValue::to_value(&self.channel),
            );
            properties.insert(
                "Concurrency".to_string(),
                crate::value::ToValue::to_value(&self.concurrency),
            );
            if let Some(ref value) = self.cross_channel_behavior {
                properties.insert(
                    "CrossChannelBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilemanualassignmentqueueconfig.html
    pub struct RoutingProfileManualAssignmentQueueConfig_ {
        pub queue_reference: Box<RoutingProfileQueueReference_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_RoutingProfile_RoutingProfileManualAssignmentQueueConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::RoutingProfile.RoutingProfileManualAssignmentQueueConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_RoutingProfile_RoutingProfileManualAssignmentQueueConfig as RoutingProfileManualAssignmentQueueConfig;
    impl crate::value::ToValue for RoutingProfileManualAssignmentQueueConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "QueueReference".to_string(),
                crate::value::ToValue::to_value(&self.queue_reference),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeueconfig.html
    pub struct RoutingProfileQueueConfig_ {
        pub delay: i64,
        pub priority: i64,
        pub queue_reference: Box<RoutingProfileQueueReference_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_RoutingProfile_RoutingProfileQueueConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::RoutingProfile.RoutingProfileQueueConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_RoutingProfile_RoutingProfileQueueConfig as RoutingProfileQueueConfig;
    impl crate::value::ToValue for RoutingProfileQueueConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Delay".to_string(),
                crate::value::ToValue::to_value(&self.delay),
            );
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "QueueReference".to_string(),
                crate::value::ToValue::to_value(&self.queue_reference),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-routingprofile-routingprofilequeuereference.html
    pub struct RoutingProfileQueueReference_ {
        pub channel: crate::value::ExpString,
        pub queue_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_RoutingProfile_RoutingProfileQueueReference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::RoutingProfile.RoutingProfileQueueReference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_RoutingProfile_RoutingProfileQueueReference as RoutingProfileQueueReference;
    impl crate::value::ToValue for RoutingProfileQueueReference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Channel".to_string(),
                crate::value::ToValue::to_value(&self.channel),
            );
            properties.insert(
                "QueueArn".to_string(),
                crate::value::ToValue::to_value(&self.queue_arn),
            );
            properties.into()
        }
    }
}
pub mod rule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-actions.html
    pub struct Actions_ {
        pub assign_contact_category_actions: Option<Vec<serde_json::Value>>,
        pub create_case_actions: Option<Vec<CreateCaseAction_>>,
        pub end_associated_tasks_actions: Option<Vec<serde_json::Value>>,
        pub event_bridge_actions: Option<Vec<EventBridgeAction_>>,
        pub send_notification_actions: Option<Vec<SendNotificationAction_>>,
        pub submit_auto_evaluation_actions: Option<Vec<SubmitAutoEvaluationAction_>>,
        pub task_actions: Option<Vec<TaskAction_>>,
        pub update_case_actions: Option<Vec<UpdateCaseAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_Actions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.Actions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_Actions as Actions;
    impl crate::value::ToValue for Actions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.assign_contact_category_actions {
                properties.insert(
                    "AssignContactCategoryActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.create_case_actions {
                properties.insert(
                    "CreateCaseActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_associated_tasks_actions {
                properties.insert(
                    "EndAssociatedTasksActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_bridge_actions {
                properties.insert(
                    "EventBridgeActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.send_notification_actions {
                properties.insert(
                    "SendNotificationActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.submit_auto_evaluation_actions {
                properties.insert(
                    "SubmitAutoEvaluationActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.task_actions {
                properties.insert(
                    "TaskActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.update_case_actions {
                properties.insert(
                    "UpdateCaseActions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-createcaseaction.html
    pub struct CreateCaseAction_ {
        pub fields: Vec<Field_>,
        pub template_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_CreateCaseAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.CreateCaseAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_CreateCaseAction as CreateCaseAction;
    impl crate::value::ToValue for CreateCaseAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Fields".to_string(),
                crate::value::ToValue::to_value(&self.fields),
            );
            properties.insert(
                "TemplateId".to_string(),
                crate::value::ToValue::to_value(&self.template_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-eventbridgeaction.html
    pub struct EventBridgeAction_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_EventBridgeAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.EventBridgeAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_EventBridgeAction as EventBridgeAction;
    impl crate::value::ToValue for EventBridgeAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-field.html
    pub struct Field_ {
        pub id: crate::value::ExpString,
        pub value: Box<FieldValue_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_Field {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.Field"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_Field as Field;
    impl crate::value::ToValue for Field_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-fieldvalue.html
    pub struct FieldValue_ {
        pub boolean_value: Option<crate::value::ExpBool>,
        pub double_value: Option<f64>,
        pub empty_value: Option<serde_json::Value>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_FieldValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.FieldValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_FieldValue as FieldValue;
    impl crate::value::ToValue for FieldValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean_value {
                properties.insert(
                    "BooleanValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.double_value {
                properties.insert(
                    "DoubleValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.empty_value {
                properties.insert(
                    "EmptyValue".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-notificationrecipienttype.html
    pub struct NotificationRecipientType_ {
        pub user_arns: Option<Vec<crate::value::ExpString>>,
        pub user_tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_NotificationRecipientType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.NotificationRecipientType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_NotificationRecipientType as NotificationRecipientType;
    impl crate::value::ToValue for NotificationRecipientType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.user_arns {
                properties.insert(
                    "UserArns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_tags {
                properties.insert(
                    "UserTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-reference.html
    pub struct Reference_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_Reference {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.Reference"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_Reference as Reference;
    impl crate::value::ToValue for Reference_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-ruletriggereventsource.html
    pub struct RuleTriggerEventSource_ {
        pub event_source_name: crate::value::ExpString,
        pub integration_association_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_RuleTriggerEventSource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.RuleTriggerEventSource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_RuleTriggerEventSource as RuleTriggerEventSource;
    impl crate::value::ToValue for RuleTriggerEventSource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventSourceName".to_string(),
                crate::value::ToValue::to_value(&self.event_source_name),
            );
            if let Some(ref value) = self.integration_association_arn {
                properties.insert(
                    "IntegrationAssociationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-sendnotificationaction.html
    pub struct SendNotificationAction_ {
        pub content: crate::value::ExpString,
        pub content_type: crate::value::ExpString,
        pub delivery_method: crate::value::ExpString,
        pub recipient: Box<NotificationRecipientType_>,
        pub subject: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_SendNotificationAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.SendNotificationAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_SendNotificationAction as SendNotificationAction;
    impl crate::value::ToValue for SendNotificationAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Content".to_string(),
                crate::value::ToValue::to_value(&self.content),
            );
            properties.insert(
                "ContentType".to_string(),
                crate::value::ToValue::to_value(&self.content_type),
            );
            properties.insert(
                "DeliveryMethod".to_string(),
                crate::value::ToValue::to_value(&self.delivery_method),
            );
            properties.insert(
                "Recipient".to_string(),
                crate::value::ToValue::to_value(&self.recipient),
            );
            if let Some(ref value) = self.subject {
                properties.insert(
                    "Subject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-submitautoevaluationaction.html
    pub struct SubmitAutoEvaluationAction_ {
        pub evaluation_form_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_SubmitAutoEvaluationAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.SubmitAutoEvaluationAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_SubmitAutoEvaluationAction as SubmitAutoEvaluationAction;
    impl crate::value::ToValue for SubmitAutoEvaluationAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EvaluationFormArn".to_string(),
                crate::value::ToValue::to_value(&self.evaluation_form_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-taskaction.html
    pub struct TaskAction_ {
        pub contact_flow_arn: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub references: Option<std::collections::BTreeMap<String, Reference_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_TaskAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.TaskAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_TaskAction as TaskAction;
    impl crate::value::ToValue for TaskAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ContactFlowArn".to_string(),
                crate::value::ToValue::to_value(&self.contact_flow_arn),
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
            if let Some(ref value) = self.references {
                properties.insert(
                    "References".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-rule-updatecaseaction.html
    pub struct UpdateCaseAction_ {
        pub fields: Vec<Field_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_Rule_UpdateCaseAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::Rule.UpdateCaseAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_Rule_UpdateCaseAction as UpdateCaseAction;
    impl crate::value::ToValue for UpdateCaseAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Fields".to_string(),
                crate::value::ToValue::to_value(&self.fields),
            );
            properties.into()
        }
    }
}
pub mod securityprofile {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-securityprofile-application.html
    pub struct Application_ {
        pub application_permissions: Vec<crate::value::ExpString>,
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_SecurityProfile_Application {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::SecurityProfile.Application"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_SecurityProfile_Application as Application;
    impl crate::value::ToValue for Application_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationPermissions".to_string(),
                crate::value::ToValue::to_value(&self.application_permissions),
            );
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
}
pub mod tasktemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-constraints.html
    pub struct Constraints_ {
        pub invisible_fields: Option<Vec<InvisibleFieldInfo_>>,
        pub read_only_fields: Option<Vec<ReadOnlyFieldInfo_>>,
        pub required_fields: Option<Vec<RequiredFieldInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_TaskTemplate_Constraints {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::TaskTemplate.Constraints"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_TaskTemplate_Constraints as Constraints;
    impl crate::value::ToValue for Constraints_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invisible_fields {
                properties.insert(
                    "InvisibleFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_only_fields {
                properties.insert(
                    "ReadOnlyFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.required_fields {
                properties.insert(
                    "RequiredFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-defaultfieldvalue.html
    pub struct DefaultFieldValue_ {
        pub default_value: crate::value::ExpString,
        pub id: Box<FieldIdentifier_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_TaskTemplate_DefaultFieldValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::TaskTemplate.DefaultFieldValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_TaskTemplate_DefaultFieldValue as DefaultFieldValue;
    impl crate::value::ToValue for DefaultFieldValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultValue".to_string(),
                crate::value::ToValue::to_value(&self.default_value),
            );
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-field.html
    pub struct Field_ {
        pub description: Option<crate::value::ExpString>,
        pub id: Box<FieldIdentifier_>,
        pub single_select_options: Option<Vec<crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_TaskTemplate_Field {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::TaskTemplate.Field"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_TaskTemplate_Field as Field;
    impl crate::value::ToValue for Field_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.single_select_options {
                properties.insert(
                    "SingleSelectOptions".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-fieldidentifier.html
    pub struct FieldIdentifier_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_TaskTemplate_FieldIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::TaskTemplate.FieldIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_TaskTemplate_FieldIdentifier as FieldIdentifier;
    impl crate::value::ToValue for FieldIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-invisiblefieldinfo.html
    pub struct InvisibleFieldInfo_ {
        pub id: Box<FieldIdentifier_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_TaskTemplate_InvisibleFieldInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::TaskTemplate.InvisibleFieldInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_TaskTemplate_InvisibleFieldInfo as InvisibleFieldInfo;
    impl crate::value::ToValue for InvisibleFieldInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-readonlyfieldinfo.html
    pub struct ReadOnlyFieldInfo_ {
        pub id: Box<FieldIdentifier_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_TaskTemplate_ReadOnlyFieldInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::TaskTemplate.ReadOnlyFieldInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_TaskTemplate_ReadOnlyFieldInfo as ReadOnlyFieldInfo;
    impl crate::value::ToValue for ReadOnlyFieldInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-tasktemplate-requiredfieldinfo.html
    pub struct RequiredFieldInfo_ {
        pub id: Box<FieldIdentifier_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_TaskTemplate_RequiredFieldInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::TaskTemplate.RequiredFieldInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_TaskTemplate_RequiredFieldInfo as RequiredFieldInfo;
    impl crate::value::ToValue for RequiredFieldInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
}
pub mod user {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-useridentityinfo.html
    pub struct UserIdentityInfo_ {
        pub email: Option<crate::value::ExpString>,
        pub first_name: Option<crate::value::ExpString>,
        pub last_name: Option<crate::value::ExpString>,
        pub mobile: Option<crate::value::ExpString>,
        pub secondary_email: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_User_UserIdentityInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::User.UserIdentityInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_User_UserIdentityInfo as UserIdentityInfo;
    impl crate::value::ToValue for UserIdentityInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email {
                properties.insert("Email".to_string(), crate::value::ToValue::to_value(value));
            }
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
            if let Some(ref value) = self.mobile {
                properties.insert("Mobile".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.secondary_email {
                properties.insert(
                    "SecondaryEmail".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userphoneconfig.html
    pub struct UserPhoneConfig_ {
        pub after_contact_work_time_limit: Option<i64>,
        pub auto_accept: Option<crate::value::ExpBool>,
        pub desk_phone_number: Option<crate::value::ExpString>,
        pub persistent_connection: Option<crate::value::ExpBool>,
        pub phone_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_User_UserPhoneConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::User.UserPhoneConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_User_UserPhoneConfig as UserPhoneConfig;
    impl crate::value::ToValue for UserPhoneConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.after_contact_work_time_limit {
                properties.insert(
                    "AfterContactWorkTimeLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.auto_accept {
                properties.insert(
                    "AutoAccept".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.desk_phone_number {
                properties.insert(
                    "DeskPhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.persistent_connection {
                properties.insert(
                    "PersistentConnection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PhoneType".to_string(),
                crate::value::ToValue::to_value(&self.phone_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-user-userproficiency.html
    pub struct UserProficiency_ {
        pub attribute_name: crate::value::ExpString,
        pub attribute_value: crate::value::ExpString,
        pub level: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_User_UserProficiency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::User.UserProficiency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_User_UserProficiency as UserProficiency;
    impl crate::value::ToValue for UserProficiency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AttributeName".to_string(),
                crate::value::ToValue::to_value(&self.attribute_name),
            );
            properties.insert(
                "AttributeValue".to_string(),
                crate::value::ToValue::to_value(&self.attribute_value),
            );
            properties.insert(
                "Level".to_string(),
                crate::value::ToValue::to_value(&self.level),
            );
            properties.into()
        }
    }
}
pub mod userhierarchystructure {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-userhierarchystructure-levelfive.html
    pub struct LevelFive_ {
        pub hierarchy_level_arn: Option<crate::value::ExpString>,
        pub hierarchy_level_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_UserHierarchyStructure_LevelFive {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::UserHierarchyStructure.LevelFive"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_UserHierarchyStructure_LevelFive as LevelFive;
    impl crate::value::ToValue for LevelFive_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hierarchy_level_arn {
                properties.insert(
                    "HierarchyLevelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchy_level_id {
                properties.insert(
                    "HierarchyLevelId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-userhierarchystructure-levelfour.html
    pub struct LevelFour_ {
        pub hierarchy_level_arn: Option<crate::value::ExpString>,
        pub hierarchy_level_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_UserHierarchyStructure_LevelFour {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::UserHierarchyStructure.LevelFour"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_UserHierarchyStructure_LevelFour as LevelFour;
    impl crate::value::ToValue for LevelFour_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hierarchy_level_arn {
                properties.insert(
                    "HierarchyLevelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchy_level_id {
                properties.insert(
                    "HierarchyLevelId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-userhierarchystructure-levelone.html
    pub struct LevelOne_ {
        pub hierarchy_level_arn: Option<crate::value::ExpString>,
        pub hierarchy_level_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_UserHierarchyStructure_LevelOne {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::UserHierarchyStructure.LevelOne"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_UserHierarchyStructure_LevelOne as LevelOne;
    impl crate::value::ToValue for LevelOne_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hierarchy_level_arn {
                properties.insert(
                    "HierarchyLevelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchy_level_id {
                properties.insert(
                    "HierarchyLevelId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-userhierarchystructure-levelthree.html
    pub struct LevelThree_ {
        pub hierarchy_level_arn: Option<crate::value::ExpString>,
        pub hierarchy_level_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_UserHierarchyStructure_LevelThree {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::UserHierarchyStructure.LevelThree"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_UserHierarchyStructure_LevelThree as LevelThree;
    impl crate::value::ToValue for LevelThree_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hierarchy_level_arn {
                properties.insert(
                    "HierarchyLevelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchy_level_id {
                properties.insert(
                    "HierarchyLevelId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-userhierarchystructure-leveltwo.html
    pub struct LevelTwo_ {
        pub hierarchy_level_arn: Option<crate::value::ExpString>,
        pub hierarchy_level_id: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_UserHierarchyStructure_LevelTwo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::UserHierarchyStructure.LevelTwo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_UserHierarchyStructure_LevelTwo as LevelTwo;
    impl crate::value::ToValue for LevelTwo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hierarchy_level_arn {
                properties.insert(
                    "HierarchyLevelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hierarchy_level_id {
                properties.insert(
                    "HierarchyLevelId".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connect-userhierarchystructure-userhierarchystructure.html
    pub struct UserHierarchyStructure_ {
        pub level_five: Option<Box<LevelFive_>>,
        pub level_four: Option<Box<LevelFour_>>,
        pub level_one: Option<Box<LevelOne_>>,
        pub level_three: Option<Box<LevelThree_>>,
        pub level_two: Option<Box<LevelTwo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connect_UserHierarchyStructure_UserHierarchyStructure {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::Connect::UserHierarchyStructure.UserHierarchyStructure"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connect_UserHierarchyStructure_UserHierarchyStructure as UserHierarchyStructure;
    impl crate::value::ToValue for UserHierarchyStructure_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.level_five {
                properties.insert(
                    "LevelFive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level_four {
                properties.insert(
                    "LevelFour".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level_one {
                properties.insert(
                    "LevelOne".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level_three {
                properties.insert(
                    "LevelThree".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level_two {
                properties.insert(
                    "LevelTwo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-agentstatus.html
pub struct AgentStatus_ {
    pub description: Option<crate::value::ExpString>,
    pub display_order: Option<i64>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub reset_order_number: Option<crate::value::ExpBool>,
    pub state: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_AgentStatus {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::AgentStatus"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_AgentStatus as AgentStatus;
impl crate::template::ToResource for AgentStatus_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AgentStatus"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_order {
            properties.insert(
                "DisplayOrder".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.reset_order_number {
            properties.insert(
                "ResetOrderNumber".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "State".to_string(),
            crate::value::ToValue::to_value(&self.state),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-approvedorigin.html
pub struct ApprovedOrigin_ {
    pub instance_id: crate::value::ExpString,
    pub origin: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_ApprovedOrigin {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::ApprovedOrigin"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_ApprovedOrigin as ApprovedOrigin;
impl crate::template::ToResource for ApprovedOrigin_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApprovedOrigin"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceId".to_string(),
            crate::value::ToValue::to_value(&self.instance_id),
        );
        properties.insert(
            "Origin".to_string(),
            crate::value::ToValue::to_value(&self.origin),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflow.html
pub struct ContactFlow_ {
    pub content: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_ContactFlow {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::ContactFlow"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_ContactFlow as ContactFlow;
impl crate::template::ToResource for ContactFlow_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContactFlow"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowmodule.html
pub struct ContactFlowModule_ {
    pub content: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub state: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_ContactFlowModule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::ContactFlowModule"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_ContactFlowModule as ContactFlowModule;
impl crate::template::ToResource for ContactFlowModule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContactFlowModule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-contactflowversion.html
pub struct ContactFlowVersion_ {
    pub contact_flow_id: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_ContactFlowVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::ContactFlowVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_ContactFlowVersion as ContactFlowVersion;
impl crate::template::ToResource for ContactFlowVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContactFlowVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContactFlowId".to_string(),
            crate::value::ToValue::to_value(&self.contact_flow_id),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-emailaddress.html
pub struct EmailAddress_ {
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub email_address: crate::value::ExpString,
    pub instance_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_EmailAddress {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::EmailAddress"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_EmailAddress as EmailAddress;
impl crate::template::ToResource for EmailAddress_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EmailAddress"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EmailAddress".to_string(),
            crate::value::ToValue::to_value(&self.email_address),
        );
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-evaluationform.html
pub struct EvaluationForm_ {
    pub auto_evaluation_configuration:
        Option<super::connect::evaluationform::AutoEvaluationConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub items: Vec<super::connect::evaluationform::EvaluationFormBaseItem_>,
    pub scoring_strategy: Option<super::connect::evaluationform::ScoringStrategy_>,
    pub status: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub title: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_EvaluationForm {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::EvaluationForm"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_EvaluationForm as EvaluationForm;
impl crate::template::ToResource for EvaluationForm_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EvaluationForm"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.auto_evaluation_configuration {
            properties.insert(
                "AutoEvaluationConfiguration".to_string(),
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
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Items".to_string(),
            crate::value::ToValue::to_value(&self.items),
        );
        if let Some(ref value) = self.scoring_strategy {
            properties.insert(
                "ScoringStrategy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Status".to_string(),
            crate::value::ToValue::to_value(&self.status),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Title".to_string(),
            crate::value::ToValue::to_value(&self.title),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-hoursofoperation.html
pub struct HoursOfOperation_ {
    pub config: Vec<super::connect::hoursofoperation::HoursOfOperationConfig_>,
    pub description: Option<crate::value::ExpString>,
    pub hours_of_operation_overrides:
        Option<Vec<super::connect::hoursofoperation::HoursOfOperationOverride_>>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub time_zone: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_HoursOfOperation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::HoursOfOperation"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_HoursOfOperation as HoursOfOperation;
impl crate::template::ToResource for HoursOfOperation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HoursOfOperation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Config".to_string(),
            crate::value::ToValue::to_value(&self.config),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hours_of_operation_overrides {
            properties.insert(
                "HoursOfOperationOverrides".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TimeZone".to_string(),
            crate::value::ToValue::to_value(&self.time_zone),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instance.html
pub struct Instance_ {
    pub attributes: super::connect::instance::Attributes_,
    pub directory_id: Option<crate::value::ExpString>,
    pub identity_management_type: crate::value::ExpString,
    pub instance_alias: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_Instance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::Instance"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_Instance as Instance;
impl crate::template::ToResource for Instance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Instance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Attributes".to_string(),
            crate::value::ToValue::to_value(&self.attributes),
        );
        if let Some(ref value) = self.directory_id {
            properties.insert(
                "DirectoryId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IdentityManagementType".to_string(),
            crate::value::ToValue::to_value(&self.identity_management_type),
        );
        if let Some(ref value) = self.instance_alias {
            properties.insert(
                "InstanceAlias".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-instancestorageconfig.html
pub struct InstanceStorageConfig_ {
    pub instance_arn: crate::value::ExpString,
    pub kinesis_firehose_config:
        Option<super::connect::instancestorageconfig::KinesisFirehoseConfig_>,
    pub kinesis_stream_config: Option<super::connect::instancestorageconfig::KinesisStreamConfig_>,
    pub kinesis_video_stream_config:
        Option<super::connect::instancestorageconfig::KinesisVideoStreamConfig_>,
    pub resource_type: crate::value::ExpString,
    pub s3_config: Option<super::connect::instancestorageconfig::S3Config_>,
    pub storage_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_InstanceStorageConfig {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::InstanceStorageConfig"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_InstanceStorageConfig as InstanceStorageConfig;
impl crate::template::ToResource for InstanceStorageConfig_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InstanceStorageConfig"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.kinesis_firehose_config {
            properties.insert(
                "KinesisFirehoseConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kinesis_stream_config {
            properties.insert(
                "KinesisStreamConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kinesis_video_stream_config {
            properties.insert(
                "KinesisVideoStreamConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceType".to_string(),
            crate::value::ToValue::to_value(&self.resource_type),
        );
        if let Some(ref value) = self.s3_config {
            properties.insert(
                "S3Config".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StorageType".to_string(),
            crate::value::ToValue::to_value(&self.storage_type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-integrationassociation.html
pub struct IntegrationAssociation_ {
    pub instance_id: crate::value::ExpString,
    pub integration_arn: crate::value::ExpString,
    pub integration_type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_IntegrationAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::IntegrationAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_IntegrationAssociation as IntegrationAssociation;
impl crate::template::ToResource for IntegrationAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IntegrationAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceId".to_string(),
            crate::value::ToValue::to_value(&self.instance_id),
        );
        properties.insert(
            "IntegrationArn".to_string(),
            crate::value::ToValue::to_value(&self.integration_arn),
        );
        properties.insert(
            "IntegrationType".to_string(),
            crate::value::ToValue::to_value(&self.integration_type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-phonenumber.html
pub struct PhoneNumber_ {
    pub country_code: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub prefix: Option<crate::value::ExpString>,
    pub source_phone_number_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arn: crate::value::ExpString,
    pub r#type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_PhoneNumber {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::PhoneNumber"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_PhoneNumber as PhoneNumber;
impl crate::template::ToResource for PhoneNumber_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PhoneNumber"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.country_code {
            properties.insert(
                "CountryCode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.prefix {
            properties.insert("Prefix".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.source_phone_number_arn {
            properties.insert(
                "SourcePhoneNumberArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetArn".to_string(),
            crate::value::ToValue::to_value(&self.target_arn),
        );
        if let Some(ref value) = self.r#type {
            properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-predefinedattribute.html
pub struct PredefinedAttribute_ {
    pub attribute_configuration:
        Option<super::connect::predefinedattribute::AttributeConfiguration_>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub purposes: Option<Vec<crate::value::ExpString>>,
    pub values: Option<super::connect::predefinedattribute::Values_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_PredefinedAttribute {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::PredefinedAttribute"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_PredefinedAttribute as PredefinedAttribute;
impl crate::template::ToResource for PredefinedAttribute_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PredefinedAttribute"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attribute_configuration {
            properties.insert(
                "AttributeConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.purposes {
            properties.insert(
                "Purposes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.values {
            properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-prompt.html
pub struct Prompt_ {
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub s3_uri: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_Prompt {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::Prompt" $($field
        $value)*)
    };
}
pub use crate::__aws_connect_Prompt as Prompt;
impl crate::template::ToResource for Prompt_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Prompt"),
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
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.s3_uri {
            properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-queue.html
pub struct Queue_ {
    pub description: Option<crate::value::ExpString>,
    pub hours_of_operation_arn: crate::value::ExpString,
    pub instance_arn: crate::value::ExpString,
    pub max_contacts: Option<i64>,
    pub name: crate::value::ExpString,
    pub outbound_caller_config: Option<super::connect::queue::OutboundCallerConfig_>,
    pub outbound_email_config: Option<super::connect::queue::OutboundEmailConfig_>,
    pub quick_connect_arns: Option<Vec<crate::value::ExpString>>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_Queue {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::Queue" $($field
        $value)*)
    };
}
pub use crate::__aws_connect_Queue as Queue;
impl crate::template::ToResource for Queue_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Queue"),
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
            "HoursOfOperationArn".to_string(),
            crate::value::ToValue::to_value(&self.hours_of_operation_arn),
        );
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.max_contacts {
            properties.insert(
                "MaxContacts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.outbound_caller_config {
            properties.insert(
                "OutboundCallerConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.outbound_email_config {
            properties.insert(
                "OutboundEmailConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.quick_connect_arns {
            properties.insert(
                "QuickConnectArns".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-quickconnect.html
pub struct QuickConnect_ {
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub quick_connect_config: super::connect::quickconnect::QuickConnectConfig_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_QuickConnect {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::QuickConnect"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_QuickConnect as QuickConnect;
impl crate::template::ToResource for QuickConnect_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QuickConnect"),
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
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "QuickConnectConfig".to_string(),
            crate::value::ToValue::to_value(&self.quick_connect_config),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-routingprofile.html
pub struct RoutingProfile_ {
    pub agent_availability_timer: Option<crate::value::ExpString>,
    pub default_outbound_queue_arn: crate::value::ExpString,
    pub description: crate::value::ExpString,
    pub instance_arn: crate::value::ExpString,
    pub manual_assignment_queue_configs:
        Option<Vec<super::connect::routingprofile::RoutingProfileManualAssignmentQueueConfig_>>,
    pub media_concurrencies: Vec<super::connect::routingprofile::MediaConcurrency_>,
    pub name: crate::value::ExpString,
    pub queue_configs: Option<Vec<super::connect::routingprofile::RoutingProfileQueueConfig_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_RoutingProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::RoutingProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_RoutingProfile as RoutingProfile;
impl crate::template::ToResource for RoutingProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RoutingProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.agent_availability_timer {
            properties.insert(
                "AgentAvailabilityTimer".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultOutboundQueueArn".to_string(),
            crate::value::ToValue::to_value(&self.default_outbound_queue_arn),
        );
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.manual_assignment_queue_configs {
            properties.insert(
                "ManualAssignmentQueueConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MediaConcurrencies".to_string(),
            crate::value::ToValue::to_value(&self.media_concurrencies),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.queue_configs {
            properties.insert(
                "QueueConfigs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-rule.html
pub struct Rule_ {
    pub actions: super::connect::rule::Actions_,
    pub function: crate::value::ExpString,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub publish_status: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trigger_event_source: super::connect::rule::RuleTriggerEventSource_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_Rule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::Rule" $($field
        $value)*)
    };
}
pub use crate::__aws_connect_Rule as Rule;
impl crate::template::ToResource for Rule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Rule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        properties.insert(
            "Function".to_string(),
            crate::value::ToValue::to_value(&self.function),
        );
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "PublishStatus".to_string(),
            crate::value::ToValue::to_value(&self.publish_status),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TriggerEventSource".to_string(),
            crate::value::ToValue::to_value(&self.trigger_event_source),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securitykey.html
pub struct SecurityKey_ {
    pub instance_id: crate::value::ExpString,
    pub key: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_SecurityKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::SecurityKey"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_SecurityKey as SecurityKey;
impl crate::template::ToResource for SecurityKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceId".to_string(),
            crate::value::ToValue::to_value(&self.instance_id),
        );
        properties.insert(
            "Key".to_string(),
            crate::value::ToValue::to_value(&self.key),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-securityprofile.html
pub struct SecurityProfile_ {
    pub allowed_access_control_hierarchy_group_id: Option<crate::value::ExpString>,
    pub allowed_access_control_tags: Option<Vec<crate::Tag_>>,
    pub applications: Option<Vec<super::connect::securityprofile::Application_>>,
    pub description: Option<crate::value::ExpString>,
    pub hierarchy_restricted_resources: Option<Vec<crate::value::ExpString>>,
    pub instance_arn: crate::value::ExpString,
    pub permissions: Option<Vec<crate::value::ExpString>>,
    pub security_profile_name: crate::value::ExpString,
    pub tag_restricted_resources: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_SecurityProfile {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::SecurityProfile"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_SecurityProfile as SecurityProfile;
impl crate::template::ToResource for SecurityProfile_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SecurityProfile"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allowed_access_control_hierarchy_group_id {
            properties.insert(
                "AllowedAccessControlHierarchyGroupId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.allowed_access_control_tags {
            properties.insert(
                "AllowedAccessControlTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.applications {
            properties.insert(
                "Applications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hierarchy_restricted_resources {
            properties.insert(
                "HierarchyRestrictedResources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.permissions {
            properties.insert(
                "Permissions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecurityProfileName".to_string(),
            crate::value::ToValue::to_value(&self.security_profile_name),
        );
        if let Some(ref value) = self.tag_restricted_resources {
            properties.insert(
                "TagRestrictedResources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-tasktemplate.html
pub struct TaskTemplate_ {
    pub client_token: Option<crate::value::ExpString>,
    pub constraints: Option<super::connect::tasktemplate::Constraints_>,
    pub contact_flow_arn: Option<crate::value::ExpString>,
    pub defaults: Option<Vec<super::connect::tasktemplate::DefaultFieldValue_>>,
    pub description: Option<crate::value::ExpString>,
    pub fields: Option<Vec<super::connect::tasktemplate::Field_>>,
    pub instance_arn: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub self_assign_contact_flow_arn: Option<crate::value::ExpString>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_TaskTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::TaskTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_TaskTemplate as TaskTemplate;
impl crate::template::ToResource for TaskTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TaskTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.client_token {
            properties.insert(
                "ClientToken".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.constraints {
            properties.insert(
                "Constraints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.contact_flow_arn {
            properties.insert(
                "ContactFlowArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.defaults {
            properties.insert(
                "Defaults".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fields {
            properties.insert("Fields".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.self_assign_contact_flow_arn {
            properties.insert(
                "SelfAssignContactFlowArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-trafficdistributiongroup.html
pub struct TrafficDistributionGroup_ {
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_TrafficDistributionGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::TrafficDistributionGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_TrafficDistributionGroup as TrafficDistributionGroup;
impl crate::template::ToResource for TrafficDistributionGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrafficDistributionGroup"),
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
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-user.html
pub struct User_ {
    pub directory_user_id: Option<crate::value::ExpString>,
    pub hierarchy_group_arn: Option<crate::value::ExpString>,
    pub identity_info: Option<super::connect::user::UserIdentityInfo_>,
    pub instance_arn: crate::value::ExpString,
    pub password: Option<crate::value::ExpString>,
    pub phone_config: super::connect::user::UserPhoneConfig_,
    pub routing_profile_arn: crate::value::ExpString,
    pub security_profile_arns: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_proficiencies: Option<Vec<super::connect::user::UserProficiency_>>,
    pub username: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_User {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::User" $($field
        $value)*)
    };
}
pub use crate::__aws_connect_User as User;
impl crate::template::ToResource for User_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("User"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.directory_user_id {
            properties.insert(
                "DirectoryUserId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hierarchy_group_arn {
            properties.insert(
                "HierarchyGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.identity_info {
            properties.insert(
                "IdentityInfo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.password {
            properties.insert(
                "Password".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PhoneConfig".to_string(),
            crate::value::ToValue::to_value(&self.phone_config),
        );
        properties.insert(
            "RoutingProfileArn".to_string(),
            crate::value::ToValue::to_value(&self.routing_profile_arn),
        );
        properties.insert(
            "SecurityProfileArns".to_string(),
            crate::value::ToValue::to_value(&self.security_profile_arns),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_proficiencies {
            properties.insert(
                "UserProficiencies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Username".to_string(),
            crate::value::ToValue::to_value(&self.username),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchygroup.html
pub struct UserHierarchyGroup_ {
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub parent_group_arn: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_UserHierarchyGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::UserHierarchyGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_UserHierarchyGroup as UserHierarchyGroup;
impl crate::template::ToResource for UserHierarchyGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserHierarchyGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.parent_group_arn {
            properties.insert(
                "ParentGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-userhierarchystructure.html
pub struct UserHierarchyStructure_ {
    pub instance_arn: crate::value::ExpString,
    pub user_hierarchy_structure:
        Option<super::connect::userhierarchystructure::UserHierarchyStructure_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_UserHierarchyStructure {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::UserHierarchyStructure"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_UserHierarchyStructure as UserHierarchyStructure;
impl crate::template::ToResource for UserHierarchyStructure_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("UserHierarchyStructure"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        if let Some(ref value) = self.user_hierarchy_structure {
            properties.insert(
                "UserHierarchyStructure".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-view.html
pub struct View_ {
    pub actions: Vec<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub instance_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub template: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_View {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::View" $($field
        $value)*)
    };
}
pub use crate::__aws_connect_View as View;
impl crate::template::ToResource for View_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("View"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Actions".to_string(),
            crate::value::ToValue::to_value(&self.actions),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceArn".to_string(),
            crate::value::ToValue::to_value(&self.instance_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Template".to_string(),
            crate::value::ToValue::to_value(&self.template),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connect-viewversion.html
pub struct ViewVersion_ {
    pub version_description: Option<crate::value::ExpString>,
    pub view_arn: crate::value::ExpString,
    pub view_content_sha256: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connect_ViewVersion {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::Connect::ViewVersion"
        $($field $value)*)
    };
}
pub use crate::__aws_connect_ViewVersion as ViewVersion;
impl crate::template::ToResource for ViewVersion_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Connect"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ViewVersion"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.version_description {
            properties.insert(
                "VersionDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ViewArn".to_string(),
            crate::value::ToValue::to_value(&self.view_arn),
        );
        if let Some(ref value) = self.view_content_sha256 {
            properties.insert(
                "ViewContentSha256".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
