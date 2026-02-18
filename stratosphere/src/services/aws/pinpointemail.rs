pub mod configurationset {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-deliveryoptions.html>
    pub struct DeliveryOptions_ {
        pub sending_pool_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSet_DeliveryOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSet.DeliveryOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSet_DeliveryOptions as DeliveryOptions;
    impl crate::value::ToValue for DeliveryOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.sending_pool_name {
                properties.insert(
                    "SendingPoolName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-reputationoptions.html>
    pub struct ReputationOptions_ {
        pub reputation_metrics_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSet_ReputationOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSet.ReputationOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSet_ReputationOptions as ReputationOptions;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-sendingoptions.html>
    pub struct SendingOptions_ {
        pub sending_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSet_SendingOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSet.SendingOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSet_SendingOptions as SendingOptions;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-tags.html>
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSet_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSet.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSet_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationset-trackingoptions.html>
    pub struct TrackingOptions_ {
        pub custom_redirect_domain: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSet_TrackingOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSet.TrackingOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSet_TrackingOptions as TrackingOptions;
    impl crate::value::ToValue for TrackingOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_redirect_domain {
                properties.insert(
                    "CustomRedirectDomain".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod configurationseteventdestination {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-cloudwatchdestination.html>
    pub struct CloudWatchDestination_ {
        pub dimension_configurations: Option<Vec<DimensionConfiguration_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSetEventDestination_CloudWatchDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSetEventDestination.CloudWatchDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSetEventDestination_CloudWatchDestination as CloudWatchDestination;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-dimensionconfiguration.html>
    pub struct DimensionConfiguration_ {
        pub default_dimension_value: crate::value::ExpString,
        pub dimension_name: crate::value::ExpString,
        pub dimension_value_source: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSetEventDestination_DimensionConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSetEventDestination.DimensionConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSetEventDestination_DimensionConfiguration as DimensionConfiguration;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-eventdestination.html>
    pub struct EventDestination_ {
        pub cloud_watch_destination: Option<Box<CloudWatchDestination_>>,
        pub enabled: Option<crate::value::ExpBool>,
        pub kinesis_firehose_destination: Option<Box<KinesisFirehoseDestination_>>,
        pub matching_event_types: Vec<crate::value::ExpString>,
        pub pinpoint_destination: Option<Box<PinpointDestination_>>,
        pub sns_destination: Option<Box<SnsDestination_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSetEventDestination_EventDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSetEventDestination.EventDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSetEventDestination_EventDestination as EventDestination;
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
            if let Some(ref value) = self.pinpoint_destination {
                properties.insert(
                    "PinpointDestination".to_string(),
                    crate::value::ToValue::to_value(value),
                );
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-kinesisfirehosedestination.html>
    pub struct KinesisFirehoseDestination_ {
        pub delivery_stream_arn: crate::value::ExpString,
        pub iam_role_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSetEventDestination_KinesisFirehoseDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSetEventDestination.KinesisFirehoseDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSetEventDestination_KinesisFirehoseDestination as KinesisFirehoseDestination;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-pinpointdestination.html>
    pub struct PinpointDestination_ {
        pub application_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSetEventDestination_PinpointDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSetEventDestination.PinpointDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSetEventDestination_PinpointDestination as PinpointDestination;
    impl crate::value::ToValue for PinpointDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.application_arn {
                properties.insert(
                    "ApplicationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-configurationseteventdestination-snsdestination.html>
    pub struct SnsDestination_ {
        pub topic_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_ConfigurationSetEventDestination_SnsDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::ConfigurationSetEventDestination.SnsDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_ConfigurationSetEventDestination_SnsDestination as SnsDestination;
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
pub mod dedicatedippool {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-dedicatedippool-tags.html>
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_DedicatedIpPool_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::DedicatedIpPool.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_DedicatedIpPool_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
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
}
pub mod identity {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-mailfromattributes.html>
    pub struct MailFromAttributes_ {
        pub behavior_on_mx_failure: Option<crate::value::ExpString>,
        pub mail_from_domain: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_Identity_MailFromAttributes {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::Identity.MailFromAttributes"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_Identity_MailFromAttributes as MailFromAttributes;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-pinpointemail-identity-tags.html>
    pub struct Tags_ {
        pub key: Option<crate::value::ExpString>,
        pub value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_pinpointemail_Identity_Tags {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::PinpointEmail::Identity.Tags"
            $($field $value)*)
        };
    }
    pub use crate::__aws_pinpointemail_Identity_Tags as Tags;
    impl crate::value::ToValue for Tags_ {
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
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationset.html>
pub struct ConfigurationSet_ {
    pub delivery_options: Option<super::pinpointemail::configurationset::DeliveryOptions_>,
    pub name: crate::value::ExpString,
    pub reputation_options: Option<super::pinpointemail::configurationset::ReputationOptions_>,
    pub sending_options: Option<super::pinpointemail::configurationset::SendingOptions_>,
    pub tags: Option<Vec<super::pinpointemail::configurationset::Tags_>>,
    pub tracking_options: Option<super::pinpointemail::configurationset::TrackingOptions_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpointemail_ConfigurationSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::PinpointEmail::ConfigurationSet"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpointemail_ConfigurationSet as ConfigurationSet;
impl crate::template::ToResource for ConfigurationSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PinpointEmail"),
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
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tracking_options {
            properties.insert(
                "TrackingOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-configurationseteventdestination.html>
pub struct ConfigurationSetEventDestination_ {
    pub configuration_set_name: crate::value::ExpString,
    pub event_destination:
        Option<super::pinpointemail::configurationseteventdestination::EventDestination_>,
    pub event_destination_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpointemail_ConfigurationSetEventDestination {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::PinpointEmail::ConfigurationSetEventDestination"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpointemail_ConfigurationSetEventDestination as ConfigurationSetEventDestination;
impl crate::template::ToResource for ConfigurationSetEventDestination_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PinpointEmail"),
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
        if let Some(ref value) = self.event_destination {
            properties.insert(
                "EventDestination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventDestinationName".to_string(),
            crate::value::ToValue::to_value(&self.event_destination_name),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-dedicatedippool.html>
pub struct DedicatedIpPool_ {
    pub pool_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<super::pinpointemail::dedicatedippool::Tags_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpointemail_DedicatedIpPool {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::PinpointEmail::DedicatedIpPool"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpointemail_DedicatedIpPool as DedicatedIpPool;
impl crate::template::ToResource for DedicatedIpPool_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PinpointEmail"),
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
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-pinpointemail-identity.html>
pub struct Identity_ {
    pub dkim_signing_enabled: Option<crate::value::ExpBool>,
    pub feedback_forwarding_enabled: Option<crate::value::ExpBool>,
    pub mail_from_attributes: Option<super::pinpointemail::identity::MailFromAttributes_>,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<super::pinpointemail::identity::Tags_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_pinpointemail_Identity {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::PinpointEmail::Identity"
        $($field $value)*)
    };
}
pub use crate::__aws_pinpointemail_Identity as Identity;
impl crate::template::ToResource for Identity_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("PinpointEmail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Identity"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.dkim_signing_enabled {
            properties.insert(
                "DkimSigningEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.feedback_forwarding_enabled {
            properties.insert(
                "FeedbackForwardingEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.mail_from_attributes {
            properties.insert(
                "MailFromAttributes".to_string(),
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
