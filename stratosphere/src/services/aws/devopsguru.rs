pub mod notificationchannel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationchannelconfig.html
    pub struct NotificationChannelConfig_ {
        pub filters: Option<Box<NotificationFilterConfig_>>,
        pub sns: Option<Box<SnsChannelConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsguru_NotificationChannel_NotificationChannelConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsGuru::NotificationChannel.NotificationChannelConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsguru_NotificationChannel_NotificationChannelConfig as NotificationChannelConfig;
    impl crate::value::ToValue for NotificationChannelConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.filters {
                properties.insert(
                    "Filters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.sns {
                properties.insert("Sns".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-notificationfilterconfig.html
    pub struct NotificationFilterConfig_ {
        pub message_types: Option<Vec<crate::value::ExpString>>,
        pub severities: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsguru_NotificationChannel_NotificationFilterConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsGuru::NotificationChannel.NotificationFilterConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsguru_NotificationChannel_NotificationFilterConfig as NotificationFilterConfig;
    impl crate::value::ToValue for NotificationFilterConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.message_types {
                properties.insert(
                    "MessageTypes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.severities {
                properties.insert(
                    "Severities".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-notificationchannel-snschannelconfig.html
    pub struct SnsChannelConfig_ {
        pub topic_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsguru_NotificationChannel_SnsChannelConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsGuru::NotificationChannel.SnsChannelConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsguru_NotificationChannel_SnsChannelConfig as SnsChannelConfig;
    impl crate::value::ToValue for SnsChannelConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
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
pub mod resourcecollection {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-cloudformationcollectionfilter.html
    pub struct CloudFormationCollectionFilter_ {
        pub stack_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsguru_ResourceCollection_CloudFormationCollectionFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsGuru::ResourceCollection.CloudFormationCollectionFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsguru_ResourceCollection_CloudFormationCollectionFilter as CloudFormationCollectionFilter;
    impl crate::value::ToValue for CloudFormationCollectionFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.stack_names {
                properties.insert(
                    "StackNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-resourcecollectionfilter.html
    pub struct ResourceCollectionFilter_ {
        pub cloud_formation: Option<Box<CloudFormationCollectionFilter_>>,
        pub tags: Option<Vec<TagCollection_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsguru_ResourceCollection_ResourceCollectionFilter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsGuru::ResourceCollection.ResourceCollectionFilter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsguru_ResourceCollection_ResourceCollectionFilter as ResourceCollectionFilter;
    impl crate::value::ToValue for ResourceCollectionFilter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cloud_formation {
                properties.insert(
                    "CloudFormation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-devopsguru-resourcecollection-tagcollection.html
    pub struct TagCollection_ {
        pub app_boundary_key: Option<crate::value::ExpString>,
        pub tag_values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_devopsguru_ResourceCollection_TagCollection {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::DevOpsGuru::ResourceCollection.TagCollection"
            $($field $value)*)
        };
    }
    pub use crate::__aws_devopsguru_ResourceCollection_TagCollection as TagCollection;
    impl crate::value::ToValue for TagCollection_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.app_boundary_key {
                properties.insert(
                    "AppBoundaryKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_values {
                properties.insert(
                    "TagValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-loganomalydetectionintegration.html
pub struct LogAnomalyDetectionIntegration_ {}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_devopsguru_LogAnomalyDetectionIntegration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DevOpsGuru::LogAnomalyDetectionIntegration"
        $($field $value)*)
    };
}
pub use crate::__aws_devopsguru_LogAnomalyDetectionIntegration as LogAnomalyDetectionIntegration;
impl crate::template::ToResource for LogAnomalyDetectionIntegration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DevOpsGuru"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LogAnomalyDetectionIntegration",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        crate::template::ResourceProperties::new()
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-notificationchannel.html
pub struct NotificationChannel_ {
    pub config: super::devopsguru::notificationchannel::NotificationChannelConfig_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_devopsguru_NotificationChannel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DevOpsGuru::NotificationChannel"
        $($field $value)*)
    };
}
pub use crate::__aws_devopsguru_NotificationChannel as NotificationChannel;
impl crate::template::ToResource for NotificationChannel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DevOpsGuru"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NotificationChannel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Config".to_string(),
            crate::value::ToValue::to_value(&self.config),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-devopsguru-resourcecollection.html
pub struct ResourceCollection_ {
    pub resource_collection_filter:
        super::devopsguru::resourcecollection::ResourceCollectionFilter_,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_devopsguru_ResourceCollection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::DevOpsGuru::ResourceCollection"
        $($field $value)*)
    };
}
pub use crate::__aws_devopsguru_ResourceCollection as ResourceCollection;
impl crate::template::ToResource for ResourceCollection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("DevOpsGuru"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourceCollection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourceCollectionFilter".to_string(),
            crate::value::ToValue::to_value(&self.resource_collection_filter),
        );
        properties
    }
}
