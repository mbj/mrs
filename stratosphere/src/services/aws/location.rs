pub mod apikey {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-apikey-apikeyrestrictions.html
    pub struct ApiKeyRestrictions_ {
        pub allow_actions: Vec<crate::value::ExpString>,
        pub allow_referers: Option<Vec<crate::value::ExpString>>,
        pub allow_resources: Vec<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_location_APIKey_ApiKeyRestrictions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Location::APIKey.ApiKeyRestrictions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_location_APIKey_ApiKeyRestrictions as ApiKeyRestrictions;
    impl crate::value::ToValue for ApiKeyRestrictions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AllowActions".to_string(),
                crate::value::ToValue::to_value(&self.allow_actions),
            );
            if let Some(ref value) = self.allow_referers {
                properties.insert(
                    "AllowReferers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "AllowResources".to_string(),
                crate::value::ToValue::to_value(&self.allow_resources),
            );
            properties.into()
        }
    }
}
pub mod map {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-map-mapconfiguration.html
    pub struct MapConfiguration_ {
        pub custom_layers: Option<Vec<crate::value::ExpString>>,
        pub political_view: Option<crate::value::ExpString>,
        pub style: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_location_Map_MapConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Location::Map.MapConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_location_Map_MapConfiguration as MapConfiguration;
    impl crate::value::ToValue for MapConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_layers {
                properties.insert(
                    "CustomLayers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.political_view {
                properties.insert(
                    "PoliticalView".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Style".to_string(),
                crate::value::ToValue::to_value(&self.style),
            );
            properties.into()
        }
    }
}
pub mod placeindex {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-location-placeindex-datasourceconfiguration.html
    pub struct DataSourceConfiguration_ {
        pub intended_use: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_location_PlaceIndex_DataSourceConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Location::PlaceIndex.DataSourceConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_location_PlaceIndex_DataSourceConfiguration as DataSourceConfiguration;
    impl crate::value::ToValue for DataSourceConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.intended_use {
                properties.insert(
                    "IntendedUse".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-apikey.html
pub struct APIKey_ {
    pub description: Option<crate::value::ExpString>,
    pub expire_time: Option<crate::value::ExpString>,
    pub force_delete: Option<crate::value::ExpBool>,
    pub force_update: Option<crate::value::ExpBool>,
    pub key_name: crate::value::ExpString,
    pub no_expiry: Option<crate::value::ExpBool>,
    pub restrictions: super::location::apikey::ApiKeyRestrictions_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_location_APIKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Location::APIKey"
        $($field $value)*)
    };
}
pub use crate::__aws_location_APIKey as APIKey;
impl crate::template::ToResource for APIKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Location"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("APIKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.expire_time {
            properties.insert(
                "ExpireTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force_delete {
            properties.insert(
                "ForceDelete".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.force_update {
            properties.insert(
                "ForceUpdate".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "KeyName".to_string(),
            crate::value::ToValue::to_value(&self.key_name),
        );
        if let Some(ref value) = self.no_expiry {
            properties.insert(
                "NoExpiry".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Restrictions".to_string(),
            crate::value::ToValue::to_value(&self.restrictions),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-geofencecollection.html
pub struct GeofenceCollection_ {
    pub collection_name: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_location_GeofenceCollection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Location::GeofenceCollection"
        $($field $value)*)
    };
}
pub use crate::__aws_location_GeofenceCollection as GeofenceCollection;
impl crate::template::ToResource for GeofenceCollection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Location"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("GeofenceCollection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CollectionName".to_string(),
            crate::value::ToValue::to_value(&self.collection_name),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-map.html
pub struct Map_ {
    pub configuration: super::location::map::MapConfiguration_,
    pub description: Option<crate::value::ExpString>,
    pub map_name: crate::value::ExpString,
    pub pricing_plan: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_location_Map {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Location::Map" $($field
        $value)*)
    };
}
pub use crate::__aws_location_Map as Map;
impl crate::template::ToResource for Map_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Location"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Map"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Configuration".to_string(),
            crate::value::ToValue::to_value(&self.configuration),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MapName".to_string(),
            crate::value::ToValue::to_value(&self.map_name),
        );
        if let Some(ref value) = self.pricing_plan {
            properties.insert(
                "PricingPlan".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-placeindex.html
pub struct PlaceIndex_ {
    pub data_source: crate::value::ExpString,
    pub data_source_configuration: Option<super::location::placeindex::DataSourceConfiguration_>,
    pub description: Option<crate::value::ExpString>,
    pub index_name: crate::value::ExpString,
    pub pricing_plan: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_location_PlaceIndex {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Location::PlaceIndex"
        $($field $value)*)
    };
}
pub use crate::__aws_location_PlaceIndex as PlaceIndex;
impl crate::template::ToResource for PlaceIndex_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Location"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("PlaceIndex"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DataSource".to_string(),
            crate::value::ToValue::to_value(&self.data_source),
        );
        if let Some(ref value) = self.data_source_configuration {
            properties.insert(
                "DataSourceConfiguration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "IndexName".to_string(),
            crate::value::ToValue::to_value(&self.index_name),
        );
        if let Some(ref value) = self.pricing_plan {
            properties.insert(
                "PricingPlan".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-routecalculator.html
pub struct RouteCalculator_ {
    pub calculator_name: crate::value::ExpString,
    pub data_source: crate::value::ExpString,
    pub description: Option<crate::value::ExpString>,
    pub pricing_plan: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_location_RouteCalculator {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Location::RouteCalculator"
        $($field $value)*)
    };
}
pub use crate::__aws_location_RouteCalculator as RouteCalculator;
impl crate::template::ToResource for RouteCalculator_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Location"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RouteCalculator"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CalculatorName".to_string(),
            crate::value::ToValue::to_value(&self.calculator_name),
        );
        properties.insert(
            "DataSource".to_string(),
            crate::value::ToValue::to_value(&self.data_source),
        );
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.pricing_plan {
            properties.insert(
                "PricingPlan".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-tracker.html
pub struct Tracker_ {
    pub description: Option<crate::value::ExpString>,
    pub event_bridge_enabled: Option<crate::value::ExpBool>,
    pub kms_key_enable_geospatial_queries: Option<crate::value::ExpBool>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub position_filtering: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tracker_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_location_Tracker {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Location::Tracker"
        $($field $value)*)
    };
}
pub use crate::__aws_location_Tracker as Tracker;
impl crate::template::ToResource for Tracker_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Location"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Tracker"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_bridge_enabled {
            properties.insert(
                "EventBridgeEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_enable_geospatial_queries {
            properties.insert(
                "KmsKeyEnableGeospatialQueries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.position_filtering {
            properties.insert(
                "PositionFiltering".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TrackerName".to_string(),
            crate::value::ToValue::to_value(&self.tracker_name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-location-trackerconsumer.html
pub struct TrackerConsumer_ {
    pub consumer_arn: crate::value::ExpString,
    pub tracker_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_location_TrackerConsumer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Location::TrackerConsumer"
        $($field $value)*)
    };
}
pub use crate::__aws_location_TrackerConsumer as TrackerConsumer;
impl crate::template::ToResource for TrackerConsumer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Location"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("TrackerConsumer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ConsumerArn".to_string(),
            crate::value::ToValue::to_value(&self.consumer_arn),
        );
        properties.insert(
            "TrackerName".to_string(),
            crate::value::ToValue::to_value(&self.tracker_name),
        );
        properties
    }
}
