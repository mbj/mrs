pub mod monitor {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-healtheventsconfig.html
    pub struct HealthEventsConfig_ {
        pub availability_local_health_events_config: Option<Box<LocalHealthEventsConfig_>>,
        pub availability_score_threshold: Option<f64>,
        pub performance_local_health_events_config: Option<Box<LocalHealthEventsConfig_>>,
        pub performance_score_threshold: Option<f64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_internetmonitor_Monitor_HealthEventsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::InternetMonitor::Monitor.HealthEventsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_internetmonitor_Monitor_HealthEventsConfig as HealthEventsConfig;
    impl crate::value::ToValue for HealthEventsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_local_health_events_config {
                properties.insert(
                    "AvailabilityLocalHealthEventsConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.availability_score_threshold {
                properties.insert(
                    "AvailabilityScoreThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.performance_local_health_events_config {
                properties.insert(
                    "PerformanceLocalHealthEventsConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.performance_score_threshold {
                properties.insert(
                    "PerformanceScoreThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-internetmeasurementslogdelivery.html
    pub struct InternetMeasurementsLogDelivery_ {
        pub s3_config: Option<Box<S3Config_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_internetmonitor_Monitor_InternetMeasurementsLogDelivery {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::InternetMonitor::Monitor.InternetMeasurementsLogDelivery"
            $($field $value)*)
        };
    }
    pub use crate::__aws_internetmonitor_Monitor_InternetMeasurementsLogDelivery as InternetMeasurementsLogDelivery;
    impl crate::value::ToValue for InternetMeasurementsLogDelivery_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_config {
                properties.insert(
                    "S3Config".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-localhealtheventsconfig.html
    pub struct LocalHealthEventsConfig_ {
        pub health_score_threshold: Option<f64>,
        pub min_traffic_impact: Option<f64>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_internetmonitor_Monitor_LocalHealthEventsConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::InternetMonitor::Monitor.LocalHealthEventsConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_internetmonitor_Monitor_LocalHealthEventsConfig as LocalHealthEventsConfig;
    impl crate::value::ToValue for LocalHealthEventsConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.health_score_threshold {
                properties.insert(
                    "HealthScoreThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.min_traffic_impact {
                properties.insert(
                    "MinTrafficImpact".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-internetmonitor-monitor-s3config.html
    pub struct S3Config_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub bucket_prefix: Option<crate::value::ExpString>,
        pub log_delivery_status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_internetmonitor_Monitor_S3Config {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere_generator::construct_property_type!("AWS::InternetMonitor::Monitor.S3Config"
            $($field $value)*)
        };
    }
    pub use crate::__aws_internetmonitor_Monitor_S3Config as S3Config;
    impl crate::value::ToValue for S3Config_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bucket_prefix {
                properties.insert(
                    "BucketPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_delivery_status {
                properties.insert(
                    "LogDeliveryStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-internetmonitor-monitor.html
pub struct Monitor_ {
    pub health_events_config: Option<super::internetmonitor::monitor::HealthEventsConfig_>,
    pub include_linked_accounts: Option<crate::value::ExpBool>,
    pub internet_measurements_log_delivery:
        Option<super::internetmonitor::monitor::InternetMeasurementsLogDelivery_>,
    pub linked_account_id: Option<crate::value::ExpString>,
    pub max_city_networks_to_monitor: Option<i64>,
    pub monitor_name: crate::value::ExpString,
    pub resources: Option<Vec<crate::value::ExpString>>,
    pub resources_to_add: Option<Vec<crate::value::ExpString>>,
    pub resources_to_remove: Option<Vec<crate::value::ExpString>>,
    pub status: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub traffic_percentage_to_monitor: Option<i64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_internetmonitor_Monitor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere_generator::construct_resource_type!("AWS::InternetMonitor::Monitor"
        $($field $value)*)
    };
}
pub use crate::__aws_internetmonitor_Monitor as Monitor;
impl crate::template::ToResource for Monitor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("InternetMonitor"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Monitor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.health_events_config {
            properties.insert(
                "HealthEventsConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.include_linked_accounts {
            properties.insert(
                "IncludeLinkedAccounts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.internet_measurements_log_delivery {
            properties.insert(
                "InternetMeasurementsLogDelivery".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.linked_account_id {
            properties.insert(
                "LinkedAccountId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_city_networks_to_monitor {
            properties.insert(
                "MaxCityNetworksToMonitor".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MonitorName".to_string(),
            crate::value::ToValue::to_value(&self.monitor_name),
        );
        if let Some(ref value) = self.resources {
            properties.insert(
                "Resources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resources_to_add {
            properties.insert(
                "ResourcesToAdd".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resources_to_remove {
            properties.insert(
                "ResourcesToRemove".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.status {
            properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.traffic_percentage_to_monitor {
            properties.insert(
                "TrafficPercentageToMonitor".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
