pub mod topic {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic-loggingconfig.html>
    pub struct LoggingConfig_ {
        pub failure_feedback_role_arn: Option<crate::value::ExpString>,
        pub protocol: crate::value::ExpString,
        pub success_feedback_role_arn: Option<crate::value::ExpString>,
        pub success_feedback_sample_rate: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sns_Topic_LoggingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SNS::Topic.LoggingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sns_Topic_LoggingConfig as LoggingConfig;
    impl crate::value::ToValue for LoggingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.failure_feedback_role_arn {
                properties.insert(
                    "FailureFeedbackRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            if let Some(ref value) = self.success_feedback_role_arn {
                properties.insert(
                    "SuccessFeedbackRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.success_feedback_sample_rate {
                properties.insert(
                    "SuccessFeedbackSampleRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-sns-topic-subscription.html>
    pub struct Subscription_ {
        pub endpoint: crate::value::ExpString,
        pub protocol: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_sns_Topic_Subscription {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::SNS::Topic.Subscription"
            $($field $value)*)
        };
    }
    pub use crate::__aws_sns_Topic_Subscription as Subscription;
    impl crate::value::ToValue for Subscription_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(&self.endpoint),
            );
            properties.insert(
                "Protocol".to_string(),
                crate::value::ToValue::to_value(&self.protocol),
            );
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-subscription.html>
pub struct Subscription_ {
    pub delivery_policy: Option<serde_json::Value>,
    pub endpoint: Option<crate::value::ExpString>,
    pub filter_policy: Option<serde_json::Value>,
    pub filter_policy_scope: Option<crate::value::ExpString>,
    pub protocol: crate::value::ExpString,
    pub raw_message_delivery: Option<crate::value::ExpBool>,
    pub redrive_policy: Option<serde_json::Value>,
    pub region: Option<crate::value::ExpString>,
    pub replay_policy: Option<serde_json::Value>,
    pub subscription_role_arn: Option<crate::value::ExpString>,
    pub topic_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sns_Subscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SNS::Subscription"
        $($field $value)*)
    };
}
pub use crate::__aws_sns_Subscription as Subscription;
impl crate::template::ToResource for Subscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SNS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Subscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.delivery_policy {
            properties.insert(
                "DeliveryPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint {
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_policy {
            properties.insert(
                "FilterPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.filter_policy_scope {
            properties.insert(
                "FilterPolicyScope".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Protocol".to_string(),
            crate::value::ToValue::to_value(&self.protocol),
        );
        if let Some(ref value) = self.raw_message_delivery {
            properties.insert(
                "RawMessageDelivery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redrive_policy {
            properties.insert(
                "RedrivePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.region {
            properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.replay_policy {
            properties.insert(
                "ReplayPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subscription_role_arn {
            properties.insert(
                "SubscriptionRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "TopicArn".to_string(),
            crate::value::ToValue::to_value(&self.topic_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-topic.html>
pub struct Topic_ {
    pub archive_policy: Option<serde_json::Value>,
    pub content_based_deduplication: Option<crate::value::ExpBool>,
    pub data_protection_policy: Option<serde_json::Value>,
    pub delivery_status_logging: Option<Vec<super::sns::topic::LoggingConfig_>>,
    pub display_name: Option<crate::value::ExpString>,
    pub fifo_throughput_scope: Option<crate::value::ExpString>,
    pub fifo_topic: Option<crate::value::ExpBool>,
    pub kms_master_key_id: Option<crate::value::ExpString>,
    pub signature_version: Option<crate::value::ExpString>,
    pub subscription: Option<Vec<super::sns::topic::Subscription_>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub topic_name: Option<crate::value::ExpString>,
    pub tracing_config: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sns_Topic {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SNS::Topic" $($field
        $value)*)
    };
}
pub use crate::__aws_sns_Topic as Topic;
impl crate::template::ToResource for Topic_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SNS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Topic"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.archive_policy {
            properties.insert(
                "ArchivePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.content_based_deduplication {
            properties.insert(
                "ContentBasedDeduplication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_protection_policy {
            properties.insert(
                "DataProtectionPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delivery_status_logging {
            properties.insert(
                "DeliveryStatusLogging".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fifo_throughput_scope {
            properties.insert(
                "FifoThroughputScope".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fifo_topic {
            properties.insert(
                "FifoTopic".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_master_key_id {
            properties.insert(
                "KmsMasterKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.signature_version {
            properties.insert(
                "SignatureVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subscription {
            properties.insert(
                "Subscription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.topic_name {
            properties.insert(
                "TopicName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tracing_config {
            properties.insert(
                "TracingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-topicinlinepolicy.html>
pub struct TopicInlinePolicy_ {
    pub policy_document: serde_json::Value,
    pub topic_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sns_TopicInlinePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SNS::TopicInlinePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_sns_TopicInlinePolicy as TopicInlinePolicy;
impl crate::template::ToResource for TopicInlinePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SNS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TopicInlinePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "TopicArn".to_string(),
            crate::value::ToValue::to_value(&self.topic_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sns-topicpolicy.html>
pub struct TopicPolicy_ {
    pub policy_document: serde_json::Value,
    pub topics: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sns_TopicPolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SNS::TopicPolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_sns_TopicPolicy as TopicPolicy;
impl crate::template::ToResource for TopicPolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SNS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TopicPolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "Topics".to_string(),
            crate::value::ToValue::to_value(&self.topics),
        );
        properties
    }
}
