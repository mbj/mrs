pub mod bucket {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-bucket-accessrules.html>
    pub struct AccessRules_ {
        pub allow_public_overrides: Option<crate::value::ExpBool>,
        pub get_object: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Bucket_AccessRules {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Bucket.AccessRules"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Bucket_AccessRules as AccessRules;
    impl crate::value::ToValue for AccessRules_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allow_public_overrides {
                properties.insert(
                    "AllowPublicOverrides".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.get_object {
                properties.insert(
                    "GetObject".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod container {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-container.html>
    pub struct Container_ {
        pub command: Option<Vec<crate::value::ExpString>>,
        pub container_name: Option<crate::value::ExpString>,
        pub environment: Option<Vec<EnvironmentVariable_>>,
        pub image: Option<crate::value::ExpString>,
        pub ports: Option<Vec<PortInfo_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_Container {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.Container"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_Container as Container;
    impl crate::value::ToValue for Container_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.command {
                properties.insert(
                    "Command".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_name {
                properties.insert(
                    "ContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.environment {
                properties.insert(
                    "Environment".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.image {
                properties.insert("Image".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ports {
                properties.insert("Ports".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-containerservicedeployment.html>
    pub struct ContainerServiceDeployment_ {
        pub containers: Option<Vec<Container_>>,
        pub public_endpoint: Option<Box<PublicEndpoint_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_ContainerServiceDeployment {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.ContainerServiceDeployment"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_ContainerServiceDeployment as ContainerServiceDeployment;
    impl crate::value::ToValue for ContainerServiceDeployment_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.containers {
                properties.insert(
                    "Containers".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.public_endpoint {
                properties.insert(
                    "PublicEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-ecrimagepullerrole.html>
    pub struct EcrImagePullerRole_ {
        pub is_active: Option<crate::value::ExpBool>,
        pub principal_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_EcrImagePullerRole {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.EcrImagePullerRole"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_EcrImagePullerRole as EcrImagePullerRole;
    impl crate::value::ToValue for EcrImagePullerRole_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_active {
                properties.insert(
                    "IsActive".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.principal_arn {
                properties.insert(
                    "PrincipalArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-environmentvariable.html>
    pub struct EnvironmentVariable_ {
        pub value: Option<crate::value::ExpString>,
        pub variable: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_EnvironmentVariable {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.EnvironmentVariable"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_EnvironmentVariable as EnvironmentVariable;
    impl crate::value::ToValue for EnvironmentVariable_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.value {
                properties.insert("Value".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.variable {
                properties.insert(
                    "Variable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-healthcheckconfig.html>
    pub struct HealthCheckConfig_ {
        pub healthy_threshold: Option<i32>,
        pub interval_seconds: Option<i32>,
        pub path: Option<crate::value::ExpString>,
        pub success_codes: Option<crate::value::ExpString>,
        pub timeout_seconds: Option<i32>,
        pub unhealthy_threshold: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_HealthCheckConfig {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.HealthCheckConfig"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_HealthCheckConfig as HealthCheckConfig;
    impl crate::value::ToValue for HealthCheckConfig_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.healthy_threshold {
                properties.insert(
                    "HealthyThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.interval_seconds {
                properties.insert(
                    "IntervalSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.success_codes {
                properties.insert(
                    "SuccessCodes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.timeout_seconds {
                properties.insert(
                    "TimeoutSeconds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.unhealthy_threshold {
                properties.insert(
                    "UnhealthyThreshold".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-portinfo.html>
    pub struct PortInfo_ {
        pub port: Option<crate::value::ExpString>,
        pub protocol: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_PortInfo {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.PortInfo"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_PortInfo as PortInfo;
    impl crate::value::ToValue for PortInfo_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-privateregistryaccess.html>
    pub struct PrivateRegistryAccess_ {
        pub ecr_image_puller_role: Option<Box<EcrImagePullerRole_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_PrivateRegistryAccess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.PrivateRegistryAccess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_PrivateRegistryAccess as PrivateRegistryAccess;
    impl crate::value::ToValue for PrivateRegistryAccess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ecr_image_puller_role {
                properties.insert(
                    "EcrImagePullerRole".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicdomainname.html>
    pub struct PublicDomainName_ {
        pub certificate_name: Option<crate::value::ExpString>,
        pub domain_names: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_PublicDomainName {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.PublicDomainName"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_PublicDomainName as PublicDomainName;
    impl crate::value::ToValue for PublicDomainName_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.certificate_name {
                properties.insert(
                    "CertificateName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.domain_names {
                properties.insert(
                    "DomainNames".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-container-publicendpoint.html>
    pub struct PublicEndpoint_ {
        pub container_name: Option<crate::value::ExpString>,
        pub container_port: Option<i32>,
        pub health_check_config: Option<Box<HealthCheckConfig_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Container_PublicEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Container.PublicEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Container_PublicEndpoint as PublicEndpoint;
    impl crate::value::ToValue for PublicEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.container_name {
                properties.insert(
                    "ContainerName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.container_port {
                properties.insert(
                    "ContainerPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.health_check_config {
                properties.insert(
                    "HealthCheckConfig".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod database {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-database-relationaldatabaseparameter.html>
    pub struct RelationalDatabaseParameter_ {
        pub allowed_values: Option<crate::value::ExpString>,
        pub apply_method: Option<crate::value::ExpString>,
        pub apply_type: Option<crate::value::ExpString>,
        pub data_type: Option<crate::value::ExpString>,
        pub description: Option<crate::value::ExpString>,
        pub is_modifiable: Option<crate::value::ExpBool>,
        pub parameter_name: Option<crate::value::ExpString>,
        pub parameter_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Database_RelationalDatabaseParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Database.RelationalDatabaseParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Database_RelationalDatabaseParameter as RelationalDatabaseParameter;
    impl crate::value::ToValue for RelationalDatabaseParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_values {
                properties.insert(
                    "AllowedValues".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.apply_method {
                properties.insert(
                    "ApplyMethod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.apply_type {
                properties.insert(
                    "ApplyType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.data_type {
                properties.insert(
                    "DataType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_modifiable {
                properties.insert(
                    "IsModifiable".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameter_name {
                properties.insert(
                    "ParameterName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.parameter_value {
                properties.insert(
                    "ParameterValue".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod databasesnapshot {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-databasesnapshot-location.html>
    pub struct Location_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_DatabaseSnapshot_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::DatabaseSnapshot.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_DatabaseSnapshot_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod disk {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-addon.html>
    pub struct AddOn_ {
        pub add_on_type: crate::value::ExpString,
        pub auto_snapshot_add_on_request: Option<Box<AutoSnapshotAddOn_>>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Disk_AddOn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Disk.AddOn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Disk_AddOn as AddOn;
    impl crate::value::ToValue for AddOn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AddOnType".to_string(),
                crate::value::ToValue::to_value(&self.add_on_type),
            );
            if let Some(ref value) = self.auto_snapshot_add_on_request {
                properties.insert(
                    "AutoSnapshotAddOnRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-autosnapshotaddon.html>
    pub struct AutoSnapshotAddOn_ {
        pub snapshot_time_of_day: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Disk_AutoSnapshotAddOn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Disk.AutoSnapshotAddOn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Disk_AutoSnapshotAddOn as AutoSnapshotAddOn;
    impl crate::value::ToValue for AutoSnapshotAddOn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.snapshot_time_of_day {
                properties.insert(
                    "SnapshotTimeOfDay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disk-location.html>
    pub struct Location_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Disk_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Disk.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Disk_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod disksnapshot {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-disksnapshot-location.html>
    pub struct Location_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_DiskSnapshot_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::DiskSnapshot.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_DiskSnapshot_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod distribution {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachebehavior.html>
    pub struct CacheBehavior_ {
        pub behavior: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Distribution_CacheBehavior {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Distribution.CacheBehavior"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Distribution_CacheBehavior as CacheBehavior;
    impl crate::value::ToValue for CacheBehavior_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.behavior {
                properties.insert(
                    "Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachebehaviorperpath.html>
    pub struct CacheBehaviorPerPath_ {
        pub behavior: Option<crate::value::ExpString>,
        pub path: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Distribution_CacheBehaviorPerPath {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Distribution.CacheBehaviorPerPath"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Distribution_CacheBehaviorPerPath as CacheBehaviorPerPath;
    impl crate::value::ToValue for CacheBehaviorPerPath_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.behavior {
                properties.insert(
                    "Behavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.path {
                properties.insert("Path".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cachesettings.html>
    pub struct CacheSettings_ {
        pub allowed_http_methods: Option<crate::value::ExpString>,
        pub cached_http_methods: Option<crate::value::ExpString>,
        pub default_ttl: Option<i64>,
        pub forwarded_cookies: Option<Box<CookieObject_>>,
        pub forwarded_headers: Option<Box<HeaderObject_>>,
        pub forwarded_query_strings: Option<Box<QueryStringObject_>>,
        pub maximum_ttl: Option<i64>,
        pub minimum_ttl: Option<i64>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Distribution_CacheSettings {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Distribution.CacheSettings"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Distribution_CacheSettings as CacheSettings;
    impl crate::value::ToValue for CacheSettings_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.allowed_http_methods {
                properties.insert(
                    "AllowedHTTPMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cached_http_methods {
                properties.insert(
                    "CachedHTTPMethods".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.default_ttl {
                properties.insert(
                    "DefaultTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_cookies {
                properties.insert(
                    "ForwardedCookies".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_headers {
                properties.insert(
                    "ForwardedHeaders".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.forwarded_query_strings {
                properties.insert(
                    "ForwardedQueryStrings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maximum_ttl {
                properties.insert(
                    "MaximumTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.minimum_ttl {
                properties.insert(
                    "MinimumTTL".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-cookieobject.html>
    pub struct CookieObject_ {
        pub cookies_allow_list: Option<Vec<crate::value::ExpString>>,
        pub option: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Distribution_CookieObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Distribution.CookieObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Distribution_CookieObject as CookieObject;
    impl crate::value::ToValue for CookieObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cookies_allow_list {
                properties.insert(
                    "CookiesAllowList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.option {
                properties.insert("Option".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-headerobject.html>
    pub struct HeaderObject_ {
        pub headers_allow_list: Option<Vec<crate::value::ExpString>>,
        pub option: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Distribution_HeaderObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Distribution.HeaderObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Distribution_HeaderObject as HeaderObject;
    impl crate::value::ToValue for HeaderObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.headers_allow_list {
                properties.insert(
                    "HeadersAllowList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.option {
                properties.insert("Option".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-inputorigin.html>
    pub struct InputOrigin_ {
        pub name: Option<crate::value::ExpString>,
        pub protocol_policy: Option<crate::value::ExpString>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Distribution_InputOrigin {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Distribution.InputOrigin"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Distribution_InputOrigin as InputOrigin;
    impl crate::value::ToValue for InputOrigin_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.protocol_policy {
                properties.insert(
                    "ProtocolPolicy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-distribution-querystringobject.html>
    pub struct QueryStringObject_ {
        pub option: Option<crate::value::ExpBool>,
        pub query_strings_allow_list: Option<Vec<crate::value::ExpString>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Distribution_QueryStringObject {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Distribution.QueryStringObject"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Distribution_QueryStringObject as QueryStringObject;
    impl crate::value::ToValue for QueryStringObject_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.option {
                properties.insert("Option".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.query_strings_allow_list {
                properties.insert(
                    "QueryStringsAllowList".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod domain {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-domain-domainentry.html>
    pub struct DomainEntry_ {
        pub id: Option<crate::value::ExpString>,
        pub is_alias: Option<crate::value::ExpBool>,
        pub name: crate::value::ExpString,
        pub target: crate::value::ExpString,
        pub r#type: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Domain_DomainEntry {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Domain.DomainEntry"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Domain_DomainEntry as DomainEntry;
    impl crate::value::ToValue for DomainEntry_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.id {
                properties.insert("Id".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.is_alias {
                properties.insert(
                    "IsAlias".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Name".to_string(),
                crate::value::ToValue::to_value(&self.name),
            );
            properties.insert(
                "Target".to_string(),
                crate::value::ToValue::to_value(&self.target),
            );
            properties.insert(
                "Type".to_string(),
                crate::value::ToValue::to_value(&self.r#type),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-domain-location.html>
    pub struct Location_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Domain_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Domain.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Domain_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod instance {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-addon.html>
    pub struct AddOn_ {
        pub add_on_type: crate::value::ExpString,
        pub auto_snapshot_add_on_request: Option<Box<AutoSnapshotAddOn_>>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_AddOn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.AddOn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_AddOn as AddOn;
    impl crate::value::ToValue for AddOn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "AddOnType".to_string(),
                crate::value::ToValue::to_value(&self.add_on_type),
            );
            if let Some(ref value) = self.auto_snapshot_add_on_request {
                properties.insert(
                    "AutoSnapshotAddOnRequest".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-autosnapshotaddon.html>
    pub struct AutoSnapshotAddOn_ {
        pub snapshot_time_of_day: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_AutoSnapshotAddOn {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.AutoSnapshotAddOn"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_AutoSnapshotAddOn as AutoSnapshotAddOn;
    impl crate::value::ToValue for AutoSnapshotAddOn_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.snapshot_time_of_day {
                properties.insert(
                    "SnapshotTimeOfDay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-disk.html>
    pub struct Disk_ {
        pub attached_to: Option<crate::value::ExpString>,
        pub attachment_state: Option<crate::value::ExpString>,
        pub disk_name: crate::value::ExpString,
        pub iops: Option<i32>,
        pub is_system_disk: Option<crate::value::ExpBool>,
        pub path: crate::value::ExpString,
        pub size_in_gb: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_Disk {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.Disk"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_Disk as Disk;
    impl crate::value::ToValue for Disk_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.attached_to {
                properties.insert(
                    "AttachedTo".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.attachment_state {
                properties.insert(
                    "AttachmentState".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DiskName".to_string(),
                crate::value::ToValue::to_value(&self.disk_name),
            );
            if let Some(ref value) = self.iops {
                properties.insert("IOPS".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.is_system_disk {
                properties.insert(
                    "IsSystemDisk".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Path".to_string(),
                crate::value::ToValue::to_value(&self.path),
            );
            if let Some(ref value) = self.size_in_gb {
                properties.insert(
                    "SizeInGb".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-hardware.html>
    pub struct Hardware_ {
        pub cpu_count: Option<i32>,
        pub disks: Option<Vec<Disk_>>,
        pub ram_size_in_gb: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_Hardware {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.Hardware"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_Hardware as Hardware;
    impl crate::value::ToValue for Hardware_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu_count {
                properties.insert(
                    "CpuCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disks {
                properties.insert("Disks".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.ram_size_in_gb {
                properties.insert(
                    "RamSizeInGb".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-location.html>
    pub struct Location_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-monthlytransfer.html>
    pub struct MonthlyTransfer_ {
        pub gb_per_month_allocated: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_MonthlyTransfer {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.MonthlyTransfer"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_MonthlyTransfer as MonthlyTransfer;
    impl crate::value::ToValue for MonthlyTransfer_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.gb_per_month_allocated {
                properties.insert(
                    "GbPerMonthAllocated".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-networking.html>
    pub struct Networking_ {
        pub monthly_transfer: Option<Box<MonthlyTransfer_>>,
        pub ports: Vec<Port_>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_Networking {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.Networking"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_Networking as Networking;
    impl crate::value::ToValue for Networking_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.monthly_transfer {
                properties.insert(
                    "MonthlyTransfer".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "Ports".to_string(),
                crate::value::ToValue::to_value(&self.ports),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-port.html>
    pub struct Port_ {
        pub access_direction: Option<crate::value::ExpString>,
        pub access_from: Option<crate::value::ExpString>,
        pub access_type: Option<crate::value::ExpString>,
        pub cidr_list_aliases: Option<Vec<crate::value::ExpString>>,
        pub cidrs: Option<Vec<crate::value::ExpString>>,
        pub common_name: Option<crate::value::ExpString>,
        pub from_port: Option<i32>,
        pub ipv6_cidrs: Option<Vec<crate::value::ExpString>>,
        pub protocol: Option<crate::value::ExpString>,
        pub to_port: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_Port {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.Port"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_Port as Port;
    impl crate::value::ToValue for Port_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.access_direction {
                properties.insert(
                    "AccessDirection".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.access_from {
                properties.insert(
                    "AccessFrom".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.access_type {
                properties.insert(
                    "AccessType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cidr_list_aliases {
                properties.insert(
                    "CidrListAliases".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cidrs {
                properties.insert("Cidrs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.common_name {
                properties.insert(
                    "CommonName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.from_port {
                properties.insert(
                    "FromPort".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv6_cidrs {
                properties.insert(
                    "Ipv6Cidrs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.protocol {
                properties.insert(
                    "Protocol".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.to_port {
                properties.insert("ToPort".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instance-state.html>
    pub struct State_ {
        pub code: Option<i32>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_Instance_State {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::Instance.State"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_Instance_State as State;
    impl crate::value::ToValue for State_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.code {
                properties.insert("Code".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod instancesnapshot {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-lightsail-instancesnapshot-location.html>
    pub struct Location_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub region_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_lightsail_InstanceSnapshot_Location {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Lightsail::InstanceSnapshot.Location"
            $($field $value)*)
        };
    }
    pub use crate::__aws_lightsail_InstanceSnapshot_Location as Location;
    impl crate::value::ToValue for Location_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.region_name {
                properties.insert(
                    "RegionName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-alarm.html>
pub struct Alarm_ {
    pub alarm_name: crate::value::ExpString,
    pub comparison_operator: crate::value::ExpString,
    pub contact_protocols: Option<Vec<crate::value::ExpString>>,
    pub datapoints_to_alarm: Option<i32>,
    pub evaluation_periods: i32,
    pub metric_name: crate::value::ExpString,
    pub monitored_resource_name: crate::value::ExpString,
    pub notification_enabled: Option<crate::value::ExpBool>,
    pub notification_triggers: Option<Vec<crate::value::ExpString>>,
    pub threshold: f64,
    pub treat_missing_data: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Alarm {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Alarm"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Alarm as Alarm;
impl crate::template::ToResource for Alarm_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Alarm"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AlarmName".to_string(),
            crate::value::ToValue::to_value(&self.alarm_name),
        );
        properties.insert(
            "ComparisonOperator".to_string(),
            crate::value::ToValue::to_value(&self.comparison_operator),
        );
        if let Some(ref value) = self.contact_protocols {
            properties.insert(
                "ContactProtocols".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.datapoints_to_alarm {
            properties.insert(
                "DatapointsToAlarm".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "EvaluationPeriods".to_string(),
            crate::value::ToValue::to_value(&self.evaluation_periods),
        );
        properties.insert(
            "MetricName".to_string(),
            crate::value::ToValue::to_value(&self.metric_name),
        );
        properties.insert(
            "MonitoredResourceName".to_string(),
            crate::value::ToValue::to_value(&self.monitored_resource_name),
        );
        if let Some(ref value) = self.notification_enabled {
            properties.insert(
                "NotificationEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.notification_triggers {
            properties.insert(
                "NotificationTriggers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Threshold".to_string(),
            crate::value::ToValue::to_value(&self.threshold),
        );
        if let Some(ref value) = self.treat_missing_data {
            properties.insert(
                "TreatMissingData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-bucket.html>
pub struct Bucket_ {
    pub access_rules: Option<super::lightsail::bucket::AccessRules_>,
    pub bucket_name: crate::value::ExpString,
    pub bundle_id: crate::value::ExpString,
    pub object_versioning: Option<crate::value::ExpBool>,
    pub read_only_access_accounts: Option<Vec<crate::value::ExpString>>,
    pub resources_receiving_access: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Bucket {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Bucket"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Bucket as Bucket;
impl crate::template::ToResource for Bucket_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Bucket"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.access_rules {
            properties.insert(
                "AccessRules".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BucketName".to_string(),
            crate::value::ToValue::to_value(&self.bucket_name),
        );
        properties.insert(
            "BundleId".to_string(),
            crate::value::ToValue::to_value(&self.bundle_id),
        );
        if let Some(ref value) = self.object_versioning {
            properties.insert(
                "ObjectVersioning".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.read_only_access_accounts {
            properties.insert(
                "ReadOnlyAccessAccounts".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.resources_receiving_access {
            properties.insert(
                "ResourcesReceivingAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-certificate.html>
pub struct Certificate_ {
    pub certificate_name: crate::value::ExpString,
    pub domain_name: crate::value::ExpString,
    pub subject_alternative_names: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Certificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Certificate"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Certificate as Certificate;
impl crate::template::ToResource for Certificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Certificate"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "CertificateName".to_string(),
            crate::value::ToValue::to_value(&self.certificate_name),
        );
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.subject_alternative_names {
            properties.insert(
                "SubjectAlternativeNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-container.html>
pub struct Container_ {
    pub container_service_deployment:
        Option<super::lightsail::container::ContainerServiceDeployment_>,
    pub is_disabled: Option<crate::value::ExpBool>,
    pub power: crate::value::ExpString,
    pub private_registry_access: Option<super::lightsail::container::PrivateRegistryAccess_>,
    pub public_domain_names: Option<Vec<super::lightsail::container::PublicDomainName_>>,
    pub scale: i32,
    pub service_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Container {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Container"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Container as Container;
impl crate::template::ToResource for Container_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Container"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.container_service_deployment {
            properties.insert(
                "ContainerServiceDeployment".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_disabled {
            properties.insert(
                "IsDisabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Power".to_string(),
            crate::value::ToValue::to_value(&self.power),
        );
        if let Some(ref value) = self.private_registry_access {
            properties.insert(
                "PrivateRegistryAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.public_domain_names {
            properties.insert(
                "PublicDomainNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Scale".to_string(),
            crate::value::ToValue::to_value(&self.scale),
        );
        properties.insert(
            "ServiceName".to_string(),
            crate::value::ToValue::to_value(&self.service_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-database.html>
pub struct Database_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub backup_retention: Option<crate::value::ExpBool>,
    pub ca_certificate_identifier: Option<crate::value::ExpString>,
    pub master_database_name: crate::value::ExpString,
    pub master_user_password: Option<crate::value::ExpString>,
    pub master_username: crate::value::ExpString,
    pub preferred_backup_window: Option<crate::value::ExpString>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub relational_database_blueprint_id: crate::value::ExpString,
    pub relational_database_bundle_id: crate::value::ExpString,
    pub relational_database_name: crate::value::ExpString,
    pub relational_database_parameters:
        Option<Vec<super::lightsail::database::RelationalDatabaseParameter_>>,
    pub rotate_master_user_password: Option<crate::value::ExpBool>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Database {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Database"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Database as Database;
impl crate::template::ToResource for Database_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Database"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_retention {
            properties.insert(
                "BackupRetention".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ca_certificate_identifier {
            properties.insert(
                "CaCertificateIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MasterDatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.master_database_name),
        );
        if let Some(ref value) = self.master_user_password {
            properties.insert(
                "MasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "MasterUsername".to_string(),
            crate::value::ToValue::to_value(&self.master_username),
        );
        if let Some(ref value) = self.preferred_backup_window {
            properties.insert(
                "PreferredBackupWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.preferred_maintenance_window {
            properties.insert(
                "PreferredMaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "RelationalDatabaseBlueprintId".to_string(),
            crate::value::ToValue::to_value(&self.relational_database_blueprint_id),
        );
        properties.insert(
            "RelationalDatabaseBundleId".to_string(),
            crate::value::ToValue::to_value(&self.relational_database_bundle_id),
        );
        properties.insert(
            "RelationalDatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.relational_database_name),
        );
        if let Some(ref value) = self.relational_database_parameters {
            properties.insert(
                "RelationalDatabaseParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rotate_master_user_password {
            properties.insert(
                "RotateMasterUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-databasesnapshot.html>
pub struct DatabaseSnapshot_ {
    pub relational_database_name: crate::value::ExpString,
    pub relational_database_snapshot_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_DatabaseSnapshot {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::DatabaseSnapshot"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_DatabaseSnapshot as DatabaseSnapshot;
impl crate::template::ToResource for DatabaseSnapshot_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DatabaseSnapshot"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "RelationalDatabaseName".to_string(),
            crate::value::ToValue::to_value(&self.relational_database_name),
        );
        properties.insert(
            "RelationalDatabaseSnapshotName".to_string(),
            crate::value::ToValue::to_value(&self.relational_database_snapshot_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disk.html>
pub struct Disk_ {
    pub add_ons: Option<Vec<super::lightsail::disk::AddOn_>>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub disk_name: crate::value::ExpString,
    pub location: Option<super::lightsail::disk::Location_>,
    pub size_in_gb: i32,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Disk {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Disk" $($field
        $value)*)
    };
}
pub use crate::__aws_lightsail_Disk as Disk;
impl crate::template::ToResource for Disk_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Disk"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.add_ons {
            properties.insert("AddOns".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DiskName".to_string(),
            crate::value::ToValue::to_value(&self.disk_name),
        );
        if let Some(ref value) = self.location {
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SizeInGb".to_string(),
            crate::value::ToValue::to_value(&self.size_in_gb),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-disksnapshot.html>
pub struct DiskSnapshot_ {
    pub disk_name: crate::value::ExpString,
    pub disk_snapshot_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_DiskSnapshot {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::DiskSnapshot"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_DiskSnapshot as DiskSnapshot;
impl crate::template::ToResource for DiskSnapshot_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("DiskSnapshot"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "DiskName".to_string(),
            crate::value::ToValue::to_value(&self.disk_name),
        );
        properties.insert(
            "DiskSnapshotName".to_string(),
            crate::value::ToValue::to_value(&self.disk_snapshot_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-distribution.html>
pub struct Distribution_ {
    pub bundle_id: crate::value::ExpString,
    pub cache_behavior_settings: Option<super::lightsail::distribution::CacheSettings_>,
    pub cache_behaviors: Option<Vec<super::lightsail::distribution::CacheBehaviorPerPath_>>,
    pub certificate_name: Option<crate::value::ExpString>,
    pub default_cache_behavior: super::lightsail::distribution::CacheBehavior_,
    pub distribution_name: crate::value::ExpString,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub is_enabled: Option<crate::value::ExpBool>,
    pub origin: super::lightsail::distribution::InputOrigin_,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Distribution {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Distribution"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Distribution as Distribution;
impl crate::template::ToResource for Distribution_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Distribution"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "BundleId".to_string(),
            crate::value::ToValue::to_value(&self.bundle_id),
        );
        if let Some(ref value) = self.cache_behavior_settings {
            properties.insert(
                "CacheBehaviorSettings".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cache_behaviors {
            properties.insert(
                "CacheBehaviors".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.certificate_name {
            properties.insert(
                "CertificateName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DefaultCacheBehavior".to_string(),
            crate::value::ToValue::to_value(&self.default_cache_behavior),
        );
        properties.insert(
            "DistributionName".to_string(),
            crate::value::ToValue::to_value(&self.distribution_name),
        );
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_enabled {
            properties.insert(
                "IsEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "Origin".to_string(),
            crate::value::ToValue::to_value(&self.origin),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-domain.html>
pub struct Domain_ {
    pub domain_entries: Option<Vec<super::lightsail::domain::DomainEntry_>>,
    pub domain_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Domain {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Domain"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Domain as Domain;
impl crate::template::ToResource for Domain_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Domain"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.domain_entries {
            properties.insert(
                "DomainEntries".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DomainName".to_string(),
            crate::value::ToValue::to_value(&self.domain_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instance.html>
pub struct Instance_ {
    pub add_ons: Option<Vec<super::lightsail::instance::AddOn_>>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub blueprint_id: crate::value::ExpString,
    pub bundle_id: crate::value::ExpString,
    pub hardware: Option<super::lightsail::instance::Hardware_>,
    pub instance_name: crate::value::ExpString,
    pub key_pair_name: Option<crate::value::ExpString>,
    pub location: Option<super::lightsail::instance::Location_>,
    pub networking: Option<super::lightsail::instance::Networking_>,
    pub state: Option<super::lightsail::instance::State_>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub user_data: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_Instance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::Instance"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_Instance as Instance;
impl crate::template::ToResource for Instance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Instance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.add_ons {
            properties.insert("AddOns".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "BlueprintId".to_string(),
            crate::value::ToValue::to_value(&self.blueprint_id),
        );
        properties.insert(
            "BundleId".to_string(),
            crate::value::ToValue::to_value(&self.bundle_id),
        );
        if let Some(ref value) = self.hardware {
            properties.insert(
                "Hardware".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstanceName".to_string(),
            crate::value::ToValue::to_value(&self.instance_name),
        );
        if let Some(ref value) = self.key_pair_name {
            properties.insert(
                "KeyPairName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.location {
            properties.insert(
                "Location".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.networking {
            properties.insert(
                "Networking".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.state {
            properties.insert("State".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.user_data {
            properties.insert(
                "UserData".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-instancesnapshot.html>
pub struct InstanceSnapshot_ {
    pub instance_name: crate::value::ExpString,
    pub instance_snapshot_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_InstanceSnapshot {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::InstanceSnapshot"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_InstanceSnapshot as InstanceSnapshot;
impl crate::template::ToResource for InstanceSnapshot_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("InstanceSnapshot"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "InstanceName".to_string(),
            crate::value::ToValue::to_value(&self.instance_name),
        );
        properties.insert(
            "InstanceSnapshotName".to_string(),
            crate::value::ToValue::to_value(&self.instance_snapshot_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancer.html>
pub struct LoadBalancer_ {
    pub attached_instances: Option<Vec<crate::value::ExpString>>,
    pub health_check_path: Option<crate::value::ExpString>,
    pub instance_port: i32,
    pub ip_address_type: Option<crate::value::ExpString>,
    pub load_balancer_name: crate::value::ExpString,
    pub session_stickiness_enabled: Option<crate::value::ExpBool>,
    pub session_stickiness_lb_cookie_duration_seconds: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub tls_policy_name: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_LoadBalancer {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::LoadBalancer"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_LoadBalancer as LoadBalancer;
impl crate::template::ToResource for LoadBalancer_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("LoadBalancer"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attached_instances {
            properties.insert(
                "AttachedInstances".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.health_check_path {
            properties.insert(
                "HealthCheckPath".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "InstancePort".to_string(),
            crate::value::ToValue::to_value(&self.instance_port),
        );
        if let Some(ref value) = self.ip_address_type {
            properties.insert(
                "IpAddressType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LoadBalancerName".to_string(),
            crate::value::ToValue::to_value(&self.load_balancer_name),
        );
        if let Some(ref value) = self.session_stickiness_enabled {
            properties.insert(
                "SessionStickinessEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.session_stickiness_lb_cookie_duration_seconds {
            properties.insert(
                "SessionStickinessLBCookieDurationSeconds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.tls_policy_name {
            properties.insert(
                "TlsPolicyName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-loadbalancertlscertificate.html>
pub struct LoadBalancerTlsCertificate_ {
    pub certificate_alternative_names: Option<Vec<crate::value::ExpString>>,
    pub certificate_domain_name: crate::value::ExpString,
    pub certificate_name: crate::value::ExpString,
    pub https_redirection_enabled: Option<crate::value::ExpBool>,
    pub is_attached: Option<crate::value::ExpBool>,
    pub load_balancer_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_LoadBalancerTlsCertificate {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::LoadBalancerTlsCertificate"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_LoadBalancerTlsCertificate as LoadBalancerTlsCertificate;
impl crate::template::ToResource for LoadBalancerTlsCertificate_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "LoadBalancerTlsCertificate",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.certificate_alternative_names {
            properties.insert(
                "CertificateAlternativeNames".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "CertificateDomainName".to_string(),
            crate::value::ToValue::to_value(&self.certificate_domain_name),
        );
        properties.insert(
            "CertificateName".to_string(),
            crate::value::ToValue::to_value(&self.certificate_name),
        );
        if let Some(ref value) = self.https_redirection_enabled {
            properties.insert(
                "HttpsRedirectionEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_attached {
            properties.insert(
                "IsAttached".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "LoadBalancerName".to_string(),
            crate::value::ToValue::to_value(&self.load_balancer_name),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-lightsail-staticip.html>
pub struct StaticIp_ {
    pub attached_to: Option<crate::value::ExpString>,
    pub static_ip_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_lightsail_StaticIp {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Lightsail::StaticIp"
        $($field $value)*)
    };
}
pub use crate::__aws_lightsail_StaticIp as StaticIp;
impl crate::template::ToResource for StaticIp_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Lightsail"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("StaticIp"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.attached_to {
            properties.insert(
                "AttachedTo".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "StaticIpName".to_string(),
            crate::value::ToValue::to_value(&self.static_ip_name),
        );
        properties
    }
}
