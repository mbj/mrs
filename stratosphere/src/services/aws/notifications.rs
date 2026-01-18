pub mod eventrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-notifications-eventrule-eventrulestatussummary.html
    pub struct EventRuleStatusSummary_ {
        pub reason: crate::value::ExpString,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_notifications_EventRule_EventRuleStatusSummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Notifications::EventRule.EventRuleStatusSummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_notifications_EventRule_EventRuleStatusSummary as EventRuleStatusSummary;
    impl crate::value::ToValue for EventRuleStatusSummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Reason".to_string(),
                crate::value::ToValue::to_value(&self.reason),
            );
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
}
pub mod notificationhub {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-notifications-notificationhub-notificationhubstatussummary.html
    pub struct NotificationHubStatusSummary_ {
        pub notification_hub_status: crate::value::ExpString,
        pub notification_hub_status_reason: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_notifications_NotificationHub_NotificationHubStatusSummary {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Notifications::NotificationHub.NotificationHubStatusSummary"
            $($field $value)*)
        };
    }
    pub use crate::__aws_notifications_NotificationHub_NotificationHubStatusSummary as NotificationHubStatusSummary;
    impl crate::value::ToValue for NotificationHubStatusSummary_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "NotificationHubStatus".to_string(),
                crate::value::ToValue::to_value(&self.notification_hub_status),
            );
            properties.insert(
                "NotificationHubStatusReason".to_string(),
                crate::value::ToValue::to_value(&self.notification_hub_status_reason),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notifications-channelassociation.html
pub struct ChannelAssociation_ {
    pub arn: crate::value::ExpString,
    pub notification_configuration_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notifications_ChannelAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Notifications::ChannelAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_notifications_ChannelAssociation as ChannelAssociation;
impl crate::template::ToResource for ChannelAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Notifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ChannelAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Arn".to_string(),
            crate::value::ToValue::to_value(&self.arn),
        );
        properties.insert(
            "NotificationConfigurationArn".to_string(),
            crate::value::ToValue::to_value(&self.notification_configuration_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notifications-eventrule.html
pub struct EventRule_ {
    pub event_pattern: Option<crate::value::ExpString>,
    pub event_type: crate::value::ExpString,
    pub notification_configuration_arn: crate::value::ExpString,
    pub regions: Vec<crate::value::ExpString>,
    pub source: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notifications_EventRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Notifications::EventRule"
        $($field $value)*)
    };
}
pub use crate::__aws_notifications_EventRule as EventRule;
impl crate::template::ToResource for EventRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Notifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.event_pattern {
            properties.insert(
                "EventPattern".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventType".to_string(),
            crate::value::ToValue::to_value(&self.event_type),
        );
        properties.insert(
            "NotificationConfigurationArn".to_string(),
            crate::value::ToValue::to_value(&self.notification_configuration_arn),
        );
        properties.insert(
            "Regions".to_string(),
            crate::value::ToValue::to_value(&self.regions),
        );
        properties.insert(
            "Source".to_string(),
            crate::value::ToValue::to_value(&self.source),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notifications-managednotificationaccountcontactassociation.html
pub struct ManagedNotificationAccountContactAssociation_ {
    pub contact_identifier: crate::value::ExpString,
    pub managed_notification_configuration_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notifications_ManagedNotificationAccountContactAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Notifications::ManagedNotificationAccountContactAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_notifications_ManagedNotificationAccountContactAssociation as ManagedNotificationAccountContactAssociation;
impl crate::template::ToResource for ManagedNotificationAccountContactAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Notifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ManagedNotificationAccountContactAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ContactIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.contact_identifier),
        );
        properties.insert(
            "ManagedNotificationConfigurationArn".to_string(),
            crate::value::ToValue::to_value(&self.managed_notification_configuration_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notifications-managednotificationadditionalchannelassociation.html
pub struct ManagedNotificationAdditionalChannelAssociation_ {
    pub channel_arn: crate::value::ExpString,
    pub managed_notification_configuration_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notifications_ManagedNotificationAdditionalChannelAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Notifications::ManagedNotificationAdditionalChannelAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_notifications_ManagedNotificationAdditionalChannelAssociation as ManagedNotificationAdditionalChannelAssociation;
impl crate::template::ToResource for ManagedNotificationAdditionalChannelAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Notifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ManagedNotificationAdditionalChannelAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ChannelArn".to_string(),
            crate::value::ToValue::to_value(&self.channel_arn),
        );
        properties.insert(
            "ManagedNotificationConfigurationArn".to_string(),
            crate::value::ToValue::to_value(&self.managed_notification_configuration_arn),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notifications-notificationconfiguration.html
pub struct NotificationConfiguration_ {
    pub aggregation_duration: Option<crate::value::ExpString>,
    pub description: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notifications_NotificationConfiguration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Notifications::NotificationConfiguration"
        $($field $value)*)
    };
}
pub use crate::__aws_notifications_NotificationConfiguration as NotificationConfiguration;
impl crate::template::ToResource for NotificationConfiguration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Notifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NotificationConfiguration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.aggregation_duration {
            properties.insert(
                "AggregationDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
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
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notifications-notificationhub.html
pub struct NotificationHub_ {
    pub region: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notifications_NotificationHub {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Notifications::NotificationHub"
        $($field $value)*)
    };
}
pub use crate::__aws_notifications_NotificationHub as NotificationHub;
impl crate::template::ToResource for NotificationHub_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Notifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NotificationHub"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Region".to_string(),
            crate::value::ToValue::to_value(&self.region),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-notifications-organizationalunitassociation.html
pub struct OrganizationalUnitAssociation_ {
    pub notification_configuration_arn: crate::value::ExpString,
    pub organizational_unit_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_notifications_OrganizationalUnitAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Notifications::OrganizationalUnitAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_notifications_OrganizationalUnitAssociation as OrganizationalUnitAssociation;
impl crate::template::ToResource for OrganizationalUnitAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Notifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "OrganizationalUnitAssociation",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "NotificationConfigurationArn".to_string(),
            crate::value::ToValue::to_value(&self.notification_configuration_arn),
        );
        properties.insert(
            "OrganizationalUnitId".to_string(),
            crate::value::ToValue::to_value(&self.organizational_unit_id),
        );
        properties
    }
}
