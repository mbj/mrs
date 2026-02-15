pub mod inboundexternallink {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-inboundexternallink-applicationlogs.html
    pub struct ApplicationLogs_ {
        pub link_application_log_sampling: Box<LinkApplicationLogSampling_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_InboundExternalLink_ApplicationLogs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::InboundExternalLink.ApplicationLogs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_InboundExternalLink_ApplicationLogs as ApplicationLogs;
    impl crate::value::ToValue for ApplicationLogs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LinkApplicationLogSampling".to_string(),
                crate::value::ToValue::to_value(&self.link_application_log_sampling),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-inboundexternallink-linkapplicationlogsampling.html
    pub struct LinkApplicationLogSampling_ {
        pub error_log: f64,
        pub filter_log: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_InboundExternalLink_LinkApplicationLogSampling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::InboundExternalLink.LinkApplicationLogSampling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_InboundExternalLink_LinkApplicationLogSampling as LinkApplicationLogSampling;
    impl crate::value::ToValue for LinkApplicationLogSampling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ErrorLog".to_string(),
                crate::value::ToValue::to_value(&self.error_log),
            );
            properties.insert(
                "FilterLog".to_string(),
                crate::value::ToValue::to_value(&self.filter_log),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-inboundexternallink-linkattributes.html
    pub struct LinkAttributes_ {
        pub customer_provided_id: Option<crate::value::ExpString>,
        pub responder_error_masking: Option<Vec<ResponderErrorMaskingForHttpCode_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_InboundExternalLink_LinkAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::InboundExternalLink.LinkAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_InboundExternalLink_LinkAttributes as LinkAttributes;
    impl crate::value::ToValue for LinkAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_provided_id {
                properties.insert(
                    "CustomerProvidedId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.responder_error_masking {
                properties.insert(
                    "ResponderErrorMasking".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-inboundexternallink-linklogsettings.html
    pub struct LinkLogSettings_ {
        pub application_logs: Box<ApplicationLogs_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_InboundExternalLink_LinkLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::InboundExternalLink.LinkLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_InboundExternalLink_LinkLogSettings as LinkLogSettings;
    impl crate::value::ToValue for LinkLogSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationLogs".to_string(),
                crate::value::ToValue::to_value(&self.application_logs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-inboundexternallink-respondererrormaskingforhttpcode.html
    pub struct ResponderErrorMaskingForHttpCode_ {
        pub action: crate::value::ExpString,
        pub http_code: crate::value::ExpString,
        pub logging_types: Vec<crate::value::ExpString>,
        pub response_logging_percentage: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_InboundExternalLink_ResponderErrorMaskingForHttpCode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::InboundExternalLink.ResponderErrorMaskingForHttpCode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_InboundExternalLink_ResponderErrorMaskingForHttpCode as ResponderErrorMaskingForHttpCode;
    impl crate::value::ToValue for ResponderErrorMaskingForHttpCode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "HttpCode".to_string(),
                crate::value::ToValue::to_value(&self.http_code),
            );
            properties.insert(
                "LoggingTypes".to_string(),
                crate::value::ToValue::to_value(&self.logging_types),
            );
            if let Some(ref value) = self.response_logging_percentage {
                properties.insert(
                    "ResponseLoggingPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod link {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-action.html
    pub struct Action_ {
        pub header_tag: Option<Box<HeaderTagAction_>>,
        pub no_bid: Option<Box<NoBidAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.header_tag {
                properties.insert(
                    "HeaderTag".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.no_bid {
                properties.insert("NoBid".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-applicationlogs.html
    pub struct ApplicationLogs_ {
        pub link_application_log_sampling: Box<LinkApplicationLogSampling_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_ApplicationLogs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.ApplicationLogs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_ApplicationLogs as ApplicationLogs;
    impl crate::value::ToValue for ApplicationLogs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LinkApplicationLogSampling".to_string(),
                crate::value::ToValue::to_value(&self.link_application_log_sampling),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-filter.html
    pub struct Filter_ {
        pub criteria: Vec<FilterCriterion_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Criteria".to_string(),
                crate::value::ToValue::to_value(&self.criteria),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-filtercriterion.html
    pub struct FilterCriterion_ {
        pub path: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_FilterCriterion {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.FilterCriterion"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_FilterCriterion as FilterCriterion;
    impl crate::value::ToValue for FilterCriterion_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-headertagaction.html
    pub struct HeaderTagAction_ {
        pub name: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_HeaderTagAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.HeaderTagAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_HeaderTagAction as HeaderTagAction;
    impl crate::value::ToValue for HeaderTagAction_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-linkapplicationlogsampling.html
    pub struct LinkApplicationLogSampling_ {
        pub error_log: f64,
        pub filter_log: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_LinkApplicationLogSampling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.LinkApplicationLogSampling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_LinkApplicationLogSampling as LinkApplicationLogSampling;
    impl crate::value::ToValue for LinkApplicationLogSampling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ErrorLog".to_string(),
                crate::value::ToValue::to_value(&self.error_log),
            );
            properties.insert(
                "FilterLog".to_string(),
                crate::value::ToValue::to_value(&self.filter_log),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-linkattributes.html
    pub struct LinkAttributes_ {
        pub customer_provided_id: Option<crate::value::ExpString>,
        pub responder_error_masking: Option<Vec<ResponderErrorMaskingForHttpCode_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_LinkAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.LinkAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_LinkAttributes as LinkAttributes;
    impl crate::value::ToValue for LinkAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_provided_id {
                properties.insert(
                    "CustomerProvidedId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.responder_error_masking {
                properties.insert(
                    "ResponderErrorMasking".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-linklogsettings.html
    pub struct LinkLogSettings_ {
        pub application_logs: Box<ApplicationLogs_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_LinkLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.LinkLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_LinkLogSettings as LinkLogSettings;
    impl crate::value::ToValue for LinkLogSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationLogs".to_string(),
                crate::value::ToValue::to_value(&self.application_logs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-moduleconfiguration.html
    pub struct ModuleConfiguration_ {
        pub depends_on: Option<Vec<crate::value::ExpString>>,
        pub module_parameters: Option<Box<ModuleParameters_>>,
        pub name: crate::value::ExpString,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_ModuleConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.ModuleConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_ModuleConfiguration as ModuleConfiguration;
    impl crate::value::ToValue for ModuleConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.depends_on {
                properties.insert(
                    "DependsOn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.module_parameters {
                properties.insert(
                    "ModuleParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-moduleparameters.html
    pub struct ModuleParameters_ {
        pub no_bid: Option<Box<NoBidModuleParameters_>>,
        pub open_rtb_attribute: Option<Box<OpenRtbAttributeModuleParameters_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_ModuleParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.ModuleParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_ModuleParameters as ModuleParameters;
    impl crate::value::ToValue for ModuleParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.no_bid {
                properties.insert("NoBid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.open_rtb_attribute {
                properties.insert(
                    "OpenRtbAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-nobidaction.html
    pub struct NoBidAction_ {
        pub no_bid_reason_code: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_NoBidAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.NoBidAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_NoBidAction as NoBidAction;
    impl crate::value::ToValue for NoBidAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.no_bid_reason_code {
                properties.insert(
                    "NoBidReasonCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-nobidmoduleparameters.html
    pub struct NoBidModuleParameters_ {
        pub pass_through_percentage: Option<f64>,
        pub reason: Option<crate::value::ExpString>,
        pub reason_code: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_NoBidModuleParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.NoBidModuleParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_NoBidModuleParameters as NoBidModuleParameters;
    impl crate::value::ToValue for NoBidModuleParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pass_through_percentage {
                properties.insert(
                    "PassThroughPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.reason {
                properties.insert("Reason".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.reason_code {
                properties.insert(
                    "ReasonCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-openrtbattributemoduleparameters.html
    pub struct OpenRtbAttributeModuleParameters_ {
        pub action: Box<Action_>,
        pub filter_configuration: Vec<Filter_>,
        pub filter_type: crate::value::ExpString,
        pub holdback_percentage: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_OpenRtbAttributeModuleParameters {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.OpenRtbAttributeModuleParameters"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_OpenRtbAttributeModuleParameters as OpenRtbAttributeModuleParameters;
    impl crate::value::ToValue for OpenRtbAttributeModuleParameters_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "FilterConfiguration".to_string(),
                crate::value::ToValue::to_value(&self.filter_configuration),
            );
            properties.insert(
                "FilterType".to_string(),
                crate::value::ToValue::to_value(&self.filter_type),
            );
            properties.insert(
                "HoldbackPercentage".to_string(),
                crate::value::ToValue::to_value(&self.holdback_percentage),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-link-respondererrormaskingforhttpcode.html
    pub struct ResponderErrorMaskingForHttpCode_ {
        pub action: crate::value::ExpString,
        pub http_code: crate::value::ExpString,
        pub logging_types: Vec<crate::value::ExpString>,
        pub response_logging_percentage: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_Link_ResponderErrorMaskingForHttpCode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::Link.ResponderErrorMaskingForHttpCode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_Link_ResponderErrorMaskingForHttpCode as ResponderErrorMaskingForHttpCode;
    impl crate::value::ToValue for ResponderErrorMaskingForHttpCode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "HttpCode".to_string(),
                crate::value::ToValue::to_value(&self.http_code),
            );
            properties.insert(
                "LoggingTypes".to_string(),
                crate::value::ToValue::to_value(&self.logging_types),
            );
            if let Some(ref value) = self.response_logging_percentage {
                properties.insert(
                    "ResponseLoggingPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod outboundexternallink {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-outboundexternallink-applicationlogs.html
    pub struct ApplicationLogs_ {
        pub link_application_log_sampling: Box<LinkApplicationLogSampling_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_OutboundExternalLink_ApplicationLogs {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::OutboundExternalLink.ApplicationLogs"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_OutboundExternalLink_ApplicationLogs as ApplicationLogs;
    impl crate::value::ToValue for ApplicationLogs_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "LinkApplicationLogSampling".to_string(),
                crate::value::ToValue::to_value(&self.link_application_log_sampling),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-outboundexternallink-linkapplicationlogsampling.html
    pub struct LinkApplicationLogSampling_ {
        pub error_log: f64,
        pub filter_log: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_OutboundExternalLink_LinkApplicationLogSampling {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::OutboundExternalLink.LinkApplicationLogSampling"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_OutboundExternalLink_LinkApplicationLogSampling as LinkApplicationLogSampling;
    impl crate::value::ToValue for LinkApplicationLogSampling_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ErrorLog".to_string(),
                crate::value::ToValue::to_value(&self.error_log),
            );
            properties.insert(
                "FilterLog".to_string(),
                crate::value::ToValue::to_value(&self.filter_log),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-outboundexternallink-linkattributes.html
    pub struct LinkAttributes_ {
        pub customer_provided_id: Option<crate::value::ExpString>,
        pub responder_error_masking: Option<Vec<ResponderErrorMaskingForHttpCode_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_OutboundExternalLink_LinkAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::OutboundExternalLink.LinkAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_OutboundExternalLink_LinkAttributes as LinkAttributes;
    impl crate::value::ToValue for LinkAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_provided_id {
                properties.insert(
                    "CustomerProvidedId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.responder_error_masking {
                properties.insert(
                    "ResponderErrorMasking".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-outboundexternallink-linklogsettings.html
    pub struct LinkLogSettings_ {
        pub application_logs: Box<ApplicationLogs_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_OutboundExternalLink_LinkLogSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::OutboundExternalLink.LinkLogSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_OutboundExternalLink_LinkLogSettings as LinkLogSettings;
    impl crate::value::ToValue for LinkLogSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ApplicationLogs".to_string(),
                crate::value::ToValue::to_value(&self.application_logs),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-outboundexternallink-respondererrormaskingforhttpcode.html
    pub struct ResponderErrorMaskingForHttpCode_ {
        pub action: crate::value::ExpString,
        pub http_code: crate::value::ExpString,
        pub logging_types: Vec<crate::value::ExpString>,
        pub response_logging_percentage: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_OutboundExternalLink_ResponderErrorMaskingForHttpCode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::OutboundExternalLink.ResponderErrorMaskingForHttpCode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_OutboundExternalLink_ResponderErrorMaskingForHttpCode as ResponderErrorMaskingForHttpCode;
    impl crate::value::ToValue for ResponderErrorMaskingForHttpCode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "HttpCode".to_string(),
                crate::value::ToValue::to_value(&self.http_code),
            );
            properties.insert(
                "LoggingTypes".to_string(),
                crate::value::ToValue::to_value(&self.logging_types),
            );
            if let Some(ref value) = self.response_logging_percentage {
                properties.insert(
                    "ResponseLoggingPercentage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod respondergateway {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-respondergateway-autoscalinggroupsconfiguration.html
    pub struct AutoScalingGroupsConfiguration_ {
        pub auto_scaling_group_name_list: Vec<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_ResponderGateway_AutoScalingGroupsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::ResponderGateway.AutoScalingGroupsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_ResponderGateway_AutoScalingGroupsConfiguration as AutoScalingGroupsConfiguration;
    impl crate::value::ToValue for AutoScalingGroupsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AutoScalingGroupNameList".to_string(),
                crate::value::ToValue::to_value(&self.auto_scaling_group_name_list),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-respondergateway-eksendpointsconfiguration.html
    pub struct EksEndpointsConfiguration_ {
        pub cluster_api_server_ca_certificate_chain: crate::value::ExpString,
        pub cluster_api_server_endpoint_uri: crate::value::ExpString,
        pub cluster_name: crate::value::ExpString,
        pub endpoints_resource_name: crate::value::ExpString,
        pub endpoints_resource_namespace: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_ResponderGateway_EksEndpointsConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::ResponderGateway.EksEndpointsConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_ResponderGateway_EksEndpointsConfiguration as EksEndpointsConfiguration;
    impl crate::value::ToValue for EksEndpointsConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterApiServerCaCertificateChain".to_string(),
                crate::value::ToValue::to_value(&self.cluster_api_server_ca_certificate_chain),
            );
            properties.insert(
                "ClusterApiServerEndpointUri".to_string(),
                crate::value::ToValue::to_value(&self.cluster_api_server_endpoint_uri),
            );
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(&self.cluster_name),
            );
            properties.insert(
                "EndpointsResourceName".to_string(),
                crate::value::ToValue::to_value(&self.endpoints_resource_name),
            );
            properties.insert(
                "EndpointsResourceNamespace".to_string(),
                crate::value::ToValue::to_value(&self.endpoints_resource_namespace),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-respondergateway-managedendpointconfiguration.html
    pub struct ManagedEndpointConfiguration_ {
        pub auto_scaling_groups_configuration: Option<Box<AutoScalingGroupsConfiguration_>>,
        pub eks_endpoints_configuration: Option<Box<EksEndpointsConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_ResponderGateway_ManagedEndpointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::ResponderGateway.ManagedEndpointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_ResponderGateway_ManagedEndpointConfiguration as ManagedEndpointConfiguration;
    impl crate::value::ToValue for ManagedEndpointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_scaling_groups_configuration {
                properties.insert(
                    "AutoScalingGroupsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.eks_endpoints_configuration {
                properties.insert(
                    "EksEndpointsConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rtbfabric-respondergateway-truststoreconfiguration.html
    pub struct TrustStoreConfiguration_ {
        pub certificate_authority_certificates: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rtbfabric_ResponderGateway_TrustStoreConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RTBFabric::ResponderGateway.TrustStoreConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rtbfabric_ResponderGateway_TrustStoreConfiguration as TrustStoreConfiguration;
    impl crate::value::ToValue for TrustStoreConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CertificateAuthorityCertificates".to_string(),
                crate::value::ToValue::to_value(&self.certificate_authority_certificates),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rtbfabric-inboundexternallink.html
pub struct InboundExternalLink_ {
    pub gateway_id: crate::value::ExpString,
    pub link_attributes: Option<super::rtbfabric::inboundexternallink::LinkAttributes_>,
    pub link_log_settings: super::rtbfabric::inboundexternallink::LinkLogSettings_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rtbfabric_InboundExternalLink {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RTBFabric::InboundExternalLink"
        $($field $value)*)
    };
}
pub use crate::__aws_rtbfabric_InboundExternalLink as InboundExternalLink;
impl crate::template::ToResource for InboundExternalLink_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RTBFabric"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InboundExternalLink"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GatewayId".to_string(),
            crate::value::ToValue::to_value(&self.gateway_id),
        );
        if let Some(ref value) = self.link_attributes {
            properties.insert(
                "LinkAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LinkLogSettings".to_string(),
            crate::value::ToValue::to_value(&self.link_log_settings),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rtbfabric-link.html
pub struct Link_ {
    pub gateway_id: crate::value::ExpString,
    pub http_responder_allowed: Option<crate::value::ExpBool>,
    pub link_attributes: Option<super::rtbfabric::link::LinkAttributes_>,
    pub link_log_settings: super::rtbfabric::link::LinkLogSettings_,
    pub module_configuration_list: Option<Vec<super::rtbfabric::link::ModuleConfiguration_>>,
    pub peer_gateway_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rtbfabric_Link {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RTBFabric::Link" $($field
        $value)*)
    };
}
pub use crate::__aws_rtbfabric_Link as Link;
impl crate::template::ToResource for Link_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RTBFabric"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Link"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GatewayId".to_string(),
            crate::value::ToValue::to_value(&self.gateway_id),
        );
        if let Some(ref value) = self.http_responder_allowed {
            properties.insert(
                "HttpResponderAllowed".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.link_attributes {
            properties.insert(
                "LinkAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LinkLogSettings".to_string(),
            crate::value::ToValue::to_value(&self.link_log_settings),
        );
        if let Some(ref value) = self.module_configuration_list {
            properties.insert(
                "ModuleConfigurationList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PeerGatewayId".to_string(),
            crate::value::ToValue::to_value(&self.peer_gateway_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rtbfabric-outboundexternallink.html
pub struct OutboundExternalLink_ {
    pub gateway_id: crate::value::ExpString,
    pub link_attributes: Option<super::rtbfabric::outboundexternallink::LinkAttributes_>,
    pub link_log_settings: super::rtbfabric::outboundexternallink::LinkLogSettings_,
    pub public_endpoint: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rtbfabric_OutboundExternalLink {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RTBFabric::OutboundExternalLink"
        $($field $value)*)
    };
}
pub use crate::__aws_rtbfabric_OutboundExternalLink as OutboundExternalLink;
impl crate::template::ToResource for OutboundExternalLink_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RTBFabric"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OutboundExternalLink"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "GatewayId".to_string(),
            crate::value::ToValue::to_value(&self.gateway_id),
        );
        if let Some(ref value) = self.link_attributes {
            properties.insert(
                "LinkAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LinkLogSettings".to_string(),
            crate::value::ToValue::to_value(&self.link_log_settings),
        );
        properties.insert(
            "PublicEndpoint".to_string(),
            crate::value::ToValue::to_value(&self.public_endpoint),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rtbfabric-requestergateway.html
pub struct RequesterGateway_ {
    pub description: Option<crate::value::ExpString>,
    pub security_group_ids: Vec<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rtbfabric_RequesterGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RTBFabric::RequesterGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_rtbfabric_RequesterGateway as RequesterGateway;
impl crate::template::ToResource for RequesterGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RTBFabric"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RequesterGateway"),
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
            "SecurityGroupIds".to_string(),
            crate::value::ToValue::to_value(&self.security_group_ids),
        );
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rtbfabric-respondergateway.html
pub struct ResponderGateway_ {
    pub description: Option<crate::value::ExpString>,
    pub domain_name: Option<crate::value::ExpString>,
    pub managed_endpoint_configuration:
        Option<super::rtbfabric::respondergateway::ManagedEndpointConfiguration_>,
    pub port: i32,
    pub protocol: crate::value::ExpString,
    pub security_group_ids: Vec<crate::value::ExpString>,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trust_store_configuration:
        Option<super::rtbfabric::respondergateway::TrustStoreConfiguration_>,
    pub vpc_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rtbfabric_ResponderGateway {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RTBFabric::ResponderGateway"
        $($field $value)*)
    };
}
pub use crate::__aws_rtbfabric_ResponderGateway as ResponderGateway;
impl crate::template::ToResource for ResponderGateway_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RTBFabric"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResponderGateway"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain_name {
            properties.insert(
                "DomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.managed_endpoint_configuration {
            properties.insert(
                "ManagedEndpointConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Port".to_string(),
            crate::value::ToValue::to_value(&self.port),
        );
        properties.insert(
            "Protocol".to_string(),
            crate::value::ToValue::to_value(&self.protocol),
        );
        properties.insert(
            "SecurityGroupIds".to_string(),
            crate::value::ToValue::to_value(&self.security_group_ids),
        );
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trust_store_configuration {
            properties.insert(
                "TrustStoreConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VpcId".to_string(),
            crate::value::ToValue::to_value(&self.vpc_id),
        );
        properties
    }
}
