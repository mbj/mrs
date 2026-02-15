pub mod caserule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-caserule-booleancondition.html
    pub struct BooleanCondition_ {
        pub equal_to: Option<Box<BooleanOperands_>>,
        pub not_equal_to: Option<Box<BooleanOperands_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_CaseRule_BooleanCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::CaseRule.BooleanCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_CaseRule_BooleanCondition as BooleanCondition;
    impl crate::value::ToValue for BooleanCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.equal_to {
                properties.insert(
                    "EqualTo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_equal_to {
                properties.insert(
                    "NotEqualTo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-caserule-booleanoperands.html
    pub struct BooleanOperands_ {
        pub operand_one: Box<OperandOne_>,
        pub operand_two: Box<OperandTwo_>,
        pub result: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_CaseRule_BooleanOperands {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::CaseRule.BooleanOperands"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_CaseRule_BooleanOperands as BooleanOperands;
    impl crate::value::ToValue for BooleanOperands_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OperandOne".to_string(),
                crate::value::ToValue::to_value(&self.operand_one),
            );
            properties.insert(
                "OperandTwo".to_string(),
                crate::value::ToValue::to_value(&self.operand_two),
            );
            properties.insert(
                "Result".to_string(),
                crate::value::ToValue::to_value(&self.result),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-caserule-caseruledetails.html
    pub struct CaseRuleDetails_ {
        pub hidden: Option<Box<HiddenCaseRule_>>,
        pub required: Option<Box<RequiredCaseRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_CaseRule_CaseRuleDetails {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::CaseRule.CaseRuleDetails"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_CaseRule_CaseRuleDetails as CaseRuleDetails;
    impl crate::value::ToValue for CaseRuleDetails_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.hidden {
                properties.insert("Hidden".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.required {
                properties.insert(
                    "Required".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-caserule-hiddencaserule.html
    pub struct HiddenCaseRule_ {
        pub conditions: Vec<BooleanCondition_>,
        pub default_value: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_CaseRule_HiddenCaseRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::CaseRule.HiddenCaseRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_CaseRule_HiddenCaseRule as HiddenCaseRule;
    impl crate::value::ToValue for HiddenCaseRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(&self.conditions),
            );
            properties.insert(
                "DefaultValue".to_string(),
                crate::value::ToValue::to_value(&self.default_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-caserule-operandone.html
    pub struct OperandOne_ {
        pub field_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_CaseRule_OperandOne {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::CaseRule.OperandOne"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_CaseRule_OperandOne as OperandOne;
    impl crate::value::ToValue for OperandOne_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldId".to_string(),
                crate::value::ToValue::to_value(&self.field_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-caserule-operandtwo.html
    pub struct OperandTwo_ {
        pub boolean_value: Option<crate::value::ExpBool>,
        pub double_value: Option<f64>,
        pub empty_value: Option<serde_json::Value>,
        pub string_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_CaseRule_OperandTwo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::CaseRule.OperandTwo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_CaseRule_OperandTwo as OperandTwo;
    impl crate::value::ToValue for OperandTwo_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-caserule-requiredcaserule.html
    pub struct RequiredCaseRule_ {
        pub conditions: Vec<BooleanCondition_>,
        pub default_value: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_CaseRule_RequiredCaseRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::CaseRule.RequiredCaseRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_CaseRule_RequiredCaseRule as RequiredCaseRule;
    impl crate::value::ToValue for RequiredCaseRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(&self.conditions),
            );
            properties.insert(
                "DefaultValue".to_string(),
                crate::value::ToValue::to_value(&self.default_value),
            );
            properties.into()
        }
    }
}
pub mod layout {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-layout-basiclayout.html
    pub struct BasicLayout_ {
        pub more_info: Option<Box<LayoutSections_>>,
        pub top_panel: Option<Box<LayoutSections_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Layout_BasicLayout {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Layout.BasicLayout"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Layout_BasicLayout as BasicLayout;
    impl crate::value::ToValue for BasicLayout_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.more_info {
                properties.insert(
                    "MoreInfo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.top_panel {
                properties.insert(
                    "TopPanel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-layout-fieldgroup.html
    pub struct FieldGroup_ {
        pub fields: Vec<FieldItem_>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Layout_FieldGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Layout.FieldGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Layout_FieldGroup as FieldGroup;
    impl crate::value::ToValue for FieldGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Fields".to_string(),
                crate::value::ToValue::to_value(&self.fields),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-layout-fielditem.html
    pub struct FieldItem_ {
        pub id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Layout_FieldItem {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Layout.FieldItem"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Layout_FieldItem as FieldItem;
    impl crate::value::ToValue for FieldItem_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-layout-layoutcontent.html
    pub struct LayoutContent_ {
        pub basic: Box<BasicLayout_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Layout_LayoutContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Layout.LayoutContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Layout_LayoutContent as LayoutContent;
    impl crate::value::ToValue for LayoutContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Basic".to_string(),
                crate::value::ToValue::to_value(&self.basic),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-layout-layoutsections.html
    pub struct LayoutSections_ {
        pub sections: Option<Vec<Section_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Layout_LayoutSections {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Layout.LayoutSections"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Layout_LayoutSections as LayoutSections;
    impl crate::value::ToValue for LayoutSections_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sections {
                properties.insert(
                    "Sections".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-layout-section.html
    pub struct Section_ {
        pub field_group: Box<FieldGroup_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Layout_Section {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Layout.Section"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Layout_Section as Section;
    impl crate::value::ToValue for Section_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldGroup".to_string(),
                crate::value::ToValue::to_value(&self.field_group),
            );
            properties.into()
        }
    }
}
pub mod template {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-template-layoutconfiguration.html
    pub struct LayoutConfiguration_ {
        pub default_layout: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Template_LayoutConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Template.LayoutConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Template_LayoutConfiguration as LayoutConfiguration;
    impl crate::value::ToValue for LayoutConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_layout {
                properties.insert(
                    "DefaultLayout".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-template-requiredfield.html
    pub struct RequiredField_ {
        pub field_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Template_RequiredField {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Template.RequiredField"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Template_RequiredField as RequiredField;
    impl crate::value::ToValue for RequiredField_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldId".to_string(),
                crate::value::ToValue::to_value(&self.field_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cases-template-templaterule.html
    pub struct TemplateRule_ {
        pub case_rule_id: crate::value::ExpString,
        pub field_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cases_Template_TemplateRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Cases::Template.TemplateRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cases_Template_TemplateRule as TemplateRule;
    impl crate::value::ToValue for TemplateRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CaseRuleId".to_string(),
                crate::value::ToValue::to_value(&self.case_rule_id),
            );
            if let Some(ref value) = self.field_id {
                properties.insert(
                    "FieldId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cases-caserule.html
pub struct CaseRule_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub rule: super::cases::caserule::CaseRuleDetails_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cases_CaseRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cases::CaseRule" $($field
        $value)*)
    };
}
pub use crate::__aws_cases_CaseRule as CaseRule;
impl crate::template::ToResource for CaseRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cases"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CaseRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_id {
            properties.insert(
                "DomainId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cases-domain.html
pub struct Domain_ {
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cases_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cases::Domain" $($field
        $value)*)
    };
}
pub use crate::__aws_cases_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cases"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cases-field.html
pub struct Field_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cases_Field {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cases::Field" $($field
        $value)*)
    };
}
pub use crate::__aws_cases_Field as Field;
impl crate::template::ToResource for Field_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cases"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Field"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_id {
            properties.insert(
                "DomainId".to_string(),
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
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cases-layout.html
pub struct Layout_ {
    pub content: super::cases::layout::LayoutContent_,
    pub domain_id: Option<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cases_Layout {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cases::Layout" $($field
        $value)*)
    };
}
pub use crate::__aws_cases_Layout as Layout;
impl crate::template::ToResource for Layout_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cases"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Layout"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Content".to_string(),
            crate::value::ToValue::to_value(&self.content),
        );
        if let Some(ref value) = self.domain_id {
            properties.insert(
                "DomainId".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cases-template.html
pub struct Template_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_id: Option<crate::value::ExpString>,
    pub layout_configuration: Option<super::cases::template::LayoutConfiguration_>,
    pub name: crate::value::ExpString,
    pub required_fields: Option<Vec<super::cases::template::RequiredField_>>,
    pub rules: Option<Vec<super::cases::template::TemplateRule_>>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cases_Template {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Cases::Template" $($field
        $value)*)
    };
}
pub use crate::__aws_cases_Template as Template;
impl crate::template::ToResource for Template_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Cases"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Template"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_id {
            properties.insert(
                "DomainId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.layout_configuration {
            properties.insert(
                "LayoutConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.required_fields {
            properties.insert(
                "RequiredFields".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rules {
            properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
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
