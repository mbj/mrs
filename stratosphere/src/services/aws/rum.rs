pub mod appmonitor {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-appmonitorconfiguration.html>
    pub struct AppMonitorConfiguration_ {
        pub allow_cookies: Option<crate::value::ExpBool>,
        pub enable_x_ray: Option<crate::value::ExpBool>,
        pub excluded_pages: Option<Vec<crate::value::ExpString>>,
        pub favorite_pages: Option<Vec<crate::value::ExpString>>,
        pub guest_role_arn: Option<crate::value::ExpString>,
        pub identity_pool_id: Option<crate::value::ExpString>,
        pub included_pages: Option<Vec<crate::value::ExpString>>,
        pub metric_destinations: Option<Vec<MetricDestination_>>,
        pub session_sample_rate: Option<f64>,
        pub telemetries: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rum_AppMonitor_AppMonitorConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RUM::AppMonitor.AppMonitorConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rum_AppMonitor_AppMonitorConfiguration as AppMonitorConfiguration;
    impl crate::value::ToValue for AppMonitorConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_cookies {
                properties.insert(
                    "AllowCookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_x_ray {
                properties.insert(
                    "EnableXRay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.excluded_pages {
                properties.insert(
                    "ExcludedPages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.favorite_pages {
                properties.insert(
                    "FavoritePages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.guest_role_arn {
                properties.insert(
                    "GuestRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.identity_pool_id {
                properties.insert(
                    "IdentityPoolId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.included_pages {
                properties.insert(
                    "IncludedPages".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_destinations {
                properties.insert(
                    "MetricDestinations".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.session_sample_rate {
                properties.insert(
                    "SessionSampleRate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.telemetries {
                properties.insert(
                    "Telemetries".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-customevents.html>
    pub struct CustomEvents_ {
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rum_AppMonitor_CustomEvents {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RUM::AppMonitor.CustomEvents"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rum_AppMonitor_CustomEvents as CustomEvents;
    impl crate::value::ToValue for CustomEvents_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-deobfuscationconfiguration.html>
    pub struct DeobfuscationConfiguration_ {
        pub java_script_source_maps: Option<Box<JavaScriptSourceMaps_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rum_AppMonitor_DeobfuscationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RUM::AppMonitor.DeobfuscationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rum_AppMonitor_DeobfuscationConfiguration as DeobfuscationConfiguration;
    impl crate::value::ToValue for DeobfuscationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.java_script_source_maps {
                properties.insert(
                    "JavaScriptSourceMaps".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-javascriptsourcemaps.html>
    pub struct JavaScriptSourceMaps_ {
        pub s3_uri: Option<crate::value::ExpString>,
        pub status: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rum_AppMonitor_JavaScriptSourceMaps {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RUM::AppMonitor.JavaScriptSourceMaps"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rum_AppMonitor_JavaScriptSourceMaps as JavaScriptSourceMaps;
    impl crate::value::ToValue for JavaScriptSourceMaps_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.s3_uri {
                properties.insert("S3Uri".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Status".to_string(),
                crate::value::ToValue::to_value(&self.status),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-metricdefinition.html>
    pub struct MetricDefinition_ {
        pub dimension_keys: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
        pub event_pattern: Option<crate::value::ExpString>,
        pub name: crate::value::ExpString,
        pub namespace: Option<crate::value::ExpString>,
        pub unit_label: Option<crate::value::ExpString>,
        pub value_key: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rum_AppMonitor_MetricDefinition {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RUM::AppMonitor.MetricDefinition"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rum_AppMonitor_MetricDefinition as MetricDefinition;
    impl crate::value::ToValue for MetricDefinition_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.dimension_keys {
                properties.insert(
                    "DimensionKeys".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.event_pattern {
                properties.insert(
                    "EventPattern".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.namespace {
                properties.insert(
                    "Namespace".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unit_label {
                properties.insert(
                    "UnitLabel".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.value_key {
                properties.insert(
                    "ValueKey".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-metricdestination.html>
    pub struct MetricDestination_ {
        pub destination: crate::value::ExpString,
        pub destination_arn: Option<crate::value::ExpString>,
        pub iam_role_arn: Option<crate::value::ExpString>,
        pub metric_definitions: Option<Vec<MetricDefinition_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rum_AppMonitor_MetricDestination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RUM::AppMonitor.MetricDestination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rum_AppMonitor_MetricDestination as MetricDestination;
    impl crate::value::ToValue for MetricDestination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Destination".to_string(),
                crate::value::ToValue::to_value(&self.destination),
            );
            if let Some(ref value) = self.destination_arn {
                properties.insert(
                    "DestinationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_role_arn {
                properties.insert(
                    "IamRoleArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metric_definitions {
                properties.insert(
                    "MetricDefinitions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-rum-appmonitor-resourcepolicy.html>
    pub struct ResourcePolicy_ {
        pub policy_document: crate::value::ExpString,
        pub policy_revision_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_rum_AppMonitor_ResourcePolicy {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RUM::AppMonitor.ResourcePolicy"
            $($field $value)*)
        };
    }
    pub use crate::__aws_rum_AppMonitor_ResourcePolicy as ResourcePolicy;
    impl crate::value::ToValue for ResourcePolicy_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "PolicyDocument".to_string(),
                crate::value::ToValue::to_value(&self.policy_document),
            );
            if let Some(ref value) = self.policy_revision_id {
                properties.insert(
                    "PolicyRevisionId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-rum-appmonitor.html>
pub struct AppMonitor_ {
    pub app_monitor_configuration: Option<super::rum::appmonitor::AppMonitorConfiguration_>,
    pub custom_events: Option<super::rum::appmonitor::CustomEvents_>,
    pub cw_log_enabled: Option<crate::value::ExpBool>,
    pub deobfuscation_configuration: Option<super::rum::appmonitor::DeobfuscationConfiguration_>,
    pub domain: Option<crate::value::ExpString>,
    pub domain_list: Option<Vec<crate::value::ExpString>>,
    pub name: crate::value::ExpString,
    pub platform: Option<crate::value::ExpString>,
    pub resource_policy: Option<super::rum::appmonitor::ResourcePolicy_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_rum_AppMonitor {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RUM::AppMonitor" $($field
        $value)*)
    };
}
pub use crate::__aws_rum_AppMonitor as AppMonitor;
impl crate::template::ToResource for AppMonitor_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RUM"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("AppMonitor"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.app_monitor_configuration {
            properties.insert(
                "AppMonitorConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_events {
            properties.insert(
                "CustomEvents".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cw_log_enabled {
            properties.insert(
                "CwLogEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.deobfuscation_configuration {
            properties.insert(
                "DeobfuscationConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.domain {
            properties.insert("Domain".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.domain_list {
            properties.insert(
                "DomainList".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.platform {
            properties.insert(
                "Platform".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resource_policy {
            properties.insert(
                "ResourcePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
