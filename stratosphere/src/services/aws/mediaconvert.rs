pub mod jobtemplate {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconvert-jobtemplate-accelerationsettings.html>
    pub struct AccelerationSettings_ {
        pub mode: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconvert_JobTemplate_AccelerationSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConvert::JobTemplate.AccelerationSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconvert_JobTemplate_AccelerationSettings as AccelerationSettings;
    impl crate::value::ToValue for AccelerationSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Mode".to_string(),
                crate::value::ToValue::to_value(&self.mode),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-mediaconvert-jobtemplate-hopdestination.html>
    pub struct HopDestination_ {
        pub priority: Option<i32>,
        pub queue: Option<crate::value::ExpString>,
        pub wait_minutes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_mediaconvert_JobTemplate_HopDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::MediaConvert::JobTemplate.HopDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_mediaconvert_JobTemplate_HopDestination as HopDestination;
    impl crate::value::ToValue for HopDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.priority {
                properties.insert(
                    "Priority".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.queue {
                properties.insert("Queue".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.wait_minutes {
                properties.insert(
                    "WaitMinutes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconvert-jobtemplate.html>
pub struct JobTemplate_ {
    pub acceleration_settings: Option<super::mediaconvert::jobtemplate::AccelerationSettings_>,
    pub category: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub hop_destinations: Option<Vec<super::mediaconvert::jobtemplate::HopDestination_>>,
    pub name: Option<crate::value::ExpString>,
    pub priority: Option<i32>,
    pub queue: Option<crate::value::ExpString>,
    pub settings_json: serde_json::Value,
    pub status_update_interval: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconvert_JobTemplate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConvert::JobTemplate"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconvert_JobTemplate as JobTemplate;
impl crate::template::ToResource for JobTemplate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConvert"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("JobTemplate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.acceleration_settings {
            properties.insert(
                "AccelerationSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.category {
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hop_destinations {
            properties.insert(
                "HopDestinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.priority {
            properties.insert(
                "Priority".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.queue {
            properties.insert("Queue".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "SettingsJson".to_string(),
            crate::value::ToValue::to_value(&self.settings_json),
        );
        if let Some(ref value) = self.status_update_interval {
            properties.insert(
                "StatusUpdateInterval".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconvert-preset.html>
pub struct Preset_ {
    pub category: Option<crate::value::ExpString>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub settings_json: serde_json::Value,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconvert_Preset {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConvert::Preset"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconvert_Preset as Preset;
impl crate::template::ToResource for Preset_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConvert"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Preset"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.category {
            properties.insert(
                "Category".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "SettingsJson".to_string(),
            crate::value::ToValue::to_value(&self.settings_json),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-mediaconvert-queue.html>
pub struct Queue_ {
    pub concurrent_jobs: Option<i32>,
    pub description: Option<crate::value::ExpString>,
    pub name: Option<crate::value::ExpString>,
    pub pricing_plan: Option<crate::value::ExpString>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<serde_json::Value>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_mediaconvert_Queue {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::MediaConvert::Queue"
        $($field $value)*)
    };
}
pub use crate::__aws_mediaconvert_Queue as Queue;
impl crate::template::ToResource for Queue_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("MediaConvert"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Queue"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.concurrent_jobs {
            properties.insert(
                "ConcurrentJobs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.pricing_plan {
            properties.insert(
                "PricingPlan".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
