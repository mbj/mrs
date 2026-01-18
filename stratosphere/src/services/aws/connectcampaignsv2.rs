pub mod campaign {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-answermachinedetectionconfig.html
    pub struct AnswerMachineDetectionConfig_ {
        pub await_answer_machine_prompt: Option<crate::value::ExpBool>,
        pub enable_answer_machine_detection: crate::value::ExpBool,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_AnswerMachineDetectionConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.AnswerMachineDetectionConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_AnswerMachineDetectionConfig as AnswerMachineDetectionConfig;
    impl crate::value::ToValue for AnswerMachineDetectionConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.await_answer_machine_prompt {
                properties.insert(
                    "AwaitAnswerMachinePrompt".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "EnableAnswerMachineDetection".to_string(),
                crate::value::ToValue::to_value(&self.enable_answer_machine_detection),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-channelsubtypeconfig.html
    pub struct ChannelSubtypeConfig_ {
        pub email: Option<Box<EmailChannelSubtypeConfig_>>,
        pub sms: Option<Box<SmsChannelSubtypeConfig_>>,
        pub telephony: Option<Box<TelephonyChannelSubtypeConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_ChannelSubtypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.ChannelSubtypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_ChannelSubtypeConfig as ChannelSubtypeConfig;
    impl crate::value::ToValue for ChannelSubtypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email {
                properties.insert("Email".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sms {
                properties.insert("Sms".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.telephony {
                properties.insert(
                    "Telephony".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-communicationlimit.html
    pub struct CommunicationLimit_ {
        pub frequency: i64,
        pub max_count_per_recipient: i64,
        pub unit: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_CommunicationLimit {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.CommunicationLimit"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_CommunicationLimit as CommunicationLimit;
    impl crate::value::ToValue for CommunicationLimit_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Frequency".to_string(),
                crate::value::ToValue::to_value(&self.frequency),
            );
            properties.insert(
                "MaxCountPerRecipient".to_string(),
                crate::value::ToValue::to_value(&self.max_count_per_recipient),
            );
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-communicationlimits.html
    pub struct CommunicationLimits_ {
        pub communication_limit_list: Option<Vec<CommunicationLimit_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_CommunicationLimits {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.CommunicationLimits"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_CommunicationLimits as CommunicationLimits;
    impl crate::value::ToValue for CommunicationLimits_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.communication_limit_list {
                properties.insert(
                    "CommunicationLimitList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-communicationlimitsconfig.html
    pub struct CommunicationLimitsConfig_ {
        pub all_channels_subtypes: Option<Box<CommunicationLimits_>>,
        pub instance_limits_handling: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_CommunicationLimitsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.CommunicationLimitsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_CommunicationLimitsConfig as CommunicationLimitsConfig;
    impl crate::value::ToValue for CommunicationLimitsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.all_channels_subtypes {
                properties.insert(
                    "AllChannelsSubtypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_limits_handling {
                properties.insert(
                    "InstanceLimitsHandling".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-communicationtimeconfig.html
    pub struct CommunicationTimeConfig_ {
        pub email: Option<Box<TimeWindow_>>,
        pub local_time_zone_config: Box<LocalTimeZoneConfig_>,
        pub sms: Option<Box<TimeWindow_>>,
        pub telephony: Option<Box<TimeWindow_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_CommunicationTimeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.CommunicationTimeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_CommunicationTimeConfig as CommunicationTimeConfig;
    impl crate::value::ToValue for CommunicationTimeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email {
                properties.insert("Email".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "LocalTimeZoneConfig".to_string(),
                crate::value::ToValue::to_value(&self.local_time_zone_config),
            );
            if let Some(ref value) = self.sms {
                properties.insert("Sms".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.telephony {
                properties.insert(
                    "Telephony".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-dailyhour.html
    pub struct DailyHour_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<Vec<TimeRange_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_DailyHour {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.DailyHour"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_DailyHour as DailyHour;
    impl crate::value::ToValue for DailyHour_ {
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-emailchannelsubtypeconfig.html
    pub struct EmailChannelSubtypeConfig_ {
        pub capacity: Option<f64>,
        pub default_outbound_config: Box<EmailOutboundConfig_>,
        pub outbound_mode: Box<EmailOutboundMode_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_EmailChannelSubtypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.EmailChannelSubtypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_EmailChannelSubtypeConfig as EmailChannelSubtypeConfig;
    impl crate::value::ToValue for EmailChannelSubtypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity {
                properties.insert(
                    "Capacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DefaultOutboundConfig".to_string(),
                crate::value::ToValue::to_value(&self.default_outbound_config),
            );
            properties.insert(
                "OutboundMode".to_string(),
                crate::value::ToValue::to_value(&self.outbound_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-emailoutboundconfig.html
    pub struct EmailOutboundConfig_ {
        pub connect_source_email_address: crate::value::ExpString,
        pub source_email_address_display_name: Option<crate::value::ExpString>,
        pub wisdom_template_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_EmailOutboundConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.EmailOutboundConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_EmailOutboundConfig as EmailOutboundConfig;
    impl crate::value::ToValue for EmailOutboundConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConnectSourceEmailAddress".to_string(),
                crate::value::ToValue::to_value(&self.connect_source_email_address),
            );
            if let Some(ref value) = self.source_email_address_display_name {
                properties.insert(
                    "SourceEmailAddressDisplayName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "WisdomTemplateArn".to_string(),
                crate::value::ToValue::to_value(&self.wisdom_template_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-emailoutboundmode.html
    pub struct EmailOutboundMode_ {
        pub agentless_config: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_EmailOutboundMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.EmailOutboundMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_EmailOutboundMode as EmailOutboundMode;
    impl crate::value::ToValue for EmailOutboundMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agentless_config {
                properties.insert(
                    "AgentlessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-eventtrigger.html
    pub struct EventTrigger_ {
        pub customer_profiles_domain_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_EventTrigger {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.EventTrigger"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_EventTrigger as EventTrigger;
    impl crate::value::ToValue for EventTrigger_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_profiles_domain_arn {
                properties.insert(
                    "CustomerProfilesDomainArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-localtimezoneconfig.html
    pub struct LocalTimeZoneConfig_ {
        pub default_time_zone: Option<crate::value::ExpString>,
        pub local_time_zone_detection: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_LocalTimeZoneConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.LocalTimeZoneConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_LocalTimeZoneConfig as LocalTimeZoneConfig;
    impl crate::value::ToValue for LocalTimeZoneConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.default_time_zone {
                properties.insert(
                    "DefaultTimeZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_time_zone_detection {
                properties.insert(
                    "LocalTimeZoneDetection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-openhours.html
    pub struct OpenHours_ {
        pub daily_hours: Vec<DailyHour_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_OpenHours {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.OpenHours"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_OpenHours as OpenHours;
    impl crate::value::ToValue for OpenHours_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DailyHours".to_string(),
                crate::value::ToValue::to_value(&self.daily_hours),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-predictiveconfig.html
    pub struct PredictiveConfig_ {
        pub bandwidth_allocation: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_PredictiveConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.PredictiveConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_PredictiveConfig as PredictiveConfig;
    impl crate::value::ToValue for PredictiveConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BandwidthAllocation".to_string(),
                crate::value::ToValue::to_value(&self.bandwidth_allocation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-progressiveconfig.html
    pub struct ProgressiveConfig_ {
        pub bandwidth_allocation: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_ProgressiveConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.ProgressiveConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_ProgressiveConfig as ProgressiveConfig;
    impl crate::value::ToValue for ProgressiveConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BandwidthAllocation".to_string(),
                crate::value::ToValue::to_value(&self.bandwidth_allocation),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-restrictedperiod.html
    pub struct RestrictedPeriod_ {
        pub end_date: crate::value::ExpString,
        pub name: Option<crate::value::ExpString>,
        pub start_date: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_RestrictedPeriod {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.RestrictedPeriod"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_RestrictedPeriod as RestrictedPeriod;
    impl crate::value::ToValue for RestrictedPeriod_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndDate".to_string(),
                crate::value::ToValue::to_value(&self.end_date),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "StartDate".to_string(),
                crate::value::ToValue::to_value(&self.start_date),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-restrictedperiods.html
    pub struct RestrictedPeriods_ {
        pub restricted_period_list: Vec<RestrictedPeriod_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_RestrictedPeriods {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.RestrictedPeriods"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_RestrictedPeriods as RestrictedPeriods;
    impl crate::value::ToValue for RestrictedPeriods_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RestrictedPeriodList".to_string(),
                crate::value::ToValue::to_value(&self.restricted_period_list),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-schedule.html
    pub struct Schedule_ {
        pub end_time: crate::value::ExpString,
        pub refresh_frequency: Option<crate::value::ExpString>,
        pub start_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_Schedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.Schedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_Schedule as Schedule;
    impl crate::value::ToValue for Schedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(&self.end_time),
            );
            if let Some(ref value) = self.refresh_frequency {
                properties.insert(
                    "RefreshFrequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-smschannelsubtypeconfig.html
    pub struct SmsChannelSubtypeConfig_ {
        pub capacity: Option<f64>,
        pub default_outbound_config: Box<SmsOutboundConfig_>,
        pub outbound_mode: Box<SmsOutboundMode_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_SmsChannelSubtypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.SmsChannelSubtypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_SmsChannelSubtypeConfig as SmsChannelSubtypeConfig;
    impl crate::value::ToValue for SmsChannelSubtypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity {
                properties.insert(
                    "Capacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DefaultOutboundConfig".to_string(),
                crate::value::ToValue::to_value(&self.default_outbound_config),
            );
            properties.insert(
                "OutboundMode".to_string(),
                crate::value::ToValue::to_value(&self.outbound_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-smsoutboundconfig.html
    pub struct SmsOutboundConfig_ {
        pub connect_source_phone_number_arn: crate::value::ExpString,
        pub wisdom_template_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_SmsOutboundConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.SmsOutboundConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_SmsOutboundConfig as SmsOutboundConfig;
    impl crate::value::ToValue for SmsOutboundConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ConnectSourcePhoneNumberArn".to_string(),
                crate::value::ToValue::to_value(&self.connect_source_phone_number_arn),
            );
            properties.insert(
                "WisdomTemplateArn".to_string(),
                crate::value::ToValue::to_value(&self.wisdom_template_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-smsoutboundmode.html
    pub struct SmsOutboundMode_ {
        pub agentless_config: Option<serde_json::Value>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_SmsOutboundMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.SmsOutboundMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_SmsOutboundMode as SmsOutboundMode;
    impl crate::value::ToValue for SmsOutboundMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agentless_config {
                properties.insert(
                    "AgentlessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-source.html
    pub struct Source_ {
        pub customer_profiles_segment_arn: Option<crate::value::ExpString>,
        pub event_trigger: Option<Box<EventTrigger_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_Source {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.Source"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_Source as Source;
    impl crate::value::ToValue for Source_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.customer_profiles_segment_arn {
                properties.insert(
                    "CustomerProfilesSegmentArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_trigger {
                properties.insert(
                    "EventTrigger".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-telephonychannelsubtypeconfig.html
    pub struct TelephonyChannelSubtypeConfig_ {
        pub capacity: Option<f64>,
        pub connect_queue_id: Option<crate::value::ExpString>,
        pub default_outbound_config: Box<TelephonyOutboundConfig_>,
        pub outbound_mode: Box<TelephonyOutboundMode_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_TelephonyChannelSubtypeConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.TelephonyChannelSubtypeConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_TelephonyChannelSubtypeConfig as TelephonyChannelSubtypeConfig;
    impl crate::value::ToValue for TelephonyChannelSubtypeConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity {
                properties.insert(
                    "Capacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connect_queue_id {
                properties.insert(
                    "ConnectQueueId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DefaultOutboundConfig".to_string(),
                crate::value::ToValue::to_value(&self.default_outbound_config),
            );
            properties.insert(
                "OutboundMode".to_string(),
                crate::value::ToValue::to_value(&self.outbound_mode),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-telephonyoutboundconfig.html
    pub struct TelephonyOutboundConfig_ {
        pub answer_machine_detection_config: Option<Box<AnswerMachineDetectionConfig_>>,
        pub connect_contact_flow_id: crate::value::ExpString,
        pub connect_source_phone_number: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_TelephonyOutboundConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.TelephonyOutboundConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_TelephonyOutboundConfig as TelephonyOutboundConfig;
    impl crate::value::ToValue for TelephonyOutboundConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.answer_machine_detection_config {
                properties.insert(
                    "AnswerMachineDetectionConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ConnectContactFlowId".to_string(),
                crate::value::ToValue::to_value(&self.connect_contact_flow_id),
            );
            if let Some(ref value) = self.connect_source_phone_number {
                properties.insert(
                    "ConnectSourcePhoneNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-telephonyoutboundmode.html
    pub struct TelephonyOutboundMode_ {
        pub agentless_config: Option<serde_json::Value>,
        pub predictive_config: Option<Box<PredictiveConfig_>>,
        pub progressive_config: Option<Box<ProgressiveConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_TelephonyOutboundMode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.TelephonyOutboundMode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_TelephonyOutboundMode as TelephonyOutboundMode;
    impl crate::value::ToValue for TelephonyOutboundMode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.agentless_config {
                properties.insert(
                    "AgentlessConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.predictive_config {
                properties.insert(
                    "PredictiveConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.progressive_config {
                properties.insert(
                    "ProgressiveConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-timerange.html
    pub struct TimeRange_ {
        pub end_time: crate::value::ExpString,
        pub start_time: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_TimeRange {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.TimeRange"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_TimeRange as TimeRange;
    impl crate::value::ToValue for TimeRange_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(&self.end_time),
            );
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(&self.start_time),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-connectcampaignsv2-campaign-timewindow.html
    pub struct TimeWindow_ {
        pub open_hours: Box<OpenHours_>,
        pub restricted_periods: Option<Box<RestrictedPeriods_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_connectcampaignsv2_Campaign_TimeWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ConnectCampaignsV2::Campaign.TimeWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_connectcampaignsv2_Campaign_TimeWindow as TimeWindow;
    impl crate::value::ToValue for TimeWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OpenHours".to_string(),
                crate::value::ToValue::to_value(&self.open_hours),
            );
            if let Some(ref value) = self.restricted_periods {
                properties.insert(
                    "RestrictedPeriods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-connectcampaignsv2-campaign.html
pub struct Campaign_ {
    pub channel_subtype_config: super::connectcampaignsv2::campaign::ChannelSubtypeConfig_,
    pub communication_limits_override:
        Option<super::connectcampaignsv2::campaign::CommunicationLimitsConfig_>,
    pub communication_time_config:
        Option<super::connectcampaignsv2::campaign::CommunicationTimeConfig_>,
    pub connect_campaign_flow_arn: Option<crate::value::ExpString>,
    pub connect_instance_id: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub schedule: Option<super::connectcampaignsv2::campaign::Schedule_>,
    pub source: Option<super::connectcampaignsv2::campaign::Source_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_connectcampaignsv2_Campaign {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ConnectCampaignsV2::Campaign"
        $($field $value)*)
    };
}
pub use crate::__aws_connectcampaignsv2_Campaign as Campaign;
impl crate::template::ToResource for Campaign_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ConnectCampaignsV2"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Campaign"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelSubtypeConfig".to_string(),
            crate::value::ToValue::to_value(&self.channel_subtype_config),
        );
        if let Some(ref value) = self.communication_limits_override {
            properties.insert(
                "CommunicationLimitsOverride".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.communication_time_config {
            properties.insert(
                "CommunicationTimeConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.connect_campaign_flow_arn {
            properties.insert(
                "ConnectCampaignFlowArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ConnectInstanceId".to_string(),
            crate::value::ToValue::to_value(&self.connect_instance_id),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source {
            properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
