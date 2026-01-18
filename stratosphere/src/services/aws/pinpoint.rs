pub mod applicationsettings {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-campaignhook.html
    pub struct CampaignHook_ {
        pub lambda_function_name: Option<crate::value::ExpString>,
        pub mode: Option<crate::value::ExpString>,
        pub web_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_ApplicationSettings_CampaignHook {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::ApplicationSettings.CampaignHook"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_ApplicationSettings_CampaignHook as CampaignHook;
    impl crate::value::ToValue for CampaignHook_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_function_name {
                properties.insert(
                    "LambdaFunctionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.web_url {
                properties.insert("WebUrl".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-limits.html
    pub struct Limits_ {
        pub daily: Option<i64>,
        pub maximum_duration: Option<i64>,
        pub messages_per_second: Option<i64>,
        pub total: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_ApplicationSettings_Limits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::ApplicationSettings.Limits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_ApplicationSettings_Limits as Limits;
    impl crate::value::ToValue for Limits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.daily {
                properties.insert("Daily".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.maximum_duration {
                properties.insert(
                    "MaximumDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.messages_per_second {
                properties.insert(
                    "MessagesPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total {
                properties.insert("Total".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-applicationsettings-quiettime.html
    pub struct QuietTime_ {
        pub end: crate::value::ExpString,
        pub start: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_ApplicationSettings_QuietTime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::ApplicationSettings.QuietTime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_ApplicationSettings_QuietTime as QuietTime;
    impl crate::value::ToValue for QuietTime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "End".to_string(),
                crate::value::ToValue::to_value(&self.end),
            );
            properties.insert(
                "Start".to_string(),
                crate::value::ToValue::to_value(&self.start),
            );
            properties.into()
        }
    }
}
pub mod campaign {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-attributedimension.html
    pub struct AttributeDimension_ {
        pub attribute_type: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_AttributeDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.AttributeDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_AttributeDimension as AttributeDimension;
    impl crate::value::ToValue for AttributeDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_type {
                properties.insert(
                    "AttributeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigncustommessage.html
    pub struct CampaignCustomMessage_ {
        pub data: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_CampaignCustomMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.CampaignCustomMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_CampaignCustomMessage as CampaignCustomMessage;
    impl crate::value::ToValue for CampaignCustomMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data {
                properties.insert("Data".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignemailmessage.html
    pub struct CampaignEmailMessage_ {
        pub body: Option<crate::value::ExpString>,
        pub from_address: Option<crate::value::ExpString>,
        pub html_body: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_CampaignEmailMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.CampaignEmailMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_CampaignEmailMessage as CampaignEmailMessage;
    impl crate::value::ToValue for CampaignEmailMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.from_address {
                properties.insert(
                    "FromAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.html_body {
                properties.insert(
                    "HtmlBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigneventfilter.html
    pub struct CampaignEventFilter_ {
        pub dimensions: Option<Box<EventDimensions_>>,
        pub filter_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_CampaignEventFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.CampaignEventFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_CampaignEventFilter as CampaignEventFilter;
    impl crate::value::ToValue for CampaignEventFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.filter_type {
                properties.insert(
                    "FilterType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignhook.html
    pub struct CampaignHook_ {
        pub lambda_function_name: Option<crate::value::ExpString>,
        pub mode: Option<crate::value::ExpString>,
        pub web_url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_CampaignHook {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.CampaignHook"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_CampaignHook as CampaignHook;
    impl crate::value::ToValue for CampaignHook_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.lambda_function_name {
                properties.insert(
                    "LambdaFunctionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mode {
                properties.insert("Mode".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.web_url {
                properties.insert("WebUrl".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaigninappmessage.html
    pub struct CampaignInAppMessage_ {
        pub content: Option<Vec<InAppMessageContent_>>,
        pub custom_config: Option<serde_json::Value>,
        pub layout: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_CampaignInAppMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.CampaignInAppMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_CampaignInAppMessage as CampaignInAppMessage;
    impl crate::value::ToValue for CampaignInAppMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.content {
                properties.insert(
                    "Content".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_config {
                properties.insert(
                    "CustomConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.layout {
                properties.insert("Layout".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-campaignsmsmessage.html
    pub struct CampaignSmsMessage_ {
        pub body: Option<crate::value::ExpString>,
        pub entity_id: Option<crate::value::ExpString>,
        pub message_type: Option<crate::value::ExpString>,
        pub origination_number: Option<crate::value::ExpString>,
        pub sender_id: Option<crate::value::ExpString>,
        pub template_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_CampaignSmsMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.CampaignSmsMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_CampaignSmsMessage as CampaignSmsMessage;
    impl crate::value::ToValue for CampaignSmsMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.entity_id {
                properties.insert(
                    "EntityId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_type {
                properties.insert(
                    "MessageType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.origination_number {
                properties.insert(
                    "OriginationNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sender_id {
                properties.insert(
                    "SenderId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.template_id {
                properties.insert(
                    "TemplateId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-customdeliveryconfiguration.html
    pub struct CustomDeliveryConfiguration_ {
        pub delivery_uri: Option<crate::value::ExpString>,
        pub endpoint_types: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_CustomDeliveryConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.CustomDeliveryConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_CustomDeliveryConfiguration as CustomDeliveryConfiguration;
    impl crate::value::ToValue for CustomDeliveryConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.delivery_uri {
                properties.insert(
                    "DeliveryUri".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.endpoint_types {
                properties.insert(
                    "EndpointTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-defaultbuttonconfiguration.html
    pub struct DefaultButtonConfiguration_ {
        pub background_color: Option<crate::value::ExpString>,
        pub border_radius: Option<i64>,
        pub button_action: Option<crate::value::ExpString>,
        pub link: Option<crate::value::ExpString>,
        pub text: Option<crate::value::ExpString>,
        pub text_color: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_DefaultButtonConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.DefaultButtonConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_DefaultButtonConfiguration as DefaultButtonConfiguration;
    impl crate::value::ToValue for DefaultButtonConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.background_color {
                properties.insert(
                    "BackgroundColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.border_radius {
                properties.insert(
                    "BorderRadius".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.button_action {
                properties.insert(
                    "ButtonAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.link {
                properties.insert("Link".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text_color {
                properties.insert(
                    "TextColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-eventdimensions.html
    pub struct EventDimensions_ {
        pub attributes: Option<serde_json::Value>,
        pub event_type: Option<Box<SetDimension_>>,
        pub metrics: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_EventDimensions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.EventDimensions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_EventDimensions as EventDimensions;
    impl crate::value::ToValue for EventDimensions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_type {
                properties.insert(
                    "EventType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebodyconfig.html
    pub struct InAppMessageBodyConfig_ {
        pub alignment: Option<crate::value::ExpString>,
        pub body: Option<crate::value::ExpString>,
        pub text_color: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_InAppMessageBodyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.InAppMessageBodyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_InAppMessageBodyConfig as InAppMessageBodyConfig;
    impl crate::value::ToValue for InAppMessageBodyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alignment {
                properties.insert(
                    "Alignment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text_color {
                properties.insert(
                    "TextColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagebutton.html
    pub struct InAppMessageButton_ {
        pub android: Option<Box<OverrideButtonConfiguration_>>,
        pub default_config: Option<Box<DefaultButtonConfiguration_>>,
        pub ios: Option<Box<OverrideButtonConfiguration_>>,
        pub web: Option<Box<OverrideButtonConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_InAppMessageButton {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.InAppMessageButton"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_InAppMessageButton as InAppMessageButton;
    impl crate::value::ToValue for InAppMessageButton_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.android {
                properties.insert(
                    "Android".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_config {
                properties.insert(
                    "DefaultConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ios {
                properties.insert("IOS".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.web {
                properties.insert("Web".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessagecontent.html
    pub struct InAppMessageContent_ {
        pub background_color: Option<crate::value::ExpString>,
        pub body_config: Option<Box<InAppMessageBodyConfig_>>,
        pub header_config: Option<Box<InAppMessageHeaderConfig_>>,
        pub image_url: Option<crate::value::ExpString>,
        pub primary_btn: Option<Box<InAppMessageButton_>>,
        pub secondary_btn: Option<Box<InAppMessageButton_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_InAppMessageContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.InAppMessageContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_InAppMessageContent as InAppMessageContent;
    impl crate::value::ToValue for InAppMessageContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.background_color {
                properties.insert(
                    "BackgroundColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.body_config {
                properties.insert(
                    "BodyConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_config {
                properties.insert(
                    "HeaderConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_url {
                properties.insert(
                    "ImageUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.primary_btn {
                properties.insert(
                    "PrimaryBtn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secondary_btn {
                properties.insert(
                    "SecondaryBtn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-inappmessageheaderconfig.html
    pub struct InAppMessageHeaderConfig_ {
        pub alignment: Option<crate::value::ExpString>,
        pub header: Option<crate::value::ExpString>,
        pub text_color: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_InAppMessageHeaderConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.InAppMessageHeaderConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_InAppMessageHeaderConfig as InAppMessageHeaderConfig;
    impl crate::value::ToValue for InAppMessageHeaderConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alignment {
                properties.insert(
                    "Alignment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text_color {
                properties.insert(
                    "TextColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-limits.html
    pub struct Limits_ {
        pub daily: Option<i64>,
        pub maximum_duration: Option<i64>,
        pub messages_per_second: Option<i64>,
        pub session: Option<i64>,
        pub total: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_Limits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.Limits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_Limits as Limits;
    impl crate::value::ToValue for Limits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.daily {
                properties.insert("Daily".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.maximum_duration {
                properties.insert(
                    "MaximumDuration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.messages_per_second {
                properties.insert(
                    "MessagesPerSecond".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session {
                properties.insert(
                    "Session".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.total {
                properties.insert("Total".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-message.html
    pub struct Message_ {
        pub action: Option<crate::value::ExpString>,
        pub body: Option<crate::value::ExpString>,
        pub image_icon_url: Option<crate::value::ExpString>,
        pub image_small_icon_url: Option<crate::value::ExpString>,
        pub image_url: Option<crate::value::ExpString>,
        pub json_body: Option<crate::value::ExpString>,
        pub media_url: Option<crate::value::ExpString>,
        pub raw_content: Option<crate::value::ExpString>,
        pub silent_push: Option<crate::value::ExpBool>,
        pub time_to_live: Option<i64>,
        pub title: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_Message {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.Message"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_Message as Message;
    impl crate::value::ToValue for Message_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.image_icon_url {
                properties.insert(
                    "ImageIconUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_small_icon_url {
                properties.insert(
                    "ImageSmallIconUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_url {
                properties.insert(
                    "ImageUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.json_body {
                properties.insert(
                    "JsonBody".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.media_url {
                properties.insert(
                    "MediaUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.raw_content {
                properties.insert(
                    "RawContent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.silent_push {
                properties.insert(
                    "SilentPush".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_to_live {
                properties.insert(
                    "TimeToLive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-messageconfiguration.html
    pub struct MessageConfiguration_ {
        pub adm_message: Option<Box<Message_>>,
        pub apns_message: Option<Box<Message_>>,
        pub baidu_message: Option<Box<Message_>>,
        pub custom_message: Option<Box<CampaignCustomMessage_>>,
        pub default_message: Option<Box<Message_>>,
        pub email_message: Option<Box<CampaignEmailMessage_>>,
        pub gcm_message: Option<Box<Message_>>,
        pub in_app_message: Option<Box<CampaignInAppMessage_>>,
        pub sms_message: Option<Box<CampaignSmsMessage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_MessageConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.MessageConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_MessageConfiguration as MessageConfiguration;
    impl crate::value::ToValue for MessageConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.adm_message {
                properties.insert(
                    "ADMMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.apns_message {
                properties.insert(
                    "APNSMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.baidu_message {
                properties.insert(
                    "BaiduMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.custom_message {
                properties.insert(
                    "CustomMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_message {
                properties.insert(
                    "DefaultMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.email_message {
                properties.insert(
                    "EmailMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gcm_message {
                properties.insert(
                    "GCMMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.in_app_message {
                properties.insert(
                    "InAppMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sms_message {
                properties.insert(
                    "SMSMessage".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-metricdimension.html
    pub struct MetricDimension_ {
        pub comparison_operator: Option<crate::value::ExpString>,
        pub value: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_MetricDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.MetricDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_MetricDimension as MetricDimension;
    impl crate::value::ToValue for MetricDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comparison_operator {
                properties.insert(
                    "ComparisonOperator".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-overridebuttonconfiguration.html
    pub struct OverrideButtonConfiguration_ {
        pub button_action: Option<crate::value::ExpString>,
        pub link: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_OverrideButtonConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.OverrideButtonConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_OverrideButtonConfiguration as OverrideButtonConfiguration;
    impl crate::value::ToValue for OverrideButtonConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.button_action {
                properties.insert(
                    "ButtonAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.link {
                properties.insert("Link".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule-quiettime.html
    pub struct QuietTime_ {
        pub end: crate::value::ExpString,
        pub start: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_QuietTime {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.QuietTime"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_QuietTime as QuietTime;
    impl crate::value::ToValue for QuietTime_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "End".to_string(),
                crate::value::ToValue::to_value(&self.end),
            );
            properties.insert(
                "Start".to_string(),
                crate::value::ToValue::to_value(&self.start),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-schedule.html
    pub struct Schedule_ {
        pub end_time: Option<crate::value::ExpString>,
        pub event_filter: Option<Box<CampaignEventFilter_>>,
        pub frequency: Option<crate::value::ExpString>,
        pub is_local_time: Option<crate::value::ExpBool>,
        pub quiet_time: Option<Box<QuietTime_>>,
        pub start_time: Option<crate::value::ExpString>,
        pub time_zone: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.end_time {
                properties.insert(
                    "EndTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_filter {
                properties.insert(
                    "EventFilter".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.frequency {
                properties.insert(
                    "Frequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_local_time {
                properties.insert(
                    "IsLocalTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.quiet_time {
                properties.insert(
                    "QuietTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time {
                properties.insert(
                    "StartTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.time_zone {
                properties.insert(
                    "TimeZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-setdimension.html
    pub struct SetDimension_ {
        pub dimension_type: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_SetDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.SetDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_SetDimension as SetDimension;
    impl crate::value::ToValue for SetDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimension_type {
                properties.insert(
                    "DimensionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-template.html
    pub struct Template_ {
        pub name: Option<crate::value::ExpString>,
        pub version: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_Template {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.Template"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_Template as Template;
    impl crate::value::ToValue for Template_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-templateconfiguration.html
    pub struct TemplateConfiguration_ {
        pub email_template: Option<Box<Template_>>,
        pub push_template: Option<Box<Template_>>,
        pub sms_template: Option<Box<Template_>>,
        pub voice_template: Option<Box<Template_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_TemplateConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.TemplateConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_TemplateConfiguration as TemplateConfiguration;
    impl crate::value::ToValue for TemplateConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email_template {
                properties.insert(
                    "EmailTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.push_template {
                properties.insert(
                    "PushTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sms_template {
                properties.insert(
                    "SMSTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.voice_template {
                properties.insert(
                    "VoiceTemplate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-campaign-writetreatmentresource.html
    pub struct WriteTreatmentResource_ {
        pub custom_delivery_configuration: Option<Box<CustomDeliveryConfiguration_>>,
        pub message_configuration: Option<Box<MessageConfiguration_>>,
        pub schedule: Option<Box<Schedule_>>,
        pub size_percent: Option<i64>,
        pub template_configuration: Option<Box<TemplateConfiguration_>>,
        pub treatment_description: Option<crate::value::ExpString>,
        pub treatment_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Campaign_WriteTreatmentResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Campaign.WriteTreatmentResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Campaign_WriteTreatmentResource as WriteTreatmentResource;
    impl crate::value::ToValue for WriteTreatmentResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_delivery_configuration {
                properties.insert(
                    "CustomDeliveryConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.message_configuration {
                properties.insert(
                    "MessageConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.schedule {
                properties.insert(
                    "Schedule".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.size_percent {
                properties.insert(
                    "SizePercent".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.template_configuration {
                properties.insert(
                    "TemplateConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.treatment_description {
                properties.insert(
                    "TreatmentDescription".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.treatment_name {
                properties.insert(
                    "TreatmentName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod inapptemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-bodyconfig.html
    pub struct BodyConfig_ {
        pub alignment: Option<crate::value::ExpString>,
        pub body: Option<crate::value::ExpString>,
        pub text_color: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_InAppTemplate_BodyConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::InAppTemplate.BodyConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_InAppTemplate_BodyConfig as BodyConfig;
    impl crate::value::ToValue for BodyConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alignment {
                properties.insert(
                    "Alignment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text_color {
                properties.insert(
                    "TextColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-buttonconfig.html
    pub struct ButtonConfig_ {
        pub android: Option<Box<OverrideButtonConfiguration_>>,
        pub default_config: Option<Box<DefaultButtonConfiguration_>>,
        pub ios: Option<Box<OverrideButtonConfiguration_>>,
        pub web: Option<Box<OverrideButtonConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_InAppTemplate_ButtonConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::InAppTemplate.ButtonConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_InAppTemplate_ButtonConfig as ButtonConfig;
    impl crate::value::ToValue for ButtonConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.android {
                properties.insert(
                    "Android".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_config {
                properties.insert(
                    "DefaultConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ios {
                properties.insert("IOS".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.web {
                properties.insert("Web".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-defaultbuttonconfiguration.html
    pub struct DefaultButtonConfiguration_ {
        pub background_color: Option<crate::value::ExpString>,
        pub border_radius: Option<i64>,
        pub button_action: Option<crate::value::ExpString>,
        pub link: Option<crate::value::ExpString>,
        pub text: Option<crate::value::ExpString>,
        pub text_color: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_InAppTemplate_DefaultButtonConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::InAppTemplate.DefaultButtonConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_InAppTemplate_DefaultButtonConfiguration as DefaultButtonConfiguration;
    impl crate::value::ToValue for DefaultButtonConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.background_color {
                properties.insert(
                    "BackgroundColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.border_radius {
                properties.insert(
                    "BorderRadius".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.button_action {
                properties.insert(
                    "ButtonAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.link {
                properties.insert("Link".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text {
                properties.insert("Text".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text_color {
                properties.insert(
                    "TextColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-headerconfig.html
    pub struct HeaderConfig_ {
        pub alignment: Option<crate::value::ExpString>,
        pub header: Option<crate::value::ExpString>,
        pub text_color: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_InAppTemplate_HeaderConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::InAppTemplate.HeaderConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_InAppTemplate_HeaderConfig as HeaderConfig;
    impl crate::value::ToValue for HeaderConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alignment {
                properties.insert(
                    "Alignment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header {
                properties.insert("Header".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.text_color {
                properties.insert(
                    "TextColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-inappmessagecontent.html
    pub struct InAppMessageContent_ {
        pub background_color: Option<crate::value::ExpString>,
        pub body_config: Option<Box<BodyConfig_>>,
        pub header_config: Option<Box<HeaderConfig_>>,
        pub image_url: Option<crate::value::ExpString>,
        pub primary_btn: Option<Box<ButtonConfig_>>,
        pub secondary_btn: Option<Box<ButtonConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_InAppTemplate_InAppMessageContent {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::InAppTemplate.InAppMessageContent"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_InAppTemplate_InAppMessageContent as InAppMessageContent;
    impl crate::value::ToValue for InAppMessageContent_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.background_color {
                properties.insert(
                    "BackgroundColor".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.body_config {
                properties.insert(
                    "BodyConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.header_config {
                properties.insert(
                    "HeaderConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_url {
                properties.insert(
                    "ImageUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.primary_btn {
                properties.insert(
                    "PrimaryBtn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secondary_btn {
                properties.insert(
                    "SecondaryBtn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-inapptemplate-overridebuttonconfiguration.html
    pub struct OverrideButtonConfiguration_ {
        pub button_action: Option<crate::value::ExpString>,
        pub link: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_InAppTemplate_OverrideButtonConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::InAppTemplate.OverrideButtonConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_InAppTemplate_OverrideButtonConfiguration as OverrideButtonConfiguration;
    impl crate::value::ToValue for OverrideButtonConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.button_action {
                properties.insert(
                    "ButtonAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.link {
                properties.insert("Link".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod pushtemplate {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-apnspushnotificationtemplate.html
    pub struct APNSPushNotificationTemplate_ {
        pub action: Option<crate::value::ExpString>,
        pub body: Option<crate::value::ExpString>,
        pub media_url: Option<crate::value::ExpString>,
        pub sound: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_PushTemplate_APNSPushNotificationTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::PushTemplate.APNSPushNotificationTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_PushTemplate_APNSPushNotificationTemplate as APNSPushNotificationTemplate;
    impl crate::value::ToValue for APNSPushNotificationTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.media_url {
                properties.insert(
                    "MediaUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sound {
                properties.insert("Sound".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-androidpushnotificationtemplate.html
    pub struct AndroidPushNotificationTemplate_ {
        pub action: Option<crate::value::ExpString>,
        pub body: Option<crate::value::ExpString>,
        pub image_icon_url: Option<crate::value::ExpString>,
        pub image_url: Option<crate::value::ExpString>,
        pub small_image_icon_url: Option<crate::value::ExpString>,
        pub sound: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_PushTemplate_AndroidPushNotificationTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::PushTemplate.AndroidPushNotificationTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_PushTemplate_AndroidPushNotificationTemplate as AndroidPushNotificationTemplate;
    impl crate::value::ToValue for AndroidPushNotificationTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.image_icon_url {
                properties.insert(
                    "ImageIconUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image_url {
                properties.insert(
                    "ImageUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.small_image_icon_url {
                properties.insert(
                    "SmallImageIconUrl".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sound {
                properties.insert("Sound".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-pushtemplate-defaultpushnotificationtemplate.html
    pub struct DefaultPushNotificationTemplate_ {
        pub action: Option<crate::value::ExpString>,
        pub body: Option<crate::value::ExpString>,
        pub sound: Option<crate::value::ExpString>,
        pub title: Option<crate::value::ExpString>,
        pub url: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_PushTemplate_DefaultPushNotificationTemplate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::PushTemplate.DefaultPushNotificationTemplate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_PushTemplate_DefaultPushNotificationTemplate as DefaultPushNotificationTemplate;
    impl crate::value::ToValue for DefaultPushNotificationTemplate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action {
                properties.insert("Action".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.body {
                properties.insert("Body".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sound {
                properties.insert("Sound".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.title {
                properties.insert("Title".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.url {
                properties.insert("Url".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod segment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-attributedimension.html
    pub struct AttributeDimension_ {
        pub attribute_type: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_AttributeDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.AttributeDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_AttributeDimension as AttributeDimension;
    impl crate::value::ToValue for AttributeDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attribute_type {
                properties.insert(
                    "AttributeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-behavior.html
    pub struct Behavior_ {
        pub recency: Option<Box<Recency_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_Behavior {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.Behavior"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_Behavior as Behavior;
    impl crate::value::ToValue for Behavior_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.recency {
                properties.insert(
                    "Recency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint-coordinates.html
    pub struct Coordinates_ {
        pub latitude: f64,
        pub longitude: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_Coordinates {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.Coordinates"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_Coordinates as Coordinates;
    impl crate::value::ToValue for Coordinates_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Latitude".to_string(),
                crate::value::ToValue::to_value(&self.latitude),
            );
            properties.insert(
                "Longitude".to_string(),
                crate::value::ToValue::to_value(&self.longitude),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-demographic.html
    pub struct Demographic_ {
        pub app_version: Option<Box<SetDimension_>>,
        pub channel: Option<Box<SetDimension_>>,
        pub device_type: Option<Box<SetDimension_>>,
        pub make: Option<Box<SetDimension_>>,
        pub model: Option<Box<SetDimension_>>,
        pub platform: Option<Box<SetDimension_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_Demographic {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.Demographic"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_Demographic as Demographic;
    impl crate::value::ToValue for Demographic_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_version {
                properties.insert(
                    "AppVersion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channel {
                properties.insert(
                    "Channel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_type {
                properties.insert(
                    "DeviceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.make {
                properties.insert("Make".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.model {
                properties.insert("Model".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.platform {
                properties.insert(
                    "Platform".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location-gpspoint.html
    pub struct GPSPoint_ {
        pub coordinates: Box<Coordinates_>,
        pub range_in_kilometers: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_GPSPoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.GPSPoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_GPSPoint as GPSPoint;
    impl crate::value::ToValue for GPSPoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Coordinates".to_string(),
                crate::value::ToValue::to_value(&self.coordinates),
            );
            properties.insert(
                "RangeInKilometers".to_string(),
                crate::value::ToValue::to_value(&self.range_in_kilometers),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups.html
    pub struct Groups_ {
        pub dimensions: Option<Vec<SegmentDimensions_>>,
        pub source_segments: Option<Vec<SourceSegments_>>,
        pub source_type: Option<crate::value::ExpString>,
        pub r#type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_Groups {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.Groups"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_Groups as Groups;
    impl crate::value::ToValue for Groups_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimensions {
                properties.insert(
                    "Dimensions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_segments {
                properties.insert(
                    "SourceSegments".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.source_type {
                properties.insert(
                    "SourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.r#type {
                properties.insert("Type".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-location.html
    pub struct Location_ {
        pub country: Option<Box<SetDimension_>>,
        pub gps_point: Option<Box<GPSPoint_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.country {
                properties.insert(
                    "Country".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.gps_point {
                properties.insert(
                    "GPSPoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions-behavior-recency.html
    pub struct Recency_ {
        pub duration: crate::value::ExpString,
        pub recency_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_Recency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.Recency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_Recency as Recency;
    impl crate::value::ToValue for Recency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Duration".to_string(),
                crate::value::ToValue::to_value(&self.duration),
            );
            properties.insert(
                "RecencyType".to_string(),
                crate::value::ToValue::to_value(&self.recency_type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentdimensions.html
    pub struct SegmentDimensions_ {
        pub attributes: Option<serde_json::Value>,
        pub behavior: Option<Box<Behavior_>>,
        pub demographic: Option<Box<Demographic_>>,
        pub location: Option<Box<Location_>>,
        pub metrics: Option<serde_json::Value>,
        pub user_attributes: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_SegmentDimensions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.SegmentDimensions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_SegmentDimensions as SegmentDimensions;
    impl crate::value::ToValue for SegmentDimensions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attributes {
                properties.insert(
                    "Attributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.behavior {
                properties.insert(
                    "Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.demographic {
                properties.insert(
                    "Demographic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.location {
                properties.insert(
                    "Location".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metrics {
                properties.insert(
                    "Metrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_attributes {
                properties.insert(
                    "UserAttributes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups.html
    pub struct SegmentGroups_ {
        pub groups: Option<Vec<Groups_>>,
        pub include: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_SegmentGroups {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.SegmentGroups"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_SegmentGroups as SegmentGroups;
    impl crate::value::ToValue for SegmentGroups_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.groups {
                properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.include {
                properties.insert(
                    "Include".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-setdimension.html
    pub struct SetDimension_ {
        pub dimension_type: Option<crate::value::ExpString>,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_SetDimension {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.SetDimension"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_SetDimension as SetDimension;
    impl crate::value::ToValue for SetDimension_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimension_type {
                properties.insert(
                    "DimensionType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpoint-segment-segmentgroups-groups-sourcesegments.html
    pub struct SourceSegments_ {
        pub id: crate::value::ExpString,
        pub version: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpoint_Segment_SourceSegments {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Pinpoint::Segment.SourceSegments"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpoint_Segment_SourceSegments as SourceSegments;
    impl crate::value::ToValue for SourceSegments_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert("Id".to_string(), crate::value::ToValue::to_value(&self.id));
            if let Some(ref value) = self.version {
                properties.insert(
                    "Version".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-admchannel.html
pub struct ADMChannel_ {
    pub application_id: crate::value::ExpString,
    pub client_id: crate::value::ExpString,
    pub client_secret: crate::value::ExpString,
    pub enabled: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_ADMChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::ADMChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_ADMChannel as ADMChannel;
impl crate::template::ToResource for ADMChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ADMChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "ClientId".to_string(),
            crate::value::ToValue::to_value(&self.client_id),
        );
        properties.insert(
            "ClientSecret".to_string(),
            crate::value::ToValue::to_value(&self.client_secret),
        );
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnschannel.html
pub struct APNSChannel_ {
    pub application_id: crate::value::ExpString,
    pub bundle_id: Option<crate::value::ExpString>,
    pub certificate: Option<crate::value::ExpString>,
    pub default_authentication_method: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub private_key: Option<crate::value::ExpString>,
    pub team_id: Option<crate::value::ExpString>,
    pub token_key: Option<crate::value::ExpString>,
    pub token_key_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_APNSChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::APNSChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_APNSChannel as APNSChannel;
impl crate::template::ToResource for APNSChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("APNSChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.bundle_id {
            properties.insert(
                "BundleId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate {
            properties.insert(
                "Certificate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_authentication_method {
            properties.insert(
                "DefaultAuthenticationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_key {
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.team_id {
            properties.insert("TeamId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.token_key {
            properties.insert(
                "TokenKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.token_key_id {
            properties.insert(
                "TokenKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnssandboxchannel.html
pub struct APNSSandboxChannel_ {
    pub application_id: crate::value::ExpString,
    pub bundle_id: Option<crate::value::ExpString>,
    pub certificate: Option<crate::value::ExpString>,
    pub default_authentication_method: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub private_key: Option<crate::value::ExpString>,
    pub team_id: Option<crate::value::ExpString>,
    pub token_key: Option<crate::value::ExpString>,
    pub token_key_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_APNSSandboxChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::APNSSandboxChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_APNSSandboxChannel as APNSSandboxChannel;
impl crate::template::ToResource for APNSSandboxChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("APNSSandboxChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.bundle_id {
            properties.insert(
                "BundleId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate {
            properties.insert(
                "Certificate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_authentication_method {
            properties.insert(
                "DefaultAuthenticationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_key {
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.team_id {
            properties.insert("TeamId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.token_key {
            properties.insert(
                "TokenKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.token_key_id {
            properties.insert(
                "TokenKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipchannel.html
pub struct APNSVoipChannel_ {
    pub application_id: crate::value::ExpString,
    pub bundle_id: Option<crate::value::ExpString>,
    pub certificate: Option<crate::value::ExpString>,
    pub default_authentication_method: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub private_key: Option<crate::value::ExpString>,
    pub team_id: Option<crate::value::ExpString>,
    pub token_key: Option<crate::value::ExpString>,
    pub token_key_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_APNSVoipChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::APNSVoipChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_APNSVoipChannel as APNSVoipChannel;
impl crate::template::ToResource for APNSVoipChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("APNSVoipChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.bundle_id {
            properties.insert(
                "BundleId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate {
            properties.insert(
                "Certificate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_authentication_method {
            properties.insert(
                "DefaultAuthenticationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_key {
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.team_id {
            properties.insert("TeamId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.token_key {
            properties.insert(
                "TokenKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.token_key_id {
            properties.insert(
                "TokenKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-apnsvoipsandboxchannel.html
pub struct APNSVoipSandboxChannel_ {
    pub application_id: crate::value::ExpString,
    pub bundle_id: Option<crate::value::ExpString>,
    pub certificate: Option<crate::value::ExpString>,
    pub default_authentication_method: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub private_key: Option<crate::value::ExpString>,
    pub team_id: Option<crate::value::ExpString>,
    pub token_key: Option<crate::value::ExpString>,
    pub token_key_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_APNSVoipSandboxChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::APNSVoipSandboxChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_APNSVoipSandboxChannel as APNSVoipSandboxChannel;
impl crate::template::ToResource for APNSVoipSandboxChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("APNSVoipSandboxChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.bundle_id {
            properties.insert(
                "BundleId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate {
            properties.insert(
                "Certificate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_authentication_method {
            properties.insert(
                "DefaultAuthenticationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.private_key {
            properties.insert(
                "PrivateKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.team_id {
            properties.insert("TeamId".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.token_key {
            properties.insert(
                "TokenKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.token_key_id {
            properties.insert(
                "TokenKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-app.html
pub struct App_ {
    pub name: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_App {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::App" $($field
        $value)*)
    };
}
pub use crate::__aws_pinpoint_App as App;
impl crate::template::ToResource for App_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("App"),
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-applicationsettings.html
pub struct ApplicationSettings_ {
    pub application_id: crate::value::ExpString,
    pub campaign_hook: Option<super::pinpoint::applicationsettings::CampaignHook_>,
    pub cloud_watch_metrics_enabled: Option<crate::value::ExpBool>,
    pub limits: Option<super::pinpoint::applicationsettings::Limits_>,
    pub quiet_time: Option<super::pinpoint::applicationsettings::QuietTime_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_ApplicationSettings {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::ApplicationSettings"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_ApplicationSettings as ApplicationSettings;
impl crate::template::ToResource for ApplicationSettings_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ApplicationSettings"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.campaign_hook {
            properties.insert(
                "CampaignHook".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cloud_watch_metrics_enabled {
            properties.insert(
                "CloudWatchMetricsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.limits {
            properties.insert("Limits".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.quiet_time {
            properties.insert(
                "QuietTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-baiduchannel.html
pub struct BaiduChannel_ {
    pub api_key: crate::value::ExpString,
    pub application_id: crate::value::ExpString,
    pub enabled: Option<crate::value::ExpBool>,
    pub secret_key: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_BaiduChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::BaiduChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_BaiduChannel as BaiduChannel;
impl crate::template::ToResource for BaiduChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("BaiduChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApiKey".to_string(),
            crate::value::ToValue::to_value(&self.api_key),
        );
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SecretKey".to_string(),
            crate::value::ToValue::to_value(&self.secret_key),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-campaign.html
pub struct Campaign_ {
    pub additional_treatments: Option<Vec<super::pinpoint::campaign::WriteTreatmentResource_>>,
    pub application_id: crate::value::ExpString,
    pub campaign_hook: Option<super::pinpoint::campaign::CampaignHook_>,
    pub custom_delivery_configuration:
        Option<super::pinpoint::campaign::CustomDeliveryConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub holdout_percent: Option<i64>,
    pub is_paused: Option<crate::value::ExpBool>,
    pub limits: Option<super::pinpoint::campaign::Limits_>,
    pub message_configuration: Option<super::pinpoint::campaign::MessageConfiguration_>,
    pub name: crate::value::ExpString,
    pub priority: Option<i64>,
    pub schedule: super::pinpoint::campaign::Schedule_,
    pub segment_id: crate::value::ExpString,
    pub segment_version: Option<i64>,
    pub tags: Option<serde_json::Value>,
    pub template_configuration: Option<super::pinpoint::campaign::TemplateConfiguration_>,
    pub treatment_description: Option<crate::value::ExpString>,
    pub treatment_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_Campaign {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::Campaign"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_Campaign as Campaign;
impl crate::template::ToResource for Campaign_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Campaign"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_treatments {
            properties.insert(
                "AdditionalTreatments".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.campaign_hook {
            properties.insert(
                "CampaignHook".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_delivery_configuration {
            properties.insert(
                "CustomDeliveryConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.holdout_percent {
            properties.insert(
                "HoldoutPercent".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_paused {
            properties.insert(
                "IsPaused".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.limits {
            properties.insert("Limits".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.message_configuration {
            properties.insert(
                "MessageConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.priority {
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Schedule".to_string(),
            crate::value::ToValue::to_value(&self.schedule),
        );
        properties.insert(
            "SegmentId".to_string(),
            crate::value::ToValue::to_value(&self.segment_id),
        );
        if let Some(ref value) = self.segment_version {
            properties.insert(
                "SegmentVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_configuration {
            properties.insert(
                "TemplateConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.treatment_description {
            properties.insert(
                "TreatmentDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.treatment_name {
            properties.insert(
                "TreatmentName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailchannel.html
pub struct EmailChannel_ {
    pub application_id: crate::value::ExpString,
    pub configuration_set: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub from_address: crate::value::ExpString,
    pub identity: crate::value::ExpString,
    pub orchestration_sending_role_arn: Option<crate::value::ExpString>,
    pub role_arn: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_EmailChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::EmailChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_EmailChannel as EmailChannel;
impl crate::template::ToResource for EmailChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EmailChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.configuration_set {
            properties.insert(
                "ConfigurationSet".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "FromAddress".to_string(),
            crate::value::ToValue::to_value(&self.from_address),
        );
        properties.insert(
            "Identity".to_string(),
            crate::value::ToValue::to_value(&self.identity),
        );
        if let Some(ref value) = self.orchestration_sending_role_arn {
            properties.insert(
                "OrchestrationSendingRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.role_arn {
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-emailtemplate.html
pub struct EmailTemplate_ {
    pub default_substitutions: Option<crate::value::ExpString>,
    pub html_part: Option<crate::value::ExpString>,
    pub subject: crate::value::ExpString,
    pub tags: Option<serde_json::Value>,
    pub template_description: Option<crate::value::ExpString>,
    pub template_name: crate::value::ExpString,
    pub text_part: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_EmailTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::EmailTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_EmailTemplate as EmailTemplate;
impl crate::template::ToResource for EmailTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EmailTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.default_substitutions {
            properties.insert(
                "DefaultSubstitutions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.html_part {
            properties.insert(
                "HtmlPart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Subject".to_string(),
            crate::value::ToValue::to_value(&self.subject),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_description {
            properties.insert(
                "TemplateDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TemplateName".to_string(),
            crate::value::ToValue::to_value(&self.template_name),
        );
        if let Some(ref value) = self.text_part {
            properties.insert(
                "TextPart".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-eventstream.html
pub struct EventStream_ {
    pub application_id: crate::value::ExpString,
    pub destination_stream_arn: crate::value::ExpString,
    pub role_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_EventStream {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::EventStream"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_EventStream as EventStream;
impl crate::template::ToResource for EventStream_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventStream"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        properties.insert(
            "DestinationStreamArn".to_string(),
            crate::value::ToValue::to_value(&self.destination_stream_arn),
        );
        properties.insert(
            "RoleArn".to_string(),
            crate::value::ToValue::to_value(&self.role_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-gcmchannel.html
pub struct GCMChannel_ {
    pub api_key: Option<crate::value::ExpString>,
    pub application_id: crate::value::ExpString,
    pub default_authentication_method: Option<crate::value::ExpString>,
    pub enabled: Option<crate::value::ExpBool>,
    pub service_json: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_GCMChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::GCMChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_GCMChannel as GCMChannel;
impl crate::template::ToResource for GCMChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GCMChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.api_key {
            properties.insert("ApiKey".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.default_authentication_method {
            properties.insert(
                "DefaultAuthenticationMethod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.service_json {
            properties.insert(
                "ServiceJson".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-inapptemplate.html
pub struct InAppTemplate_ {
    pub content: Option<Vec<super::pinpoint::inapptemplate::InAppMessageContent_>>,
    pub custom_config: Option<serde_json::Value>,
    pub layout: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub template_description: Option<crate::value::ExpString>,
    pub template_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_InAppTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::InAppTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_InAppTemplate as InAppTemplate;
impl crate::template::ToResource for InAppTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InAppTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.content {
            properties.insert(
                "Content".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_config {
            properties.insert(
                "CustomConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.layout {
            properties.insert("Layout".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_description {
            properties.insert(
                "TemplateDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TemplateName".to_string(),
            crate::value::ToValue::to_value(&self.template_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-pushtemplate.html
pub struct PushTemplate_ {
    pub adm: Option<super::pinpoint::pushtemplate::AndroidPushNotificationTemplate_>,
    pub apns: Option<super::pinpoint::pushtemplate::APNSPushNotificationTemplate_>,
    pub baidu: Option<super::pinpoint::pushtemplate::AndroidPushNotificationTemplate_>,
    pub default: Option<super::pinpoint::pushtemplate::DefaultPushNotificationTemplate_>,
    pub default_substitutions: Option<crate::value::ExpString>,
    pub gcm: Option<super::pinpoint::pushtemplate::AndroidPushNotificationTemplate_>,
    pub tags: Option<serde_json::Value>,
    pub template_description: Option<crate::value::ExpString>,
    pub template_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_PushTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::PushTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_PushTemplate as PushTemplate;
impl crate::template::ToResource for PushTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PushTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.adm {
            properties.insert("ADM".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.apns {
            properties.insert("APNS".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.baidu {
            properties.insert("Baidu".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.default {
            properties.insert(
                "Default".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_substitutions {
            properties.insert(
                "DefaultSubstitutions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gcm {
            properties.insert("GCM".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_description {
            properties.insert(
                "TemplateDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TemplateName".to_string(),
            crate::value::ToValue::to_value(&self.template_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smschannel.html
pub struct SMSChannel_ {
    pub application_id: crate::value::ExpString,
    pub enabled: Option<crate::value::ExpBool>,
    pub sender_id: Option<crate::value::ExpString>,
    pub short_code: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_SMSChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::SMSChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_SMSChannel as SMSChannel;
impl crate::template::ToResource for SMSChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SMSChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sender_id {
            properties.insert(
                "SenderId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.short_code {
            properties.insert(
                "ShortCode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-segment.html
pub struct Segment_ {
    pub application_id: crate::value::ExpString,
    pub dimensions: Option<super::pinpoint::segment::SegmentDimensions_>,
    pub name: crate::value::ExpString,
    pub segment_groups: Option<super::pinpoint::segment::SegmentGroups_>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_Segment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::Segment"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_Segment as Segment;
impl crate::template::ToResource for Segment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Segment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.dimensions {
            properties.insert(
                "Dimensions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.segment_groups {
            properties.insert(
                "SegmentGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-smstemplate.html
pub struct SmsTemplate_ {
    pub body: crate::value::ExpString,
    pub default_substitutions: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
    pub template_description: Option<crate::value::ExpString>,
    pub template_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_SmsTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::SmsTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_SmsTemplate as SmsTemplate;
impl crate::template::ToResource for SmsTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SmsTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Body".to_string(),
            crate::value::ToValue::to_value(&self.body),
        );
        if let Some(ref value) = self.default_substitutions {
            properties.insert(
                "DefaultSubstitutions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.template_description {
            properties.insert(
                "TemplateDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TemplateName".to_string(),
            crate::value::ToValue::to_value(&self.template_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpoint-voicechannel.html
pub struct VoiceChannel_ {
    pub application_id: crate::value::ExpString,
    pub enabled: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpoint_VoiceChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Pinpoint::VoiceChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpoint_VoiceChannel as VoiceChannel;
impl crate::template::ToResource for VoiceChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Pinpoint"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VoiceChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ApplicationId".to_string(),
            crate::value::ToValue::to_value(&self.application_id),
        );
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
