pub mod component {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-actionparameters.html
    pub struct ActionParameters_ {
        pub anchor: Option<Box<ComponentProperty_>>,
        pub fields: Option<std::collections::BTreeMap<String, ComponentProperty_>>,
        pub global: Option<Box<ComponentProperty_>>,
        pub id: Option<Box<ComponentProperty_>>,
        pub model: Option<crate::value::ExpString>,
        pub state: Option<Box<MutationActionSetStateParameter_>>,
        pub target: Option<Box<ComponentProperty_>>,
        pub r#type: Option<Box<ComponentProperty_>>,
        pub url: Option<Box<ComponentProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ActionParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ActionParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ActionParameters as ActionParameters;
    impl crate::value::ToValue for ActionParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.anchor {
                properties.insert("Anchor".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.fields {
                properties.insert("Fields".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.global {
                properties.insert("Global".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.model {
                properties.insert("Model".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.target {
                properties.insert("Target".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalue.html
    pub struct ComponentBindingPropertiesValue_ {
        pub binding_properties: Option<Box<ComponentBindingPropertiesValueProperties_>>,
        pub default_value: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentBindingPropertiesValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentBindingPropertiesValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentBindingPropertiesValue as ComponentBindingPropertiesValue;
    impl crate::value::ToValue for ComponentBindingPropertiesValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.binding_properties {
                properties.insert(
                    "BindingProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentbindingpropertiesvalueproperties.html
    pub struct ComponentBindingPropertiesValueProperties_ {
        pub bucket: Option<crate::value::ExpString>,
        pub default_value: Option<crate::value::ExpString>,
        pub field: Option<crate::value::ExpString>,
        pub key: Option<crate::value::ExpString>,
        pub model: Option<crate::value::ExpString>,
        pub predicates: Option<Vec<Predicate_>>,
        pub slot_name: Option<crate::value::ExpString>,
        pub user_attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentBindingPropertiesValueProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentBindingPropertiesValueProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentBindingPropertiesValueProperties as ComponentBindingPropertiesValueProperties;
    impl crate::value::ToValue for ComponentBindingPropertiesValueProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket {
                properties.insert("Bucket".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.model {
                properties.insert("Model".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.predicates {
                properties.insert(
                    "Predicates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.slot_name {
                properties.insert(
                    "SlotName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_attribute {
                properties.insert(
                    "UserAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentchild.html
    pub struct ComponentChild_ {
        pub children: Option<Vec<ComponentChild_>>,
        pub component_type: crate::value::ExpString,
        pub events: Option<std::collections::BTreeMap<String, ComponentEvent_>>,
        pub name: crate::value::ExpString,
        pub properties: std::collections::BTreeMap<String, ComponentProperty_>,
        pub source_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentChild {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentChild"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentChild as ComponentChild;
    impl crate::value::ToValue for ComponentChild_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.children {
                properties.insert(
                    "Children".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ComponentType".to_string(),
                crate::value::ToValue::to_value(&self.component_type),
            );
            if let Some(ref value) = self.events {
                properties.insert("Events".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Properties".to_string(),
                crate::value::ToValue::to_value(&self.properties),
            );
            if let Some(ref value) = self.source_id {
                properties.insert(
                    "SourceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentconditionproperty.html
    pub struct ComponentConditionProperty_ {
        pub r#else: Option<Box<ComponentProperty_>>,
        pub field: Option<crate::value::ExpString>,
        pub operand: Option<crate::value::ExpString>,
        pub operand_type: Option<crate::value::ExpString>,
        pub operator: Option<crate::value::ExpString>,
        pub property: Option<crate::value::ExpString>,
        pub then: Option<Box<ComponentProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentConditionProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentConditionProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentConditionProperty as ComponentConditionProperty;
    impl crate::value::ToValue for ComponentConditionProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.r#else {
                properties.insert("Else".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.operand {
                properties.insert(
                    "Operand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operand_type {
                properties.insert(
                    "OperandType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operator {
                properties.insert(
                    "Operator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property {
                properties.insert(
                    "Property".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.then {
                properties.insert("Then".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentdataconfiguration.html
    pub struct ComponentDataConfiguration_ {
        pub identifiers: Option<Vec<crate::value::ExpString>>,
        pub model: crate::value::ExpString,
        pub predicate: Option<Box<Predicate_>>,
        pub sort: Option<Vec<SortProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentDataConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentDataConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentDataConfiguration as ComponentDataConfiguration;
    impl crate::value::ToValue for ComponentDataConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.identifiers {
                properties.insert(
                    "Identifiers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Model".to_string(),
                crate::value::ToValue::to_value(&self.model),
            );
            if let Some(ref value) = self.predicate {
                properties.insert(
                    "Predicate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sort {
                properties.insert("Sort".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentevent.html
    pub struct ComponentEvent_ {
        pub action: Option<crate::value::ExpString>,
        pub binding_event: Option<crate::value::ExpString>,
        pub parameters: Option<Box<ActionParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentEvent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentEvent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentEvent as ComponentEvent;
    impl crate::value::ToValue for ComponentEvent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.binding_event {
                properties.insert(
                    "BindingEvent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameters {
                properties.insert(
                    "Parameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentproperty.html
    pub struct ComponentProperty_ {
        pub binding_properties: Option<Box<ComponentPropertyBindingProperties_>>,
        pub bindings: Option<std::collections::BTreeMap<String, FormBindingElement_>>,
        pub collection_binding_properties: Option<Box<ComponentPropertyBindingProperties_>>,
        pub component_name: Option<crate::value::ExpString>,
        pub concat: Option<Vec<ComponentProperty_>>,
        pub condition: Option<Box<ComponentConditionProperty_>>,
        pub configured: Option<crate::value::ExpBool>,
        pub default_value: Option<crate::value::ExpString>,
        pub event: Option<crate::value::ExpString>,
        pub imported_value: Option<crate::value::ExpString>,
        pub model: Option<crate::value::ExpString>,
        pub property: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
        pub user_attribute: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentProperty as ComponentProperty;
    impl crate::value::ToValue for ComponentProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.binding_properties {
                properties.insert(
                    "BindingProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bindings {
                properties.insert(
                    "Bindings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.collection_binding_properties {
                properties.insert(
                    "CollectionBindingProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_name {
                properties.insert(
                    "ComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.concat {
                properties.insert("Concat".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.condition {
                properties.insert(
                    "Condition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.configured {
                properties.insert(
                    "Configured".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event {
                properties.insert("Event".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.imported_value {
                properties.insert(
                    "ImportedValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.model {
                properties.insert("Model".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.property {
                properties.insert(
                    "Property".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.user_attribute {
                properties.insert(
                    "UserAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentpropertybindingproperties.html
    pub struct ComponentPropertyBindingProperties_ {
        pub field: Option<crate::value::ExpString>,
        pub property: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentPropertyBindingProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentPropertyBindingProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentPropertyBindingProperties as ComponentPropertyBindingProperties;
    impl crate::value::ToValue for ComponentPropertyBindingProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Property".to_string(),
                crate::value::ToValue::to_value(&self.property),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-componentvariant.html
    pub struct ComponentVariant_ {
        pub overrides: Option<serde_json::Value>,
        pub variant_values: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_ComponentVariant {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.ComponentVariant"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_ComponentVariant as ComponentVariant;
    impl crate::value::ToValue for ComponentVariant_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.overrides {
                properties.insert(
                    "Overrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.variant_values {
                properties.insert(
                    "VariantValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-formbindingelement.html
    pub struct FormBindingElement_ {
        pub element: crate::value::ExpString,
        pub property: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_FormBindingElement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.FormBindingElement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_FormBindingElement as FormBindingElement;
    impl crate::value::ToValue for FormBindingElement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Element".to_string(),
                crate::value::ToValue::to_value(&self.element),
            );
            properties.insert(
                "Property".to_string(),
                crate::value::ToValue::to_value(&self.property),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-mutationactionsetstateparameter.html
    pub struct MutationActionSetStateParameter_ {
        pub component_name: crate::value::ExpString,
        pub property: crate::value::ExpString,
        pub set: Box<ComponentProperty_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_MutationActionSetStateParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.MutationActionSetStateParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_MutationActionSetStateParameter as MutationActionSetStateParameter;
    impl crate::value::ToValue for MutationActionSetStateParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComponentName".to_string(),
                crate::value::ToValue::to_value(&self.component_name),
            );
            properties.insert(
                "Property".to_string(),
                crate::value::ToValue::to_value(&self.property),
            );
            properties.insert(
                "Set".to_string(),
                crate::value::ToValue::to_value(&self.set),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-predicate.html
    pub struct Predicate_ {
        pub and: Option<Vec<Predicate_>>,
        pub field: Option<crate::value::ExpString>,
        pub operand: Option<crate::value::ExpString>,
        pub operand_type: Option<crate::value::ExpString>,
        pub operator: Option<crate::value::ExpString>,
        pub or: Option<Vec<Predicate_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_Predicate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.Predicate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_Predicate as Predicate;
    impl crate::value::ToValue for Predicate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and {
                properties.insert("And".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.operand {
                properties.insert(
                    "Operand".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operand_type {
                properties.insert(
                    "OperandType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.operator {
                properties.insert(
                    "Operator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.or {
                properties.insert("Or".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-component-sortproperty.html
    pub struct SortProperty_ {
        pub direction: crate::value::ExpString,
        pub field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Component_SortProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Component.SortProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Component_SortProperty as SortProperty;
    impl crate::value::ToValue for SortProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Direction".to_string(),
                crate::value::ToValue::to_value(&self.direction),
            );
            properties.insert(
                "Field".to_string(),
                crate::value::ToValue::to_value(&self.field),
            );
            properties.into()
        }
    }
}
pub mod form {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldconfig.html
    pub struct FieldConfig_ {
        pub excluded: Option<crate::value::ExpBool>,
        pub input_type: Option<Box<FieldInputConfig_>>,
        pub label: Option<crate::value::ExpString>,
        pub position: Option<Box<FieldPosition_>>,
        pub validations: Option<Vec<FieldValidationConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FieldConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FieldConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FieldConfig as FieldConfig;
    impl crate::value::ToValue for FieldConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excluded {
                properties.insert(
                    "Excluded".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.input_type {
                properties.insert(
                    "InputType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label {
                properties.insert("Label".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.position {
                properties.insert(
                    "Position".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.validations {
                properties.insert(
                    "Validations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldinputconfig.html
    pub struct FieldInputConfig_ {
        pub default_checked: Option<crate::value::ExpBool>,
        pub default_country_code: Option<crate::value::ExpString>,
        pub default_value: Option<crate::value::ExpString>,
        pub descriptive_text: Option<crate::value::ExpString>,
        pub file_uploader_config: Option<Box<FileUploaderFieldConfig_>>,
        pub is_array: Option<crate::value::ExpBool>,
        pub max_value: Option<f64>,
        pub min_value: Option<f64>,
        pub name: Option<crate::value::ExpString>,
        pub placeholder: Option<crate::value::ExpString>,
        pub read_only: Option<crate::value::ExpBool>,
        pub required: Option<crate::value::ExpBool>,
        pub step: Option<f64>,
        pub r#type: crate::value::ExpString,
        pub value: Option<crate::value::ExpString>,
        pub value_mappings: Option<Box<ValueMappings_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FieldInputConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FieldInputConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FieldInputConfig as FieldInputConfig;
    impl crate::value::ToValue for FieldInputConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_checked {
                properties.insert(
                    "DefaultChecked".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_country_code {
                properties.insert(
                    "DefaultCountryCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_value {
                properties.insert(
                    "DefaultValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.descriptive_text {
                properties.insert(
                    "DescriptiveText".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.file_uploader_config {
                properties.insert(
                    "FileUploaderConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_array {
                properties.insert(
                    "IsArray".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_value {
                properties.insert(
                    "MaxValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_value {
                properties.insert(
                    "MinValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.placeholder {
                properties.insert(
                    "Placeholder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_only {
                properties.insert(
                    "ReadOnly".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.required {
                properties.insert(
                    "Required".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.step {
                properties.insert("Step".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value_mappings {
                properties.insert(
                    "ValueMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldposition.html
    pub struct FieldPosition_ {
        pub below: Option<crate::value::ExpString>,
        pub fixed: Option<crate::value::ExpString>,
        pub right_of: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FieldPosition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FieldPosition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FieldPosition as FieldPosition;
    impl crate::value::ToValue for FieldPosition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.below {
                properties.insert("Below".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.fixed {
                properties.insert("Fixed".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.right_of {
                properties.insert(
                    "RightOf".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fieldvalidationconfiguration.html
    pub struct FieldValidationConfiguration_ {
        pub num_values: Option<Vec<f64>>,
        pub str_values: Option<Vec<crate::value::ExpString>>,
        pub r#type: crate::value::ExpString,
        pub validation_message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FieldValidationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FieldValidationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FieldValidationConfiguration as FieldValidationConfiguration;
    impl crate::value::ToValue for FieldValidationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.num_values {
                properties.insert(
                    "NumValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.str_values {
                properties.insert(
                    "StrValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.validation_message {
                properties.insert(
                    "ValidationMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-fileuploaderfieldconfig.html
    pub struct FileUploaderFieldConfig_ {
        pub accepted_file_types: Vec<crate::value::ExpString>,
        pub access_level: crate::value::ExpString,
        pub is_resumable: Option<crate::value::ExpBool>,
        pub max_file_count: Option<f64>,
        pub max_size: Option<f64>,
        pub show_thumbnails: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FileUploaderFieldConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FileUploaderFieldConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FileUploaderFieldConfig as FileUploaderFieldConfig;
    impl crate::value::ToValue for FileUploaderFieldConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AcceptedFileTypes".to_string(),
                crate::value::ToValue::to_value(&self.accepted_file_types),
            );
            properties.insert(
                "AccessLevel".to_string(),
                crate::value::ToValue::to_value(&self.access_level),
            );
            if let Some(ref value) = self.is_resumable {
                properties.insert(
                    "IsResumable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_file_count {
                properties.insert(
                    "MaxFileCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_size {
                properties.insert(
                    "MaxSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.show_thumbnails {
                properties.insert(
                    "ShowThumbnails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formbutton.html
    pub struct FormButton_ {
        pub children: Option<crate::value::ExpString>,
        pub excluded: Option<crate::value::ExpBool>,
        pub position: Option<Box<FieldPosition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormButton {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormButton"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormButton as FormButton;
    impl crate::value::ToValue for FormButton_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.children {
                properties.insert(
                    "Children".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.excluded {
                properties.insert(
                    "Excluded".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.position {
                properties.insert(
                    "Position".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formcta.html
    pub struct FormCTA_ {
        pub cancel: Option<Box<FormButton_>>,
        pub clear: Option<Box<FormButton_>>,
        pub position: Option<crate::value::ExpString>,
        pub submit: Option<Box<FormButton_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormCTA {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormCTA"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormCTA as FormCTA;
    impl crate::value::ToValue for FormCTA_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cancel {
                properties.insert("Cancel".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.clear {
                properties.insert("Clear".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.position {
                properties.insert(
                    "Position".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.submit {
                properties.insert("Submit".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formdatatypeconfig.html
    pub struct FormDataTypeConfig_ {
        pub data_source_type: crate::value::ExpString,
        pub data_type_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormDataTypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormDataTypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormDataTypeConfig as FormDataTypeConfig;
    impl crate::value::ToValue for FormDataTypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataSourceType".to_string(),
                crate::value::ToValue::to_value(&self.data_source_type),
            );
            properties.insert(
                "DataTypeName".to_string(),
                crate::value::ToValue::to_value(&self.data_type_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-forminputbindingpropertiesvalue.html
    pub struct FormInputBindingPropertiesValue_ {
        pub binding_properties: Option<Box<FormInputBindingPropertiesValueProperties_>>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormInputBindingPropertiesValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormInputBindingPropertiesValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormInputBindingPropertiesValue as FormInputBindingPropertiesValue;
    impl crate::value::ToValue for FormInputBindingPropertiesValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.binding_properties {
                properties.insert(
                    "BindingProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-forminputbindingpropertiesvalueproperties.html
    pub struct FormInputBindingPropertiesValueProperties_ {
        pub model: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormInputBindingPropertiesValueProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormInputBindingPropertiesValueProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormInputBindingPropertiesValueProperties as FormInputBindingPropertiesValueProperties;
    impl crate::value::ToValue for FormInputBindingPropertiesValueProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.model {
                properties.insert("Model".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-forminputvalueproperty.html
    pub struct FormInputValueProperty_ {
        pub binding_properties: Option<Box<FormInputValuePropertyBindingProperties_>>,
        pub concat: Option<Vec<FormInputValueProperty_>>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormInputValueProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormInputValueProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormInputValueProperty as FormInputValueProperty;
    impl crate::value::ToValue for FormInputValueProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.binding_properties {
                properties.insert(
                    "BindingProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.concat {
                properties.insert("Concat".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-forminputvaluepropertybindingproperties.html
    pub struct FormInputValuePropertyBindingProperties_ {
        pub field: Option<crate::value::ExpString>,
        pub property: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormInputValuePropertyBindingProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormInputValuePropertyBindingProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormInputValuePropertyBindingProperties as FormInputValuePropertyBindingProperties;
    impl crate::value::ToValue for FormInputValuePropertyBindingProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field {
                properties.insert("Field".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Property".to_string(),
                crate::value::ToValue::to_value(&self.property),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyle.html
    pub struct FormStyle_ {
        pub horizontal_gap: Option<Box<FormStyleConfig_>>,
        pub outer_padding: Option<Box<FormStyleConfig_>>,
        pub vertical_gap: Option<Box<FormStyleConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormStyle {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormStyle"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormStyle as FormStyle;
    impl crate::value::ToValue for FormStyle_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.horizontal_gap {
                properties.insert(
                    "HorizontalGap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.outer_padding {
                properties.insert(
                    "OuterPadding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vertical_gap {
                properties.insert(
                    "VerticalGap".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-formstyleconfig.html
    pub struct FormStyleConfig_ {
        pub token_reference: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_FormStyleConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.FormStyleConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_FormStyleConfig as FormStyleConfig;
    impl crate::value::ToValue for FormStyleConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.token_reference {
                properties.insert(
                    "TokenReference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-sectionalelement.html
    pub struct SectionalElement_ {
        pub excluded: Option<crate::value::ExpBool>,
        pub level: Option<f64>,
        pub orientation: Option<crate::value::ExpString>,
        pub position: Option<Box<FieldPosition_>>,
        pub text: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_SectionalElement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.SectionalElement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_SectionalElement as SectionalElement;
    impl crate::value::ToValue for SectionalElement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excluded {
                properties.insert(
                    "Excluded".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.level {
                properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.orientation {
                properties.insert(
                    "Orientation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.position {
                properties.insert(
                    "Position".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-valuemapping.html
    pub struct ValueMapping_ {
        pub display_value: Option<Box<FormInputValueProperty_>>,
        pub value: Box<FormInputValueProperty_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_ValueMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.ValueMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_ValueMapping as ValueMapping;
    impl crate::value::ToValue for ValueMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.display_value {
                properties.insert(
                    "DisplayValue".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-form-valuemappings.html
    pub struct ValueMappings_ {
        pub binding_properties:
            Option<std::collections::BTreeMap<String, FormInputBindingPropertiesValue_>>,
        pub values: Vec<ValueMapping_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Form_ValueMappings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Form.ValueMappings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Form_ValueMappings as ValueMappings;
    impl crate::value::ToValue for ValueMappings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.binding_properties {
                properties.insert(
                    "BindingProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
}
pub mod theme {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalue.html
    pub struct ThemeValue_ {
        pub children: Option<Vec<ThemeValues_>>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Theme_ThemeValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Theme.ThemeValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Theme_ThemeValue as ThemeValue;
    impl crate::value::ToValue for ThemeValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.children {
                properties.insert(
                    "Children".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-amplifyuibuilder-theme-themevalues.html
    pub struct ThemeValues_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<Box<ThemeValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_amplifyuibuilder_Theme_ThemeValues {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::AmplifyUIBuilder::Theme.ThemeValues"
            $($field $value)*)
        };
    }
    pub use crate::__aws_amplifyuibuilder_Theme_ThemeValues as ThemeValues;
    impl crate::value::ToValue for ThemeValues_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.key {
                properties.insert("Key".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-component.html
pub struct Component_ {
    pub app_id: Option<crate::value::ExpString>,
    pub binding_properties: Option<
        std::collections::BTreeMap<
            String,
            super::amplifyuibuilder::component::ComponentBindingPropertiesValue_,
        >,
    >,
    pub children: Option<Vec<super::amplifyuibuilder::component::ComponentChild_>>,
    pub collection_properties: Option<
        std::collections::BTreeMap<
            String,
            super::amplifyuibuilder::component::ComponentDataConfiguration_,
        >,
    >,
    pub component_type: Option<crate::value::ExpString>,
    pub environment_name: Option<crate::value::ExpString>,
    pub events: Option<
        std::collections::BTreeMap<String, super::amplifyuibuilder::component::ComponentEvent_>,
    >,
    pub name: Option<crate::value::ExpString>,
    pub overrides: Option<serde_json::Value>,
    pub properties: Option<
        std::collections::BTreeMap<String, super::amplifyuibuilder::component::ComponentProperty_>,
    >,
    pub schema_version: Option<crate::value::ExpString>,
    pub source_id: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub variants: Option<Vec<super::amplifyuibuilder::component::ComponentVariant_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amplifyuibuilder_Component {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AmplifyUIBuilder::Component"
        $($field $value)*)
    };
}
pub use crate::__aws_amplifyuibuilder_Component as Component;
impl crate::template::ToResource for Component_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AmplifyUIBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Component"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_id {
            properties.insert("AppId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.binding_properties {
            properties.insert(
                "BindingProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.children {
            properties.insert(
                "Children".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.collection_properties {
            properties.insert(
                "CollectionProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.component_type {
            properties.insert(
                "ComponentType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_name {
            properties.insert(
                "EnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.events {
            properties.insert("Events".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.overrides {
            properties.insert(
                "Overrides".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.properties {
            properties.insert(
                "Properties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schema_version {
            properties.insert(
                "SchemaVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_id {
            properties.insert(
                "SourceId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-form.html
pub struct Form_ {
    pub app_id: Option<crate::value::ExpString>,
    pub cta: Option<super::amplifyuibuilder::form::FormCTA_>,
    pub data_type: Option<super::amplifyuibuilder::form::FormDataTypeConfig_>,
    pub environment_name: Option<crate::value::ExpString>,
    pub fields:
        Option<std::collections::BTreeMap<String, super::amplifyuibuilder::form::FieldConfig_>>,
    pub form_action_type: Option<crate::value::ExpString>,
    pub label_decorator: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub schema_version: Option<crate::value::ExpString>,
    pub sectional_elements: Option<
        std::collections::BTreeMap<String, super::amplifyuibuilder::form::SectionalElement_>,
    >,
    pub style: Option<super::amplifyuibuilder::form::FormStyle_>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amplifyuibuilder_Form {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AmplifyUIBuilder::Form"
        $($field $value)*)
    };
}
pub use crate::__aws_amplifyuibuilder_Form as Form;
impl crate::template::ToResource for Form_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AmplifyUIBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Form"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_id {
            properties.insert("AppId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.cta {
            properties.insert("Cta".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.data_type {
            properties.insert(
                "DataType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.environment_name {
            properties.insert(
                "EnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fields {
            properties.insert("Fields".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.form_action_type {
            properties.insert(
                "FormActionType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.label_decorator {
            properties.insert(
                "LabelDecorator".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.schema_version {
            properties.insert(
                "SchemaVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sectional_elements {
            properties.insert(
                "SectionalElements".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.style {
            properties.insert("Style".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-amplifyuibuilder-theme.html
pub struct Theme_ {
    pub app_id: Option<crate::value::ExpString>,
    pub environment_name: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub overrides: Option<Vec<super::amplifyuibuilder::theme::ThemeValues_>>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub values: Option<Vec<super::amplifyuibuilder::theme::ThemeValues_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_amplifyuibuilder_Theme {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::AmplifyUIBuilder::Theme"
        $($field $value)*)
    };
}
pub use crate::__aws_amplifyuibuilder_Theme as Theme;
impl crate::template::ToResource for Theme_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("AmplifyUIBuilder"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Theme"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_id {
            properties.insert("AppId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.environment_name {
            properties.insert(
                "EnvironmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.overrides {
            properties.insert(
                "Overrides".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.values {
            properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
