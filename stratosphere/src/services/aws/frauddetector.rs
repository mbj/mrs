pub mod detector {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-entitytype.html
    pub struct EntityType_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub inline: Option<crate::value::ExpBool>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_Detector_EntityType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::Detector.EntityType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_Detector_EntityType as EntityType;
    impl crate::value::ToValue for EntityType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventtype.html
    pub struct EventType_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub entity_types: Option<Vec<EntityType_>>,
        pub event_variables: Option<Vec<EventVariable_>>,
        pub inline: Option<crate::value::ExpBool>,
        pub labels: Option<Vec<Label_>>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_Detector_EventType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::Detector.EventType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_Detector_EventType as EventType;
    impl crate::value::ToValue for EventType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.entity_types {
                properties.insert(
                    "EntityTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_variables {
                properties.insert(
                    "EventVariables".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.labels {
                properties.insert("Labels".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-eventvariable.html
    pub struct EventVariable_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub data_source: Option<crate::value::ExpString>,
        pub data_type: Option<crate::value::ExpString>,
        pub default_value: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub inline: Option<crate::value::ExpBool>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
        pub variable_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_Detector_EventVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::Detector.EventVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_Detector_EventVariable as EventVariable;
    impl crate::value::ToValue for EventVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_source {
                properties.insert(
                    "DataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_type {
                properties.insert(
                    "DataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.variable_type {
                properties.insert(
                    "VariableType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-label.html
    pub struct Label_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub inline: Option<crate::value::ExpBool>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_Detector_Label {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::Detector.Label"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_Detector_Label as Label;
    impl crate::value::ToValue for Label_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-model.html
    pub struct Model_ {
        pub arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_Detector_Model {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::Detector.Model"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_Detector_Model as Model;
    impl crate::value::ToValue for Model_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-outcome.html
    pub struct Outcome_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub inline: Option<crate::value::ExpBool>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_Detector_Outcome {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::Detector.Outcome"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_Detector_Outcome as Outcome;
    impl crate::value::ToValue for Outcome_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-detector-rule.html
    pub struct Rule_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub detector_id: Option<crate::value::ExpString>,
        pub expression: Option<crate::value::ExpString>,
        pub language: Option<crate::value::ExpString>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub outcomes: Option<Vec<Outcome_>>,
        pub rule_id: Option<crate::value::ExpString>,
        pub rule_version: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_Detector_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::Detector.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_Detector_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.detector_id {
                properties.insert(
                    "DetectorId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.language {
                properties.insert(
                    "Language".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outcomes {
                properties.insert(
                    "Outcomes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_id {
                properties.insert("RuleId".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.rule_version {
                properties.insert(
                    "RuleVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod eventtype {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-entitytype.html
    pub struct EntityType_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub inline: Option<crate::value::ExpBool>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_EventType_EntityType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::EventType.EntityType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_EventType_EntityType as EntityType;
    impl crate::value::ToValue for EntityType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-eventvariable.html
    pub struct EventVariable_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub data_source: Option<crate::value::ExpString>,
        pub data_type: Option<crate::value::ExpString>,
        pub default_value: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub inline: Option<crate::value::ExpBool>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
        pub variable_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_EventType_EventVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::EventType.EventVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_EventType_EventVariable as EventVariable;
    impl crate::value::ToValue for EventVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_source {
                properties.insert(
                    "DataSource".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_type {
                properties.insert(
                    "DataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.variable_type {
                properties.insert(
                    "VariableType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-frauddetector-eventtype-label.html
    pub struct Label_ {
        pub arn: Option<crate::value::ExpString>,
        pub created_time: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub inline: Option<crate::value::ExpBool>,
        pub last_updated_time: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_frauddetector_EventType_Label {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::FraudDetector::EventType.Label"
            $($field $value)*)
        };
    }
    pub use crate::__aws_frauddetector_EventType_Label as Label;
    impl crate::value::ToValue for Label_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.created_time {
                properties.insert(
                    "CreatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inline {
                properties.insert("Inline".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.last_updated_time {
                properties.insert(
                    "LastUpdatedTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-detector.html
pub struct Detector_ {
    pub associated_models: Option<Vec<super::frauddetector::detector::Model_>>,
    pub description: Option<crate::value::ExpString>,
    pub detector_id: crate::value::ExpString,
    pub detector_version_status: Option<crate::value::ExpString>,
    pub event_type: super::frauddetector::detector::EventType_,
    pub rule_execution_mode: Option<crate::value::ExpString>,
    pub rules: Vec<super::frauddetector::detector::Rule_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_frauddetector_Detector {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FraudDetector::Detector"
        $($field $value)*)
    };
}
pub use crate::__aws_frauddetector_Detector as Detector;
impl crate::template::ToResource for Detector_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FraudDetector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Detector"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.associated_models {
            properties.insert(
                "AssociatedModels".to_string(),
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
            "DetectorId".to_string(),
            crate::value::ToValue::to_value(&self.detector_id),
        );
        if let Some(ref value) = self.detector_version_status {
            properties.insert(
                "DetectorVersionStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventType".to_string(),
            crate::value::ToValue::to_value(&self.event_type),
        );
        if let Some(ref value) = self.rule_execution_mode {
            properties.insert(
                "RuleExecutionMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Rules".to_string(),
            crate::value::ToValue::to_value(&self.rules),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-entitytype.html
pub struct EntityType_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_frauddetector_EntityType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FraudDetector::EntityType"
        $($field $value)*)
    };
}
pub use crate::__aws_frauddetector_EntityType as EntityType;
impl crate::template::ToResource for EntityType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FraudDetector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EntityType"),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-eventtype.html
pub struct EventType_ {
    pub description: Option<crate::value::ExpString>,
    pub entity_types: Vec<super::frauddetector::eventtype::EntityType_>,
    pub event_variables: Vec<super::frauddetector::eventtype::EventVariable_>,
    pub labels: Vec<super::frauddetector::eventtype::Label_>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_frauddetector_EventType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FraudDetector::EventType"
        $($field $value)*)
    };
}
pub use crate::__aws_frauddetector_EventType as EventType;
impl crate::template::ToResource for EventType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FraudDetector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventType"),
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
            "EntityTypes".to_string(),
            crate::value::ToValue::to_value(&self.entity_types),
        );
        properties.insert(
            "EventVariables".to_string(),
            crate::value::ToValue::to_value(&self.event_variables),
        );
        properties.insert(
            "Labels".to_string(),
            crate::value::ToValue::to_value(&self.labels),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-label.html
pub struct Label_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_frauddetector_Label {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FraudDetector::Label"
        $($field $value)*)
    };
}
pub use crate::__aws_frauddetector_Label as Label;
impl crate::template::ToResource for Label_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FraudDetector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Label"),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-list.html
pub struct List_ {
    pub description: Option<crate::value::ExpString>,
    pub elements: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub variable_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_frauddetector_List {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FraudDetector::List"
        $($field $value)*)
    };
}
pub use crate::__aws_frauddetector_List as List;
impl crate::template::ToResource for List_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FraudDetector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("List"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elements {
            properties.insert(
                "Elements".to_string(),
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
        if let Some(ref value) = self.variable_type {
            properties.insert(
                "VariableType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-outcome.html
pub struct Outcome_ {
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_frauddetector_Outcome {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FraudDetector::Outcome"
        $($field $value)*)
    };
}
pub use crate::__aws_frauddetector_Outcome as Outcome;
impl crate::template::ToResource for Outcome_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FraudDetector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Outcome"),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-frauddetector-variable.html
pub struct Variable_ {
    pub data_source: crate::value::ExpString,
    pub data_type: crate::value::ExpString,
    pub default_value: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub variable_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_frauddetector_Variable {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::FraudDetector::Variable"
        $($field $value)*)
    };
}
pub use crate::__aws_frauddetector_Variable as Variable;
impl crate::template::ToResource for Variable_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("FraudDetector"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Variable"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DataSource".to_string(),
            crate::value::ToValue::to_value(&self.data_source),
        );
        properties.insert(
            "DataType".to_string(),
            crate::value::ToValue::to_value(&self.data_type),
        );
        properties.insert(
            "DefaultValue".to_string(),
            crate::value::ToValue::to_value(&self.default_value),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.variable_type {
            properties.insert(
                "VariableType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
