pub mod bytematchset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-bytematchtuple.html
    pub struct ByteMatchTuple_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub positional_constraint: crate::value::ExpString,
        pub target_string: Option<crate::value::ExpString>,
        pub target_string_base64: Option<crate::value::ExpString>,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_ByteMatchSet_ByteMatchTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::ByteMatchSet.ByteMatchTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_ByteMatchSet_ByteMatchTuple as ByteMatchTuple;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-bytematchset-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_ByteMatchSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::ByteMatchSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_ByteMatchSet_FieldToMatch as FieldToMatch;
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
pub mod geomatchset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-geomatchset-geomatchconstraint.html
    pub struct GeoMatchConstraint_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_GeoMatchSet_GeoMatchConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::GeoMatchSet.GeoMatchConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_GeoMatchSet_GeoMatchConstraint as GeoMatchConstraint;
    impl crate::value::ToValue for GeoMatchConstraint_ {
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
pub mod ipset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-ipset-ipsetdescriptor.html
    pub struct IPSetDescriptor_ {
        pub r#type: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_IPSet_IPSetDescriptor {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::IPSet.IPSetDescriptor"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_IPSet_IPSetDescriptor as IPSetDescriptor;
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
pub mod ratebasedrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-ratebasedrule-predicate.html
    pub struct Predicate_ {
        pub data_id: crate::value::ExpString,
        pub negated: crate::value::ExpBool,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_RateBasedRule_Predicate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::RateBasedRule.Predicate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_RateBasedRule_Predicate as Predicate;
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
pub mod rule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-rule-predicate.html
    pub struct Predicate_ {
        pub data_id: crate::value::ExpString,
        pub negated: crate::value::ExpBool,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_Rule_Predicate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::Rule.Predicate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_Rule_Predicate as Predicate;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_SizeConstraintSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::SizeConstraintSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_SizeConstraintSet_FieldToMatch as FieldToMatch;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sizeconstraintset-sizeconstraint.html
    pub struct SizeConstraint_ {
        pub comparison_operator: crate::value::ExpString,
        pub field_to_match: Box<FieldToMatch_>,
        pub size: i32,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_SizeConstraintSet_SizeConstraint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::SizeConstraintSet.SizeConstraint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_SizeConstraintSet_SizeConstraint as SizeConstraint;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_SqlInjectionMatchSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::SqlInjectionMatchSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_SqlInjectionMatchSet_FieldToMatch as FieldToMatch;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-sqlinjectionmatchset-sqlinjectionmatchtuple.html
    pub struct SqlInjectionMatchTuple_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_SqlInjectionMatchSet_SqlInjectionMatchTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::SqlInjectionMatchSet.SqlInjectionMatchTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_SqlInjectionMatchSet_SqlInjectionMatchTuple as SqlInjectionMatchTuple;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-action.html
    pub struct Action_ {
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_WebACL_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::WebACL.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_WebACL_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-webacl-rule.html
    pub struct Rule_ {
        pub action: Box<Action_>,
        pub priority: i32,
        pub rule_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_WebACL_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::WebACL.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_WebACL_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
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
}
pub mod xssmatchset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-fieldtomatch.html
    pub struct FieldToMatch_ {
        pub data: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_XssMatchSet_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::XssMatchSet.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_XssMatchSet_FieldToMatch as FieldToMatch;
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafregional-xssmatchset-xssmatchtuple.html
    pub struct XssMatchTuple_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformation: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafregional_XssMatchSet_XssMatchTuple {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFRegional::XssMatchSet.XssMatchTuple"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafregional_XssMatchSet_XssMatchTuple as XssMatchTuple;
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-bytematchset.html
pub struct ByteMatchSet_ {
    pub byte_match_tuples: Option<Vec<super::wafregional::bytematchset::ByteMatchTuple_>>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_ByteMatchSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::ByteMatchSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_ByteMatchSet as ByteMatchSet;
impl crate::template::ToResource for ByteMatchSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-geomatchset.html
pub struct GeoMatchSet_ {
    pub geo_match_constraints: Option<Vec<super::wafregional::geomatchset::GeoMatchConstraint_>>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_GeoMatchSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::GeoMatchSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_GeoMatchSet as GeoMatchSet;
impl crate::template::ToResource for GeoMatchSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GeoMatchSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.geo_match_constraints {
            properties.insert(
                "GeoMatchConstraints".to_string(),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-ipset.html
pub struct IPSet_ {
    pub ip_set_descriptors: Option<Vec<super::wafregional::ipset::IPSetDescriptor_>>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_IPSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::IPSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_IPSet as IPSet;
impl crate::template::ToResource for IPSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-ratebasedrule.html
pub struct RateBasedRule_ {
    pub match_predicates: Option<Vec<super::wafregional::ratebasedrule::Predicate_>>,
    pub metric_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub rate_key: crate::value::ExpString,
    pub rate_limit: i32,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_RateBasedRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::RateBasedRule"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_RateBasedRule as RateBasedRule;
impl crate::template::ToResource for RateBasedRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RateBasedRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.match_predicates {
            properties.insert(
                "MatchPredicates".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MetricName".to_string(),
            crate::value::ToValue::to_value(&self.metric_name),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RateKey".to_string(),
            crate::value::ToValue::to_value(&self.rate_key),
        );
        properties.insert(
            "RateLimit".to_string(),
            crate::value::ToValue::to_value(&self.rate_limit),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-regexpatternset.html
pub struct RegexPatternSet_ {
    pub name: crate::value::ExpString,
    pub regex_pattern_strings: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_RegexPatternSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::RegexPatternSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_RegexPatternSet as RegexPatternSet;
impl crate::template::ToResource for RegexPatternSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RegexPatternSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "RegexPatternStrings".to_string(),
            crate::value::ToValue::to_value(&self.regex_pattern_strings),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-rule.html
pub struct Rule_ {
    pub metric_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub predicates: Option<Vec<super::wafregional::rule::Predicate_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_Rule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::Rule"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_Rule as Rule;
impl crate::template::ToResource for Rule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sizeconstraintset.html
pub struct SizeConstraintSet_ {
    pub name: crate::value::ExpString,
    pub size_constraints: Option<Vec<super::wafregional::sizeconstraintset::SizeConstraint_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_SizeConstraintSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::SizeConstraintSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_SizeConstraintSet as SizeConstraintSet;
impl crate::template::ToResource for SizeConstraintSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
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
        if let Some(ref value) = self.size_constraints {
            properties.insert(
                "SizeConstraints".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-sqlinjectionmatchset.html
pub struct SqlInjectionMatchSet_ {
    pub name: crate::value::ExpString,
    pub sql_injection_match_tuples:
        Option<Vec<super::wafregional::sqlinjectionmatchset::SqlInjectionMatchTuple_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_SqlInjectionMatchSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::SqlInjectionMatchSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_SqlInjectionMatchSet as SqlInjectionMatchSet;
impl crate::template::ToResource for SqlInjectionMatchSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webacl.html
pub struct WebACL_ {
    pub default_action: super::wafregional::webacl::Action_,
    pub metric_name: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub rules: Option<Vec<super::wafregional::webacl::Rule_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_WebACL {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::WebACL"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_WebACL as WebACL;
impl crate::template::ToResource for WebACL_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-webaclassociation.html
pub struct WebACLAssociation_ {
    pub resource_arn: crate::value::ExpString,
    pub web_acl_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_WebACLAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::WebACLAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_WebACLAssociation as WebACLAssociation;
impl crate::template::ToResource for WebACLAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WebACLAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties.insert(
            "WebACLId".to_string(),
            crate::value::ToValue::to_value(&self.web_acl_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafregional-xssmatchset.html
pub struct XssMatchSet_ {
    pub name: crate::value::ExpString,
    pub xss_match_tuples: Option<Vec<super::wafregional::xssmatchset::XssMatchTuple_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafregional_XssMatchSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFRegional::XssMatchSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafregional_XssMatchSet as XssMatchSet;
impl crate::template::ToResource for XssMatchSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFRegional"),
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
        if let Some(ref value) = self.xss_match_tuples {
            properties.insert(
                "XssMatchTuples".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
