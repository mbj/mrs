pub mod channel {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-channel-destination.html
    pub struct Destination_ {
        pub location: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Channel_Destination {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Channel.Destination"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Channel_Destination as Destination;
    impl crate::value::ToValue for Destination_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(&self.location),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
}
pub mod dashboard {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-dashboard-frequency.html
    pub struct Frequency_ {
        pub unit: crate::value::ExpString,
        pub value: i32,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Dashboard_Frequency {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Dashboard.Frequency"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Dashboard_Frequency as Frequency;
    impl crate::value::ToValue for Frequency_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Unit".to_string(),
                crate::value::ToValue::to_value(&self.unit),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-dashboard-refreshschedule.html
    pub struct RefreshSchedule_ {
        pub frequency: Option<Box<Frequency_>>,
        pub status: Option<crate::value::ExpString>,
        pub time_of_day: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Dashboard_RefreshSchedule {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Dashboard.RefreshSchedule"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Dashboard_RefreshSchedule as RefreshSchedule;
    impl crate::value::ToValue for RefreshSchedule_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.frequency {
                properties.insert(
                    "Frequency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.time_of_day {
                properties.insert(
                    "TimeOfDay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-dashboard-widget.html
    pub struct Widget_ {
        pub query_parameters: Option<Vec<crate::value::ExpString>>,
        pub query_statement: crate::value::ExpString,
        pub view_properties: Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Dashboard_Widget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Dashboard.Widget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Dashboard_Widget as Widget;
    impl crate::value::ToValue for Widget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.query_parameters {
                properties.insert(
                    "QueryParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "QueryStatement".to_string(),
                crate::value::ToValue::to_value(&self.query_statement),
            );
            if let Some(ref value) = self.view_properties {
                properties.insert(
                    "ViewProperties".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod eventdatastore {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedeventselector.html
    pub struct AdvancedEventSelector_ {
        pub field_selectors: Vec<AdvancedFieldSelector_>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_EventDataStore_AdvancedEventSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::EventDataStore.AdvancedEventSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_EventDataStore_AdvancedEventSelector as AdvancedEventSelector;
    impl crate::value::ToValue for AdvancedEventSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldSelectors".to_string(),
                crate::value::ToValue::to_value(&self.field_selectors),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-advancedfieldselector.html
    pub struct AdvancedFieldSelector_ {
        pub ends_with: Option<Vec<crate::value::ExpString>>,
        pub equals: Option<Vec<crate::value::ExpString>>,
        pub field: crate::value::ExpString,
        pub not_ends_with: Option<Vec<crate::value::ExpString>>,
        pub not_equals: Option<Vec<crate::value::ExpString>>,
        pub not_starts_with: Option<Vec<crate::value::ExpString>>,
        pub starts_with: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_EventDataStore_AdvancedFieldSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::EventDataStore.AdvancedFieldSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_EventDataStore_AdvancedFieldSelector as AdvancedFieldSelector;
    impl crate::value::ToValue for AdvancedFieldSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ends_with {
                properties.insert(
                    "EndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.equals {
                properties.insert("Equals".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Field".to_string(),
                crate::value::ToValue::to_value(&self.field),
            );
            if let Some(ref value) = self.not_ends_with {
                properties.insert(
                    "NotEndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_equals {
                properties.insert(
                    "NotEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_starts_with {
                properties.insert(
                    "NotStartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starts_with {
                properties.insert(
                    "StartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-contextkeyselector.html
    pub struct ContextKeySelector_ {
        pub equals: Vec<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_EventDataStore_ContextKeySelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::EventDataStore.ContextKeySelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_EventDataStore_ContextKeySelector as ContextKeySelector;
    impl crate::value::ToValue for ContextKeySelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Equals".to_string(),
                crate::value::ToValue::to_value(&self.equals),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-eventdatastore-insightselector.html
    pub struct InsightSelector_ {
        pub insight_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_EventDataStore_InsightSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::EventDataStore.InsightSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_EventDataStore_InsightSelector as InsightSelector;
    impl crate::value::ToValue for InsightSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.insight_type {
                properties.insert(
                    "InsightType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod trail {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedeventselector.html
    pub struct AdvancedEventSelector_ {
        pub field_selectors: Vec<AdvancedFieldSelector_>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Trail_AdvancedEventSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Trail.AdvancedEventSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Trail_AdvancedEventSelector as AdvancedEventSelector;
    impl crate::value::ToValue for AdvancedEventSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "FieldSelectors".to_string(),
                crate::value::ToValue::to_value(&self.field_selectors),
            );
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-advancedfieldselector.html
    pub struct AdvancedFieldSelector_ {
        pub ends_with: Option<Vec<crate::value::ExpString>>,
        pub equals: Option<Vec<crate::value::ExpString>>,
        pub field: crate::value::ExpString,
        pub not_ends_with: Option<Vec<crate::value::ExpString>>,
        pub not_equals: Option<Vec<crate::value::ExpString>>,
        pub not_starts_with: Option<Vec<crate::value::ExpString>>,
        pub starts_with: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Trail_AdvancedFieldSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Trail.AdvancedFieldSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Trail_AdvancedFieldSelector as AdvancedFieldSelector;
    impl crate::value::ToValue for AdvancedFieldSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ends_with {
                properties.insert(
                    "EndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.equals {
                properties.insert("Equals".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Field".to_string(),
                crate::value::ToValue::to_value(&self.field),
            );
            if let Some(ref value) = self.not_ends_with {
                properties.insert(
                    "NotEndsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_equals {
                properties.insert(
                    "NotEquals".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.not_starts_with {
                properties.insert(
                    "NotStartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.starts_with {
                properties.insert(
                    "StartsWith".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-aggregationconfiguration.html
    pub struct AggregationConfiguration_ {
        pub event_category: crate::value::ExpString,
        pub templates: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Trail_AggregationConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Trail.AggregationConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Trail_AggregationConfiguration as AggregationConfiguration;
    impl crate::value::ToValue for AggregationConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "EventCategory".to_string(),
                crate::value::ToValue::to_value(&self.event_category),
            );
            properties.insert(
                "Templates".to_string(),
                crate::value::ToValue::to_value(&self.templates),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-dataresource.html
    pub struct DataResource_ {
        pub r#type: crate::value::ExpString,
        pub values: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Trail_DataResource {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Trail.DataResource"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Trail_DataResource as DataResource;
    impl crate::value::ToValue for DataResource_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.values {
                properties.insert("Values".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-eventselector.html
    pub struct EventSelector_ {
        pub data_resources: Option<Vec<DataResource_>>,
        pub exclude_management_event_sources: Option<Vec<crate::value::ExpString>>,
        pub include_management_events: Option<crate::value::ExpBool>,
        pub read_write_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Trail_EventSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Trail.EventSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Trail_EventSelector as EventSelector;
    impl crate::value::ToValue for EventSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.data_resources {
                properties.insert(
                    "DataResources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.exclude_management_event_sources {
                properties.insert(
                    "ExcludeManagementEventSources".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.include_management_events {
                properties.insert(
                    "IncludeManagementEvents".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.read_write_type {
                properties.insert(
                    "ReadWriteType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-cloudtrail-trail-insightselector.html
    pub struct InsightSelector_ {
        pub event_categories: Option<Vec<crate::value::ExpString>>,
        pub insight_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_cloudtrail_Trail_InsightSelector {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::CloudTrail::Trail.InsightSelector"
            $($field $value)*)
        };
    }
    pub use crate::__aws_cloudtrail_Trail_InsightSelector as InsightSelector;
    impl crate::value::ToValue for InsightSelector_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.event_categories {
                properties.insert(
                    "EventCategories".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.insight_type {
                properties.insert(
                    "InsightType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-channel.html
pub struct Channel_ {
    pub destinations: Option<Vec<super::cloudtrail::channel::Destination_>>,
    pub name: Option<crate::value::ExpString>,
    pub source: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudtrail_Channel {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudTrail::Channel"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudtrail_Channel as Channel;
impl crate::template::ToResource for Channel_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudTrail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Channel"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.destinations {
            properties.insert(
                "Destinations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.source {
            properties.insert("Source".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-dashboard.html
pub struct Dashboard_ {
    pub name: Option<crate::value::ExpString>,
    pub refresh_schedule: Option<super::cloudtrail::dashboard::RefreshSchedule_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub termination_protection_enabled: Option<crate::value::ExpBool>,
    pub widgets: Option<Vec<super::cloudtrail::dashboard::Widget_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudtrail_Dashboard {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudTrail::Dashboard"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudtrail_Dashboard as Dashboard;
impl crate::template::ToResource for Dashboard_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudTrail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Dashboard"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.refresh_schedule {
            properties.insert(
                "RefreshSchedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.termination_protection_enabled {
            properties.insert(
                "TerminationProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.widgets {
            properties.insert(
                "Widgets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-eventdatastore.html
pub struct EventDataStore_ {
    pub advanced_event_selectors:
        Option<Vec<super::cloudtrail::eventdatastore::AdvancedEventSelector_>>,
    pub billing_mode: Option<crate::value::ExpString>,
    pub context_key_selectors: Option<Vec<super::cloudtrail::eventdatastore::ContextKeySelector_>>,
    pub federation_enabled: Option<crate::value::ExpBool>,
    pub federation_role_arn: Option<crate::value::ExpString>,
    pub ingestion_enabled: Option<crate::value::ExpBool>,
    pub insight_selectors: Option<Vec<super::cloudtrail::eventdatastore::InsightSelector_>>,
    pub insights_destination: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub max_event_size: Option<crate::value::ExpString>,
    pub multi_region_enabled: Option<crate::value::ExpBool>,
    pub name: Option<crate::value::ExpString>,
    pub organization_enabled: Option<crate::value::ExpBool>,
    pub retention_period: Option<i32>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub termination_protection_enabled: Option<crate::value::ExpBool>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudtrail_EventDataStore {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudTrail::EventDataStore"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudtrail_EventDataStore as EventDataStore;
impl crate::template::ToResource for EventDataStore_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudTrail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventDataStore"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.advanced_event_selectors {
            properties.insert(
                "AdvancedEventSelectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.billing_mode {
            properties.insert(
                "BillingMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.context_key_selectors {
            properties.insert(
                "ContextKeySelectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.federation_enabled {
            properties.insert(
                "FederationEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.federation_role_arn {
            properties.insert(
                "FederationRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ingestion_enabled {
            properties.insert(
                "IngestionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.insight_selectors {
            properties.insert(
                "InsightSelectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.insights_destination {
            properties.insert(
                "InsightsDestination".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_event_size {
            properties.insert(
                "MaxEventSize".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_region_enabled {
            properties.insert(
                "MultiRegionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.organization_enabled {
            properties.insert(
                "OrganizationEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_period {
            properties.insert(
                "RetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.termination_protection_enabled {
            properties.insert(
                "TerminationProtectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-resourcepolicy.html
pub struct ResourcePolicy_ {
    pub resource_arn: crate::value::ExpString,
    pub resource_policy: serde_json::Value,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudtrail_ResourcePolicy {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudTrail::ResourcePolicy"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudtrail_ResourcePolicy as ResourcePolicy;
impl crate::template::ToResource for ResourcePolicy_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudTrail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ResourcePolicy"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ResourceArn".to_string(),
            crate::value::ToValue::to_value(&self.resource_arn),
        );
        properties.insert(
            "ResourcePolicy".to_string(),
            crate::value::ToValue::to_value(&self.resource_policy),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-cloudtrail-trail.html
pub struct Trail_ {
    pub advanced_event_selectors: Option<Vec<super::cloudtrail::trail::AdvancedEventSelector_>>,
    pub aggregation_configurations:
        Option<Vec<super::cloudtrail::trail::AggregationConfiguration_>>,
    pub cloud_watch_logs_log_group_arn: Option<crate::value::ExpString>,
    pub cloud_watch_logs_role_arn: Option<crate::value::ExpString>,
    pub enable_log_file_validation: Option<crate::value::ExpBool>,
    pub event_selectors: Option<Vec<super::cloudtrail::trail::EventSelector_>>,
    pub include_global_service_events: Option<crate::value::ExpBool>,
    pub insight_selectors: Option<Vec<super::cloudtrail::trail::InsightSelector_>>,
    pub is_logging: crate::value::ExpBool,
    pub is_multi_region_trail: Option<crate::value::ExpBool>,
    pub is_organization_trail: Option<crate::value::ExpBool>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub s3_bucket_name: crate::value::ExpString,
    pub s3_key_prefix: Option<crate::value::ExpString>,
    pub sns_topic_name: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub trail_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_cloudtrail_Trail {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::CloudTrail::Trail"
        $($field $value)*)
    };
}
pub use crate::__aws_cloudtrail_Trail as Trail;
impl crate::template::ToResource for Trail_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("CloudTrail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Trail"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.advanced_event_selectors {
            properties.insert(
                "AdvancedEventSelectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.aggregation_configurations {
            properties.insert(
                "AggregationConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cloud_watch_logs_log_group_arn {
            properties.insert(
                "CloudWatchLogsLogGroupArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cloud_watch_logs_role_arn {
            properties.insert(
                "CloudWatchLogsRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enable_log_file_validation {
            properties.insert(
                "EnableLogFileValidation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_selectors {
            properties.insert(
                "EventSelectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.include_global_service_events {
            properties.insert(
                "IncludeGlobalServiceEvents".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.insight_selectors {
            properties.insert(
                "InsightSelectors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IsLogging".to_string(),
            crate::value::ToValue::to_value(&self.is_logging),
        );
        if let Some(ref value) = self.is_multi_region_trail {
            properties.insert(
                "IsMultiRegionTrail".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_organization_trail {
            properties.insert(
                "IsOrganizationTrail".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KMSKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "S3BucketName".to_string(),
            crate::value::ToValue::to_value(&self.s3_bucket_name),
        );
        if let Some(ref value) = self.s3_key_prefix {
            properties.insert(
                "S3KeyPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sns_topic_name {
            properties.insert(
                "SnsTopicName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.trail_name {
            properties.insert(
                "TrailName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
