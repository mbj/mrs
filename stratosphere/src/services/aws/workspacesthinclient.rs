pub mod environment {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesthinclient-environment-maintenancewindow.html
    pub struct MaintenanceWindow_ {
        pub apply_time_of: Option<crate::value::ExpString>,
        pub days_of_the_week: Option<Vec<crate::value::ExpString>>,
        pub end_time_hour: Option<i32>,
        pub end_time_minute: Option<i32>,
        pub start_time_hour: Option<i32>,
        pub start_time_minute: Option<i32>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesthinclient_Environment_MaintenanceWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkSpacesThinClient::Environment.MaintenanceWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesthinclient_Environment_MaintenanceWindow as MaintenanceWindow;
    impl crate::value::ToValue for MaintenanceWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.apply_time_of {
                properties.insert(
                    "ApplyTimeOf".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.days_of_the_week {
                properties.insert(
                    "DaysOfTheWeek".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_time_hour {
                properties.insert(
                    "EndTimeHour".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.end_time_minute {
                properties.insert(
                    "EndTimeMinute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time_hour {
                properties.insert(
                    "StartTimeHour".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.start_time_minute {
                properties.insert(
                    "StartTimeMinute".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesthinclient-environment.html
pub struct Environment_ {
    pub desired_software_set_id: Option<crate::value::ExpString>,
    pub desktop_arn: crate::value::ExpString,
    pub desktop_endpoint: Option<crate::value::ExpString>,
    pub device_creation_tags: Option<Vec<crate::Tag_>>,
    pub kms_key_arn: Option<crate::value::ExpString>,
    pub maintenance_window: Option<super::workspacesthinclient::environment::MaintenanceWindow_>,
    pub name: Option<crate::value::ExpString>,
    pub software_set_update_mode: Option<crate::value::ExpString>,
    pub software_set_update_schedule: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesthinclient_Environment {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkSpacesThinClient::Environment"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesthinclient_Environment as Environment;
impl crate::template::ToResource for Environment_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkSpacesThinClient"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Environment"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.desired_software_set_id {
            properties.insert(
                "DesiredSoftwareSetId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DesktopArn".to_string(),
            crate::value::ToValue::to_value(&self.desktop_arn),
        );
        if let Some(ref value) = self.desktop_endpoint {
            properties.insert(
                "DesktopEndpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.device_creation_tags {
            properties.insert(
                "DeviceCreationTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_arn {
            properties.insert(
                "KmsKeyArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maintenance_window {
            properties.insert(
                "MaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.software_set_update_mode {
            properties.insert(
                "SoftwareSetUpdateMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.software_set_update_schedule {
            properties.insert(
                "SoftwareSetUpdateSchedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
