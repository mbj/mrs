pub mod bytematchset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples.html
    pub struct ByteMatchTuple_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub positional_constraint: crate::value::ExpString,
        pub target_string: Option<crate::value::ExpString>,
        pub target_string_base64: Option<crate::value::ExpString>,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_ByteMatchSet_ByteMatchTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::ByteMatchSet.ByteMatchTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_ByteMatchSet_ByteMatchTuple as ByteMatchTuple;
    impl crate::value::ToValue for ByteMatchTuple_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "PositionalConstraint".to_string(),
                crate::value::ToValue::to_value(&self.positional_constraint),
            );
            if let Some(ref value) = self.target_string {
                properties.insert(
                    "TargetString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.target_string_base64 {
                properties.insert(
                    "TargetStringBase64".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TextTransformation".to_string(),
                crate::value::ToValue::to_value(&self.text_transformation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_ByteMatchSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::ByteMatchSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_ByteMatchSet_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data {
                properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod ipset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-ipset-ipsetdescriptors.html
    pub struct IPSetDescriptor_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_IPSet_IPSetDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::IPSet.IPSetDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_IPSet_IPSetDescriptor as IPSetDescriptor;
    impl crate::value::ToValue for IPSetDescriptor_ {
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
}
pub mod rule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-rule-predicates.html
    pub struct Predicate_ {
        pub data_id: crate::value::ExpString,
        pub negated: crate::value::ExpBool,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_Rule_Predicate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::Rule.Predicate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_Rule_Predicate as Predicate;
    impl crate::value::ToValue for Predicate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataId".to_string(),
                crate::value::ToValue::to_value(&self.data_id),
            );
            properties.insert(
                "Negated".to_string(),
                crate::value::ToValue::to_value(&self.negated),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod sizeconstraintset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sizeconstraintset-sizeconstraint-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_SizeConstraintSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::SizeConstraintSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_SizeConstraintSet_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data {
                properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sizeconstraintset-sizeconstraint.html
    pub struct SizeConstraint_ {
        pub comparison_operator: crate::value::ExpString,
        pub field_to_match: Box<FieldToMatch_>,
        pub size: i64,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_SizeConstraintSet_SizeConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::SizeConstraintSet.SizeConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_SizeConstraintSet_SizeConstraint as SizeConstraint;
    impl crate::value::ToValue for SizeConstraint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ComparisonOperator".to_string(),
                crate::value::ToValue::to_value(&self.comparison_operator),
            );
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "Size".to_string(),
                crate::value::ToValue::to_value(&self.size),
            );
            properties.insert(
                "TextTransformation".to_string(),
                crate::value::ToValue::to_value(&self.text_transformation),
            );
            properties.into()
        }
    }
}
pub mod sqlinjectionmatchset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-bytematchset-bytematchtuples-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_SqlInjectionMatchSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::SqlInjectionMatchSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_SqlInjectionMatchSet_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data {
                properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-sqlinjectionmatchset-sqlinjectionmatchtuples.html
    pub struct SqlInjectionMatchTuple_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_SqlInjectionMatchSet_SqlInjectionMatchTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::SqlInjectionMatchSet.SqlInjectionMatchTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_SqlInjectionMatchSet_SqlInjectionMatchTuple as SqlInjectionMatchTuple;
    impl crate::value::ToValue for SqlInjectionMatchTuple_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "TextTransformation".to_string(),
                crate::value::ToValue::to_value(&self.text_transformation),
            );
            properties.into()
        }
    }
}
pub mod webacl {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-webacl-rules.html
    pub struct ActivatedRule_ {
        pub action: Option<Box<WafAction_>>,
        pub priority: i64,
        pub rule_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_WebACL_ActivatedRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::WebACL.ActivatedRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_WebACL_ActivatedRule as ActivatedRule;
    impl crate::value::ToValue for ActivatedRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "RuleId".to_string(),
                crate::value::ToValue::to_value(&self.rule_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-webacl-action.html
    pub struct WafAction_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_WebACL_WafAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::WebACL.WafAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_WebACL_WafAction as WafAction;
    impl crate::value::ToValue for WafAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod xssmatchset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-xssmatchset-xssmatchtuple-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_XssMatchSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::XssMatchSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_XssMatchSet_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data {
                properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-waf-xssmatchset-xssmatchtuple.html
    pub struct XssMatchTuple_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_waf_XssMatchSet_XssMatchTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAF::XssMatchSet.XssMatchTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_waf_XssMatchSet_XssMatchTuple as XssMatchTuple;
    impl crate::value::ToValue for XssMatchTuple_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "TextTransformation".to_string(),
                crate::value::ToValue::to_value(&self.text_transformation),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-bytematchset.html
pub struct ByteMatchSet_ {
    pub byte_match_tuples: Option<Vec<super::waf::bytematchset::ByteMatchTuple_>>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_waf_ByteMatchSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAF::ByteMatchSet"
        $($field $value)*)
    };
}
pub use crate::__aws_waf_ByteMatchSet as ByteMatchSet;
impl crate::template::ToResource for ByteMatchSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAF"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ByteMatchSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.byte_match_tuples {
            properties.insert(
                "ByteMatchTuples".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-ipset.html
pub struct IPSet_ {
    pub ip_set_descriptors: Option<Vec<super::waf::ipset::IPSetDescriptor_>>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_waf_IPSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAF::IPSet" $($field
        $value)*)
    };
}
pub use crate::__aws_waf_IPSet as IPSet;
impl crate::template::ToResource for IPSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAF"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ip_set_descriptors {
            properties.insert(
                "IPSetDescriptors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-rule.html
pub struct Rule_ {
    pub metric_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub predicates: Option<Vec<super::waf::rule::Predicate_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_waf_Rule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAF::Rule" $($field
        $value)*)
    };
}
pub use crate::__aws_waf_Rule as Rule;
impl crate::template::ToResource for Rule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAF"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Rule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "MetricName".to_string(),
            crate::value::ToValue::to_value(&self.metric_name),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.predicates {
            properties.insert(
                "Predicates".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-sizeconstraintset.html
pub struct SizeConstraintSet_ {
    pub name: crate::value::ExpString,
    pub size_constraints: Vec<super::waf::sizeconstraintset::SizeConstraint_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_waf_SizeConstraintSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAF::SizeConstraintSet"
        $($field $value)*)
    };
}
pub use crate::__aws_waf_SizeConstraintSet as SizeConstraintSet;
impl crate::template::ToResource for SizeConstraintSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAF"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SizeConstraintSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "SizeConstraints".to_string(),
            crate::value::ToValue::to_value(&self.size_constraints),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-sqlinjectionmatchset.html
pub struct SqlInjectionMatchSet_ {
    pub name: crate::value::ExpString,
    pub sql_injection_match_tuples:
        Option<Vec<super::waf::sqlinjectionmatchset::SqlInjectionMatchTuple_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_waf_SqlInjectionMatchSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAF::SqlInjectionMatchSet"
        $($field $value)*)
    };
}
pub use crate::__aws_waf_SqlInjectionMatchSet as SqlInjectionMatchSet;
impl crate::template::ToResource for SqlInjectionMatchSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAF"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SqlInjectionMatchSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.sql_injection_match_tuples {
            properties.insert(
                "SqlInjectionMatchTuples".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-webacl.html
pub struct WebACL_ {
    pub default_action: super::waf::webacl::WafAction_,
    pub metric_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub rules: Option<Vec<super::waf::webacl::ActivatedRule_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_waf_WebACL {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAF::WebACL" $($field
        $value)*)
    };
}
pub use crate::__aws_waf_WebACL as WebACL;
impl crate::template::ToResource for WebACL_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAF"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WebACL"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DefaultAction".to_string(),
            crate::value::ToValue::to_value(&self.default_action),
        );
        properties.insert(
            "MetricName".to_string(),
            crate::value::ToValue::to_value(&self.metric_name),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.rules {
            properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-waf-xssmatchset.html
pub struct XssMatchSet_ {
    pub name: crate::value::ExpString,
    pub xss_match_tuples: Vec<super::waf::xssmatchset::XssMatchTuple_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_waf_XssMatchSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAF::XssMatchSet"
        $($field $value)*)
    };
}
pub use crate::__aws_waf_XssMatchSet as XssMatchSet;
impl crate::template::ToResource for XssMatchSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAF"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("XssMatchSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "XssMatchTuples".to_string(),
            crate::value::ToValue::to_value(&self.xss_match_tuples),
        );
        properties
    }
}
