pub mod cloudautonomousvmcluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-cloudautonomousvmcluster-maintenancewindow.html
    pub struct MaintenanceWindow_ {
        pub days_of_week: Option<Vec<crate::value::ExpString>>,
        pub hours_of_day: Option<Vec<i64>>,
        pub lead_time_in_weeks: Option<i64>,
        pub months: Option<Vec<crate::value::ExpString>>,
        pub preference: Option<crate::value::ExpString>,
        pub weeks_of_month: Option<Vec<i64>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_CloudAutonomousVmCluster_MaintenanceWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::CloudAutonomousVmCluster.MaintenanceWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_CloudAutonomousVmCluster_MaintenanceWindow as MaintenanceWindow;
    impl crate::value::ToValue for MaintenanceWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.days_of_week {
                properties.insert(
                    "DaysOfWeek".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hours_of_day {
                properties.insert(
                    "HoursOfDay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lead_time_in_weeks {
                properties.insert(
                    "LeadTimeInWeeks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.months {
                properties.insert("Months".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.preference {
                properties.insert(
                    "Preference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weeks_of_month {
                properties.insert(
                    "WeeksOfMonth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod cloudexadatainfrastructure {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-cloudexadatainfrastructure-customercontact.html
    pub struct CustomerContact_ {
        pub email: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_CloudExadataInfrastructure_CustomerContact {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::CloudExadataInfrastructure.CustomerContact"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_CloudExadataInfrastructure_CustomerContact as CustomerContact;
    impl crate::value::ToValue for CustomerContact_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.email {
                properties.insert("Email".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-cloudexadatainfrastructure-maintenancewindow.html
    pub struct MaintenanceWindow_ {
        pub custom_action_timeout_in_mins: Option<i64>,
        pub days_of_week: Option<Vec<crate::value::ExpString>>,
        pub hours_of_day: Option<Vec<i64>>,
        pub is_custom_action_timeout_enabled: Option<crate::value::ExpBool>,
        pub lead_time_in_weeks: Option<i64>,
        pub months: Option<Vec<crate::value::ExpString>>,
        pub patching_mode: Option<crate::value::ExpString>,
        pub preference: Option<crate::value::ExpString>,
        pub weeks_of_month: Option<Vec<i64>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_CloudExadataInfrastructure_MaintenanceWindow {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::CloudExadataInfrastructure.MaintenanceWindow"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_CloudExadataInfrastructure_MaintenanceWindow as MaintenanceWindow;
    impl crate::value::ToValue for MaintenanceWindow_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.custom_action_timeout_in_mins {
                properties.insert(
                    "CustomActionTimeoutInMins".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.days_of_week {
                properties.insert(
                    "DaysOfWeek".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hours_of_day {
                properties.insert(
                    "HoursOfDay".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_custom_action_timeout_enabled {
                properties.insert(
                    "IsCustomActionTimeoutEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.lead_time_in_weeks {
                properties.insert(
                    "LeadTimeInWeeks".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.months {
                properties.insert("Months".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.patching_mode {
                properties.insert(
                    "PatchingMode".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.preference {
                properties.insert(
                    "Preference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.weeks_of_month {
                properties.insert(
                    "WeeksOfMonth".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod cloudvmcluster {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-cloudvmcluster-datacollectionoptions.html
    pub struct DataCollectionOptions_ {
        pub is_diagnostics_events_enabled: Option<crate::value::ExpBool>,
        pub is_health_monitoring_enabled: Option<crate::value::ExpBool>,
        pub is_incident_logs_enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_CloudVmCluster_DataCollectionOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::CloudVmCluster.DataCollectionOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_CloudVmCluster_DataCollectionOptions as DataCollectionOptions;
    impl crate::value::ToValue for DataCollectionOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.is_diagnostics_events_enabled {
                properties.insert(
                    "IsDiagnosticsEventsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_health_monitoring_enabled {
                properties.insert(
                    "IsHealthMonitoringEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.is_incident_logs_enabled {
                properties.insert(
                    "IsIncidentLogsEnabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-cloudvmcluster-dbnode.html
    pub struct DbNode_ {
        pub backup_ip_id: Option<crate::value::ExpString>,
        pub backup_vnic2_id: Option<crate::value::ExpString>,
        pub cpu_core_count: Option<i64>,
        pub db_node_arn: Option<crate::value::ExpString>,
        pub db_node_id: Option<crate::value::ExpString>,
        pub db_node_storage_size_in_g_bs: Option<i64>,
        pub db_server_id: crate::value::ExpString,
        pub db_system_id: Option<crate::value::ExpString>,
        pub host_ip_id: Option<crate::value::ExpString>,
        pub hostname: Option<crate::value::ExpString>,
        pub memory_size_in_g_bs: Option<i64>,
        pub ocid: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
        pub vnic2_id: Option<crate::value::ExpString>,
        pub vnic_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_CloudVmCluster_DbNode {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::CloudVmCluster.DbNode"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_CloudVmCluster_DbNode as DbNode;
    impl crate::value::ToValue for DbNode_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.backup_ip_id {
                properties.insert(
                    "BackupIpId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.backup_vnic2_id {
                properties.insert(
                    "BackupVnic2Id".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cpu_core_count {
                properties.insert(
                    "CpuCoreCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.db_node_arn {
                properties.insert(
                    "DbNodeArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.db_node_id {
                properties.insert(
                    "DbNodeId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.db_node_storage_size_in_g_bs {
                properties.insert(
                    "DbNodeStorageSizeInGBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DbServerId".to_string(),
                crate::value::ToValue::to_value(&self.db_server_id),
            );
            if let Some(ref value) = self.db_system_id {
                properties.insert(
                    "DbSystemId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.host_ip_id {
                properties.insert(
                    "HostIpId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hostname {
                properties.insert(
                    "Hostname".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.memory_size_in_g_bs {
                properties.insert(
                    "MemorySizeInGBs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ocid {
                properties.insert("Ocid".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vnic2_id {
                properties.insert(
                    "Vnic2Id".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vnic_id {
                properties.insert("VnicId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod odbnetwork {
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-odbnetwork-manageds3backupaccess.html
    pub struct ManagedS3BackupAccess_ {
        pub ipv4_addresses: Option<Vec<crate::value::ExpString>>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_OdbNetwork_ManagedS3BackupAccess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::OdbNetwork.ManagedS3BackupAccess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_OdbNetwork_ManagedS3BackupAccess as ManagedS3BackupAccess;
    impl crate::value::ToValue for ManagedS3BackupAccess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.ipv4_addresses {
                properties.insert(
                    "Ipv4Addresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-odbnetwork-managedservices.html
    pub struct ManagedServices_ {
        pub managed_s3_backup_access: Option<Box<ManagedS3BackupAccess_>>,
        pub managed_services_ipv4_cidrs: Option<Vec<crate::value::ExpString>>,
        pub resource_gateway_arn: Option<crate::value::ExpString>,
        pub s3_access: Option<Box<S3Access_>>,
        pub service_network_arn: Option<crate::value::ExpString>,
        pub service_network_endpoint: Option<Box<ServiceNetworkEndpoint_>>,
        pub zero_etl_access: Option<Box<ZeroEtlAccess_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_OdbNetwork_ManagedServices {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::OdbNetwork.ManagedServices"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_OdbNetwork_ManagedServices as ManagedServices;
    impl crate::value::ToValue for ManagedServices_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.managed_s3_backup_access {
                properties.insert(
                    "ManagedS3BackupAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.managed_services_ipv4_cidrs {
                properties.insert(
                    "ManagedServicesIpv4Cidrs".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resource_gateway_arn {
                properties.insert(
                    "ResourceGatewayArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_access {
                properties.insert(
                    "S3Access".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_network_arn {
                properties.insert(
                    "ServiceNetworkArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.service_network_endpoint {
                properties.insert(
                    "ServiceNetworkEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.zero_etl_access {
                properties.insert(
                    "ZeroEtlAccess".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-odbnetwork-s3access.html
    pub struct S3Access_ {
        pub domain_name: Option<crate::value::ExpString>,
        pub ipv4_addresses: Option<Vec<crate::value::ExpString>>,
        pub s3_policy_document: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_OdbNetwork_S3Access {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::OdbNetwork.S3Access"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_OdbNetwork_S3Access as S3Access;
    impl crate::value::ToValue for S3Access_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.domain_name {
                properties.insert(
                    "DomainName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ipv4_addresses {
                properties.insert(
                    "Ipv4Addresses".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_policy_document {
                properties.insert(
                    "S3PolicyDocument".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-odbnetwork-servicenetworkendpoint.html
    pub struct ServiceNetworkEndpoint_ {
        pub vpc_endpoint_id: Option<crate::value::ExpString>,
        pub vpc_endpoint_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_OdbNetwork_ServiceNetworkEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::OdbNetwork.ServiceNetworkEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_OdbNetwork_ServiceNetworkEndpoint as ServiceNetworkEndpoint;
    impl crate::value::ToValue for ServiceNetworkEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.vpc_endpoint_id {
                properties.insert(
                    "VpcEndpointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_endpoint_type {
                properties.insert(
                    "VpcEndpointType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-odb-odbnetwork-zeroetlaccess.html
    pub struct ZeroEtlAccess_ {
        pub cidr: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_odb_OdbNetwork_ZeroEtlAccess {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::ODB::OdbNetwork.ZeroEtlAccess"
            $($field $value)*)
        };
    }
    pub use crate::__aws_odb_OdbNetwork_ZeroEtlAccess as ZeroEtlAccess;
    impl crate::value::ToValue for ZeroEtlAccess_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cidr {
                properties.insert("Cidr".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-odb-cloudautonomousvmcluster.html
pub struct CloudAutonomousVmCluster_ {
    pub autonomous_data_storage_size_in_t_bs: Option<f64>,
    pub cloud_exadata_infrastructure_id: Option<crate::value::ExpString>,
    pub cpu_core_count_per_node: Option<i64>,
    pub db_servers: Option<Vec<crate::value::ExpString>>,
    pub description: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub is_mtls_enabled_vm_cluster: Option<crate::value::ExpBool>,
    pub license_model: Option<crate::value::ExpString>,
    pub maintenance_window: Option<super::odb::cloudautonomousvmcluster::MaintenanceWindow_>,
    pub memory_per_oracle_compute_unit_in_g_bs: Option<i64>,
    pub odb_network_id: Option<crate::value::ExpString>,
    pub scan_listener_port_non_tls: Option<i64>,
    pub scan_listener_port_tls: Option<i64>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub time_zone: Option<crate::value::ExpString>,
    pub total_container_databases: Option<i64>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_odb_CloudAutonomousVmCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ODB::CloudAutonomousVmCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_odb_CloudAutonomousVmCluster as CloudAutonomousVmCluster;
impl crate::template::ToResource for CloudAutonomousVmCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ODB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CloudAutonomousVmCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.autonomous_data_storage_size_in_t_bs {
            properties.insert(
                "AutonomousDataStorageSizeInTBs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cloud_exadata_infrastructure_id {
            properties.insert(
                "CloudExadataInfrastructureId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cpu_core_count_per_node {
            properties.insert(
                "CpuCoreCountPerNode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_servers {
            properties.insert(
                "DbServers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.description {
            properties.insert(
                "Description".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_mtls_enabled_vm_cluster {
            properties.insert(
                "IsMtlsEnabledVmCluster".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.license_model {
            properties.insert(
                "LicenseModel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maintenance_window {
            properties.insert(
                "MaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.memory_per_oracle_compute_unit_in_g_bs {
            properties.insert(
                "MemoryPerOracleComputeUnitInGBs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.odb_network_id {
            properties.insert(
                "OdbNetworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scan_listener_port_non_tls {
            properties.insert(
                "ScanListenerPortNonTls".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scan_listener_port_tls {
            properties.insert(
                "ScanListenerPortTls".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.time_zone {
            properties.insert(
                "TimeZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.total_container_databases {
            properties.insert(
                "TotalContainerDatabases".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-odb-cloudexadatainfrastructure.html
pub struct CloudExadataInfrastructure_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub availability_zone_id: Option<crate::value::ExpString>,
    pub compute_count: Option<i64>,
    pub customer_contacts_to_send_to_oci:
        Option<Vec<super::odb::cloudexadatainfrastructure::CustomerContact_>>,
    pub database_server_type: Option<crate::value::ExpString>,
    pub display_name: Option<crate::value::ExpString>,
    pub maintenance_window: Option<super::odb::cloudexadatainfrastructure::MaintenanceWindow_>,
    pub shape: Option<crate::value::ExpString>,
    pub storage_count: Option<i64>,
    pub storage_server_type: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_odb_CloudExadataInfrastructure {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ODB::CloudExadataInfrastructure"
        $($field $value)*)
    };
}
pub use crate::__aws_odb_CloudExadataInfrastructure as CloudExadataInfrastructure;
impl crate::template::ToResource for CloudExadataInfrastructure_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ODB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "CloudExadataInfrastructure",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_id {
            properties.insert(
                "AvailabilityZoneId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.compute_count {
            properties.insert(
                "ComputeCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.customer_contacts_to_send_to_oci {
            properties.insert(
                "CustomerContactsToSendToOCI".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.database_server_type {
            properties.insert(
                "DatabaseServerType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maintenance_window {
            properties.insert(
                "MaintenanceWindow".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.shape {
            properties.insert("Shape".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.storage_count {
            properties.insert(
                "StorageCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.storage_server_type {
            properties.insert(
                "StorageServerType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-odb-cloudvmcluster.html
pub struct CloudVmCluster_ {
    pub cloud_exadata_infrastructure_id: Option<crate::value::ExpString>,
    pub cluster_name: Option<crate::value::ExpString>,
    pub cpu_core_count: Option<i64>,
    pub data_collection_options: Option<super::odb::cloudvmcluster::DataCollectionOptions_>,
    pub data_storage_size_in_t_bs: Option<f64>,
    pub db_node_storage_size_in_g_bs: Option<i64>,
    pub db_nodes: Option<Vec<super::odb::cloudvmcluster::DbNode_>>,
    pub db_servers: Option<Vec<crate::value::ExpString>>,
    pub display_name: Option<crate::value::ExpString>,
    pub gi_version: Option<crate::value::ExpString>,
    pub hostname: Option<crate::value::ExpString>,
    pub is_local_backup_enabled: Option<crate::value::ExpBool>,
    pub is_sparse_diskgroup_enabled: Option<crate::value::ExpBool>,
    pub license_model: Option<crate::value::ExpString>,
    pub memory_size_in_g_bs: Option<i64>,
    pub odb_network_id: Option<crate::value::ExpString>,
    pub scan_listener_port_tcp: Option<i64>,
    pub ssh_public_keys: Option<Vec<crate::value::ExpString>>,
    pub system_version: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub time_zone: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_odb_CloudVmCluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ODB::CloudVmCluster"
        $($field $value)*)
    };
}
pub use crate::__aws_odb_CloudVmCluster as CloudVmCluster;
impl crate::template::ToResource for CloudVmCluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ODB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("CloudVmCluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cloud_exadata_infrastructure_id {
            properties.insert(
                "CloudExadataInfrastructureId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_name {
            properties.insert(
                "ClusterName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cpu_core_count {
            properties.insert(
                "CpuCoreCount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_collection_options {
            properties.insert(
                "DataCollectionOptions".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.data_storage_size_in_t_bs {
            properties.insert(
                "DataStorageSizeInTBs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_node_storage_size_in_g_bs {
            properties.insert(
                "DbNodeStorageSizeInGBs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_nodes {
            properties.insert(
                "DbNodes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_servers {
            properties.insert(
                "DbServers".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.gi_version {
            properties.insert(
                "GiVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hostname {
            properties.insert(
                "Hostname".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_local_backup_enabled {
            properties.insert(
                "IsLocalBackupEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.is_sparse_diskgroup_enabled {
            properties.insert(
                "IsSparseDiskgroupEnabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.license_model {
            properties.insert(
                "LicenseModel".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.memory_size_in_g_bs {
            properties.insert(
                "MemorySizeInGBs".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.odb_network_id {
            properties.insert(
                "OdbNetworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scan_listener_port_tcp {
            properties.insert(
                "ScanListenerPortTcp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ssh_public_keys {
            properties.insert(
                "SshPublicKeys".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.system_version {
            properties.insert(
                "SystemVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.time_zone {
            properties.insert(
                "TimeZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-odb-odbnetwork.html
pub struct OdbNetwork_ {
    pub availability_zone: Option<crate::value::ExpString>,
    pub availability_zone_id: Option<crate::value::ExpString>,
    pub backup_subnet_cidr: Option<crate::value::ExpString>,
    pub client_subnet_cidr: Option<crate::value::ExpString>,
    pub custom_domain_name: Option<crate::value::ExpString>,
    pub default_dns_prefix: Option<crate::value::ExpString>,
    pub delete_associated_resources: Option<crate::value::ExpBool>,
    pub display_name: Option<crate::value::ExpString>,
    pub s3_access: Option<crate::value::ExpString>,
    pub s3_policy_document: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub zero_etl_access: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_odb_OdbNetwork {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ODB::OdbNetwork" $($field
        $value)*)
    };
}
pub use crate::__aws_odb_OdbNetwork as OdbNetwork;
impl crate::template::ToResource for OdbNetwork_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ODB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OdbNetwork"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_id {
            properties.insert(
                "AvailabilityZoneId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.backup_subnet_cidr {
            properties.insert(
                "BackupSubnetCidr".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.client_subnet_cidr {
            properties.insert(
                "ClientSubnetCidr".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.custom_domain_name {
            properties.insert(
                "CustomDomainName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.default_dns_prefix {
            properties.insert(
                "DefaultDnsPrefix".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.delete_associated_resources {
            properties.insert(
                "DeleteAssociatedResources".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_access {
            properties.insert(
                "S3Access".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.s3_policy_document {
            properties.insert(
                "S3PolicyDocument".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.zero_etl_access {
            properties.insert(
                "ZeroEtlAccess".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-odb-odbpeeringconnection.html
pub struct OdbPeeringConnection_ {
    pub display_name: Option<crate::value::ExpString>,
    pub odb_network_id: Option<crate::value::ExpString>,
    pub peer_network_id: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_odb_OdbPeeringConnection {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::ODB::OdbPeeringConnection"
        $($field $value)*)
    };
}
pub use crate::__aws_odb_OdbPeeringConnection as OdbPeeringConnection;
impl crate::template::ToResource for OdbPeeringConnection_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("ODB"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("OdbPeeringConnection"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.display_name {
            properties.insert(
                "DisplayName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.odb_network_id {
            properties.insert(
                "OdbNetworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.peer_network_id {
            properties.insert(
                "PeerNetworkId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
