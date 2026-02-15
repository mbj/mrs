pub mod componenttype {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-compositecomponenttype.html
    pub struct CompositeComponentType_ {
        pub component_type_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_CompositeComponentType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.CompositeComponentType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_CompositeComponentType as CompositeComponentType;
    impl crate::value::ToValue for CompositeComponentType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_type_id {
                properties.insert(
                    "ComponentTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-dataconnector.html
    pub struct DataConnector_ {
        pub is_native: Option<crate::value::ExpBool>,
        pub lambda: Option<Box<LambdaFunction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_DataConnector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.DataConnector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_DataConnector as DataConnector;
    impl crate::value::ToValue for DataConnector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_native {
                properties.insert(
                    "IsNative".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda {
                properties.insert("Lambda".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datatype.html
    pub struct DataType_ {
        pub allowed_values: Option<Vec<DataValue_>>,
        pub nested_type: Option<Box<DataType_>>,
        pub relationship: Option<Box<Relationship_>>,
        pub r#type: crate::value::ExpString,
        pub unit_of_measure: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_DataType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.DataType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_DataType as DataType;
    impl crate::value::ToValue for DataType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_values {
                properties.insert(
                    "AllowedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nested_type {
                properties.insert(
                    "NestedType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.relationship {
                properties.insert(
                    "Relationship".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.unit_of_measure {
                properties.insert(
                    "UnitOfMeasure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-datavalue.html
    pub struct DataValue_ {
        pub boolean_value: Option<crate::value::ExpBool>,
        pub double_value: Option<f64>,
        pub expression: Option<crate::value::ExpString>,
        pub integer_value: Option<i32>,
        pub list_value: Option<Vec<DataValue_>>,
        pub long_value: Option<f64>,
        pub map_value: Option<std::collections::BTreeMap<String, DataValue_>>,
        pub relationship_value: Option<Box<RelationshipValue_>>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_DataValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.DataValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_DataValue as DataValue;
    impl crate::value::ToValue for DataValue_ {
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
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer_value {
                properties.insert(
                    "IntegerValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.list_value {
                properties.insert(
                    "ListValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.long_value {
                properties.insert(
                    "LongValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.map_value {
                properties.insert(
                    "MapValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.relationship_value {
                properties.insert(
                    "RelationshipValue".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-error.html
    pub struct Error_ {
        pub code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_Error {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.Error"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_Error as Error;
    impl crate::value::ToValue for Error_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code {
                properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-function.html
    pub struct Function_ {
        pub implemented_by: Option<Box<DataConnector_>>,
        pub required_properties: Option<Vec<crate::value::ExpString>>,
        pub scope: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_Function {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.Function"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_Function as Function;
    impl crate::value::ToValue for Function_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.implemented_by {
                properties.insert(
                    "ImplementedBy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.required_properties {
                properties.insert(
                    "RequiredProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope {
                properties.insert("Scope".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-lambdafunction.html
    pub struct LambdaFunction_ {
        pub arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_LambdaFunction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.LambdaFunction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_LambdaFunction as LambdaFunction;
    impl crate::value::ToValue for LambdaFunction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertydefinition.html
    pub struct PropertyDefinition_ {
        pub configurations: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub data_type: Option<Box<DataType_>>,
        pub default_value: Option<Box<DataValue_>>,
        pub is_external_id: Option<crate::value::ExpBool>,
        pub is_required_in_entity: Option<crate::value::ExpBool>,
        pub is_stored_externally: Option<crate::value::ExpBool>,
        pub is_time_series: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_PropertyDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.PropertyDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_PropertyDefinition as PropertyDefinition;
    impl crate::value::ToValue for PropertyDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configurations {
                properties.insert(
                    "Configurations".to_string(),
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
            if let Some(ref value) = self.is_external_id {
                properties.insert(
                    "IsExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_required_in_entity {
                properties.insert(
                    "IsRequiredInEntity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_stored_externally {
                properties.insert(
                    "IsStoredExternally".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_time_series {
                properties.insert(
                    "IsTimeSeries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-propertygroup.html
    pub struct PropertyGroup_ {
        pub group_type: Option<crate::value::ExpString>,
        pub property_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_PropertyGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.PropertyGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_PropertyGroup as PropertyGroup;
    impl crate::value::ToValue for PropertyGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_type {
                properties.insert(
                    "GroupType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_names {
                properties.insert(
                    "PropertyNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationship.html
    pub struct Relationship_ {
        pub relationship_type: Option<crate::value::ExpString>,
        pub target_component_type_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_Relationship {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.Relationship"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_Relationship as Relationship;
    impl crate::value::ToValue for Relationship_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.relationship_type {
                properties.insert(
                    "RelationshipType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_component_type_id {
                properties.insert(
                    "TargetComponentTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-relationshipvalue.html
    pub struct RelationshipValue_ {
        pub target_component_name: Option<crate::value::ExpString>,
        pub target_entity_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_RelationshipValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.RelationshipValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_RelationshipValue as RelationshipValue;
    impl crate::value::ToValue for RelationshipValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_component_name {
                properties.insert(
                    "TargetComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_entity_id {
                properties.insert(
                    "TargetEntityId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-componenttype-status.html
    pub struct Status_ {
        pub error: Option<Box<Error_>>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_ComponentType_Status {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::ComponentType.Status"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_ComponentType_Status as Status;
    impl crate::value::ToValue for Status_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error {
                properties.insert("Error".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod entity {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-component.html
    pub struct Component_ {
        pub component_name: Option<crate::value::ExpString>,
        pub component_type_id: Option<crate::value::ExpString>,
        pub defined_in: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub properties: Option<std::collections::BTreeMap<String, Property_>>,
        pub property_groups: Option<std::collections::BTreeMap<String, PropertyGroup_>>,
        pub status: Option<Box<Status_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_Component {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.Component"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_Component as Component;
    impl crate::value::ToValue for Component_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_name {
                properties.insert(
                    "ComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_type_id {
                properties.insert(
                    "ComponentTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.defined_in {
                properties.insert(
                    "DefinedIn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_groups {
                properties.insert(
                    "PropertyGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-compositecomponent.html
    pub struct CompositeComponent_ {
        pub component_name: Option<crate::value::ExpString>,
        pub component_path: Option<crate::value::ExpString>,
        pub component_type_id: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub properties: Option<std::collections::BTreeMap<String, Property_>>,
        pub property_groups: Option<std::collections::BTreeMap<String, PropertyGroup_>>,
        pub status: Option<Box<Status_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_CompositeComponent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.CompositeComponent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_CompositeComponent as CompositeComponent;
    impl crate::value::ToValue for CompositeComponent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.component_name {
                properties.insert(
                    "ComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_path {
                properties.insert(
                    "ComponentPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.component_type_id {
                properties.insert(
                    "ComponentTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.properties {
                properties.insert(
                    "Properties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_groups {
                properties.insert(
                    "PropertyGroups".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datatype.html
    pub struct DataType_ {
        pub allowed_values: Option<Vec<DataValue_>>,
        pub nested_type: Option<Box<DataType_>>,
        pub relationship: Option<Box<Relationship_>>,
        pub r#type: Option<crate::value::ExpString>,
        pub unit_of_measure: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_DataType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.DataType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_DataType as DataType;
    impl crate::value::ToValue for DataType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_values {
                properties.insert(
                    "AllowedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.nested_type {
                properties.insert(
                    "NestedType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.relationship {
                properties.insert(
                    "Relationship".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unit_of_measure {
                properties.insert(
                    "UnitOfMeasure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-datavalue.html
    pub struct DataValue_ {
        pub boolean_value: Option<crate::value::ExpBool>,
        pub double_value: Option<f64>,
        pub expression: Option<crate::value::ExpString>,
        pub integer_value: Option<i32>,
        pub list_value: Option<Vec<DataValue_>>,
        pub long_value: Option<f64>,
        pub map_value: Option<std::collections::BTreeMap<String, DataValue_>>,
        pub relationship_value: Option<Box<RelationshipValue_>>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_DataValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.DataValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_DataValue as DataValue;
    impl crate::value::ToValue for DataValue_ {
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
            if let Some(ref value) = self.expression {
                properties.insert(
                    "Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.integer_value {
                properties.insert(
                    "IntegerValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.list_value {
                properties.insert(
                    "ListValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.long_value {
                properties.insert(
                    "LongValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.map_value {
                properties.insert(
                    "MapValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.relationship_value {
                properties.insert(
                    "RelationshipValue".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-definition.html
    pub struct Definition_ {
        pub configuration: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub data_type: Option<Box<DataType_>>,
        pub default_value: Option<Box<DataValue_>>,
        pub is_external_id: Option<crate::value::ExpBool>,
        pub is_final: Option<crate::value::ExpBool>,
        pub is_imported: Option<crate::value::ExpBool>,
        pub is_inherited: Option<crate::value::ExpBool>,
        pub is_required_in_entity: Option<crate::value::ExpBool>,
        pub is_stored_externally: Option<crate::value::ExpBool>,
        pub is_time_series: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_Definition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.Definition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_Definition as Definition;
    impl crate::value::ToValue for Definition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration {
                properties.insert(
                    "Configuration".to_string(),
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
            if let Some(ref value) = self.is_external_id {
                properties.insert(
                    "IsExternalId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_final {
                properties.insert(
                    "IsFinal".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_imported {
                properties.insert(
                    "IsImported".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_inherited {
                properties.insert(
                    "IsInherited".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_required_in_entity {
                properties.insert(
                    "IsRequiredInEntity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_stored_externally {
                properties.insert(
                    "IsStoredExternally".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_time_series {
                properties.insert(
                    "IsTimeSeries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-error.html
    pub struct Error_ {
        pub code: Option<crate::value::ExpString>,
        pub message: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_Error {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.Error"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_Error as Error;
    impl crate::value::ToValue for Error_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code {
                properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.message {
                properties.insert(
                    "Message".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-property.html
    pub struct Property_ {
        pub definition: Option<Box<Definition_>>,
        pub value: Option<Box<DataValue_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_Property {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.Property"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_Property as Property;
    impl crate::value::ToValue for Property_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.definition {
                properties.insert(
                    "Definition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-propertygroup.html
    pub struct PropertyGroup_ {
        pub group_type: Option<crate::value::ExpString>,
        pub property_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_PropertyGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.PropertyGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_PropertyGroup as PropertyGroup;
    impl crate::value::ToValue for PropertyGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.group_type {
                properties.insert(
                    "GroupType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.property_names {
                properties.insert(
                    "PropertyNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationship.html
    pub struct Relationship_ {
        pub relationship_type: Option<crate::value::ExpString>,
        pub target_component_type_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_Relationship {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.Relationship"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_Relationship as Relationship;
    impl crate::value::ToValue for Relationship_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.relationship_type {
                properties.insert(
                    "RelationshipType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_component_type_id {
                properties.insert(
                    "TargetComponentTypeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-relationshipvalue.html
    pub struct RelationshipValue_ {
        pub target_component_name: Option<crate::value::ExpString>,
        pub target_entity_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_RelationshipValue {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.RelationshipValue"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_RelationshipValue as RelationshipValue;
    impl crate::value::ToValue for RelationshipValue_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.target_component_name {
                properties.insert(
                    "TargetComponentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_entity_id {
                properties.insert(
                    "TargetEntityId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-iottwinmaker-entity-status.html
    pub struct Status_ {
        pub error: Option<Box<Error_>>,
        pub state: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_iottwinmaker_Entity_Status {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::IoTTwinMaker::Entity.Status"
            $($field $value)*)
        };
    }
    pub use crate::__aws_iottwinmaker_Entity_Status as Status;
    impl crate::value::ToValue for Status_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.error {
                properties.insert("Error".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.state {
                properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-componenttype.html
pub struct ComponentType_ {
    pub component_type_id: crate::value::ExpString,
    pub composite_component_types: Option<
        std::collections::BTreeMap<
            String,
            super::iottwinmaker::componenttype::CompositeComponentType_,
        >,
    >,
    pub description: Option<crate::value::ExpString>,
    pub extends_from: Option<Vec<crate::value::ExpString>>,
    pub functions:
        Option<std::collections::BTreeMap<String, super::iottwinmaker::componenttype::Function_>>,
    pub is_singleton: Option<crate::value::ExpBool>,
    pub property_definitions: Option<
        std::collections::BTreeMap<String, super::iottwinmaker::componenttype::PropertyDefinition_>,
    >,
    pub property_groups: Option<
        std::collections::BTreeMap<String, super::iottwinmaker::componenttype::PropertyGroup_>,
    >,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workspace_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iottwinmaker_ComponentType {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTTwinMaker::ComponentType"
        $($field $value)*)
    };
}
pub use crate::__aws_iottwinmaker_ComponentType as ComponentType;
impl crate::template::ToResource for ComponentType_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTTwinMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ComponentType"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ComponentTypeId".to_string(),
            crate::value::ToValue::to_value(&self.component_type_id),
        );
        if let Some(ref value) = self.composite_component_types {
            properties.insert(
                "CompositeComponentTypes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.extends_from {
            properties.insert(
                "ExtendsFrom".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.functions {
            properties.insert(
                "Functions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_singleton {
            properties.insert(
                "IsSingleton".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.property_definitions {
            properties.insert(
                "PropertyDefinitions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.property_groups {
            properties.insert(
                "PropertyGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkspaceId".to_string(),
            crate::value::ToValue::to_value(&self.workspace_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-entity.html
pub struct Entity_ {
    pub components:
        Option<std::collections::BTreeMap<String, super::iottwinmaker::entity::Component_>>,
    pub composite_components: Option<
        std::collections::BTreeMap<String, super::iottwinmaker::entity::CompositeComponent_>,
    >,
    pub description: Option<crate::value::ExpString>,
    pub entity_id: Option<crate::value::ExpString>,
    pub entity_name: crate::value::ExpString,
    pub parent_entity_id: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workspace_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iottwinmaker_Entity {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTTwinMaker::Entity"
        $($field $value)*)
    };
}
pub use crate::__aws_iottwinmaker_Entity as Entity;
impl crate::template::ToResource for Entity_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTTwinMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Entity"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.components {
            properties.insert(
                "Components".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.composite_components {
            properties.insert(
                "CompositeComponents".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.entity_id {
            properties.insert(
                "EntityId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EntityName".to_string(),
            crate::value::ToValue::to_value(&self.entity_name),
        );
        if let Some(ref value) = self.parent_entity_id {
            properties.insert(
                "ParentEntityId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkspaceId".to_string(),
            crate::value::ToValue::to_value(&self.workspace_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-scene.html
pub struct Scene_ {
    pub capabilities: Option<Vec<crate::value::ExpString>>,
    pub content_location: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub scene_id: crate::value::ExpString,
    pub scene_metadata: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workspace_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iottwinmaker_Scene {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTTwinMaker::Scene"
        $($field $value)*)
    };
}
pub use crate::__aws_iottwinmaker_Scene as Scene;
impl crate::template::ToResource for Scene_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTTwinMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Scene"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.capabilities {
            properties.insert(
                "Capabilities".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ContentLocation".to_string(),
            crate::value::ToValue::to_value(&self.content_location),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SceneId".to_string(),
            crate::value::ToValue::to_value(&self.scene_id),
        );
        if let Some(ref value) = self.scene_metadata {
            properties.insert(
                "SceneMetadata".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkspaceId".to_string(),
            crate::value::ToValue::to_value(&self.workspace_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-syncjob.html
pub struct SyncJob_ {
    pub sync_role: crate::value::ExpString,
    pub sync_source: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workspace_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iottwinmaker_SyncJob {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTTwinMaker::SyncJob"
        $($field $value)*)
    };
}
pub use crate::__aws_iottwinmaker_SyncJob as SyncJob;
impl crate::template::ToResource for SyncJob_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTTwinMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SyncJob"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "SyncRole".to_string(),
            crate::value::ToValue::to_value(&self.sync_role),
        );
        properties.insert(
            "SyncSource".to_string(),
            crate::value::ToValue::to_value(&self.sync_source),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkspaceId".to_string(),
            crate::value::ToValue::to_value(&self.workspace_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-iottwinmaker-workspace.html
pub struct Workspace_ {
    pub description: Option<crate::value::ExpString>,
    pub role: crate::value::ExpString,
    pub s3_location: crate::value::ExpString,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub workspace_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_iottwinmaker_Workspace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::IoTTwinMaker::Workspace"
        $($field $value)*)
    };
}
pub use crate::__aws_iottwinmaker_Workspace as Workspace;
impl crate::template::ToResource for Workspace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("IoTTwinMaker"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workspace"),
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
            "Role".to_string(),
            crate::value::ToValue::to_value(&self.role),
        );
        properties.insert(
            "S3Location".to_string(),
            crate::value::ToValue::to_value(&self.s3_location),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "WorkspaceId".to_string(),
            crate::value::ToValue::to_value(&self.workspace_id),
        );
        properties
    }
}
