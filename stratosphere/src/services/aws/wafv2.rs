pub mod loggingconfiguration {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-actioncondition.html>
    pub struct ActionCondition_ {
        pub action: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_LoggingConfiguration_ActionCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::LoggingConfiguration.ActionCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_LoggingConfiguration_ActionCondition as ActionCondition;
    impl crate::value::ToValue for ActionCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-condition.html>
    pub struct Condition_ {
        pub action_condition: Option<Box<ActionCondition_>>,
        pub label_name_condition: Option<Box<LabelNameCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_LoggingConfiguration_Condition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::LoggingConfiguration.Condition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_LoggingConfiguration_Condition as Condition;
    impl crate::value::ToValue for Condition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_condition {
                properties.insert(
                    "ActionCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_name_condition {
                properties.insert(
                    "LabelNameCondition".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-fieldtomatch.html>
    pub struct FieldToMatch_ {
        pub method: Option<serde_json::Value>,
        pub query_string: Option<serde_json::Value>,
        pub single_header: Option<Box<SingleHeader_>>,
        pub uri_path: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_LoggingConfiguration_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::LoggingConfiguration.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_LoggingConfiguration_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_header {
                properties.insert(
                    "SingleHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_path {
                properties.insert(
                    "UriPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-filter.html>
    pub struct Filter_ {
        pub behavior: crate::value::ExpString,
        pub conditions: Vec<Condition_>,
        pub requirement: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_LoggingConfiguration_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::LoggingConfiguration.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_LoggingConfiguration_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Behavior".to_string(),
                crate::value::ToValue::to_value(&self.behavior),
            );
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(&self.conditions),
            );
            properties.insert(
                "Requirement".to_string(),
                crate::value::ToValue::to_value(&self.requirement),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-labelnamecondition.html>
    pub struct LabelNameCondition_ {
        pub label_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_LoggingConfiguration_LabelNameCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::LoggingConfiguration.LabelNameCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_LoggingConfiguration_LabelNameCondition as LabelNameCondition;
    impl crate::value::ToValue for LabelNameCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LabelName".to_string(),
                crate::value::ToValue::to_value(&self.label_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-loggingfilter.html>
    pub struct LoggingFilter_ {
        pub default_behavior: crate::value::ExpString,
        pub filters: Vec<Filter_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_LoggingConfiguration_LoggingFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::LoggingConfiguration.LoggingFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_LoggingConfiguration_LoggingFilter as LoggingFilter;
    impl crate::value::ToValue for LoggingFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultBehavior".to_string(),
                crate::value::ToValue::to_value(&self.default_behavior),
            );
            properties.insert(
                "Filters".to_string(),
                crate::value::ToValue::to_value(&self.filters),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-loggingconfiguration-singleheader.html>
    pub struct SingleHeader_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_LoggingConfiguration_SingleHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::LoggingConfiguration.SingleHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_LoggingConfiguration_SingleHeader as SingleHeader;
    impl crate::value::ToValue for SingleHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
}
pub mod rulegroup {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-allowaction.html>
    pub struct AllowAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_AllowAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.AllowAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_AllowAction as AllowAction;
    impl crate::value::ToValue for AllowAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-andstatement.html>
    pub struct AndStatement_ {
        pub statements: Vec<Statement_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_AndStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.AndStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_AndStatement as AndStatement;
    impl crate::value::ToValue for AndStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Statements".to_string(),
                crate::value::ToValue::to_value(&self.statements),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-asnmatchstatement.html>
    pub struct AsnMatchStatement_ {
        pub asn_list: Option<Vec<i32>>,
        pub forwarded_ip_config: Option<Box<ForwardedIPConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_AsnMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.AsnMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_AsnMatchStatement as AsnMatchStatement;
    impl crate::value::ToValue for AsnMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asn_list {
                properties.insert(
                    "AsnList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_ip_config {
                properties.insert(
                    "ForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-blockaction.html>
    pub struct BlockAction_ {
        pub custom_response: Option<Box<CustomResponse_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_BlockAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.BlockAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_BlockAction as BlockAction;
    impl crate::value::ToValue for BlockAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_response {
                properties.insert(
                    "CustomResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-body.html>
    pub struct Body_ {
        pub oversize_handling: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_Body {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.Body"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_Body as Body;
    impl crate::value::ToValue for Body_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.oversize_handling {
                properties.insert(
                    "OversizeHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-bytematchstatement.html>
    pub struct ByteMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub positional_constraint: crate::value::ExpString,
        pub search_string: Option<crate::value::ExpString>,
        pub search_string_base64: Option<crate::value::ExpString>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_ByteMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.ByteMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_ByteMatchStatement as ByteMatchStatement;
    impl crate::value::ToValue for ByteMatchStatement_ {
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
            if let Some(ref value) = self.search_string {
                properties.insert(
                    "SearchString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.search_string_base64 {
                properties.insert(
                    "SearchStringBase64".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-captchaaction.html>
    pub struct CaptchaAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CaptchaAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CaptchaAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CaptchaAction as CaptchaAction;
    impl crate::value::ToValue for CaptchaAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-captchaconfig.html>
    pub struct CaptchaConfig_ {
        pub immunity_time_property: Option<Box<ImmunityTimeProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CaptchaConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CaptchaConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CaptchaConfig as CaptchaConfig;
    impl crate::value::ToValue for CaptchaConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.immunity_time_property {
                properties.insert(
                    "ImmunityTimeProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-challengeaction.html>
    pub struct ChallengeAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_ChallengeAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.ChallengeAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_ChallengeAction as ChallengeAction;
    impl crate::value::ToValue for ChallengeAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-challengeconfig.html>
    pub struct ChallengeConfig_ {
        pub immunity_time_property: Option<Box<ImmunityTimeProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_ChallengeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.ChallengeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_ChallengeConfig as ChallengeConfig;
    impl crate::value::ToValue for ChallengeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.immunity_time_property {
                properties.insert(
                    "ImmunityTimeProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookiematchpattern.html>
    pub struct CookieMatchPattern_ {
        pub all: Option<serde_json::Value>,
        pub excluded_cookies: Option<Vec<crate::value::ExpString>>,
        pub included_cookies: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CookieMatchPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CookieMatchPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CookieMatchPattern as CookieMatchPattern;
    impl crate::value::ToValue for CookieMatchPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all {
                properties.insert("All".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.excluded_cookies {
                properties.insert(
                    "ExcludedCookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.included_cookies {
                properties.insert(
                    "IncludedCookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-cookies.html>
    pub struct Cookies_ {
        pub match_pattern: Box<CookieMatchPattern_>,
        pub match_scope: crate::value::ExpString,
        pub oversize_handling: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_Cookies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.Cookies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_Cookies as Cookies;
    impl crate::value::ToValue for Cookies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchPattern".to_string(),
                crate::value::ToValue::to_value(&self.match_pattern),
            );
            properties.insert(
                "MatchScope".to_string(),
                crate::value::ToValue::to_value(&self.match_scope),
            );
            properties.insert(
                "OversizeHandling".to_string(),
                crate::value::ToValue::to_value(&self.oversize_handling),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-countaction.html>
    pub struct CountAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CountAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CountAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CountAction as CountAction;
    impl crate::value::ToValue for CountAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customhttpheader.html>
    pub struct CustomHTTPHeader_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CustomHTTPHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CustomHTTPHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CustomHTTPHeader as CustomHTTPHeader;
    impl crate::value::ToValue for CustomHTTPHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customrequesthandling.html>
    pub struct CustomRequestHandling_ {
        pub insert_headers: Vec<CustomHTTPHeader_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CustomRequestHandling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CustomRequestHandling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CustomRequestHandling as CustomRequestHandling;
    impl crate::value::ToValue for CustomRequestHandling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InsertHeaders".to_string(),
                crate::value::ToValue::to_value(&self.insert_headers),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponse.html>
    pub struct CustomResponse_ {
        pub custom_response_body_key: Option<crate::value::ExpString>,
        pub response_code: i32,
        pub response_headers: Option<Vec<CustomHTTPHeader_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CustomResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CustomResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CustomResponse as CustomResponse;
    impl crate::value::ToValue for CustomResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_response_body_key {
                properties.insert(
                    "CustomResponseBodyKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResponseCode".to_string(),
                crate::value::ToValue::to_value(&self.response_code),
            );
            if let Some(ref value) = self.response_headers {
                properties.insert(
                    "ResponseHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-customresponsebody.html>
    pub struct CustomResponseBody_ {
        pub content: crate::value::ExpString,
        pub content_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_CustomResponseBody {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.CustomResponseBody"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_CustomResponseBody as CustomResponseBody;
    impl crate::value::ToValue for CustomResponseBody_ {
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
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-fieldtomatch.html>
    pub struct FieldToMatch_ {
        pub all_query_arguments: Option<serde_json::Value>,
        pub body: Option<Box<Body_>>,
        pub cookies: Option<Box<Cookies_>>,
        pub header_order: Option<Box<HeaderOrder_>>,
        pub headers: Option<Box<Headers_>>,
        pub ja3_fingerprint: Option<Box<JA3Fingerprint_>>,
        pub ja4_fingerprint: Option<Box<JA4Fingerprint_>>,
        pub json_body: Option<Box<JsonBody_>>,
        pub method: Option<serde_json::Value>,
        pub query_string: Option<serde_json::Value>,
        pub single_header: Option<Box<SingleHeader_>>,
        pub single_query_argument: Option<Box<SingleQueryArgument_>>,
        pub uri_fragment: Option<Box<UriFragment_>>,
        pub uri_path: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all_query_arguments {
                properties.insert(
                    "AllQueryArguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cookies {
                properties.insert(
                    "Cookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_order {
                properties.insert(
                    "HeaderOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ja3_fingerprint {
                properties.insert(
                    "JA3Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ja4_fingerprint {
                properties.insert(
                    "JA4Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_body {
                properties.insert(
                    "JsonBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_header {
                properties.insert(
                    "SingleHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_query_argument {
                properties.insert(
                    "SingleQueryArgument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_fragment {
                properties.insert(
                    "UriFragment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_path {
                properties.insert(
                    "UriPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-forwardedipconfiguration.html>
    pub struct ForwardedIPConfiguration_ {
        pub fallback_behavior: crate::value::ExpString,
        pub header_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_ForwardedIPConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.ForwardedIPConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_ForwardedIPConfiguration as ForwardedIPConfiguration;
    impl crate::value::ToValue for ForwardedIPConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.insert(
                "HeaderName".to_string(),
                crate::value::ToValue::to_value(&self.header_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-geomatchstatement.html>
    pub struct GeoMatchStatement_ {
        pub country_codes: Option<Vec<crate::value::ExpString>>,
        pub forwarded_ip_config: Option<Box<ForwardedIPConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_GeoMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.GeoMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_GeoMatchStatement as GeoMatchStatement;
    impl crate::value::ToValue for GeoMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.country_codes {
                properties.insert(
                    "CountryCodes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_ip_config {
                properties.insert(
                    "ForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headermatchpattern.html>
    pub struct HeaderMatchPattern_ {
        pub all: Option<serde_json::Value>,
        pub excluded_headers: Option<Vec<crate::value::ExpString>>,
        pub included_headers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_HeaderMatchPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.HeaderMatchPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_HeaderMatchPattern as HeaderMatchPattern;
    impl crate::value::ToValue for HeaderMatchPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all {
                properties.insert("All".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.excluded_headers {
                properties.insert(
                    "ExcludedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.included_headers {
                properties.insert(
                    "IncludedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headerorder.html>
    pub struct HeaderOrder_ {
        pub oversize_handling: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_HeaderOrder {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.HeaderOrder"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_HeaderOrder as HeaderOrder;
    impl crate::value::ToValue for HeaderOrder_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OversizeHandling".to_string(),
                crate::value::ToValue::to_value(&self.oversize_handling),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-headers.html>
    pub struct Headers_ {
        pub match_pattern: Box<HeaderMatchPattern_>,
        pub match_scope: crate::value::ExpString,
        pub oversize_handling: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_Headers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.Headers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_Headers as Headers;
    impl crate::value::ToValue for Headers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchPattern".to_string(),
                crate::value::ToValue::to_value(&self.match_pattern),
            );
            properties.insert(
                "MatchScope".to_string(),
                crate::value::ToValue::to_value(&self.match_scope),
            );
            properties.insert(
                "OversizeHandling".to_string(),
                crate::value::ToValue::to_value(&self.oversize_handling),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetforwardedipconfiguration.html>
    pub struct IPSetForwardedIPConfiguration_ {
        pub fallback_behavior: crate::value::ExpString,
        pub header_name: crate::value::ExpString,
        pub position: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_IPSetForwardedIPConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.IPSetForwardedIPConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_IPSetForwardedIPConfiguration as IPSetForwardedIPConfiguration;
    impl crate::value::ToValue for IPSetForwardedIPConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.insert(
                "HeaderName".to_string(),
                crate::value::ToValue::to_value(&self.header_name),
            );
            properties.insert(
                "Position".to_string(),
                crate::value::ToValue::to_value(&self.position),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ipsetreferencestatement.html>
    pub struct IPSetReferenceStatement_ {
        pub arn: crate::value::ExpString,
        pub ip_set_forwarded_ip_config: Option<Box<IPSetForwardedIPConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_IPSetReferenceStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.IPSetReferenceStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_IPSetReferenceStatement as IPSetReferenceStatement;
    impl crate::value::ToValue for IPSetReferenceStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.ip_set_forwarded_ip_config {
                properties.insert(
                    "IPSetForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-immunitytimeproperty.html>
    pub struct ImmunityTimeProperty_ {
        pub immunity_time: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_ImmunityTimeProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.ImmunityTimeProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_ImmunityTimeProperty as ImmunityTimeProperty;
    impl crate::value::ToValue for ImmunityTimeProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ImmunityTime".to_string(),
                crate::value::ToValue::to_value(&self.immunity_time),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ja3fingerprint.html>
    pub struct JA3Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_JA3Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.JA3Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_JA3Fingerprint as JA3Fingerprint;
    impl crate::value::ToValue for JA3Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ja4fingerprint.html>
    pub struct JA4Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_JA4Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.JA4Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_JA4Fingerprint as JA4Fingerprint;
    impl crate::value::ToValue for JA4Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonbody.html>
    pub struct JsonBody_ {
        pub invalid_fallback_behavior: Option<crate::value::ExpString>,
        pub match_pattern: Box<JsonMatchPattern_>,
        pub match_scope: crate::value::ExpString,
        pub oversize_handling: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_JsonBody {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.JsonBody"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_JsonBody as JsonBody;
    impl crate::value::ToValue for JsonBody_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invalid_fallback_behavior {
                properties.insert(
                    "InvalidFallbackBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MatchPattern".to_string(),
                crate::value::ToValue::to_value(&self.match_pattern),
            );
            properties.insert(
                "MatchScope".to_string(),
                crate::value::ToValue::to_value(&self.match_scope),
            );
            if let Some(ref value) = self.oversize_handling {
                properties.insert(
                    "OversizeHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-jsonmatchpattern.html>
    pub struct JsonMatchPattern_ {
        pub all: Option<serde_json::Value>,
        pub included_paths: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_JsonMatchPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.JsonMatchPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_JsonMatchPattern as JsonMatchPattern;
    impl crate::value::ToValue for JsonMatchPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all {
                properties.insert("All".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.included_paths {
                properties.insert(
                    "IncludedPaths".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-label.html>
    pub struct Label_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_Label {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.Label"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_Label as Label;
    impl crate::value::ToValue for Label_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-labelmatchstatement.html>
    pub struct LabelMatchStatement_ {
        pub key: crate::value::ExpString,
        pub scope: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_LabelMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.LabelMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_LabelMatchStatement as LabelMatchStatement;
    impl crate::value::ToValue for LabelMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Scope".to_string(),
                crate::value::ToValue::to_value(&self.scope),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-labelsummary.html>
    pub struct LabelSummary_ {
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_LabelSummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.LabelSummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_LabelSummary as LabelSummary;
    impl crate::value::ToValue for LabelSummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-notstatement.html>
    pub struct NotStatement_ {
        pub statement: Box<Statement_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_NotStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.NotStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_NotStatement as NotStatement;
    impl crate::value::ToValue for NotStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Statement".to_string(),
                crate::value::ToValue::to_value(&self.statement),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-orstatement.html>
    pub struct OrStatement_ {
        pub statements: Vec<Statement_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_OrStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.OrStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_OrStatement as OrStatement;
    impl crate::value::ToValue for OrStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Statements".to_string(),
                crate::value::ToValue::to_value(&self.statements),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatement.html>
    pub struct RateBasedStatement_ {
        pub aggregate_key_type: crate::value::ExpString,
        pub custom_keys: Option<Vec<RateBasedStatementCustomKey_>>,
        pub evaluation_window_sec: Option<i32>,
        pub forwarded_ip_config: Option<Box<ForwardedIPConfiguration_>>,
        pub limit: i32,
        pub scope_down_statement: Option<Box<Statement_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateBasedStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateBasedStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateBasedStatement as RateBasedStatement;
    impl crate::value::ToValue for RateBasedStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AggregateKeyType".to_string(),
                crate::value::ToValue::to_value(&self.aggregate_key_type),
            );
            if let Some(ref value) = self.custom_keys {
                properties.insert(
                    "CustomKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluation_window_sec {
                properties.insert(
                    "EvaluationWindowSec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_ip_config {
                properties.insert(
                    "ForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Limit".to_string(),
                crate::value::ToValue::to_value(&self.limit),
            );
            if let Some(ref value) = self.scope_down_statement {
                properties.insert(
                    "ScopeDownStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratebasedstatementcustomkey.html>
    pub struct RateBasedStatementCustomKey_ {
        pub asn: Option<serde_json::Value>,
        pub cookie: Option<Box<RateLimitCookie_>>,
        pub forwarded_ip: Option<serde_json::Value>,
        pub http_method: Option<serde_json::Value>,
        pub header: Option<Box<RateLimitHeader_>>,
        pub ip: Option<serde_json::Value>,
        pub ja3_fingerprint: Option<Box<RateLimitJA3Fingerprint_>>,
        pub ja4_fingerprint: Option<Box<RateLimitJA4Fingerprint_>>,
        pub label_namespace: Option<Box<RateLimitLabelNamespace_>>,
        pub query_argument: Option<Box<RateLimitQueryArgument_>>,
        pub query_string: Option<Box<RateLimitQueryString_>>,
        pub uri_path: Option<Box<RateLimitUriPath_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateBasedStatementCustomKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateBasedStatementCustomKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateBasedStatementCustomKey as RateBasedStatementCustomKey;
    impl crate::value::ToValue for RateBasedStatementCustomKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asn {
                properties.insert("ASN".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cookie {
                properties.insert("Cookie".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.forwarded_ip {
                properties.insert(
                    "ForwardedIP".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_method {
                properties.insert(
                    "HTTPMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ip {
                properties.insert("IP".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ja3_fingerprint {
                properties.insert(
                    "JA3Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ja4_fingerprint {
                properties.insert(
                    "JA4Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_namespace {
                properties.insert(
                    "LabelNamespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_argument {
                properties.insert(
                    "QueryArgument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_path {
                properties.insert(
                    "UriPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitcookie.html>
    pub struct RateLimitCookie_ {
        pub name: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitCookie {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitCookie"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitCookie as RateLimitCookie;
    impl crate::value::ToValue for RateLimitCookie_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitheader.html>
    pub struct RateLimitHeader_ {
        pub name: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitHeader as RateLimitHeader;
    impl crate::value::ToValue for RateLimitHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitja3fingerprint.html>
    pub struct RateLimitJA3Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitJA3Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitJA3Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitJA3Fingerprint as RateLimitJA3Fingerprint;
    impl crate::value::ToValue for RateLimitJA3Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitja4fingerprint.html>
    pub struct RateLimitJA4Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitJA4Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitJA4Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitJA4Fingerprint as RateLimitJA4Fingerprint;
    impl crate::value::ToValue for RateLimitJA4Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitlabelnamespace.html>
    pub struct RateLimitLabelNamespace_ {
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitLabelNamespace {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitLabelNamespace"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitLabelNamespace as RateLimitLabelNamespace;
    impl crate::value::ToValue for RateLimitLabelNamespace_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitqueryargument.html>
    pub struct RateLimitQueryArgument_ {
        pub name: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitQueryArgument {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitQueryArgument"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitQueryArgument as RateLimitQueryArgument;
    impl crate::value::ToValue for RateLimitQueryArgument_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimitquerystring.html>
    pub struct RateLimitQueryString_ {
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitQueryString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitQueryString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitQueryString as RateLimitQueryString;
    impl crate::value::ToValue for RateLimitQueryString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ratelimituripath.html>
    pub struct RateLimitUriPath_ {
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RateLimitUriPath {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RateLimitUriPath"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RateLimitUriPath as RateLimitUriPath;
    impl crate::value::ToValue for RateLimitUriPath_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexmatchstatement.html>
    pub struct RegexMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub regex_string: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RegexMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RegexMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RegexMatchStatement as RegexMatchStatement;
    impl crate::value::ToValue for RegexMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "RegexString".to_string(),
                crate::value::ToValue::to_value(&self.regex_string),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-regexpatternsetreferencestatement.html>
    pub struct RegexPatternSetReferenceStatement_ {
        pub arn: crate::value::ExpString,
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RegexPatternSetReferenceStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RegexPatternSetReferenceStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RegexPatternSetReferenceStatement as RegexPatternSetReferenceStatement;
    impl crate::value::ToValue for RegexPatternSetReferenceStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-rule.html>
    pub struct Rule_ {
        pub action: Option<Box<RuleAction_>>,
        pub captcha_config: Option<Box<CaptchaConfig_>>,
        pub challenge_config: Option<Box<ChallengeConfig_>>,
        pub name: crate::value::ExpString,
        pub priority: i32,
        pub rule_labels: Option<Vec<Label_>>,
        pub statement: Box<Statement_>,
        pub visibility_config: Box<VisibilityConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.captcha_config {
                properties.insert(
                    "CaptchaConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.challenge_config {
                properties.insert(
                    "ChallengeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            if let Some(ref value) = self.rule_labels {
                properties.insert(
                    "RuleLabels".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Statement".to_string(),
                crate::value::ToValue::to_value(&self.statement),
            );
            properties.insert(
                "VisibilityConfig".to_string(),
                crate::value::ToValue::to_value(&self.visibility_config),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-ruleaction.html>
    pub struct RuleAction_ {
        pub allow: Option<Box<AllowAction_>>,
        pub block: Option<Box<BlockAction_>>,
        pub captcha: Option<Box<CaptchaAction_>>,
        pub challenge: Option<Box<ChallengeAction_>>,
        pub count: Option<Box<CountAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_RuleAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.RuleAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_RuleAction as RuleAction;
    impl crate::value::ToValue for RuleAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow {
                properties.insert("Allow".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.block {
                properties.insert("Block".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.captcha {
                properties.insert(
                    "Captcha".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.challenge {
                properties.insert(
                    "Challenge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-singleheader.html>
    pub struct SingleHeader_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_SingleHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.SingleHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_SingleHeader as SingleHeader;
    impl crate::value::ToValue for SingleHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-singlequeryargument.html>
    pub struct SingleQueryArgument_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_SingleQueryArgument {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.SingleQueryArgument"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_SingleQueryArgument as SingleQueryArgument;
    impl crate::value::ToValue for SingleQueryArgument_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sizeconstraintstatement.html>
    pub struct SizeConstraintStatement_ {
        pub comparison_operator: crate::value::ExpString,
        pub field_to_match: Box<FieldToMatch_>,
        pub size: f64,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_SizeConstraintStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.SizeConstraintStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_SizeConstraintStatement as SizeConstraintStatement;
    impl crate::value::ToValue for SizeConstraintStatement_ {
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
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-sqlimatchstatement.html>
    pub struct SqliMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub sensitivity_level: Option<crate::value::ExpString>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_SqliMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.SqliMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_SqliMatchStatement as SqliMatchStatement;
    impl crate::value::ToValue for SqliMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            if let Some(ref value) = self.sensitivity_level {
                properties.insert(
                    "SensitivityLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-statement.html>
    pub struct Statement_ {
        pub and_statement: Option<Box<AndStatement_>>,
        pub asn_match_statement: Option<Box<AsnMatchStatement_>>,
        pub byte_match_statement: Option<Box<ByteMatchStatement_>>,
        pub geo_match_statement: Option<Box<GeoMatchStatement_>>,
        pub ip_set_reference_statement: Option<Box<IPSetReferenceStatement_>>,
        pub label_match_statement: Option<Box<LabelMatchStatement_>>,
        pub not_statement: Option<Box<NotStatement_>>,
        pub or_statement: Option<Box<OrStatement_>>,
        pub rate_based_statement: Option<Box<RateBasedStatement_>>,
        pub regex_match_statement: Option<Box<RegexMatchStatement_>>,
        pub regex_pattern_set_reference_statement: Option<Box<RegexPatternSetReferenceStatement_>>,
        pub size_constraint_statement: Option<Box<SizeConstraintStatement_>>,
        pub sqli_match_statement: Option<Box<SqliMatchStatement_>>,
        pub xss_match_statement: Option<Box<XssMatchStatement_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_Statement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.Statement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_Statement as Statement;
    impl crate::value::ToValue for Statement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and_statement {
                properties.insert(
                    "AndStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.asn_match_statement {
                properties.insert(
                    "AsnMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.byte_match_statement {
                properties.insert(
                    "ByteMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.geo_match_statement {
                properties.insert(
                    "GeoMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_set_reference_statement {
                properties.insert(
                    "IPSetReferenceStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_match_statement {
                properties.insert(
                    "LabelMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_statement {
                properties.insert(
                    "NotStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.or_statement {
                properties.insert(
                    "OrStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_based_statement {
                properties.insert(
                    "RateBasedStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regex_match_statement {
                properties.insert(
                    "RegexMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regex_pattern_set_reference_statement {
                properties.insert(
                    "RegexPatternSetReferenceStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_constraint_statement {
                properties.insert(
                    "SizeConstraintStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sqli_match_statement {
                properties.insert(
                    "SqliMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.xss_match_statement {
                properties.insert(
                    "XssMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-texttransformation.html>
    pub struct TextTransformation_ {
        pub priority: i32,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_TextTransformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.TextTransformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_TextTransformation as TextTransformation;
    impl crate::value::ToValue for TextTransformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-urifragment.html>
    pub struct UriFragment_ {
        pub fallback_behavior: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_UriFragment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.UriFragment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_UriFragment as UriFragment;
    impl crate::value::ToValue for UriFragment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fallback_behavior {
                properties.insert(
                    "FallbackBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-visibilityconfig.html>
    pub struct VisibilityConfig_ {
        pub cloud_watch_metrics_enabled: crate::value::ExpBool,
        pub metric_name: crate::value::ExpString,
        pub sampled_requests_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_VisibilityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.VisibilityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_VisibilityConfig as VisibilityConfig;
    impl crate::value::ToValue for VisibilityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchMetricsEnabled".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_metrics_enabled),
            );
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "SampledRequestsEnabled".to_string(),
                crate::value::ToValue::to_value(&self.sampled_requests_enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-rulegroup-xssmatchstatement.html>
    pub struct XssMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_RuleGroup_XssMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::RuleGroup.XssMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_RuleGroup_XssMatchStatement as XssMatchStatement;
    impl crate::value::ToValue for XssMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
}
pub mod webacl {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesacfpruleset.html>
    pub struct AWSManagedRulesACFPRuleSet_ {
        pub creation_path: crate::value::ExpString,
        pub enable_regex_in_path: Option<crate::value::ExpBool>,
        pub registration_page_path: crate::value::ExpString,
        pub request_inspection: Box<RequestInspectionACFP_>,
        pub response_inspection: Option<Box<ResponseInspection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AWSManagedRulesACFPRuleSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AWSManagedRulesACFPRuleSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AWSManagedRulesACFPRuleSet as AWSManagedRulesACFPRuleSet;
    impl crate::value::ToValue for AWSManagedRulesACFPRuleSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CreationPath".to_string(),
                crate::value::ToValue::to_value(&self.creation_path),
            );
            if let Some(ref value) = self.enable_regex_in_path {
                properties.insert(
                    "EnableRegexInPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RegistrationPagePath".to_string(),
                crate::value::ToValue::to_value(&self.registration_page_path),
            );
            properties.insert(
                "RequestInspection".to_string(),
                crate::value::ToValue::to_value(&self.request_inspection),
            );
            if let Some(ref value) = self.response_inspection {
                properties.insert(
                    "ResponseInspection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesatpruleset.html>
    pub struct AWSManagedRulesATPRuleSet_ {
        pub enable_regex_in_path: Option<crate::value::ExpBool>,
        pub login_path: crate::value::ExpString,
        pub request_inspection: Option<Box<RequestInspection_>>,
        pub response_inspection: Option<Box<ResponseInspection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AWSManagedRulesATPRuleSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AWSManagedRulesATPRuleSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AWSManagedRulesATPRuleSet as AWSManagedRulesATPRuleSet;
    impl crate::value::ToValue for AWSManagedRulesATPRuleSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_regex_in_path {
                properties.insert(
                    "EnableRegexInPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "LoginPath".to_string(),
                crate::value::ToValue::to_value(&self.login_path),
            );
            if let Some(ref value) = self.request_inspection {
                properties.insert(
                    "RequestInspection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.response_inspection {
                properties.insert(
                    "ResponseInspection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesantiddosruleset.html>
    pub struct AWSManagedRulesAntiDDoSRuleSet_ {
        pub client_side_action_config: Box<ClientSideActionConfig_>,
        pub sensitivity_to_block: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AWSManagedRulesAntiDDoSRuleSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AWSManagedRulesAntiDDoSRuleSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AWSManagedRulesAntiDDoSRuleSet as AWSManagedRulesAntiDDoSRuleSet;
    impl crate::value::ToValue for AWSManagedRulesAntiDDoSRuleSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClientSideActionConfig".to_string(),
                crate::value::ToValue::to_value(&self.client_side_action_config),
            );
            if let Some(ref value) = self.sensitivity_to_block {
                properties.insert(
                    "SensitivityToBlock".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-awsmanagedrulesbotcontrolruleset.html>
    pub struct AWSManagedRulesBotControlRuleSet_ {
        pub enable_machine_learning: Option<crate::value::ExpBool>,
        pub inspection_level: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AWSManagedRulesBotControlRuleSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AWSManagedRulesBotControlRuleSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AWSManagedRulesBotControlRuleSet as AWSManagedRulesBotControlRuleSet;
    impl crate::value::ToValue for AWSManagedRulesBotControlRuleSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_machine_learning {
                properties.insert(
                    "EnableMachineLearning".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InspectionLevel".to_string(),
                crate::value::ToValue::to_value(&self.inspection_level),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-allowaction.html>
    pub struct AllowAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AllowAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AllowAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AllowAction as AllowAction;
    impl crate::value::ToValue for AllowAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-andstatement.html>
    pub struct AndStatement_ {
        pub statements: Vec<Statement_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AndStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AndStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AndStatement as AndStatement;
    impl crate::value::ToValue for AndStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Statements".to_string(),
                crate::value::ToValue::to_value(&self.statements),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-applicationattribute.html>
    pub struct ApplicationAttribute_ {
        pub name: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ApplicationAttribute {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ApplicationAttribute"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ApplicationAttribute as ApplicationAttribute;
    impl crate::value::ToValue for ApplicationAttribute_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-applicationconfig.html>
    pub struct ApplicationConfig_ {
        pub attributes: Vec<ApplicationAttribute_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ApplicationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ApplicationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ApplicationConfig as ApplicationConfig;
    impl crate::value::ToValue for ApplicationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attributes".to_string(),
                crate::value::ToValue::to_value(&self.attributes),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-asnmatchstatement.html>
    pub struct AsnMatchStatement_ {
        pub asn_list: Option<Vec<i32>>,
        pub forwarded_ip_config: Option<Box<ForwardedIPConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AsnMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AsnMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AsnMatchStatement as AsnMatchStatement;
    impl crate::value::ToValue for AsnMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asn_list {
                properties.insert(
                    "AsnList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_ip_config {
                properties.insert(
                    "ForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-associationconfig.html>
    pub struct AssociationConfig_ {
        pub request_body:
            Option<std::collections::BTreeMap<String, RequestBodyAssociatedResourceTypeConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_AssociationConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.AssociationConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_AssociationConfig as AssociationConfig;
    impl crate::value::ToValue for AssociationConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.request_body {
                properties.insert(
                    "RequestBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-blockaction.html>
    pub struct BlockAction_ {
        pub custom_response: Option<Box<CustomResponse_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_BlockAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.BlockAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_BlockAction as BlockAction;
    impl crate::value::ToValue for BlockAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_response {
                properties.insert(
                    "CustomResponse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-body.html>
    pub struct Body_ {
        pub oversize_handling: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_Body {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.Body"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_Body as Body;
    impl crate::value::ToValue for Body_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.oversize_handling {
                properties.insert(
                    "OversizeHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-bytematchstatement.html>
    pub struct ByteMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub positional_constraint: crate::value::ExpString,
        pub search_string: Option<crate::value::ExpString>,
        pub search_string_base64: Option<crate::value::ExpString>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ByteMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ByteMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ByteMatchStatement as ByteMatchStatement;
    impl crate::value::ToValue for ByteMatchStatement_ {
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
            if let Some(ref value) = self.search_string {
                properties.insert(
                    "SearchString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.search_string_base64 {
                properties.insert(
                    "SearchStringBase64".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-captchaaction.html>
    pub struct CaptchaAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CaptchaAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CaptchaAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CaptchaAction as CaptchaAction;
    impl crate::value::ToValue for CaptchaAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-captchaconfig.html>
    pub struct CaptchaConfig_ {
        pub immunity_time_property: Option<Box<ImmunityTimeProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CaptchaConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CaptchaConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CaptchaConfig as CaptchaConfig;
    impl crate::value::ToValue for CaptchaConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.immunity_time_property {
                properties.insert(
                    "ImmunityTimeProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-challengeaction.html>
    pub struct ChallengeAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ChallengeAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ChallengeAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ChallengeAction as ChallengeAction;
    impl crate::value::ToValue for ChallengeAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-challengeconfig.html>
    pub struct ChallengeConfig_ {
        pub immunity_time_property: Option<Box<ImmunityTimeProperty_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ChallengeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ChallengeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ChallengeConfig as ChallengeConfig;
    impl crate::value::ToValue for ChallengeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.immunity_time_property {
                properties.insert(
                    "ImmunityTimeProperty".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-clientsideaction.html>
    pub struct ClientSideAction_ {
        pub exempt_uri_regular_expressions: Option<Vec<Regex_>>,
        pub sensitivity: Option<crate::value::ExpString>,
        pub usage_of_action: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ClientSideAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ClientSideAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ClientSideAction as ClientSideAction;
    impl crate::value::ToValue for ClientSideAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.exempt_uri_regular_expressions {
                properties.insert(
                    "ExemptUriRegularExpressions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sensitivity {
                properties.insert(
                    "Sensitivity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "UsageOfAction".to_string(),
                crate::value::ToValue::to_value(&self.usage_of_action),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-clientsideactionconfig.html>
    pub struct ClientSideActionConfig_ {
        pub challenge: Box<ClientSideAction_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ClientSideActionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ClientSideActionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ClientSideActionConfig as ClientSideActionConfig;
    impl crate::value::ToValue for ClientSideActionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Challenge".to_string(),
                crate::value::ToValue::to_value(&self.challenge),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookiematchpattern.html>
    pub struct CookieMatchPattern_ {
        pub all: Option<serde_json::Value>,
        pub excluded_cookies: Option<Vec<crate::value::ExpString>>,
        pub included_cookies: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CookieMatchPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CookieMatchPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CookieMatchPattern as CookieMatchPattern;
    impl crate::value::ToValue for CookieMatchPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all {
                properties.insert("All".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.excluded_cookies {
                properties.insert(
                    "ExcludedCookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.included_cookies {
                properties.insert(
                    "IncludedCookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-cookies.html>
    pub struct Cookies_ {
        pub match_pattern: Box<CookieMatchPattern_>,
        pub match_scope: crate::value::ExpString,
        pub oversize_handling: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_Cookies {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.Cookies"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_Cookies as Cookies;
    impl crate::value::ToValue for Cookies_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchPattern".to_string(),
                crate::value::ToValue::to_value(&self.match_pattern),
            );
            properties.insert(
                "MatchScope".to_string(),
                crate::value::ToValue::to_value(&self.match_scope),
            );
            properties.insert(
                "OversizeHandling".to_string(),
                crate::value::ToValue::to_value(&self.oversize_handling),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-countaction.html>
    pub struct CountAction_ {
        pub custom_request_handling: Option<Box<CustomRequestHandling_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CountAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CountAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CountAction as CountAction;
    impl crate::value::ToValue for CountAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_request_handling {
                properties.insert(
                    "CustomRequestHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customhttpheader.html>
    pub struct CustomHTTPHeader_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CustomHTTPHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CustomHTTPHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CustomHTTPHeader as CustomHTTPHeader;
    impl crate::value::ToValue for CustomHTTPHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customrequesthandling.html>
    pub struct CustomRequestHandling_ {
        pub insert_headers: Vec<CustomHTTPHeader_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CustomRequestHandling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CustomRequestHandling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CustomRequestHandling as CustomRequestHandling;
    impl crate::value::ToValue for CustomRequestHandling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "InsertHeaders".to_string(),
                crate::value::ToValue::to_value(&self.insert_headers),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponse.html>
    pub struct CustomResponse_ {
        pub custom_response_body_key: Option<crate::value::ExpString>,
        pub response_code: i32,
        pub response_headers: Option<Vec<CustomHTTPHeader_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CustomResponse {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CustomResponse"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CustomResponse as CustomResponse;
    impl crate::value::ToValue for CustomResponse_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_response_body_key {
                properties.insert(
                    "CustomResponseBodyKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ResponseCode".to_string(),
                crate::value::ToValue::to_value(&self.response_code),
            );
            if let Some(ref value) = self.response_headers {
                properties.insert(
                    "ResponseHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-customresponsebody.html>
    pub struct CustomResponseBody_ {
        pub content: crate::value::ExpString,
        pub content_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_CustomResponseBody {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.CustomResponseBody"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_CustomResponseBody as CustomResponseBody;
    impl crate::value::ToValue for CustomResponseBody_ {
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
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-dataprotect.html>
    pub struct DataProtect_ {
        pub action: crate::value::ExpString,
        pub exclude_rate_based_details: Option<crate::value::ExpBool>,
        pub exclude_rule_match_details: Option<crate::value::ExpBool>,
        pub field: Box<FieldToProtect_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_DataProtect {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.DataProtect"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_DataProtect as DataProtect;
    impl crate::value::ToValue for DataProtect_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            if let Some(ref value) = self.exclude_rate_based_details {
                properties.insert(
                    "ExcludeRateBasedDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_rule_match_details {
                properties.insert(
                    "ExcludeRuleMatchDetails".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Field".to_string(),
                crate::value::ToValue::to_value(&self.field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-dataprotectionconfig.html>
    pub struct DataProtectionConfig_ {
        pub data_protections: Vec<DataProtect_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_DataProtectionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.DataProtectionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_DataProtectionConfig as DataProtectionConfig;
    impl crate::value::ToValue for DataProtectionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DataProtections".to_string(),
                crate::value::ToValue::to_value(&self.data_protections),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-defaultaction.html>
    pub struct DefaultAction_ {
        pub allow: Option<Box<AllowAction_>>,
        pub block: Option<Box<BlockAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_DefaultAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.DefaultAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_DefaultAction as DefaultAction;
    impl crate::value::ToValue for DefaultAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow {
                properties.insert("Allow".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.block {
                properties.insert("Block".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-excludedrule.html>
    pub struct ExcludedRule_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ExcludedRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ExcludedRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ExcludedRule as ExcludedRule;
    impl crate::value::ToValue for ExcludedRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldidentifier.html>
    pub struct FieldIdentifier_ {
        pub identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_FieldIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.FieldIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_FieldIdentifier as FieldIdentifier;
    impl crate::value::ToValue for FieldIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Identifier".to_string(),
                crate::value::ToValue::to_value(&self.identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtomatch.html>
    pub struct FieldToMatch_ {
        pub all_query_arguments: Option<serde_json::Value>,
        pub body: Option<Box<Body_>>,
        pub cookies: Option<Box<Cookies_>>,
        pub header_order: Option<Box<HeaderOrder_>>,
        pub headers: Option<Box<Headers_>>,
        pub ja3_fingerprint: Option<Box<JA3Fingerprint_>>,
        pub ja4_fingerprint: Option<Box<JA4Fingerprint_>>,
        pub json_body: Option<Box<JsonBody_>>,
        pub method: Option<serde_json::Value>,
        pub query_string: Option<serde_json::Value>,
        pub single_header: Option<Box<SingleHeader_>>,
        pub single_query_argument: Option<Box<SingleQueryArgument_>>,
        pub uri_fragment: Option<Box<UriFragment_>>,
        pub uri_path: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_FieldToMatch {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.FieldToMatch"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_FieldToMatch as FieldToMatch;
    impl crate::value::ToValue for FieldToMatch_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all_query_arguments {
                properties.insert(
                    "AllQueryArguments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cookies {
                properties.insert(
                    "Cookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_order {
                properties.insert(
                    "HeaderOrder".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.headers {
                properties.insert(
                    "Headers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ja3_fingerprint {
                properties.insert(
                    "JA3Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ja4_fingerprint {
                properties.insert(
                    "JA4Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_body {
                properties.insert(
                    "JsonBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.method {
                properties.insert("Method".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_header {
                properties.insert(
                    "SingleHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.single_query_argument {
                properties.insert(
                    "SingleQueryArgument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_fragment {
                properties.insert(
                    "UriFragment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_path {
                properties.insert(
                    "UriPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-fieldtoprotect.html>
    pub struct FieldToProtect_ {
        pub field_keys: Option<Vec<crate::value::ExpString>>,
        pub field_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_FieldToProtect {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.FieldToProtect"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_FieldToProtect as FieldToProtect;
    impl crate::value::ToValue for FieldToProtect_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.field_keys {
                properties.insert(
                    "FieldKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "FieldType".to_string(),
                crate::value::ToValue::to_value(&self.field_type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-forwardedipconfiguration.html>
    pub struct ForwardedIPConfiguration_ {
        pub fallback_behavior: crate::value::ExpString,
        pub header_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ForwardedIPConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ForwardedIPConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ForwardedIPConfiguration as ForwardedIPConfiguration;
    impl crate::value::ToValue for ForwardedIPConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.insert(
                "HeaderName".to_string(),
                crate::value::ToValue::to_value(&self.header_name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-geomatchstatement.html>
    pub struct GeoMatchStatement_ {
        pub country_codes: Option<Vec<crate::value::ExpString>>,
        pub forwarded_ip_config: Option<Box<ForwardedIPConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_GeoMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.GeoMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_GeoMatchStatement as GeoMatchStatement;
    impl crate::value::ToValue for GeoMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.country_codes {
                properties.insert(
                    "CountryCodes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_ip_config {
                properties.insert(
                    "ForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headermatchpattern.html>
    pub struct HeaderMatchPattern_ {
        pub all: Option<serde_json::Value>,
        pub excluded_headers: Option<Vec<crate::value::ExpString>>,
        pub included_headers: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_HeaderMatchPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.HeaderMatchPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_HeaderMatchPattern as HeaderMatchPattern;
    impl crate::value::ToValue for HeaderMatchPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all {
                properties.insert("All".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.excluded_headers {
                properties.insert(
                    "ExcludedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.included_headers {
                properties.insert(
                    "IncludedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headerorder.html>
    pub struct HeaderOrder_ {
        pub oversize_handling: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_HeaderOrder {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.HeaderOrder"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_HeaderOrder as HeaderOrder;
    impl crate::value::ToValue for HeaderOrder_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OversizeHandling".to_string(),
                crate::value::ToValue::to_value(&self.oversize_handling),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-headers.html>
    pub struct Headers_ {
        pub match_pattern: Box<HeaderMatchPattern_>,
        pub match_scope: crate::value::ExpString,
        pub oversize_handling: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_Headers {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.Headers"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_Headers as Headers;
    impl crate::value::ToValue for Headers_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "MatchPattern".to_string(),
                crate::value::ToValue::to_value(&self.match_pattern),
            );
            properties.insert(
                "MatchScope".to_string(),
                crate::value::ToValue::to_value(&self.match_scope),
            );
            properties.insert(
                "OversizeHandling".to_string(),
                crate::value::ToValue::to_value(&self.oversize_handling),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetforwardedipconfiguration.html>
    pub struct IPSetForwardedIPConfiguration_ {
        pub fallback_behavior: crate::value::ExpString,
        pub header_name: crate::value::ExpString,
        pub position: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_IPSetForwardedIPConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.IPSetForwardedIPConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_IPSetForwardedIPConfiguration as IPSetForwardedIPConfiguration;
    impl crate::value::ToValue for IPSetForwardedIPConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.insert(
                "HeaderName".to_string(),
                crate::value::ToValue::to_value(&self.header_name),
            );
            properties.insert(
                "Position".to_string(),
                crate::value::ToValue::to_value(&self.position),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ipsetreferencestatement.html>
    pub struct IPSetReferenceStatement_ {
        pub arn: crate::value::ExpString,
        pub ip_set_forwarded_ip_config: Option<Box<IPSetForwardedIPConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_IPSetReferenceStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.IPSetReferenceStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_IPSetReferenceStatement as IPSetReferenceStatement;
    impl crate::value::ToValue for IPSetReferenceStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.ip_set_forwarded_ip_config {
                properties.insert(
                    "IPSetForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-immunitytimeproperty.html>
    pub struct ImmunityTimeProperty_ {
        pub immunity_time: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ImmunityTimeProperty {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ImmunityTimeProperty"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ImmunityTimeProperty as ImmunityTimeProperty;
    impl crate::value::ToValue for ImmunityTimeProperty_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ImmunityTime".to_string(),
                crate::value::ToValue::to_value(&self.immunity_time),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ja3fingerprint.html>
    pub struct JA3Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_JA3Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.JA3Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_JA3Fingerprint as JA3Fingerprint;
    impl crate::value::ToValue for JA3Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ja4fingerprint.html>
    pub struct JA4Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_JA4Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.JA4Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_JA4Fingerprint as JA4Fingerprint;
    impl crate::value::ToValue for JA4Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonbody.html>
    pub struct JsonBody_ {
        pub invalid_fallback_behavior: Option<crate::value::ExpString>,
        pub match_pattern: Box<JsonMatchPattern_>,
        pub match_scope: crate::value::ExpString,
        pub oversize_handling: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_JsonBody {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.JsonBody"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_JsonBody as JsonBody;
    impl crate::value::ToValue for JsonBody_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.invalid_fallback_behavior {
                properties.insert(
                    "InvalidFallbackBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MatchPattern".to_string(),
                crate::value::ToValue::to_value(&self.match_pattern),
            );
            properties.insert(
                "MatchScope".to_string(),
                crate::value::ToValue::to_value(&self.match_scope),
            );
            if let Some(ref value) = self.oversize_handling {
                properties.insert(
                    "OversizeHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-jsonmatchpattern.html>
    pub struct JsonMatchPattern_ {
        pub all: Option<serde_json::Value>,
        pub included_paths: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_JsonMatchPattern {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.JsonMatchPattern"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_JsonMatchPattern as JsonMatchPattern;
    impl crate::value::ToValue for JsonMatchPattern_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all {
                properties.insert("All".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.included_paths {
                properties.insert(
                    "IncludedPaths".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-label.html>
    pub struct Label_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_Label {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.Label"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_Label as Label;
    impl crate::value::ToValue for Label_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-labelmatchstatement.html>
    pub struct LabelMatchStatement_ {
        pub key: crate::value::ExpString,
        pub scope: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_LabelMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.LabelMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_LabelMatchStatement as LabelMatchStatement;
    impl crate::value::ToValue for LabelMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Scope".to_string(),
                crate::value::ToValue::to_value(&self.scope),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupconfig.html>
    pub struct ManagedRuleGroupConfig_ {
        pub aws_managed_rules_acfp_rule_set: Option<Box<AWSManagedRulesACFPRuleSet_>>,
        pub aws_managed_rules_atp_rule_set: Option<Box<AWSManagedRulesATPRuleSet_>>,
        pub aws_managed_rules_anti_d_do_s_rule_set: Option<Box<AWSManagedRulesAntiDDoSRuleSet_>>,
        pub aws_managed_rules_bot_control_rule_set: Option<Box<AWSManagedRulesBotControlRuleSet_>>,
        pub login_path: Option<crate::value::ExpString>,
        pub password_field: Option<Box<FieldIdentifier_>>,
        pub payload_type: Option<crate::value::ExpString>,
        pub username_field: Option<Box<FieldIdentifier_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ManagedRuleGroupConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ManagedRuleGroupConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ManagedRuleGroupConfig as ManagedRuleGroupConfig;
    impl crate::value::ToValue for ManagedRuleGroupConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_managed_rules_acfp_rule_set {
                properties.insert(
                    "AWSManagedRulesACFPRuleSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_managed_rules_atp_rule_set {
                properties.insert(
                    "AWSManagedRulesATPRuleSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_managed_rules_anti_d_do_s_rule_set {
                properties.insert(
                    "AWSManagedRulesAntiDDoSRuleSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.aws_managed_rules_bot_control_rule_set {
                properties.insert(
                    "AWSManagedRulesBotControlRuleSet".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.login_path {
                properties.insert(
                    "LoginPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password_field {
                properties.insert(
                    "PasswordField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload_type {
                properties.insert(
                    "PayloadType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username_field {
                properties.insert(
                    "UsernameField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-managedrulegroupstatement.html>
    pub struct ManagedRuleGroupStatement_ {
        pub excluded_rules: Option<Vec<ExcludedRule_>>,
        pub managed_rule_group_configs: Option<Vec<ManagedRuleGroupConfig_>>,
        pub name: crate::value::ExpString,
        pub rule_action_overrides: Option<Vec<RuleActionOverride_>>,
        pub scope_down_statement: Option<Box<Statement_>>,
        pub vendor_name: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ManagedRuleGroupStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ManagedRuleGroupStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ManagedRuleGroupStatement as ManagedRuleGroupStatement;
    impl crate::value::ToValue for ManagedRuleGroupStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.excluded_rules {
                properties.insert(
                    "ExcludedRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_rule_group_configs {
                properties.insert(
                    "ManagedRuleGroupConfigs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.rule_action_overrides {
                properties.insert(
                    "RuleActionOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scope_down_statement {
                properties.insert(
                    "ScopeDownStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "VendorName".to_string(),
                crate::value::ToValue::to_value(&self.vendor_name),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-notstatement.html>
    pub struct NotStatement_ {
        pub statement: Box<Statement_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_NotStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.NotStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_NotStatement as NotStatement;
    impl crate::value::ToValue for NotStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Statement".to_string(),
                crate::value::ToValue::to_value(&self.statement),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-onsourceddosprotectionconfig.html>
    pub struct OnSourceDDoSProtectionConfig_ {
        pub alb_low_reputation_mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_OnSourceDDoSProtectionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.OnSourceDDoSProtectionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_OnSourceDDoSProtectionConfig as OnSourceDDoSProtectionConfig;
    impl crate::value::ToValue for OnSourceDDoSProtectionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ALBLowReputationMode".to_string(),
                crate::value::ToValue::to_value(&self.alb_low_reputation_mode),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-orstatement.html>
    pub struct OrStatement_ {
        pub statements: Vec<Statement_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_OrStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.OrStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_OrStatement as OrStatement;
    impl crate::value::ToValue for OrStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Statements".to_string(),
                crate::value::ToValue::to_value(&self.statements),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-overrideaction.html>
    pub struct OverrideAction_ {
        pub count: Option<serde_json::Value>,
        pub none: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_OverrideAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.OverrideAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_OverrideAction as OverrideAction;
    impl crate::value::ToValue for OverrideAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.none {
                properties.insert("None".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatement.html>
    pub struct RateBasedStatement_ {
        pub aggregate_key_type: crate::value::ExpString,
        pub custom_keys: Option<Vec<RateBasedStatementCustomKey_>>,
        pub evaluation_window_sec: Option<i32>,
        pub forwarded_ip_config: Option<Box<ForwardedIPConfiguration_>>,
        pub limit: i32,
        pub scope_down_statement: Option<Box<Statement_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateBasedStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateBasedStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateBasedStatement as RateBasedStatement;
    impl crate::value::ToValue for RateBasedStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AggregateKeyType".to_string(),
                crate::value::ToValue::to_value(&self.aggregate_key_type),
            );
            if let Some(ref value) = self.custom_keys {
                properties.insert(
                    "CustomKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.evaluation_window_sec {
                properties.insert(
                    "EvaluationWindowSec".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_ip_config {
                properties.insert(
                    "ForwardedIPConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Limit".to_string(),
                crate::value::ToValue::to_value(&self.limit),
            );
            if let Some(ref value) = self.scope_down_statement {
                properties.insert(
                    "ScopeDownStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratebasedstatementcustomkey.html>
    pub struct RateBasedStatementCustomKey_ {
        pub asn: Option<serde_json::Value>,
        pub cookie: Option<Box<RateLimitCookie_>>,
        pub forwarded_ip: Option<serde_json::Value>,
        pub http_method: Option<serde_json::Value>,
        pub header: Option<Box<RateLimitHeader_>>,
        pub ip: Option<serde_json::Value>,
        pub ja3_fingerprint: Option<Box<RateLimitJA3Fingerprint_>>,
        pub ja4_fingerprint: Option<Box<RateLimitJA4Fingerprint_>>,
        pub label_namespace: Option<Box<RateLimitLabelNamespace_>>,
        pub query_argument: Option<Box<RateLimitQueryArgument_>>,
        pub query_string: Option<Box<RateLimitQueryString_>>,
        pub uri_path: Option<Box<RateLimitUriPath_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateBasedStatementCustomKey {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateBasedStatementCustomKey"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateBasedStatementCustomKey as RateBasedStatementCustomKey;
    impl crate::value::ToValue for RateBasedStatementCustomKey_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.asn {
                properties.insert("ASN".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.cookie {
                properties.insert("Cookie".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.forwarded_ip {
                properties.insert(
                    "ForwardedIP".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_method {
                properties.insert(
                    "HTTPMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ip {
                properties.insert("IP".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ja3_fingerprint {
                properties.insert(
                    "JA3Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ja4_fingerprint {
                properties.insert(
                    "JA4Fingerprint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_namespace {
                properties.insert(
                    "LabelNamespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_argument {
                properties.insert(
                    "QueryArgument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.query_string {
                properties.insert(
                    "QueryString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.uri_path {
                properties.insert(
                    "UriPath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitcookie.html>
    pub struct RateLimitCookie_ {
        pub name: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitCookie {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitCookie"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitCookie as RateLimitCookie;
    impl crate::value::ToValue for RateLimitCookie_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitheader.html>
    pub struct RateLimitHeader_ {
        pub name: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitHeader as RateLimitHeader;
    impl crate::value::ToValue for RateLimitHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitja3fingerprint.html>
    pub struct RateLimitJA3Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitJA3Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitJA3Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitJA3Fingerprint as RateLimitJA3Fingerprint;
    impl crate::value::ToValue for RateLimitJA3Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitja4fingerprint.html>
    pub struct RateLimitJA4Fingerprint_ {
        pub fallback_behavior: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitJA4Fingerprint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitJA4Fingerprint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitJA4Fingerprint as RateLimitJA4Fingerprint;
    impl crate::value::ToValue for RateLimitJA4Fingerprint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FallbackBehavior".to_string(),
                crate::value::ToValue::to_value(&self.fallback_behavior),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitlabelnamespace.html>
    pub struct RateLimitLabelNamespace_ {
        pub namespace: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitLabelNamespace {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitLabelNamespace"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitLabelNamespace as RateLimitLabelNamespace;
    impl crate::value::ToValue for RateLimitLabelNamespace_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Namespace".to_string(),
                crate::value::ToValue::to_value(&self.namespace),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitqueryargument.html>
    pub struct RateLimitQueryArgument_ {
        pub name: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitQueryArgument {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitQueryArgument"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitQueryArgument as RateLimitQueryArgument;
    impl crate::value::ToValue for RateLimitQueryArgument_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimitquerystring.html>
    pub struct RateLimitQueryString_ {
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitQueryString {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitQueryString"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitQueryString as RateLimitQueryString;
    impl crate::value::ToValue for RateLimitQueryString_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ratelimituripath.html>
    pub struct RateLimitUriPath_ {
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RateLimitUriPath {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RateLimitUriPath"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RateLimitUriPath as RateLimitUriPath;
    impl crate::value::ToValue for RateLimitUriPath_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regex.html>
    pub struct Regex_ {
        pub regex_string: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_Regex {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.Regex"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_Regex as Regex;
    impl crate::value::ToValue for Regex_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.regex_string {
                properties.insert(
                    "RegexString".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexmatchstatement.html>
    pub struct RegexMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub regex_string: crate::value::ExpString,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RegexMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RegexMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RegexMatchStatement as RegexMatchStatement;
    impl crate::value::ToValue for RegexMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "RegexString".to_string(),
                crate::value::ToValue::to_value(&self.regex_string),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-regexpatternsetreferencestatement.html>
    pub struct RegexPatternSetReferenceStatement_ {
        pub arn: crate::value::ExpString,
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RegexPatternSetReferenceStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RegexPatternSetReferenceStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RegexPatternSetReferenceStatement as RegexPatternSetReferenceStatement;
    impl crate::value::ToValue for RegexPatternSetReferenceStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestbodyassociatedresourcetypeconfig.html>
    pub struct RequestBodyAssociatedResourceTypeConfig_ {
        pub default_size_inspection_limit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RequestBodyAssociatedResourceTypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RequestBodyAssociatedResourceTypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RequestBodyAssociatedResourceTypeConfig as RequestBodyAssociatedResourceTypeConfig;
    impl crate::value::ToValue for RequestBodyAssociatedResourceTypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultSizeInspectionLimit".to_string(),
                crate::value::ToValue::to_value(&self.default_size_inspection_limit),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspection.html>
    pub struct RequestInspection_ {
        pub password_field: Box<FieldIdentifier_>,
        pub payload_type: crate::value::ExpString,
        pub username_field: Box<FieldIdentifier_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RequestInspection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RequestInspection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RequestInspection as RequestInspection;
    impl crate::value::ToValue for RequestInspection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PasswordField".to_string(),
                crate::value::ToValue::to_value(&self.password_field),
            );
            properties.insert(
                "PayloadType".to_string(),
                crate::value::ToValue::to_value(&self.payload_type),
            );
            properties.insert(
                "UsernameField".to_string(),
                crate::value::ToValue::to_value(&self.username_field),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-requestinspectionacfp.html>
    pub struct RequestInspectionACFP_ {
        pub address_fields: Option<Vec<FieldIdentifier_>>,
        pub email_field: Option<Box<FieldIdentifier_>>,
        pub password_field: Option<Box<FieldIdentifier_>>,
        pub payload_type: crate::value::ExpString,
        pub phone_number_fields: Option<Vec<FieldIdentifier_>>,
        pub username_field: Option<Box<FieldIdentifier_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RequestInspectionACFP {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RequestInspectionACFP"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RequestInspectionACFP as RequestInspectionACFP;
    impl crate::value::ToValue for RequestInspectionACFP_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address_fields {
                properties.insert(
                    "AddressFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_field {
                properties.insert(
                    "EmailField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.password_field {
                properties.insert(
                    "PasswordField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "PayloadType".to_string(),
                crate::value::ToValue::to_value(&self.payload_type),
            );
            if let Some(ref value) = self.phone_number_fields {
                properties.insert(
                    "PhoneNumberFields".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.username_field {
                properties.insert(
                    "UsernameField".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspection.html>
    pub struct ResponseInspection_ {
        pub body_contains: Option<Box<ResponseInspectionBodyContains_>>,
        pub header: Option<Box<ResponseInspectionHeader_>>,
        pub json: Option<Box<ResponseInspectionJson_>>,
        pub status_code: Option<Box<ResponseInspectionStatusCode_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ResponseInspection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ResponseInspection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ResponseInspection as ResponseInspection;
    impl crate::value::ToValue for ResponseInspection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.body_contains {
                properties.insert(
                    "BodyContains".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.json {
                properties.insert("Json".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status_code {
                properties.insert(
                    "StatusCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionbodycontains.html>
    pub struct ResponseInspectionBodyContains_ {
        pub failure_strings: Vec<crate::value::ExpString>,
        pub success_strings: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ResponseInspectionBodyContains {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ResponseInspectionBodyContains"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ResponseInspectionBodyContains as ResponseInspectionBodyContains;
    impl crate::value::ToValue for ResponseInspectionBodyContains_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailureStrings".to_string(),
                crate::value::ToValue::to_value(&self.failure_strings),
            );
            properties.insert(
                "SuccessStrings".to_string(),
                crate::value::ToValue::to_value(&self.success_strings),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionheader.html>
    pub struct ResponseInspectionHeader_ {
        pub failure_values: Vec<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub success_values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ResponseInspectionHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ResponseInspectionHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ResponseInspectionHeader as ResponseInspectionHeader;
    impl crate::value::ToValue for ResponseInspectionHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailureValues".to_string(),
                crate::value::ToValue::to_value(&self.failure_values),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "SuccessValues".to_string(),
                crate::value::ToValue::to_value(&self.success_values),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionjson.html>
    pub struct ResponseInspectionJson_ {
        pub failure_values: Vec<crate::value::ExpString>,
        pub identifier: crate::value::ExpString,
        pub success_values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ResponseInspectionJson {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ResponseInspectionJson"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ResponseInspectionJson as ResponseInspectionJson;
    impl crate::value::ToValue for ResponseInspectionJson_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailureValues".to_string(),
                crate::value::ToValue::to_value(&self.failure_values),
            );
            properties.insert(
                "Identifier".to_string(),
                crate::value::ToValue::to_value(&self.identifier),
            );
            properties.insert(
                "SuccessValues".to_string(),
                crate::value::ToValue::to_value(&self.success_values),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-responseinspectionstatuscode.html>
    pub struct ResponseInspectionStatusCode_ {
        pub failure_codes: Vec<i32>,
        pub success_codes: Vec<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_ResponseInspectionStatusCode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.ResponseInspectionStatusCode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_ResponseInspectionStatusCode as ResponseInspectionStatusCode;
    impl crate::value::ToValue for ResponseInspectionStatusCode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FailureCodes".to_string(),
                crate::value::ToValue::to_value(&self.failure_codes),
            );
            properties.insert(
                "SuccessCodes".to_string(),
                crate::value::ToValue::to_value(&self.success_codes),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rule.html>
    pub struct Rule_ {
        pub action: Option<Box<RuleAction_>>,
        pub captcha_config: Option<Box<CaptchaConfig_>>,
        pub challenge_config: Option<Box<ChallengeConfig_>>,
        pub name: crate::value::ExpString,
        pub override_action: Option<Box<OverrideAction_>>,
        pub priority: i32,
        pub rule_labels: Option<Vec<Label_>>,
        pub statement: Box<Statement_>,
        pub visibility_config: Box<VisibilityConfig_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.captcha_config {
                properties.insert(
                    "CaptchaConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.challenge_config {
                properties.insert(
                    "ChallengeConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.override_action {
                properties.insert(
                    "OverrideAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            if let Some(ref value) = self.rule_labels {
                properties.insert(
                    "RuleLabels".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Statement".to_string(),
                crate::value::ToValue::to_value(&self.statement),
            );
            properties.insert(
                "VisibilityConfig".to_string(),
                crate::value::ToValue::to_value(&self.visibility_config),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleaction.html>
    pub struct RuleAction_ {
        pub allow: Option<Box<AllowAction_>>,
        pub block: Option<Box<BlockAction_>>,
        pub captcha: Option<Box<CaptchaAction_>>,
        pub challenge: Option<Box<ChallengeAction_>>,
        pub count: Option<Box<CountAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RuleAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RuleAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RuleAction as RuleAction;
    impl crate::value::ToValue for RuleAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow {
                properties.insert("Allow".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.block {
                properties.insert("Block".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.captcha {
                properties.insert(
                    "Captcha".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.challenge {
                properties.insert(
                    "Challenge".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.count {
                properties.insert("Count".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-ruleactionoverride.html>
    pub struct RuleActionOverride_ {
        pub action_to_use: Box<RuleAction_>,
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RuleActionOverride {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RuleActionOverride"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RuleActionOverride as RuleActionOverride;
    impl crate::value::ToValue for RuleActionOverride_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ActionToUse".to_string(),
                crate::value::ToValue::to_value(&self.action_to_use),
            );
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-rulegroupreferencestatement.html>
    pub struct RuleGroupReferenceStatement_ {
        pub arn: crate::value::ExpString,
        pub excluded_rules: Option<Vec<ExcludedRule_>>,
        pub rule_action_overrides: Option<Vec<RuleActionOverride_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_RuleGroupReferenceStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.RuleGroupReferenceStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_RuleGroupReferenceStatement as RuleGroupReferenceStatement;
    impl crate::value::ToValue for RuleGroupReferenceStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Arn".to_string(),
                crate::value::ToValue::to_value(&self.arn),
            );
            if let Some(ref value) = self.excluded_rules {
                properties.insert(
                    "ExcludedRules".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_action_overrides {
                properties.insert(
                    "RuleActionOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-singleheader.html>
    pub struct SingleHeader_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_SingleHeader {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.SingleHeader"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_SingleHeader as SingleHeader;
    impl crate::value::ToValue for SingleHeader_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-singlequeryargument.html>
    pub struct SingleQueryArgument_ {
        pub name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_SingleQueryArgument {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.SingleQueryArgument"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_SingleQueryArgument as SingleQueryArgument;
    impl crate::value::ToValue for SingleQueryArgument_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sizeconstraintstatement.html>
    pub struct SizeConstraintStatement_ {
        pub comparison_operator: crate::value::ExpString,
        pub field_to_match: Box<FieldToMatch_>,
        pub size: f64,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_SizeConstraintStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.SizeConstraintStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_SizeConstraintStatement as SizeConstraintStatement;
    impl crate::value::ToValue for SizeConstraintStatement_ {
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
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-sqlimatchstatement.html>
    pub struct SqliMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub sensitivity_level: Option<crate::value::ExpString>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_SqliMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.SqliMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_SqliMatchStatement as SqliMatchStatement;
    impl crate::value::ToValue for SqliMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            if let Some(ref value) = self.sensitivity_level {
                properties.insert(
                    "SensitivityLevel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-statement.html>
    pub struct Statement_ {
        pub and_statement: Option<Box<AndStatement_>>,
        pub asn_match_statement: Option<Box<AsnMatchStatement_>>,
        pub byte_match_statement: Option<Box<ByteMatchStatement_>>,
        pub geo_match_statement: Option<Box<GeoMatchStatement_>>,
        pub ip_set_reference_statement: Option<Box<IPSetReferenceStatement_>>,
        pub label_match_statement: Option<Box<LabelMatchStatement_>>,
        pub managed_rule_group_statement: Option<Box<ManagedRuleGroupStatement_>>,
        pub not_statement: Option<Box<NotStatement_>>,
        pub or_statement: Option<Box<OrStatement_>>,
        pub rate_based_statement: Option<Box<RateBasedStatement_>>,
        pub regex_match_statement: Option<Box<RegexMatchStatement_>>,
        pub regex_pattern_set_reference_statement: Option<Box<RegexPatternSetReferenceStatement_>>,
        pub rule_group_reference_statement: Option<Box<RuleGroupReferenceStatement_>>,
        pub size_constraint_statement: Option<Box<SizeConstraintStatement_>>,
        pub sqli_match_statement: Option<Box<SqliMatchStatement_>>,
        pub xss_match_statement: Option<Box<XssMatchStatement_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_Statement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.Statement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_Statement as Statement;
    impl crate::value::ToValue for Statement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.and_statement {
                properties.insert(
                    "AndStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.asn_match_statement {
                properties.insert(
                    "AsnMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.byte_match_statement {
                properties.insert(
                    "ByteMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.geo_match_statement {
                properties.insert(
                    "GeoMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_set_reference_statement {
                properties.insert(
                    "IPSetReferenceStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.label_match_statement {
                properties.insert(
                    "LabelMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_rule_group_statement {
                properties.insert(
                    "ManagedRuleGroupStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_statement {
                properties.insert(
                    "NotStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.or_statement {
                properties.insert(
                    "OrStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rate_based_statement {
                properties.insert(
                    "RateBasedStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regex_match_statement {
                properties.insert(
                    "RegexMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.regex_pattern_set_reference_statement {
                properties.insert(
                    "RegexPatternSetReferenceStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.rule_group_reference_statement {
                properties.insert(
                    "RuleGroupReferenceStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_constraint_statement {
                properties.insert(
                    "SizeConstraintStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sqli_match_statement {
                properties.insert(
                    "SqliMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.xss_match_statement {
                properties.insert(
                    "XssMatchStatement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-texttransformation.html>
    pub struct TextTransformation_ {
        pub priority: i32,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_TextTransformation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.TextTransformation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_TextTransformation as TextTransformation;
    impl crate::value::ToValue for TextTransformation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(&self.priority),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-urifragment.html>
    pub struct UriFragment_ {
        pub fallback_behavior: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_UriFragment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.UriFragment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_UriFragment as UriFragment;
    impl crate::value::ToValue for UriFragment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.fallback_behavior {
                properties.insert(
                    "FallbackBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-visibilityconfig.html>
    pub struct VisibilityConfig_ {
        pub cloud_watch_metrics_enabled: crate::value::ExpBool,
        pub metric_name: crate::value::ExpString,
        pub sampled_requests_enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_VisibilityConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.VisibilityConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_VisibilityConfig as VisibilityConfig;
    impl crate::value::ToValue for VisibilityConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchMetricsEnabled".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_metrics_enabled),
            );
            properties.insert(
                "MetricName".to_string(),
                crate::value::ToValue::to_value(&self.metric_name),
            );
            properties.insert(
                "SampledRequestsEnabled".to_string(),
                crate::value::ToValue::to_value(&self.sampled_requests_enabled),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-wafv2-webacl-xssmatchstatement.html>
    pub struct XssMatchStatement_ {
        pub field_to_match: Box<FieldToMatch_>,
        pub text_transformations: Vec<TextTransformation_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_wafv2_WebACL_XssMatchStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WAFv2::WebACL.XssMatchStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_wafv2_WebACL_XssMatchStatement as XssMatchStatement;
    impl crate::value::ToValue for XssMatchStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldToMatch".to_string(),
                crate::value::ToValue::to_value(&self.field_to_match),
            );
            properties.insert(
                "TextTransformations".to_string(),
                crate::value::ToValue::to_value(&self.text_transformations),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-ipset.html>
pub struct IPSet_ {
    pub addresses: Vec<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub ip_address_version: crate::value::ExpString,
    pub name: Option<crate::value::ExpString>,
    pub scope: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafv2_IPSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFv2::IPSet" $($field
        $value)*)
    };
}
pub use crate::__aws_wafv2_IPSet as IPSet;
impl crate::template::ToResource for IPSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFv2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("IPSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Addresses".to_string(),
            crate::value::ToValue::to_value(&self.addresses),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IPAddressVersion".to_string(),
            crate::value::ToValue::to_value(&self.ip_address_version),
        );
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Scope".to_string(),
            crate::value::ToValue::to_value(&self.scope),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-loggingconfiguration.html>
pub struct LoggingConfiguration_ {
    pub log_destination_configs: Vec<crate::value::ExpString>,
    pub logging_filter: Option<super::wafv2::loggingconfiguration::LoggingFilter_>,
    pub redacted_fields: Option<Vec<super::wafv2::loggingconfiguration::FieldToMatch_>>,
    pub resource_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafv2_LoggingConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFv2::LoggingConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_wafv2_LoggingConfiguration as LoggingConfiguration;
impl crate::template::ToResource for LoggingConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFv2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoggingConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "LogDestinationConfigs".to_string(),
            crate::value::ToValue::to_value(&self.log_destination_configs),
        );
        if let Some(ref value) = self.logging_filter {
            properties.insert(
                "LoggingFilter".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redacted_fields {
            properties.insert(
                "RedactedFields".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-regexpatternset.html>
pub struct RegexPatternSet_ {
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub regular_expression_list: Vec<crate::value::ExpString>,
    pub scope: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafv2_RegexPatternSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFv2::RegexPatternSet"
        $($field $value)*)
    };
}
pub use crate::__aws_wafv2_RegexPatternSet as RegexPatternSet;
impl crate::template::ToResource for RegexPatternSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFv2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RegexPatternSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "RegularExpressionList".to_string(),
            crate::value::ToValue::to_value(&self.regular_expression_list),
        );
        properties.insert(
            "Scope".to_string(),
            crate::value::ToValue::to_value(&self.scope),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-rulegroup.html>
pub struct RuleGroup_ {
    pub available_labels: Option<Vec<super::wafv2::rulegroup::LabelSummary_>>,
    pub capacity: i32,
    pub consumed_labels: Option<Vec<super::wafv2::rulegroup::LabelSummary_>>,
    pub custom_response_bodies:
        Option<std::collections::BTreeMap<String, super::wafv2::rulegroup::CustomResponseBody_>>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub rules: Option<Vec<super::wafv2::rulegroup::Rule_>>,
    pub scope: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub visibility_config: super::wafv2::rulegroup::VisibilityConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafv2_RuleGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFv2::RuleGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_wafv2_RuleGroup as RuleGroup;
impl crate::template::ToResource for RuleGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFv2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RuleGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.available_labels {
            properties.insert(
                "AvailableLabels".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Capacity".to_string(),
            crate::value::ToValue::to_value(&self.capacity),
        );
        if let Some(ref value) = self.consumed_labels {
            properties.insert(
                "ConsumedLabels".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_response_bodies {
            properties.insert(
                "CustomResponseBodies".to_string(),
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
        if let Some(ref value) = self.rules {
            properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Scope".to_string(),
            crate::value::ToValue::to_value(&self.scope),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VisibilityConfig".to_string(),
            crate::value::ToValue::to_value(&self.visibility_config),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webacl.html>
pub struct WebACL_ {
    pub application_config: Option<super::wafv2::webacl::ApplicationConfig_>,
    pub association_config: Option<super::wafv2::webacl::AssociationConfig_>,
    pub captcha_config: Option<super::wafv2::webacl::CaptchaConfig_>,
    pub challenge_config: Option<super::wafv2::webacl::ChallengeConfig_>,
    pub custom_response_bodies:
        Option<std::collections::BTreeMap<String, super::wafv2::webacl::CustomResponseBody_>>,
    pub data_protection_config: Option<super::wafv2::webacl::DataProtectionConfig_>,
    pub default_action: super::wafv2::webacl::DefaultAction_,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub on_source_d_do_s_protection_config:
        Option<super::wafv2::webacl::OnSourceDDoSProtectionConfig_>,
    pub rules: Option<Vec<super::wafv2::webacl::Rule_>>,
    pub scope: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub token_domains: Option<Vec<crate::value::ExpString>>,
    pub visibility_config: super::wafv2::webacl::VisibilityConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafv2_WebACL {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFv2::WebACL" $($field
        $value)*)
    };
}
pub use crate::__aws_wafv2_WebACL as WebACL;
impl crate::template::ToResource for WebACL_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFv2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WebACL"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.application_config {
            properties.insert(
                "ApplicationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.association_config {
            properties.insert(
                "AssociationConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.captcha_config {
            properties.insert(
                "CaptchaConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.challenge_config {
            properties.insert(
                "ChallengeConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_response_bodies {
            properties.insert(
                "CustomResponseBodies".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_protection_config {
            properties.insert(
                "DataProtectionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultAction".to_string(),
            crate::value::ToValue::to_value(&self.default_action),
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
        if let Some(ref value) = self.on_source_d_do_s_protection_config {
            properties.insert(
                "OnSourceDDoSProtectionConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rules {
            properties.insert("Rules".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Scope".to_string(),
            crate::value::ToValue::to_value(&self.scope),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.token_domains {
            properties.insert(
                "TokenDomains".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VisibilityConfig".to_string(),
            crate::value::ToValue::to_value(&self.visibility_config),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-wafv2-webaclassociation.html>
pub struct WebACLAssociation_ {
    pub resource_arn: crate::value::ExpString,
    pub web_acl_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_wafv2_WebACLAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WAFv2::WebACLAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_wafv2_WebACLAssociation as WebACLAssociation;
impl crate::template::ToResource for WebACLAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WAFv2"),
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
            "WebACLArn".to_string(),
            crate::value::ToValue::to_value(&self.web_acl_arn),
        );
        properties
    }
}
