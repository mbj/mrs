pub mod configurationset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-configurationset-cloudwatchlogsdestination.html
    pub struct CloudWatchLogsDestination_ {
        pub iam_role_arn: crate::value::ExpString,
        pub log_group_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_ConfigurationSet_CloudWatchLogsDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::ConfigurationSet.CloudWatchLogsDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_ConfigurationSet_CloudWatchLogsDestination as CloudWatchLogsDestination;
    impl crate::value::ToValue for CloudWatchLogsDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IamRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.iam_role_arn),
            );
            properties.insert(
                "LogGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.log_group_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-configurationset-eventdestination.html
    pub struct EventDestination_ {
        pub cloud_watch_logs_destination: Option<Box<CloudWatchLogsDestination_>>,
        pub enabled: crate::value::ExpBool,
        pub event_destination_name: crate::value::ExpString,
        pub kinesis_firehose_destination: Option<Box<KinesisFirehoseDestination_>>,
        pub matching_event_types: Vec<crate::value::ExpString>,
        pub sns_destination: Option<Box<SnsDestination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_ConfigurationSet_EventDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::ConfigurationSet.EventDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_ConfigurationSet_EventDestination as EventDestination;
    impl crate::value::ToValue for EventDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_logs_destination {
                properties.insert(
                    "CloudWatchLogsDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.insert(
                "EventDestinationName".to_string(),
                crate::value::ToValue::to_value(&self.event_destination_name),
            );
            if let Some(ref value) = self.kinesis_firehose_destination {
                properties.insert(
                    "KinesisFirehoseDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MatchingEventTypes".to_string(),
                crate::value::ToValue::to_value(&self.matching_event_types),
            );
            if let Some(ref value) = self.sns_destination {
                properties.insert(
                    "SnsDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-configurationset-kinesisfirehosedestination.html
    pub struct KinesisFirehoseDestination_ {
        pub delivery_stream_arn: crate::value::ExpString,
        pub iam_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_ConfigurationSet_KinesisFirehoseDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::ConfigurationSet.KinesisFirehoseDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_ConfigurationSet_KinesisFirehoseDestination as KinesisFirehoseDestination;
    impl crate::value::ToValue for KinesisFirehoseDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeliveryStreamArn".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream_arn),
            );
            properties.insert(
                "IamRoleArn".to_string(),
                crate::value::ToValue::to_value(&self.iam_role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-configurationset-snsdestination.html
    pub struct SnsDestination_ {
        pub topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_ConfigurationSet_SnsDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::ConfigurationSet.SnsDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_ConfigurationSet_SnsDestination as SnsDestination;
    impl crate::value::ToValue for SnsDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TopicArn".to_string(),
                crate::value::ToValue::to_value(&self.topic_arn),
            );
            properties.into()
        }
    }
}
pub mod phonenumber {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-phonenumber-mandatorykeyword.html
    pub struct MandatoryKeyword_ {
        pub message: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_PhoneNumber_MandatoryKeyword {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::PhoneNumber.MandatoryKeyword"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_PhoneNumber_MandatoryKeyword as MandatoryKeyword;
    impl crate::value::ToValue for MandatoryKeyword_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(&self.message),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-phonenumber-mandatorykeywords.html
    pub struct MandatoryKeywords_ {
        pub help: Box<MandatoryKeyword_>,
        pub stop: Box<MandatoryKeyword_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_PhoneNumber_MandatoryKeywords {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::PhoneNumber.MandatoryKeywords"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_PhoneNumber_MandatoryKeywords as MandatoryKeywords;
    impl crate::value::ToValue for MandatoryKeywords_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HELP".to_string(),
                crate::value::ToValue::to_value(&self.help),
            );
            properties.insert(
                "STOP".to_string(),
                crate::value::ToValue::to_value(&self.stop),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-phonenumber-optionalkeyword.html
    pub struct OptionalKeyword_ {
        pub action: crate::value::ExpString,
        pub keyword: crate::value::ExpString,
        pub message: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_PhoneNumber_OptionalKeyword {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::PhoneNumber.OptionalKeyword"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_PhoneNumber_OptionalKeyword as OptionalKeyword;
    impl crate::value::ToValue for OptionalKeyword_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Keyword".to_string(),
                crate::value::ToValue::to_value(&self.keyword),
            );
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(&self.message),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-phonenumber-twoway.html
    pub struct TwoWay_ {
        pub channel_arn: Option<crate::value::ExpString>,
        pub channel_role: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_PhoneNumber_TwoWay {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::PhoneNumber.TwoWay"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_PhoneNumber_TwoWay as TwoWay;
    impl crate::value::ToValue for TwoWay_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_arn {
                properties.insert(
                    "ChannelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channel_role {
                properties.insert(
                    "ChannelRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
}
pub mod pool {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-pool-mandatorykeyword.html
    pub struct MandatoryKeyword_ {
        pub message: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_Pool_MandatoryKeyword {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::Pool.MandatoryKeyword"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_Pool_MandatoryKeyword as MandatoryKeyword;
    impl crate::value::ToValue for MandatoryKeyword_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(&self.message),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-pool-mandatorykeywords.html
    pub struct MandatoryKeywords_ {
        pub help: Box<MandatoryKeyword_>,
        pub stop: Box<MandatoryKeyword_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_Pool_MandatoryKeywords {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::Pool.MandatoryKeywords"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_Pool_MandatoryKeywords as MandatoryKeywords;
    impl crate::value::ToValue for MandatoryKeywords_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HELP".to_string(),
                crate::value::ToValue::to_value(&self.help),
            );
            properties.insert(
                "STOP".to_string(),
                crate::value::ToValue::to_value(&self.stop),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-pool-optionalkeyword.html
    pub struct OptionalKeyword_ {
        pub action: crate::value::ExpString,
        pub keyword: crate::value::ExpString,
        pub message: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_Pool_OptionalKeyword {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::Pool.OptionalKeyword"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_Pool_OptionalKeyword as OptionalKeyword;
    impl crate::value::ToValue for OptionalKeyword_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Keyword".to_string(),
                crate::value::ToValue::to_value(&self.keyword),
            );
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(&self.message),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-pool-twoway.html
    pub struct TwoWay_ {
        pub channel_arn: Option<crate::value::ExpString>,
        pub channel_role: Option<crate::value::ExpString>,
        pub enabled: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_Pool_TwoWay {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::Pool.TwoWay"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_Pool_TwoWay as TwoWay;
    impl crate::value::ToValue for TwoWay_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.channel_arn {
                properties.insert(
                    "ChannelArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.channel_role {
                properties.insert(
                    "ChannelRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(&self.enabled),
            );
            properties.into()
        }
    }
}
pub mod protectconfiguration {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-protectconfiguration-countryrule.html
    pub struct CountryRule_ {
        pub country_code: crate::value::ExpString,
        pub protect_status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_ProtectConfiguration_CountryRule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::ProtectConfiguration.CountryRule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_ProtectConfiguration_CountryRule as CountryRule;
    impl crate::value::ToValue for CountryRule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CountryCode".to_string(),
                crate::value::ToValue::to_value(&self.country_code),
            );
            properties.insert(
                "ProtectStatus".to_string(),
                crate::value::ToValue::to_value(&self.protect_status),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-smsvoice-protectconfiguration-countryruleset.html
    pub struct CountryRuleSet_ {
        pub mms: Option<Vec<CountryRule_>>,
        pub sms: Option<Vec<CountryRule_>>,
        pub voice: Option<Vec<CountryRule_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_smsvoice_ProtectConfiguration_CountryRuleSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SMSVOICE::ProtectConfiguration.CountryRuleSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_smsvoice_ProtectConfiguration_CountryRuleSet as CountryRuleSet;
    impl crate::value::ToValue for CountryRuleSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.mms {
                properties.insert("MMS".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sms {
                properties.insert("SMS".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.voice {
                properties.insert("VOICE".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-smsvoice-configurationset.html
pub struct ConfigurationSet_ {
    pub configuration_set_name: Option<crate::value::ExpString>,
    pub default_sender_id: Option<crate::value::ExpString>,
    pub event_destinations: Option<Vec<super::smsvoice::configurationset::EventDestination_>>,
    pub message_feedback_enabled: Option<crate::value::ExpBool>,
    pub protect_configuration_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_smsvoice_ConfigurationSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SMSVOICE::ConfigurationSet"
        $($field $value)*)
    };
}
pub use crate::__aws_smsvoice_ConfigurationSet as ConfigurationSet;
impl crate::template::ToResource for ConfigurationSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SMSVOICE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configuration_set_name {
            properties.insert(
                "ConfigurationSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_sender_id {
            properties.insert(
                "DefaultSenderId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_destinations {
            properties.insert(
                "EventDestinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.message_feedback_enabled {
            properties.insert(
                "MessageFeedbackEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.protect_configuration_id {
            properties.insert(
                "ProtectConfigurationId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-smsvoice-optoutlist.html
pub struct OptOutList_ {
    pub opt_out_list_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_smsvoice_OptOutList {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SMSVOICE::OptOutList"
        $($field $value)*)
    };
}
pub use crate::__aws_smsvoice_OptOutList as OptOutList;
impl crate::template::ToResource for OptOutList_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SMSVOICE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OptOutList"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.opt_out_list_name {
            properties.insert(
                "OptOutListName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-smsvoice-phonenumber.html
pub struct PhoneNumber_ {
    pub deletion_protection_enabled: Option<crate::value::ExpBool>,
    pub iso_country_code: crate::value::ExpString,
    pub mandatory_keywords: super::smsvoice::phonenumber::MandatoryKeywords_,
    pub number_capabilities: Vec<crate::value::ExpString>,
    pub number_type: crate::value::ExpString,
    pub opt_out_list_name: Option<crate::value::ExpString>,
    pub optional_keywords: Option<Vec<super::smsvoice::phonenumber::OptionalKeyword_>>,
    pub self_managed_opt_outs_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub two_way: Option<super::smsvoice::phonenumber::TwoWay_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_smsvoice_PhoneNumber {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SMSVOICE::PhoneNumber"
        $($field $value)*)
    };
}
pub use crate::__aws_smsvoice_PhoneNumber as PhoneNumber;
impl crate::template::ToResource for PhoneNumber_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SMSVOICE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PhoneNumber"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection_enabled {
            properties.insert(
                "DeletionProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IsoCountryCode".to_string(),
            crate::value::ToValue::to_value(&self.iso_country_code),
        );
        properties.insert(
            "MandatoryKeywords".to_string(),
            crate::value::ToValue::to_value(&self.mandatory_keywords),
        );
        properties.insert(
            "NumberCapabilities".to_string(),
            crate::value::ToValue::to_value(&self.number_capabilities),
        );
        properties.insert(
            "NumberType".to_string(),
            crate::value::ToValue::to_value(&self.number_type),
        );
        if let Some(ref value) = self.opt_out_list_name {
            properties.insert(
                "OptOutListName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.optional_keywords {
            properties.insert(
                "OptionalKeywords".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.self_managed_opt_outs_enabled {
            properties.insert(
                "SelfManagedOptOutsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.two_way {
            properties.insert("TwoWay".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-smsvoice-pool.html
pub struct Pool_ {
    pub deletion_protection_enabled: Option<crate::value::ExpBool>,
    pub mandatory_keywords: super::smsvoice::pool::MandatoryKeywords_,
    pub opt_out_list_name: Option<crate::value::ExpString>,
    pub optional_keywords: Option<Vec<super::smsvoice::pool::OptionalKeyword_>>,
    pub origination_identities: Vec<crate::value::ExpString>,
    pub self_managed_opt_outs_enabled: Option<crate::value::ExpBool>,
    pub shared_routes_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub two_way: Option<super::smsvoice::pool::TwoWay_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_smsvoice_Pool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SMSVOICE::Pool" $($field
        $value)*)
    };
}
pub use crate::__aws_smsvoice_Pool as Pool;
impl crate::template::ToResource for Pool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SMSVOICE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Pool"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection_enabled {
            properties.insert(
                "DeletionProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MandatoryKeywords".to_string(),
            crate::value::ToValue::to_value(&self.mandatory_keywords),
        );
        if let Some(ref value) = self.opt_out_list_name {
            properties.insert(
                "OptOutListName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.optional_keywords {
            properties.insert(
                "OptionalKeywords".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "OriginationIdentities".to_string(),
            crate::value::ToValue::to_value(&self.origination_identities),
        );
        if let Some(ref value) = self.self_managed_opt_outs_enabled {
            properties.insert(
                "SelfManagedOptOutsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.shared_routes_enabled {
            properties.insert(
                "SharedRoutesEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.two_way {
            properties.insert("TwoWay".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-smsvoice-protectconfiguration.html
pub struct ProtectConfiguration_ {
    pub country_rule_set: Option<super::smsvoice::protectconfiguration::CountryRuleSet_>,
    pub deletion_protection_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_smsvoice_ProtectConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SMSVOICE::ProtectConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_smsvoice_ProtectConfiguration as ProtectConfiguration;
impl crate::template::ToResource for ProtectConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SMSVOICE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ProtectConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.country_rule_set {
            properties.insert(
                "CountryRuleSet".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deletion_protection_enabled {
            properties.insert(
                "DeletionProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-smsvoice-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub policy_document: serde_json::Value,
    pub resource_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_smsvoice_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SMSVOICE::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_smsvoice_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SMSVOICE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-smsvoice-senderid.html
pub struct SenderId_ {
    pub deletion_protection_enabled: Option<crate::value::ExpBool>,
    pub iso_country_code: crate::value::ExpString,
    pub sender_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_smsvoice_SenderId {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SMSVOICE::SenderId"
        $($field $value)*)
    };
}
pub use crate::__aws_smsvoice_SenderId as SenderId;
impl crate::template::ToResource for SenderId_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SMSVOICE"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("SenderId"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.deletion_protection_enabled {
            properties.insert(
                "DeletionProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IsoCountryCode".to_string(),
            crate::value::ToValue::to_value(&self.iso_country_code),
        );
        properties.insert(
            "SenderId".to_string(),
            crate::value::ToValue::to_value(&self.sender_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
