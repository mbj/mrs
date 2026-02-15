///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sqs-queue.html
pub struct Queue_ {
    pub content_based_deduplication: Option<crate::value::ExpBool>,
    pub deduplication_scope: Option<crate::value::ExpString>,
    pub delay_seconds: Option<i32>,
    pub fifo_queue: Option<crate::value::ExpBool>,
    pub fifo_throughput_limit: Option<crate::value::ExpString>,
    pub kms_data_key_reuse_period_seconds: Option<i32>,
    pub kms_master_key_id: Option<crate::value::ExpString>,
    pub maximum_message_size: Option<i32>,
    pub message_retention_period: Option<i32>,
    pub queue_name: Option<crate::value::ExpString>,
    pub receive_message_wait_time_seconds: Option<i32>,
    pub redrive_allow_policy: Option<serde_json::Value>,
    pub redrive_policy: Option<serde_json::Value>,
    pub sqs_managed_sse_enabled: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub visibility_timeout: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sqs_Queue {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SQS::Queue" $($field
        $value)*)
    };
}
pub use crate::__aws_sqs_Queue as Queue;
impl crate::template::ToResource for Queue_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SQS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Queue"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.content_based_deduplication {
            properties.insert(
                "ContentBasedDeduplication".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deduplication_scope {
            properties.insert(
                "DeduplicationScope".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delay_seconds {
            properties.insert(
                "DelaySeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fifo_queue {
            properties.insert(
                "FifoQueue".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.fifo_throughput_limit {
            properties.insert(
                "FifoThroughputLimit".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_data_key_reuse_period_seconds {
            properties.insert(
                "KmsDataKeyReusePeriodSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_master_key_id {
            properties.insert(
                "KmsMasterKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maximum_message_size {
            properties.insert(
                "MaximumMessageSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.message_retention_period {
            properties.insert(
                "MessageRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.queue_name {
            properties.insert(
                "QueueName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.receive_message_wait_time_seconds {
            properties.insert(
                "ReceiveMessageWaitTimeSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redrive_allow_policy {
            properties.insert(
                "RedriveAllowPolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redrive_policy {
            properties.insert(
                "RedrivePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sqs_managed_sse_enabled {
            properties.insert(
                "SqsManagedSseEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.visibility_timeout {
            properties.insert(
                "VisibilityTimeout".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sqs-queueinlinepolicy.html
pub struct QueueInlinePolicy_ {
    pub policy_document: serde_json::Value,
    pub queue: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sqs_QueueInlinePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SQS::QueueInlinePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_sqs_QueueInlinePolicy as QueueInlinePolicy;
impl crate::template::ToResource for QueueInlinePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SQS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QueueInlinePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "Queue".to_string(),
            crate::value::ToValue::to_value(&self.queue),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-sqs-queuepolicy.html
pub struct QueuePolicy_ {
    pub policy_document: serde_json::Value,
    pub queues: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_sqs_QueuePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::SQS::QueuePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_sqs_QueuePolicy as QueuePolicy;
impl crate::template::ToResource for QueuePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("SQS"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("QueuePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "PolicyDocument".to_string(),
            crate::value::ToValue::to_value(&self.policy_document),
        );
        properties.insert(
            "Queues".to_string(),
            crate::value::ToValue::to_value(&self.queues),
        );
        properties
    }
}
