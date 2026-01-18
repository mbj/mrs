pub mod notificationrule {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-codestarnotifications-notificationrule-target.html
    pub struct Target_ {
        pub target_address: crate::value::ExpString,
        pub target_type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_codestarnotifications_NotificationRule_Target {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CodeStarNotifications::NotificationRule.Target"
            $($field $value)*)
        };
    }
    pub use crate::__aws_codestarnotifications_NotificationRule_Target as Target;
    impl crate::value::ToValue for Target_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "TargetAddress".to_string(),
                crate::value::ToValue::to_value(&self.target_address),
            );
            properties.insert(
                "TargetType".to_string(),
                crate::value::ToValue::to_value(&self.target_type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-codestarnotifications-notificationrule.html
pub struct NotificationRule_ {
    pub created_by: Option<crate::value::ExpString>,
    pub detail_type: crate::value::ExpString,
    pub event_type_id: Option<crate::value::ExpString>,
    pub event_type_ids: Vec<crate::value::ExpString>,
    pub name: crate::value::ExpString,
    pub resource: crate::value::ExpString,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub target_address: Option<crate::value::ExpString>,
    pub targets: Vec<super::codestarnotifications::notificationrule::Target_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_codestarnotifications_NotificationRule {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CodeStarNotifications::NotificationRule"
        $($field $value)*)
    };
}
pub use crate::__aws_codestarnotifications_NotificationRule as NotificationRule;
impl crate::template::ToResource for NotificationRule_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CodeStarNotifications"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("NotificationRule"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.created_by {
            properties.insert(
                "CreatedBy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DetailType".to_string(),
            crate::value::ToValue::to_value(&self.detail_type),
        );
        if let Some(ref value) = self.event_type_id {
            properties.insert(
                "EventTypeId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EventTypeIds".to_string(),
            crate::value::ToValue::to_value(&self.event_type_ids),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Resource".to_string(),
            crate::value::ToValue::to_value(&self.resource),
        );
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.target_address {
            properties.insert(
                "TargetAddress".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Targets".to_string(),
            crate::value::ToValue::to_value(&self.targets),
        );
        properties
    }
}
