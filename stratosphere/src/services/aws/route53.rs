pub mod cidrcollection {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-cidrcollection-location.html
    pub struct Location_ {
        pub cidr_list: Vec<crate::value::ExpString>,
        pub location_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_CidrCollection_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::CidrCollection.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_CidrCollection_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CidrList".to_string(),
                crate::value::ToValue::to_value(&self.cidr_list),
            );
            properties.insert(
                "LocationName".to_string(),
                crate::value::ToValue::to_value(&self.location_name),
            );
            properties.into()
        }
    }
}
pub mod healthcheck {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-alarmidentifier.html
    pub struct AlarmIdentifier_ {
        pub name: crate::value::ExpString,
        pub region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_HealthCheck_AlarmIdentifier {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::HealthCheck.AlarmIdentifier"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_HealthCheck_AlarmIdentifier as AlarmIdentifier;
    impl crate::value::ToValue for AlarmIdentifier_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Region".to_string(),
                crate::value::ToValue::to_value(&self.region),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthcheckconfig.html
    pub struct HealthCheckConfig_ {
        pub alarm_identifier: Option<Box<AlarmIdentifier_>>,
        pub child_health_checks: Option<Vec<crate::value::ExpString>>,
        pub enable_sni: Option<crate::value::ExpBool>,
        pub failure_threshold: Option<i32>,
        pub fully_qualified_domain_name: Option<crate::value::ExpString>,
        pub health_threshold: Option<i32>,
        pub ip_address: Option<crate::value::ExpString>,
        pub insufficient_data_health_status: Option<crate::value::ExpString>,
        pub inverted: Option<crate::value::ExpBool>,
        pub measure_latency: Option<crate::value::ExpBool>,
        pub port: Option<i32>,
        pub regions: Option<Vec<crate::value::ExpString>>,
        pub request_interval: Option<i32>,
        pub resource_path: Option<crate::value::ExpString>,
        pub routing_control_arn: Option<crate::value::ExpString>,
        pub search_string: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_HealthCheck_HealthCheckConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::HealthCheck.HealthCheckConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_HealthCheck_HealthCheckConfig as HealthCheckConfig;
    impl crate::value::ToValue for HealthCheckConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alarm_identifier {
                properties.insert(
                    "AlarmIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.child_health_checks {
                properties.insert(
                    "ChildHealthChecks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_sni {
                properties.insert(
                    "EnableSNI".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failure_threshold {
                properties.insert(
                    "FailureThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.fully_qualified_domain_name {
                properties.insert(
                    "FullyQualifiedDomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_threshold {
                properties.insert(
                    "HealthThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ip_address {
                properties.insert(
                    "IPAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.insufficient_data_health_status {
                properties.insert(
                    "InsufficientDataHealthStatus".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.inverted {
                properties.insert(
                    "Inverted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.measure_latency {
                properties.insert(
                    "MeasureLatency".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.regions {
                properties.insert(
                    "Regions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.request_interval {
                properties.insert(
                    "RequestInterval".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_path {
                properties.insert(
                    "ResourcePath".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.routing_control_arn {
                properties.insert(
                    "RoutingControlArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.search_string {
                properties.insert(
                    "SearchString".to_string(),
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
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-healthcheck-healthchecktag.html
    pub struct HealthCheckTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_HealthCheck_HealthCheckTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::HealthCheck.HealthCheckTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_HealthCheck_HealthCheckTag as HealthCheckTag;
    impl crate::value::ToValue for HealthCheckTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
}
pub mod hostedzone {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzoneconfig.html
    pub struct HostedZoneConfig_ {
        pub comment: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_HostedZone_HostedZoneConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::HostedZone.HostedZoneConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_HostedZone_HostedZoneConfig as HostedZoneConfig;
    impl crate::value::ToValue for HostedZoneConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.comment {
                properties.insert(
                    "Comment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-hostedzonetag.html
    pub struct HostedZoneTag_ {
        pub key: crate::value::ExpString,
        pub value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_HostedZone_HostedZoneTag {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::HostedZone.HostedZoneTag"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_HostedZone_HostedZoneTag as HostedZoneTag;
    impl crate::value::ToValue for HostedZoneTag_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Key".to_string(),
                crate::value::ToValue::to_value(&self.key),
            );
            properties.insert(
                "Value".to_string(),
                crate::value::ToValue::to_value(&self.value),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-queryloggingconfig.html
    pub struct QueryLoggingConfig_ {
        pub cloud_watch_logs_log_group_arn: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_HostedZone_QueryLoggingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::HostedZone.QueryLoggingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_HostedZone_QueryLoggingConfig as QueryLoggingConfig;
    impl crate::value::ToValue for QueryLoggingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CloudWatchLogsLogGroupArn".to_string(),
                crate::value::ToValue::to_value(&self.cloud_watch_logs_log_group_arn),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-hostedzone-vpc.html
    pub struct VPC_ {
        pub vpc_id: crate::value::ExpString,
        pub vpc_region: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_HostedZone_VPC {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::HostedZone.VPC"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_HostedZone_VPC as VPC;
    impl crate::value::ToValue for VPC_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "VPCId".to_string(),
                crate::value::ToValue::to_value(&self.vpc_id),
            );
            properties.insert(
                "VPCRegion".to_string(),
                crate::value::ToValue::to_value(&self.vpc_region),
            );
            properties.into()
        }
    }
}
pub mod recordset {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html
    pub struct AliasTarget_ {
        pub dns_name: crate::value::ExpString,
        pub evaluate_target_health: Option<crate::value::ExpBool>,
        pub hosted_zone_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSet_AliasTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSet.AliasTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSet_AliasTarget as AliasTarget;
    impl crate::value::ToValue for AliasTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DNSName".to_string(),
                crate::value::ToValue::to_value(&self.dns_name),
            );
            if let Some(ref value) = self.evaluate_target_health {
                properties.insert(
                    "EvaluateTargetHealth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HostedZoneId".to_string(),
                crate::value::ToValue::to_value(&self.hosted_zone_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-cidrroutingconfig.html
    pub struct CidrRoutingConfig_ {
        pub collection_id: crate::value::ExpString,
        pub location_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSet_CidrRoutingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSet.CidrRoutingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSet_CidrRoutingConfig as CidrRoutingConfig;
    impl crate::value::ToValue for CidrRoutingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CollectionId".to_string(),
                crate::value::ToValue::to_value(&self.collection_id),
            );
            properties.insert(
                "LocationName".to_string(),
                crate::value::ToValue::to_value(&self.location_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-coordinates.html
    pub struct Coordinates_ {
        pub latitude: crate::value::ExpString,
        pub longitude: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSet_Coordinates {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSet.Coordinates"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSet_Coordinates as Coordinates;
    impl crate::value::ToValue for Coordinates_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Latitude".to_string(),
                crate::value::ToValue::to_value(&self.latitude),
            );
            properties.insert(
                "Longitude".to_string(),
                crate::value::ToValue::to_value(&self.longitude),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html
    pub struct GeoLocation_ {
        pub continent_code: Option<crate::value::ExpString>,
        pub country_code: Option<crate::value::ExpString>,
        pub subdivision_code: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSet_GeoLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSet.GeoLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSet_GeoLocation as GeoLocation;
    impl crate::value::ToValue for GeoLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.continent_code {
                properties.insert(
                    "ContinentCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.country_code {
                properties.insert(
                    "CountryCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subdivision_code {
                properties.insert(
                    "SubdivisionCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-geoproximitylocation.html
    pub struct GeoProximityLocation_ {
        pub aws_region: Option<crate::value::ExpString>,
        pub bias: Option<i32>,
        pub coordinates: Option<Box<Coordinates_>>,
        pub local_zone_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSet_GeoProximityLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSet.GeoProximityLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSet_GeoProximityLocation as GeoProximityLocation;
    impl crate::value::ToValue for GeoProximityLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_region {
                properties.insert(
                    "AWSRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bias {
                properties.insert("Bias".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.coordinates {
                properties.insert(
                    "Coordinates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_zone_group {
                properties.insert(
                    "LocalZoneGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod recordsetgroup {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-aliastarget.html
    pub struct AliasTarget_ {
        pub dns_name: crate::value::ExpString,
        pub evaluate_target_health: Option<crate::value::ExpBool>,
        pub hosted_zone_id: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSetGroup_AliasTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSetGroup.AliasTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSetGroup_AliasTarget as AliasTarget;
    impl crate::value::ToValue for AliasTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "DNSName".to_string(),
                crate::value::ToValue::to_value(&self.dns_name),
            );
            if let Some(ref value) = self.evaluate_target_health {
                properties.insert(
                    "EvaluateTargetHealth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "HostedZoneId".to_string(),
                crate::value::ToValue::to_value(&self.hosted_zone_id),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-cidrroutingconfig.html
    pub struct CidrRoutingConfig_ {
        pub collection_id: crate::value::ExpString,
        pub location_name: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSetGroup_CidrRoutingConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSetGroup.CidrRoutingConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSetGroup_CidrRoutingConfig as CidrRoutingConfig;
    impl crate::value::ToValue for CidrRoutingConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "CollectionId".to_string(),
                crate::value::ToValue::to_value(&self.collection_id),
            );
            properties.insert(
                "LocationName".to_string(),
                crate::value::ToValue::to_value(&self.location_name),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordsetgroup-coordinates.html
    pub struct Coordinates_ {
        pub latitude: crate::value::ExpString,
        pub longitude: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSetGroup_Coordinates {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSetGroup.Coordinates"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSetGroup_Coordinates as Coordinates;
    impl crate::value::ToValue for Coordinates_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "Latitude".to_string(),
                crate::value::ToValue::to_value(&self.latitude),
            );
            properties.insert(
                "Longitude".to_string(),
                crate::value::ToValue::to_value(&self.longitude),
            );
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset-geolocation.html
    pub struct GeoLocation_ {
        pub continent_code: Option<crate::value::ExpString>,
        pub country_code: Option<crate::value::ExpString>,
        pub subdivision_code: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSetGroup_GeoLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSetGroup.GeoLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSetGroup_GeoLocation as GeoLocation;
    impl crate::value::ToValue for GeoLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.continent_code {
                properties.insert(
                    "ContinentCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.country_code {
                properties.insert(
                    "CountryCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subdivision_code {
                properties.insert(
                    "SubdivisionCode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-geoproximitylocation.html
    pub struct GeoProximityLocation_ {
        pub aws_region: Option<crate::value::ExpString>,
        pub bias: Option<i32>,
        pub coordinates: Option<Box<Coordinates_>>,
        pub local_zone_group: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSetGroup_GeoProximityLocation {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSetGroup.GeoProximityLocation"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSetGroup_GeoProximityLocation as GeoProximityLocation;
    impl crate::value::ToValue for GeoProximityLocation_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.aws_region {
                properties.insert(
                    "AWSRegion".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.bias {
                properties.insert("Bias".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.coordinates {
                properties.insert(
                    "Coordinates".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.local_zone_group {
                properties.insert(
                    "LocalZoneGroup".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html
    pub struct RecordSet_ {
        pub alias_target: Option<Box<AliasTarget_>>,
        pub cidr_routing_config: Option<Box<CidrRoutingConfig_>>,
        pub failover: Option<crate::value::ExpString>,
        pub geo_location: Option<Box<GeoLocation_>>,
        pub geo_proximity_location: Option<Box<GeoProximityLocation_>>,
        pub health_check_id: Option<crate::value::ExpString>,
        pub hosted_zone_id: Option<crate::value::ExpString>,
        pub hosted_zone_name: Option<crate::value::ExpString>,
        pub multi_value_answer: Option<crate::value::ExpBool>,
        pub name: crate::value::ExpString,
        pub region: Option<crate::value::ExpString>,
        pub resource_records: Option<Vec<crate::value::ExpString>>,
        pub set_identifier: Option<crate::value::ExpString>,
        pub ttl: Option<crate::value::ExpString>,
        pub r#type: crate::value::ExpString,
        pub weight: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_route53_RecordSetGroup_RecordSet {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Route53::RecordSetGroup.RecordSet"
            $($field $value)*)
        };
    }
    pub use crate::__aws_route53_RecordSetGroup_RecordSet as RecordSet;
    impl crate::value::ToValue for RecordSet_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.alias_target {
                properties.insert(
                    "AliasTarget".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cidr_routing_config {
                properties.insert(
                    "CidrRoutingConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.failover {
                properties.insert(
                    "Failover".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.geo_location {
                properties.insert(
                    "GeoLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.geo_proximity_location {
                properties.insert(
                    "GeoProximityLocation".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check_id {
                properties.insert(
                    "HealthCheckId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hosted_zone_id {
                properties.insert(
                    "HostedZoneId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hosted_zone_name {
                properties.insert(
                    "HostedZoneName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.multi_value_answer {
                properties.insert(
                    "MultiValueAnswer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            if let Some(ref value) = self.region {
                properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.resource_records {
                properties.insert(
                    "ResourceRecords".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.set_identifier {
                properties.insert(
                    "SetIdentifier".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ttl {
                properties.insert("TTL".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            if let Some(ref value) = self.weight {
                properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-cidrcollection.html
pub struct CidrCollection_ {
    pub locations: Option<Vec<super::route53::cidrcollection::Location_>>,
    pub name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53_CidrCollection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53::CidrCollection"
        $($field $value)*)
    };
}
pub use crate::__aws_route53_CidrCollection as CidrCollection;
impl crate::template::ToResource for CidrCollection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CidrCollection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.locations {
            properties.insert(
                "Locations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-dnssec.html
pub struct DNSSEC_ {
    pub hosted_zone_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53_DNSSEC {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53::DNSSEC" $($field
        $value)*)
    };
}
pub use crate::__aws_route53_DNSSEC as DNSSEC;
impl crate::template::ToResource for DNSSEC_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DNSSEC"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "HostedZoneId".to_string(),
            crate::value::ToValue::to_value(&self.hosted_zone_id),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-healthcheck.html
pub struct HealthCheck_ {
    pub health_check_config: super::route53::healthcheck::HealthCheckConfig_,
    pub health_check_tags: Option<Vec<super::route53::healthcheck::HealthCheckTag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53_HealthCheck {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53::HealthCheck"
        $($field $value)*)
    };
}
pub use crate::__aws_route53_HealthCheck as HealthCheck;
impl crate::template::ToResource for HealthCheck_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HealthCheck"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "HealthCheckConfig".to_string(),
            crate::value::ToValue::to_value(&self.health_check_config),
        );
        if let Some(ref value) = self.health_check_tags {
            properties.insert(
                "HealthCheckTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-hostedzone.html
pub struct HostedZone_ {
    pub hosted_zone_config: Option<super::route53::hostedzone::HostedZoneConfig_>,
    pub hosted_zone_tags: Option<Vec<super::route53::hostedzone::HostedZoneTag_>>,
    pub name: Option<crate::value::ExpString>,
    pub query_logging_config: Option<super::route53::hostedzone::QueryLoggingConfig_>,
    pub vp_cs: Option<Vec<super::route53::hostedzone::VPC_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53_HostedZone {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53::HostedZone"
        $($field $value)*)
    };
}
pub use crate::__aws_route53_HostedZone as HostedZone;
impl crate::template::ToResource for HostedZone_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("HostedZone"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.hosted_zone_config {
            properties.insert(
                "HostedZoneConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hosted_zone_tags {
            properties.insert(
                "HostedZoneTags".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.name {
            properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.query_logging_config {
            properties.insert(
                "QueryLoggingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.vp_cs {
            properties.insert("VPCs".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-keysigningkey.html
pub struct KeySigningKey_ {
    pub hosted_zone_id: crate::value::ExpString,
    pub key_management_service_arn: crate::value::ExpString,
    pub name: crate::value::ExpString,
    pub status: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53_KeySigningKey {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53::KeySigningKey"
        $($field $value)*)
    };
}
pub use crate::__aws_route53_KeySigningKey as KeySigningKey;
impl crate::template::ToResource for KeySigningKey_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("KeySigningKey"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "HostedZoneId".to_string(),
            crate::value::ToValue::to_value(&self.hosted_zone_id),
        );
        properties.insert(
            "KeyManagementServiceArn".to_string(),
            crate::value::ToValue::to_value(&self.key_management_service_arn),
        );
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        properties.insert(
            "Status".to_string(),
            crate::value::ToValue::to_value(&self.status),
        );
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-route53-recordset.html
pub struct RecordSet_ {
    pub alias_target: Option<super::route53::recordset::AliasTarget_>,
    pub cidr_routing_config: Option<super::route53::recordset::CidrRoutingConfig_>,
    pub comment: Option<crate::value::ExpString>,
    pub failover: Option<crate::value::ExpString>,
    pub geo_location: Option<super::route53::recordset::GeoLocation_>,
    pub geo_proximity_location: Option<super::route53::recordset::GeoProximityLocation_>,
    pub health_check_id: Option<crate::value::ExpString>,
    pub hosted_zone_id: Option<crate::value::ExpString>,
    pub hosted_zone_name: Option<crate::value::ExpString>,
    pub multi_value_answer: Option<crate::value::ExpBool>,
    pub name: crate::value::ExpString,
    pub region: Option<crate::value::ExpString>,
    pub resource_records: Option<Vec<crate::value::ExpString>>,
    pub set_identifier: Option<crate::value::ExpString>,
    pub ttl: Option<crate::value::ExpString>,
    pub r#type: crate::value::ExpString,
    pub weight: Option<i32>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53_RecordSet {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53::RecordSet"
        $($field $value)*)
    };
}
pub use crate::__aws_route53_RecordSet as RecordSet;
impl crate::template::ToResource for RecordSet_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RecordSet"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.alias_target {
            properties.insert(
                "AliasTarget".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cidr_routing_config {
            properties.insert(
                "CidrRoutingConfig".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.comment {
            properties.insert(
                "Comment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.failover {
            properties.insert(
                "Failover".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.geo_location {
            properties.insert(
                "GeoLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.geo_proximity_location {
            properties.insert(
                "GeoProximityLocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_id {
            properties.insert(
                "HealthCheckId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hosted_zone_id {
            properties.insert(
                "HostedZoneId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hosted_zone_name {
            properties.insert(
                "HostedZoneName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.multi_value_answer {
            properties.insert(
                "MultiValueAnswer".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Name".to_string(),
            crate::value::ToValue::to_value(&self.name),
        );
        if let Some(ref value) = self.region {
            properties.insert("Region".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.resource_records {
            properties.insert(
                "ResourceRecords".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.set_identifier {
            properties.insert(
                "SetIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ttl {
            properties.insert("TTL".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "Type".to_string(),
            crate::value::ToValue::to_value(&self.r#type),
        );
        if let Some(ref value) = self.weight {
            properties.insert("Weight".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-route53-recordsetgroup.html
pub struct RecordSetGroup_ {
    pub comment: Option<crate::value::ExpString>,
    pub hosted_zone_id: Option<crate::value::ExpString>,
    pub hosted_zone_name: Option<crate::value::ExpString>,
    pub record_sets: Option<Vec<super::route53::recordsetgroup::RecordSet_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_route53_RecordSetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Route53::RecordSetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_route53_RecordSetGroup as RecordSetGroup;
impl crate::template::ToResource for RecordSetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Route53"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("RecordSetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.comment {
            properties.insert(
                "Comment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hosted_zone_id {
            properties.insert(
                "HostedZoneId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hosted_zone_name {
            properties.insert(
                "HostedZoneName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.record_sets {
            properties.insert(
                "RecordSets".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
