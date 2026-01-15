pub mod configurationset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-dashboardoptions.html
    pub struct DashboardOptions_ {
        pub engagement_metrics: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_DashboardOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.DashboardOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_DashboardOptions as DashboardOptions;
    impl crate::value::ToValue for DashboardOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EngagementMetrics".to_string(),
                crate::value::ToValue::to_value(&self.engagement_metrics),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-deliveryoptions.html
    pub struct DeliveryOptions_ {
        pub max_delivery_seconds: Option<f64>,
        pub sending_pool_name: Option<crate::value::ExpString>,
        pub tls_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_DeliveryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.DeliveryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_DeliveryOptions as DeliveryOptions;
    impl crate::value::ToValue for DeliveryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.max_delivery_seconds {
                properties.insert(
                    "MaxDeliverySeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sending_pool_name {
                properties.insert(
                    "SendingPoolName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tls_policy {
                properties.insert(
                    "TlsPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-guardianoptions.html
    pub struct GuardianOptions_ {
        pub optimized_shared_delivery: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_GuardianOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.GuardianOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_GuardianOptions as GuardianOptions;
    impl crate::value::ToValue for GuardianOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OptimizedSharedDelivery".to_string(),
                crate::value::ToValue::to_value(&self.optimized_shared_delivery),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-reputationoptions.html
    pub struct ReputationOptions_ {
        pub reputation_metrics_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_ReputationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.ReputationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_ReputationOptions as ReputationOptions;
    impl crate::value::ToValue for ReputationOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.reputation_metrics_enabled {
                properties.insert(
                    "ReputationMetricsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-sendingoptions.html
    pub struct SendingOptions_ {
        pub sending_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_SendingOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.SendingOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_SendingOptions as SendingOptions;
    impl crate::value::ToValue for SendingOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sending_enabled {
                properties.insert(
                    "SendingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-suppressionoptions.html
    pub struct SuppressionOptions_ {
        pub suppressed_reasons: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_SuppressionOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.SuppressionOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_SuppressionOptions as SuppressionOptions;
    impl crate::value::ToValue for SuppressionOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.suppressed_reasons {
                properties.insert(
                    "SuppressedReasons".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-trackingoptions.html
    pub struct TrackingOptions_ {
        pub custom_redirect_domain: Option<crate::value::ExpString>,
        pub https_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_TrackingOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.TrackingOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_TrackingOptions as TrackingOptions;
    impl crate::value::ToValue for TrackingOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_redirect_domain {
                properties.insert(
                    "CustomRedirectDomain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.https_policy {
                properties.insert(
                    "HttpsPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationset-vdmoptions.html
    pub struct VdmOptions_ {
        pub dashboard_options: Option<Box<DashboardOptions_>>,
        pub guardian_options: Option<Box<GuardianOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSet_VdmOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSet.VdmOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSet_VdmOptions as VdmOptions;
    impl crate::value::ToValue for VdmOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dashboard_options {
                properties.insert(
                    "DashboardOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.guardian_options {
                properties.insert(
                    "GuardianOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod configurationseteventdestination {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-cloudwatchdestination.html
    pub struct CloudWatchDestination_ {
        pub dimension_configurations: Option<Vec<DimensionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSetEventDestination_CloudWatchDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSetEventDestination.CloudWatchDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSetEventDestination_CloudWatchDestination as CloudWatchDestination;
    impl crate::value::ToValue for CloudWatchDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimension_configurations {
                properties.insert(
                    "DimensionConfigurations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-dimensionconfiguration.html
    pub struct DimensionConfiguration_ {
        pub default_dimension_value: crate::value::ExpString,
        pub dimension_name: crate::value::ExpString,
        pub dimension_value_source: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSetEventDestination_DimensionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSetEventDestination.DimensionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSetEventDestination_DimensionConfiguration as DimensionConfiguration;
    impl crate::value::ToValue for DimensionConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultDimensionValue".to_string(),
                crate::value::ToValue::to_value(&self.default_dimension_value),
            );
            properties.insert(
                "DimensionName".to_string(),
                crate::value::ToValue::to_value(&self.dimension_name),
            );
            properties.insert(
                "DimensionValueSource".to_string(),
                crate::value::ToValue::to_value(&self.dimension_value_source),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventbridgedestination.html
    pub struct EventBridgeDestination_ {
        pub event_bus_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSetEventDestination_EventBridgeDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSetEventDestination.EventBridgeDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSetEventDestination_EventBridgeDestination as EventBridgeDestination;
    impl crate::value::ToValue for EventBridgeDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventBusArn".to_string(),
                crate::value::ToValue::to_value(&self.event_bus_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-eventdestination.html
    pub struct EventDestination_ {
        pub cloud_watch_destination: Option<Box<CloudWatchDestination_>>,
        pub enabled: Option<crate::value::ExpBool>,
        pub event_bridge_destination: Option<Box<EventBridgeDestination_>>,
        pub kinesis_firehose_destination: Option<Box<KinesisFirehoseDestination_>>,
        pub matching_event_types: Vec<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
        pub sns_destination: Option<Box<SnsDestination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSetEventDestination_EventDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSetEventDestination.EventDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSetEventDestination_EventDestination as EventDestination;
    impl crate::value::ToValue for EventDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_watch_destination {
                properties.insert(
                    "CloudWatchDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_bridge_destination {
                properties.insert(
                    "EventBridgeDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
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
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.sns_destination {
                properties.insert(
                    "SnsDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-kinesisfirehosedestination.html
    pub struct KinesisFirehoseDestination_ {
        pub delivery_stream_arn: crate::value::ExpString,
        pub iam_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSetEventDestination_KinesisFirehoseDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSetEventDestination.KinesisFirehoseDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSetEventDestination_KinesisFirehoseDestination as KinesisFirehoseDestination;
    impl crate::value::ToValue for KinesisFirehoseDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DeliveryStreamARN".to_string(),
                crate::value::ToValue::to_value(&self.delivery_stream_arn),
            );
            properties.insert(
                "IAMRoleARN".to_string(),
                crate::value::ToValue::to_value(&self.iam_role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-configurationseteventdestination-snsdestination.html
    pub struct SnsDestination_ {
        pub topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ConfigurationSetEventDestination_SnsDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ConfigurationSetEventDestination.SnsDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ConfigurationSetEventDestination_SnsDestination as SnsDestination;
    impl crate::value::ToValue for SnsDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TopicARN".to_string(),
                crate::value::ToValue::to_value(&self.topic_arn),
            );
            properties.into()
        }
    }
}
pub mod contactlist {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-contactlist-topic.html
    pub struct Topic_ {
        pub default_subscription_status: crate::value::ExpString,
        pub description: Option<crate::value::ExpString>,
        pub display_name: crate::value::ExpString,
        pub topic_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ContactList_Topic {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ContactList.Topic"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ContactList_Topic as Topic;
    impl crate::value::ToValue for Topic_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DefaultSubscriptionStatus".to_string(),
                crate::value::ToValue::to_value(&self.default_subscription_status),
            );
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(&self.display_name),
            );
            properties.insert(
                "TopicName".to_string(),
                crate::value::ToValue::to_value(&self.topic_name),
            );
            properties.into()
        }
    }
}
pub mod emailidentity {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-configurationsetattributes.html
    pub struct ConfigurationSetAttributes_ {
        pub configuration_set_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_EmailIdentity_ConfigurationSetAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::EmailIdentity.ConfigurationSetAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_EmailIdentity_ConfigurationSetAttributes as ConfigurationSetAttributes;
    impl crate::value::ToValue for ConfigurationSetAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configuration_set_name {
                properties.insert(
                    "ConfigurationSetName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimattributes.html
    pub struct DkimAttributes_ {
        pub signing_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_EmailIdentity_DkimAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::EmailIdentity.DkimAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_EmailIdentity_DkimAttributes as DkimAttributes;
    impl crate::value::ToValue for DkimAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.signing_enabled {
                properties.insert(
                    "SigningEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-dkimsigningattributes.html
    pub struct DkimSigningAttributes_ {
        pub domain_signing_private_key: Option<crate::value::ExpString>,
        pub domain_signing_selector: Option<crate::value::ExpString>,
        pub next_signing_key_length: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_EmailIdentity_DkimSigningAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::EmailIdentity.DkimSigningAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_EmailIdentity_DkimSigningAttributes as DkimSigningAttributes;
    impl crate::value::ToValue for DkimSigningAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_signing_private_key {
                properties.insert(
                    "DomainSigningPrivateKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_signing_selector {
                properties.insert(
                    "DomainSigningSelector".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.next_signing_key_length {
                properties.insert(
                    "NextSigningKeyLength".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-feedbackattributes.html
    pub struct FeedbackAttributes_ {
        pub email_forwarding_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_EmailIdentity_FeedbackAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::EmailIdentity.FeedbackAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_EmailIdentity_FeedbackAttributes as FeedbackAttributes;
    impl crate::value::ToValue for FeedbackAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email_forwarding_enabled {
                properties.insert(
                    "EmailForwardingEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-emailidentity-mailfromattributes.html
    pub struct MailFromAttributes_ {
        pub behavior_on_mx_failure: Option<crate::value::ExpString>,
        pub mail_from_domain: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_EmailIdentity_MailFromAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::EmailIdentity.MailFromAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_EmailIdentity_MailFromAttributes as MailFromAttributes;
    impl crate::value::ToValue for MailFromAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.behavior_on_mx_failure {
                properties.insert(
                    "BehaviorOnMxFailure".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mail_from_domain {
                properties.insert(
                    "MailFromDomain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod mailmanagerarchive {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerarchive-archiveretention.html
    pub struct ArchiveRetention_ {
        pub retention_period: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerArchive_ArchiveRetention {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerArchive.ArchiveRetention"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerArchive_ArchiveRetention as ArchiveRetention;
    impl crate::value::ToValue for ArchiveRetention_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "RetentionPeriod".to_string(),
                crate::value::ToValue::to_value(&self.retention_period),
            );
            properties.into()
        }
    }
}
pub mod mailmanageringresspoint {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanageringresspoint-ingresspointconfiguration.html
    pub struct IngressPointConfiguration_ {
        pub secret_arn: Option<crate::value::ExpString>,
        pub smtp_password: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerIngressPoint_IngressPointConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerIngressPoint.IngressPointConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerIngressPoint_IngressPointConfiguration as IngressPointConfiguration;
    impl crate::value::ToValue for IngressPointConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.smtp_password {
                properties.insert(
                    "SmtpPassword".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanageringresspoint-networkconfiguration.html
    pub struct NetworkConfiguration_ {
        pub private_network_configuration: Option<Box<PrivateNetworkConfiguration_>>,
        pub public_network_configuration: Option<Box<PublicNetworkConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerIngressPoint_NetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerIngressPoint.NetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerIngressPoint_NetworkConfiguration as NetworkConfiguration;
    impl crate::value::ToValue for NetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.private_network_configuration {
                properties.insert(
                    "PrivateNetworkConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_network_configuration {
                properties.insert(
                    "PublicNetworkConfiguration".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanageringresspoint-privatenetworkconfiguration.html
    pub struct PrivateNetworkConfiguration_ {
        pub vpc_endpoint_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerIngressPoint_PrivateNetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerIngressPoint.PrivateNetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerIngressPoint_PrivateNetworkConfiguration as PrivateNetworkConfiguration;
    impl crate::value::ToValue for PrivateNetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VpcEndpointId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_endpoint_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanageringresspoint-publicnetworkconfiguration.html
    pub struct PublicNetworkConfiguration_ {
        pub ip_type: serde_json::Value,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerIngressPoint_PublicNetworkConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerIngressPoint.PublicNetworkConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerIngressPoint_PublicNetworkConfiguration as PublicNetworkConfiguration;
    impl crate::value::ToValue for PublicNetworkConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IpType".to_string(),
                crate::value::ToValue::to_value(&self.ip_type),
            );
            properties.into()
        }
    }
}
pub mod mailmanagerrelay {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerrelay-relayauthentication.html
    pub struct RelayAuthentication_ {
        pub no_authentication: Option<serde_json::Value>,
        pub secret_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRelay_RelayAuthentication {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRelay.RelayAuthentication"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRelay_RelayAuthentication as RelayAuthentication;
    impl crate::value::ToValue for RelayAuthentication_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.no_authentication {
                properties.insert(
                    "NoAuthentication".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.secret_arn {
                properties.insert(
                    "SecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod mailmanagerruleset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-addheaderaction.html
    pub struct AddHeaderAction_ {
        pub header_name: crate::value::ExpString,
        pub header_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_AddHeaderAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.AddHeaderAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_AddHeaderAction as AddHeaderAction;
    impl crate::value::ToValue for AddHeaderAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HeaderName".to_string(),
                crate::value::ToValue::to_value(&self.header_name),
            );
            properties.insert(
                "HeaderValue".to_string(),
                crate::value::ToValue::to_value(&self.header_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-analysis.html
    pub struct Analysis_ {
        pub analyzer: crate::value::ExpString,
        pub result_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_Analysis {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.Analysis"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_Analysis as Analysis;
    impl crate::value::ToValue for Analysis_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Analyzer".to_string(),
                crate::value::ToValue::to_value(&self.analyzer),
            );
            properties.insert(
                "ResultField".to_string(),
                crate::value::ToValue::to_value(&self.result_field),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-archiveaction.html
    pub struct ArchiveAction_ {
        pub action_failure_policy: Option<crate::value::ExpString>,
        pub target_archive: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_ArchiveAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.ArchiveAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_ArchiveAction as ArchiveAction;
    impl crate::value::ToValue for ArchiveAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_failure_policy {
                properties.insert(
                    "ActionFailurePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "TargetArchive".to_string(),
                crate::value::ToValue::to_value(&self.target_archive),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-delivertomailboxaction.html
    pub struct DeliverToMailboxAction_ {
        pub action_failure_policy: Option<crate::value::ExpString>,
        pub mailbox_arn: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_DeliverToMailboxAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.DeliverToMailboxAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_DeliverToMailboxAction as DeliverToMailboxAction;
    impl crate::value::ToValue for DeliverToMailboxAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_failure_policy {
                properties.insert(
                    "ActionFailurePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "MailboxArn".to_string(),
                crate::value::ToValue::to_value(&self.mailbox_arn),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-delivertoqbusinessaction.html
    pub struct DeliverToQBusinessAction_ {
        pub action_failure_policy: Option<crate::value::ExpString>,
        pub application_id: crate::value::ExpString,
        pub index_id: crate::value::ExpString,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_DeliverToQBusinessAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.DeliverToQBusinessAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_DeliverToQBusinessAction as DeliverToQBusinessAction;
    impl crate::value::ToValue for DeliverToQBusinessAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_failure_policy {
                properties.insert(
                    "ActionFailurePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ApplicationId".to_string(),
                crate::value::ToValue::to_value(&self.application_id),
            );
            properties.insert(
                "IndexId".to_string(),
                crate::value::ToValue::to_value(&self.index_id),
            );
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-relayaction.html
    pub struct RelayAction_ {
        pub action_failure_policy: Option<crate::value::ExpString>,
        pub mail_from: Option<crate::value::ExpString>,
        pub relay: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RelayAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RelayAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RelayAction as RelayAction;
    impl crate::value::ToValue for RelayAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_failure_policy {
                properties.insert(
                    "ActionFailurePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mail_from {
                properties.insert(
                    "MailFrom".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Relay".to_string(),
                crate::value::ToValue::to_value(&self.relay),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-replacerecipientaction.html
    pub struct ReplaceRecipientAction_ {
        pub replace_with: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_ReplaceRecipientAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.ReplaceRecipientAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_ReplaceRecipientAction as ReplaceRecipientAction;
    impl crate::value::ToValue for ReplaceRecipientAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.replace_with {
                properties.insert(
                    "ReplaceWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rule.html
    pub struct Rule_ {
        pub actions: Vec<RuleAction_>,
        pub conditions: Option<Vec<RuleCondition_>>,
        pub name: Option<crate::value::ExpString>,
        pub unless: Option<Vec<RuleCondition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Actions".to_string(),
                crate::value::ToValue::to_value(&self.actions),
            );
            if let Some(ref value) = self.conditions {
                properties.insert(
                    "Conditions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.unless {
                properties.insert("Unless".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-ruleaction.html
    pub struct RuleAction_ {
        pub add_header: Option<Box<AddHeaderAction_>>,
        pub archive: Option<Box<ArchiveAction_>>,
        pub deliver_to_mailbox: Option<Box<DeliverToMailboxAction_>>,
        pub deliver_to_q_business: Option<Box<DeliverToQBusinessAction_>>,
        pub drop: Option<serde_json::Value>,
        pub publish_to_sns: Option<Box<SnsAction_>>,
        pub relay: Option<Box<RelayAction_>>,
        pub replace_recipient: Option<Box<ReplaceRecipientAction_>>,
        pub send: Option<Box<SendAction_>>,
        pub write_to_s3: Option<Box<S3Action_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleAction as RuleAction;
    impl crate::value::ToValue for RuleAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_header {
                properties.insert(
                    "AddHeader".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.archive {
                properties.insert(
                    "Archive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deliver_to_mailbox {
                properties.insert(
                    "DeliverToMailbox".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.deliver_to_q_business {
                properties.insert(
                    "DeliverToQBusiness".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.drop {
                properties.insert("Drop".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.publish_to_sns {
                properties.insert(
                    "PublishToSns".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.relay {
                properties.insert("Relay".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.replace_recipient {
                properties.insert(
                    "ReplaceRecipient".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.send {
                properties.insert("Send".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.write_to_s3 {
                properties.insert(
                    "WriteToS3".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rulebooleanexpression.html
    pub struct RuleBooleanExpression_ {
        pub evaluate: Box<RuleBooleanToEvaluate_>,
        pub operator: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleBooleanExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleBooleanExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleBooleanExpression as RuleBooleanExpression;
    impl crate::value::ToValue for RuleBooleanExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rulebooleantoevaluate.html
    pub struct RuleBooleanToEvaluate_ {
        pub analysis: Option<Box<Analysis_>>,
        pub attribute: Option<crate::value::ExpString>,
        pub is_in_address_list: Option<Box<RuleIsInAddressList_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleBooleanToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleBooleanToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleBooleanToEvaluate as RuleBooleanToEvaluate;
    impl crate::value::ToValue for RuleBooleanToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.analysis {
                properties.insert(
                    "Analysis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attribute {
                properties.insert(
                    "Attribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_in_address_list {
                properties.insert(
                    "IsInAddressList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rulecondition.html
    pub struct RuleCondition_ {
        pub boolean_expression: Option<Box<RuleBooleanExpression_>>,
        pub dmarc_expression: Option<Box<RuleDmarcExpression_>>,
        pub ip_expression: Option<Box<RuleIpExpression_>>,
        pub number_expression: Option<Box<RuleNumberExpression_>>,
        pub string_expression: Option<Box<RuleStringExpression_>>,
        pub verdict_expression: Option<Box<RuleVerdictExpression_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleCondition as RuleCondition;
    impl crate::value::ToValue for RuleCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean_expression {
                properties.insert(
                    "BooleanExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.dmarc_expression {
                properties.insert(
                    "DmarcExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_expression {
                properties.insert(
                    "IpExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_expression {
                properties.insert(
                    "NumberExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_expression {
                properties.insert(
                    "StringExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.verdict_expression {
                properties.insert(
                    "VerdictExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-ruledmarcexpression.html
    pub struct RuleDmarcExpression_ {
        pub operator: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleDmarcExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleDmarcExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleDmarcExpression as RuleDmarcExpression;
    impl crate::value::ToValue for RuleDmarcExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-ruleipexpression.html
    pub struct RuleIpExpression_ {
        pub evaluate: Box<RuleIpToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleIpExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleIpExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleIpExpression as RuleIpExpression;
    impl crate::value::ToValue for RuleIpExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-ruleiptoevaluate.html
    pub struct RuleIpToEvaluate_ {
        pub attribute: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleIpToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleIpToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleIpToEvaluate as RuleIpToEvaluate;
    impl crate::value::ToValue for RuleIpToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-ruleisinaddresslist.html
    pub struct RuleIsInAddressList_ {
        pub address_lists: Vec<crate::value::ExpString>,
        pub attribute: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleIsInAddressList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleIsInAddressList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleIsInAddressList as RuleIsInAddressList;
    impl crate::value::ToValue for RuleIsInAddressList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AddressLists".to_string(),
                crate::value::ToValue::to_value(&self.address_lists),
            );
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rulenumberexpression.html
    pub struct RuleNumberExpression_ {
        pub evaluate: Box<RuleNumberToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub value: f64,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleNumberExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleNumberExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleNumberExpression as RuleNumberExpression;
    impl crate::value::ToValue for RuleNumberExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rulenumbertoevaluate.html
    pub struct RuleNumberToEvaluate_ {
        pub attribute: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleNumberToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleNumberToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleNumberToEvaluate as RuleNumberToEvaluate;
    impl crate::value::ToValue for RuleNumberToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rulestringexpression.html
    pub struct RuleStringExpression_ {
        pub evaluate: Box<RuleStringToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleStringExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleStringExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleStringExpression as RuleStringExpression;
    impl crate::value::ToValue for RuleStringExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-rulestringtoevaluate.html
    pub struct RuleStringToEvaluate_ {
        pub analysis: Option<Box<Analysis_>>,
        pub attribute: Option<crate::value::ExpString>,
        pub mime_header_attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleStringToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleStringToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleStringToEvaluate as RuleStringToEvaluate;
    impl crate::value::ToValue for RuleStringToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.analysis {
                properties.insert(
                    "Analysis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attribute {
                properties.insert(
                    "Attribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.mime_header_attribute {
                properties.insert(
                    "MimeHeaderAttribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-ruleverdictexpression.html
    pub struct RuleVerdictExpression_ {
        pub evaluate: Box<RuleVerdictToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleVerdictExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleVerdictExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleVerdictExpression as RuleVerdictExpression;
    impl crate::value::ToValue for RuleVerdictExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-ruleverdicttoevaluate.html
    pub struct RuleVerdictToEvaluate_ {
        pub analysis: Option<Box<Analysis_>>,
        pub attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_RuleVerdictToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.RuleVerdictToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_RuleVerdictToEvaluate as RuleVerdictToEvaluate;
    impl crate::value::ToValue for RuleVerdictToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.analysis {
                properties.insert(
                    "Analysis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attribute {
                properties.insert(
                    "Attribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-s3action.html
    pub struct S3Action_ {
        pub action_failure_policy: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub s3_bucket: crate::value::ExpString,
        pub s3_prefix: Option<crate::value::ExpString>,
        pub s3_sse_kms_key_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_S3Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.S3Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_S3Action as S3Action;
    impl crate::value::ToValue for S3Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_failure_policy {
                properties.insert(
                    "ActionFailurePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "S3Bucket".to_string(),
                crate::value::ToValue::to_value(&self.s3_bucket),
            );
            if let Some(ref value) = self.s3_prefix {
                properties.insert(
                    "S3Prefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_sse_kms_key_id {
                properties.insert(
                    "S3SseKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-sendaction.html
    pub struct SendAction_ {
        pub action_failure_policy: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_SendAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.SendAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_SendAction as SendAction;
    impl crate::value::ToValue for SendAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_failure_policy {
                properties.insert(
                    "ActionFailurePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagerruleset-snsaction.html
    pub struct SnsAction_ {
        pub action_failure_policy: Option<crate::value::ExpString>,
        pub encoding: Option<crate::value::ExpString>,
        pub payload_type: Option<crate::value::ExpString>,
        pub role_arn: crate::value::ExpString,
        pub topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerRuleSet_SnsAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerRuleSet.SnsAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerRuleSet_SnsAction as SnsAction;
    impl crate::value::ToValue for SnsAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.action_failure_policy {
                properties.insert(
                    "ActionFailurePolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.encoding {
                properties.insert(
                    "Encoding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.payload_type {
                properties.insert(
                    "PayloadType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "RoleArn".to_string(),
                crate::value::ToValue::to_value(&self.role_arn),
            );
            properties.insert(
                "TopicArn".to_string(),
                crate::value::ToValue::to_value(&self.topic_arn),
            );
            properties.into()
        }
    }
}
pub mod mailmanagertrafficpolicy {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressanalysis.html
    pub struct IngressAnalysis_ {
        pub analyzer: crate::value::ExpString,
        pub result_field: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressAnalysis {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressAnalysis"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressAnalysis as IngressAnalysis;
    impl crate::value::ToValue for IngressAnalysis_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Analyzer".to_string(),
                crate::value::ToValue::to_value(&self.analyzer),
            );
            properties.insert(
                "ResultField".to_string(),
                crate::value::ToValue::to_value(&self.result_field),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressbooleanexpression.html
    pub struct IngressBooleanExpression_ {
        pub evaluate: Box<IngressBooleanToEvaluate_>,
        pub operator: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressBooleanExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressBooleanExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressBooleanExpression as IngressBooleanExpression;
    impl crate::value::ToValue for IngressBooleanExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressbooleantoevaluate.html
    pub struct IngressBooleanToEvaluate_ {
        pub analysis: Option<Box<IngressAnalysis_>>,
        pub is_in_address_list: Option<Box<IngressIsInAddressList_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressBooleanToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressBooleanToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressBooleanToEvaluate as IngressBooleanToEvaluate;
    impl crate::value::ToValue for IngressBooleanToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.analysis {
                properties.insert(
                    "Analysis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_in_address_list {
                properties.insert(
                    "IsInAddressList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressiptoevaluate.html
    pub struct IngressIpToEvaluate_ {
        pub attribute: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressIpToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressIpToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressIpToEvaluate as IngressIpToEvaluate;
    impl crate::value::ToValue for IngressIpToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressipv4expression.html
    pub struct IngressIpv4Expression_ {
        pub evaluate: Box<IngressIpToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressIpv4Expression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressIpv4Expression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressIpv4Expression as IngressIpv4Expression;
    impl crate::value::ToValue for IngressIpv4Expression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressipv6expression.html
    pub struct IngressIpv6Expression_ {
        pub evaluate: Box<IngressIpv6ToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressIpv6Expression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressIpv6Expression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressIpv6Expression as IngressIpv6Expression;
    impl crate::value::ToValue for IngressIpv6Expression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressipv6toevaluate.html
    pub struct IngressIpv6ToEvaluate_ {
        pub attribute: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressIpv6ToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressIpv6ToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressIpv6ToEvaluate as IngressIpv6ToEvaluate;
    impl crate::value::ToValue for IngressIpv6ToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressisinaddresslist.html
    pub struct IngressIsInAddressList_ {
        pub address_lists: Vec<crate::value::ExpString>,
        pub attribute: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressIsInAddressList {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressIsInAddressList"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressIsInAddressList as IngressIsInAddressList;
    impl crate::value::ToValue for IngressIsInAddressList_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AddressLists".to_string(),
                crate::value::ToValue::to_value(&self.address_lists),
            );
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressstringexpression.html
    pub struct IngressStringExpression_ {
        pub evaluate: Box<IngressStringToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub values: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressStringExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressStringExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressStringExpression as IngressStringExpression;
    impl crate::value::ToValue for IngressStringExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Values".to_string(),
                crate::value::ToValue::to_value(&self.values),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingressstringtoevaluate.html
    pub struct IngressStringToEvaluate_ {
        pub analysis: Option<Box<IngressAnalysis_>>,
        pub attribute: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressStringToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressStringToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressStringToEvaluate as IngressStringToEvaluate;
    impl crate::value::ToValue for IngressStringToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.analysis {
                properties.insert(
                    "Analysis".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attribute {
                properties.insert(
                    "Attribute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingresstlsprotocolexpression.html
    pub struct IngressTlsProtocolExpression_ {
        pub evaluate: Box<IngressTlsProtocolToEvaluate_>,
        pub operator: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressTlsProtocolExpression {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressTlsProtocolExpression"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressTlsProtocolExpression as IngressTlsProtocolExpression;
    impl crate::value::ToValue for IngressTlsProtocolExpression_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Evaluate".to_string(),
                crate::value::ToValue::to_value(&self.evaluate),
            );
            properties.insert(
                "Operator".to_string(),
                crate::value::ToValue::to_value(&self.operator),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-ingresstlsprotocoltoevaluate.html
    pub struct IngressTlsProtocolToEvaluate_ {
        pub attribute: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_IngressTlsProtocolToEvaluate {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.IngressTlsProtocolToEvaluate"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_IngressTlsProtocolToEvaluate as IngressTlsProtocolToEvaluate;
    impl crate::value::ToValue for IngressTlsProtocolToEvaluate_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Attribute".to_string(),
                crate::value::ToValue::to_value(&self.attribute),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-policycondition.html
    pub struct PolicyCondition_ {
        pub boolean_expression: Option<Box<IngressBooleanExpression_>>,
        pub ip_expression: Option<Box<IngressIpv4Expression_>>,
        pub ipv6_expression: Option<Box<IngressIpv6Expression_>>,
        pub string_expression: Option<Box<IngressStringExpression_>>,
        pub tls_expression: Option<Box<IngressTlsProtocolExpression_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_PolicyCondition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.PolicyCondition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_PolicyCondition as PolicyCondition;
    impl crate::value::ToValue for PolicyCondition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.boolean_expression {
                properties.insert(
                    "BooleanExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_expression {
                properties.insert(
                    "IpExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_expression {
                properties.insert(
                    "Ipv6Expression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.string_expression {
                properties.insert(
                    "StringExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tls_expression {
                properties.insert(
                    "TlsExpression".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-mailmanagertrafficpolicy-policystatement.html
    pub struct PolicyStatement_ {
        pub action: crate::value::ExpString,
        pub conditions: Vec<PolicyCondition_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_MailManagerTrafficPolicy_PolicyStatement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::MailManagerTrafficPolicy.PolicyStatement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_MailManagerTrafficPolicy_PolicyStatement as PolicyStatement;
    impl crate::value::ToValue for PolicyStatement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Action".to_string(),
                crate::value::ToValue::to_value(&self.action),
            );
            properties.insert(
                "Conditions".to_string(),
                crate::value::ToValue::to_value(&self.conditions),
            );
            properties.into()
        }
    }
}
pub mod receiptfilter {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-filter.html
    pub struct Filter_ {
        pub ip_filter: Box<IpFilter_>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptFilter_Filter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptFilter.Filter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptFilter_Filter as Filter;
    impl crate::value::ToValue for Filter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IpFilter".to_string(),
                crate::value::ToValue::to_value(&self.ip_filter),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptfilter-ipfilter.html
    pub struct IpFilter_ {
        pub cidr: crate::value::ExpString,
        pub policy: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptFilter_IpFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptFilter.IpFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptFilter_IpFilter as IpFilter;
    impl crate::value::ToValue for IpFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Cidr".to_string(),
                crate::value::ToValue::to_value(&self.cidr),
            );
            properties.insert(
                "Policy".to_string(),
                crate::value::ToValue::to_value(&self.policy),
            );
            properties.into()
        }
    }
}
pub mod receiptrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-action.html
    pub struct Action_ {
        pub add_header_action: Option<Box<AddHeaderAction_>>,
        pub bounce_action: Option<Box<BounceAction_>>,
        pub connect_action: Option<Box<ConnectAction_>>,
        pub lambda_action: Option<Box<LambdaAction_>>,
        pub s3_action: Option<Box<S3Action_>>,
        pub sns_action: Option<Box<SNSAction_>>,
        pub stop_action: Option<Box<StopAction_>>,
        pub workmail_action: Option<Box<WorkmailAction_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_Action as Action;
    impl crate::value::ToValue for Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.add_header_action {
                properties.insert(
                    "AddHeaderAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bounce_action {
                properties.insert(
                    "BounceAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.connect_action {
                properties.insert(
                    "ConnectAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lambda_action {
                properties.insert(
                    "LambdaAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_action {
                properties.insert(
                    "S3Action".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sns_action {
                properties.insert(
                    "SNSAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.stop_action {
                properties.insert(
                    "StopAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workmail_action {
                properties.insert(
                    "WorkmailAction".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-addheaderaction.html
    pub struct AddHeaderAction_ {
        pub header_name: crate::value::ExpString,
        pub header_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_AddHeaderAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.AddHeaderAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_AddHeaderAction as AddHeaderAction;
    impl crate::value::ToValue for AddHeaderAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "HeaderName".to_string(),
                crate::value::ToValue::to_value(&self.header_name),
            );
            properties.insert(
                "HeaderValue".to_string(),
                crate::value::ToValue::to_value(&self.header_value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-bounceaction.html
    pub struct BounceAction_ {
        pub message: crate::value::ExpString,
        pub sender: crate::value::ExpString,
        pub smtp_reply_code: crate::value::ExpString,
        pub status_code: Option<crate::value::ExpString>,
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_BounceAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.BounceAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_BounceAction as BounceAction;
    impl crate::value::ToValue for BounceAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Message".to_string(),
                crate::value::ToValue::to_value(&self.message),
            );
            properties.insert(
                "Sender".to_string(),
                crate::value::ToValue::to_value(&self.sender),
            );
            properties.insert(
                "SmtpReplyCode".to_string(),
                crate::value::ToValue::to_value(&self.smtp_reply_code),
            );
            if let Some(ref value) = self.status_code {
                properties.insert(
                    "StatusCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic_arn {
                properties.insert(
                    "TopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-connectaction.html
    pub struct ConnectAction_ {
        pub iam_role_arn: crate::value::ExpString,
        pub instance_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_ConnectAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.ConnectAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_ConnectAction as ConnectAction;
    impl crate::value::ToValue for ConnectAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "IAMRoleARN".to_string(),
                crate::value::ToValue::to_value(&self.iam_role_arn),
            );
            properties.insert(
                "InstanceARN".to_string(),
                crate::value::ToValue::to_value(&self.instance_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-lambdaaction.html
    pub struct LambdaAction_ {
        pub function_arn: crate::value::ExpString,
        pub invocation_type: Option<crate::value::ExpString>,
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_LambdaAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.LambdaAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_LambdaAction as LambdaAction;
    impl crate::value::ToValue for LambdaAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FunctionArn".to_string(),
                crate::value::ToValue::to_value(&self.function_arn),
            );
            if let Some(ref value) = self.invocation_type {
                properties.insert(
                    "InvocationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic_arn {
                properties.insert(
                    "TopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-rule.html
    pub struct Rule_ {
        pub actions: Option<Vec<Action_>>,
        pub enabled: Option<crate::value::ExpBool>,
        pub name: Option<crate::value::ExpString>,
        pub recipients: Option<Vec<crate::value::ExpString>>,
        pub scan_enabled: Option<crate::value::ExpBool>,
        pub tls_policy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_Rule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.Rule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_Rule as Rule;
    impl crate::value::ToValue for Rule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.actions {
                properties.insert(
                    "Actions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.recipients {
                properties.insert(
                    "Recipients".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.scan_enabled {
                properties.insert(
                    "ScanEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tls_policy {
                properties.insert(
                    "TlsPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-s3action.html
    pub struct S3Action_ {
        pub bucket_name: crate::value::ExpString,
        pub iam_role_arn: Option<crate::value::ExpString>,
        pub kms_key_arn: Option<crate::value::ExpString>,
        pub object_key_prefix: Option<crate::value::ExpString>,
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_S3Action {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.S3Action"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_S3Action as S3Action;
    impl crate::value::ToValue for S3Action_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "BucketName".to_string(),
                crate::value::ToValue::to_value(&self.bucket_name),
            );
            if let Some(ref value) = self.iam_role_arn {
                properties.insert(
                    "IamRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_arn {
                properties.insert(
                    "KmsKeyArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.object_key_prefix {
                properties.insert(
                    "ObjectKeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic_arn {
                properties.insert(
                    "TopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-snsaction.html
    pub struct SNSAction_ {
        pub encoding: Option<crate::value::ExpString>,
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_SNSAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.SNSAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_SNSAction as SNSAction;
    impl crate::value::ToValue for SNSAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encoding {
                properties.insert(
                    "Encoding".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.topic_arn {
                properties.insert(
                    "TopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-stopaction.html
    pub struct StopAction_ {
        pub scope: crate::value::ExpString,
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_StopAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.StopAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_StopAction as StopAction;
    impl crate::value::ToValue for StopAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Scope".to_string(),
                crate::value::ToValue::to_value(&self.scope),
            );
            if let Some(ref value) = self.topic_arn {
                properties.insert(
                    "TopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-receiptrule-workmailaction.html
    pub struct WorkmailAction_ {
        pub organization_arn: crate::value::ExpString,
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_ReceiptRule_WorkmailAction {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::ReceiptRule.WorkmailAction"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_ReceiptRule_WorkmailAction as WorkmailAction;
    impl crate::value::ToValue for WorkmailAction_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "OrganizationArn".to_string(),
                crate::value::ToValue::to_value(&self.organization_arn),
            );
            if let Some(ref value) = self.topic_arn {
                properties.insert(
                    "TopicArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod template {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-template-template.html
    pub struct Template_ {
        pub html_part: Option<crate::value::ExpString>,
        pub subject_part: crate::value::ExpString,
        pub template_name: Option<crate::value::ExpString>,
        pub text_part: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_Template_Template {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::Template.Template"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_Template_Template as Template;
    impl crate::value::ToValue for Template_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.html_part {
                properties.insert(
                    "HtmlPart".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "SubjectPart".to_string(),
                crate::value::ToValue::to_value(&self.subject_part),
            );
            if let Some(ref value) = self.template_name {
                properties.insert(
                    "TemplateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.text_part {
                properties.insert(
                    "TextPart".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod vdmattributes {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-vdmattributes-dashboardattributes.html
    pub struct DashboardAttributes_ {
        pub engagement_metrics: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_VdmAttributes_DashboardAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::VdmAttributes.DashboardAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_VdmAttributes_DashboardAttributes as DashboardAttributes;
    impl crate::value::ToValue for DashboardAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.engagement_metrics {
                properties.insert(
                    "EngagementMetrics".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-ses-vdmattributes-guardianattributes.html
    pub struct GuardianAttributes_ {
        pub optimized_shared_delivery: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_ses_VdmAttributes_GuardianAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::SES::VdmAttributes.GuardianAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_ses_VdmAttributes_GuardianAttributes as GuardianAttributes;
    impl crate::value::ToValue for GuardianAttributes_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.optimized_shared_delivery {
                properties.insert(
                    "OptimizedSharedDelivery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationset.html
pub struct ConfigurationSet_ {
    pub delivery_options: Option<super::ses::configurationset::DeliveryOptions_>,
    pub name: Option<crate::value::ExpString>,
    pub reputation_options: Option<super::ses::configurationset::ReputationOptions_>,
    pub sending_options: Option<super::ses::configurationset::SendingOptions_>,
    pub suppression_options: Option<super::ses::configurationset::SuppressionOptions_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tracking_options: Option<super::ses::configurationset::TrackingOptions_>,
    pub vdm_options: Option<super::ses::configurationset::VdmOptions_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_ConfigurationSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::ConfigurationSet"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_ConfigurationSet as ConfigurationSet;
impl crate::template::ToResource for ConfigurationSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ConfigurationSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.delivery_options {
            properties.insert(
                "DeliveryOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.reputation_options {
            properties.insert(
                "ReputationOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sending_options {
            properties.insert(
                "SendingOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.suppression_options {
            properties.insert(
                "SuppressionOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tracking_options {
            properties.insert(
                "TrackingOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vdm_options {
            properties.insert(
                "VdmOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-configurationseteventdestination.html
pub struct ConfigurationSetEventDestination_ {
    pub configuration_set_name: crate::value::ExpString,
    pub event_destination: super::ses::configurationseteventdestination::EventDestination_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_ConfigurationSetEventDestination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::ConfigurationSetEventDestination"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_ConfigurationSetEventDestination as ConfigurationSetEventDestination;
impl crate::template::ToResource for ConfigurationSetEventDestination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ConfigurationSetEventDestination",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConfigurationSetName".to_string(),
            crate::value::ToValue::to_value(&self.configuration_set_name),
        );
        properties.insert(
            "EventDestination".to_string(),
            crate::value::ToValue::to_value(&self.event_destination),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-contactlist.html
pub struct ContactList_ {
    pub contact_list_name: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub topics: Option<Vec<super::ses::contactlist::Topic_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_ContactList {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::ContactList" $($field
        $value)*)
    };
}
pub use crate::__aws_ses_ContactList as ContactList;
impl crate::template::ToResource for ContactList_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ContactList"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.contact_list_name {
            properties.insert(
                "ContactListName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.topics {
            properties.insert("Topics".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-dedicatedippool.html
pub struct DedicatedIpPool_ {
    pub pool_name: Option<crate::value::ExpString>,
    pub scaling_mode: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_DedicatedIpPool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::DedicatedIpPool"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_DedicatedIpPool as DedicatedIpPool;
impl crate::template::ToResource for DedicatedIpPool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DedicatedIpPool"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.pool_name {
            properties.insert(
                "PoolName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scaling_mode {
            properties.insert(
                "ScalingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-emailidentity.html
pub struct EmailIdentity_ {
    pub configuration_set_attributes:
        Option<super::ses::emailidentity::ConfigurationSetAttributes_>,
    pub dkim_attributes: Option<super::ses::emailidentity::DkimAttributes_>,
    pub dkim_signing_attributes: Option<super::ses::emailidentity::DkimSigningAttributes_>,
    pub email_identity: crate::value::ExpString,
    pub feedback_attributes: Option<super::ses::emailidentity::FeedbackAttributes_>,
    pub mail_from_attributes: Option<super::ses::emailidentity::MailFromAttributes_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_EmailIdentity {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::EmailIdentity"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_EmailIdentity as EmailIdentity;
impl crate::template::ToResource for EmailIdentity_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EmailIdentity"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.configuration_set_attributes {
            properties.insert(
                "ConfigurationSetAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dkim_attributes {
            properties.insert(
                "DkimAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.dkim_signing_attributes {
            properties.insert(
                "DkimSigningAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EmailIdentity".to_string(),
            crate::value::ToValue::to_value(&self.email_identity),
        );
        if let Some(ref value) = self.feedback_attributes {
            properties.insert(
                "FeedbackAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mail_from_attributes {
            properties.insert(
                "MailFromAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanageraddoninstance.html
pub struct MailManagerAddonInstance_ {
    pub addon_subscription_id: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerAddonInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerAddonInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerAddonInstance as MailManagerAddonInstance;
impl crate::template::ToResource for MailManagerAddonInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MailManagerAddonInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AddonSubscriptionId".to_string(),
            crate::value::ToValue::to_value(&self.addon_subscription_id),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanageraddonsubscription.html
pub struct MailManagerAddonSubscription_ {
    pub addon_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerAddonSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerAddonSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerAddonSubscription as MailManagerAddonSubscription;
impl crate::template::ToResource for MailManagerAddonSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "MailManagerAddonSubscription",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AddonName".to_string(),
            crate::value::ToValue::to_value(&self.addon_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanageraddresslist.html
pub struct MailManagerAddressList_ {
    pub address_list_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerAddressList {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerAddressList"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerAddressList as MailManagerAddressList;
impl crate::template::ToResource for MailManagerAddressList_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MailManagerAddressList"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.address_list_name {
            properties.insert(
                "AddressListName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanagerarchive.html
pub struct MailManagerArchive_ {
    pub archive_name: Option<crate::value::ExpString>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub retention: Option<super::ses::mailmanagerarchive::ArchiveRetention_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerArchive {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerArchive"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerArchive as MailManagerArchive;
impl crate::template::ToResource for MailManagerArchive_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MailManagerArchive"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.archive_name {
            properties.insert(
                "ArchiveName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention {
            properties.insert(
                "Retention".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanageringresspoint.html
pub struct MailManagerIngressPoint_ {
    pub ingress_point_configuration:
        Option<super::ses::mailmanageringresspoint::IngressPointConfiguration_>,
    pub ingress_point_name: Option<crate::value::ExpString>,
    pub network_configuration: Option<super::ses::mailmanageringresspoint::NetworkConfiguration_>,
    pub rule_set_id: crate::value::ExpString,
    pub status_to_update: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub traffic_policy_id: crate::value::ExpString,
    pub r#type: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerIngressPoint {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerIngressPoint"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerIngressPoint as MailManagerIngressPoint;
impl crate::template::ToResource for MailManagerIngressPoint_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MailManagerIngressPoint"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.ingress_point_configuration {
            properties.insert(
                "IngressPointConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ingress_point_name {
            properties.insert(
                "IngressPointName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.network_configuration {
            properties.insert(
                "NetworkConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RuleSetId".to_string(),
            crate::value::ToValue::to_value(&self.rule_set_id),
        );
        if let Some(ref value) = self.status_to_update {
            properties.insert(
                "StatusToUpdate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TrafficPolicyId".to_string(),
            crate::value::ToValue::to_value(&self.traffic_policy_id),
        );
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanagerrelay.html
pub struct MailManagerRelay_ {
    pub authentication: super::ses::mailmanagerrelay::RelayAuthentication_,
    pub relay_name: Option<crate::value::ExpString>,
    pub server_name: crate::value::ExpString,
    pub server_port: f64,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerRelay {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerRelay"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerRelay as MailManagerRelay;
impl crate::template::ToResource for MailManagerRelay_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MailManagerRelay"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Authentication".to_string(),
            crate::value::ToValue::to_value(&self.authentication),
        );
        if let Some(ref value) = self.relay_name {
            properties.insert(
                "RelayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ServerName".to_string(),
            crate::value::ToValue::to_value(&self.server_name),
        );
        properties.insert(
            "ServerPort".to_string(),
            crate::value::ToValue::to_value(&self.server_port),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanagerruleset.html
pub struct MailManagerRuleSet_ {
    pub rule_set_name: Option<crate::value::ExpString>,
    pub rules: Vec<super::ses::mailmanagerruleset::Rule_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerRuleSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerRuleSet"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerRuleSet as MailManagerRuleSet;
impl crate::template::ToResource for MailManagerRuleSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MailManagerRuleSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.rule_set_name {
            properties.insert(
                "RuleSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Rules".to_string(),
            crate::value::ToValue::to_value(&self.rules),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-mailmanagertrafficpolicy.html
pub struct MailManagerTrafficPolicy_ {
    pub default_action: crate::value::ExpString,
    pub max_message_size_bytes: Option<f64>,
    pub policy_statements: Vec<super::ses::mailmanagertrafficpolicy::PolicyStatement_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub traffic_policy_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_MailManagerTrafficPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::MailManagerTrafficPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_MailManagerTrafficPolicy as MailManagerTrafficPolicy;
impl crate::template::ToResource for MailManagerTrafficPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("MailManagerTrafficPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DefaultAction".to_string(),
            crate::value::ToValue::to_value(&self.default_action),
        );
        if let Some(ref value) = self.max_message_size_bytes {
            properties.insert(
                "MaxMessageSizeBytes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "PolicyStatements".to_string(),
            crate::value::ToValue::to_value(&self.policy_statements),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.traffic_policy_name {
            properties.insert(
                "TrafficPolicyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptfilter.html
pub struct ReceiptFilter_ {
    pub filter: super::ses::receiptfilter::Filter_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_ReceiptFilter {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::ReceiptFilter"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_ReceiptFilter as ReceiptFilter;
impl crate::template::ToResource for ReceiptFilter_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReceiptFilter"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Filter".to_string(),
            crate::value::ToValue::to_value(&self.filter),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptrule.html
pub struct ReceiptRule_ {
    pub after: Option<crate::value::ExpString>,
    pub rule: super::ses::receiptrule::Rule_,
    pub rule_set_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_ReceiptRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::ReceiptRule" $($field
        $value)*)
    };
}
pub use crate::__aws_ses_ReceiptRule as ReceiptRule;
impl crate::template::ToResource for ReceiptRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReceiptRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.after {
            properties.insert("After".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Rule".to_string(),
            crate::value::ToValue::to_value(&self.rule),
        );
        properties.insert(
            "RuleSetName".to_string(),
            crate::value::ToValue::to_value(&self.rule_set_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-receiptruleset.html
pub struct ReceiptRuleSet_ {
    pub rule_set_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_ReceiptRuleSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::ReceiptRuleSet"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_ReceiptRuleSet as ReceiptRuleSet;
impl crate::template::ToResource for ReceiptRuleSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ReceiptRuleSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.rule_set_name {
            properties.insert(
                "RuleSetName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-template.html
pub struct Template_ {
    pub template: Option<super::ses::template::Template_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_Template {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::Template" $($field
        $value)*)
    };
}
pub use crate::__aws_ses_Template as Template;
impl crate::template::ToResource for Template_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Template"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.template {
            properties.insert(
                "Template".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-ses-vdmattributes.html
pub struct VdmAttributes_ {
    pub dashboard_attributes: Option<super::ses::vdmattributes::DashboardAttributes_>,
    pub guardian_attributes: Option<super::ses::vdmattributes::GuardianAttributes_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_ses_VdmAttributes {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::SES::VdmAttributes"
        $($field $value)*)
    };
}
pub use crate::__aws_ses_VdmAttributes as VdmAttributes;
impl crate::template::ToResource for VdmAttributes_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SES"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VdmAttributes"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dashboard_attributes {
            properties.insert(
                "DashboardAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.guardian_attributes {
            properties.insert(
                "GuardianAttributes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
