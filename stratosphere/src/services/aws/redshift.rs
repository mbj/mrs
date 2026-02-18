pub mod cluster {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-endpoint.html>
    pub struct Endpoint_ {
        pub address: Option<crate::value::ExpString>,
        pub port: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_Cluster_Endpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::Cluster.Endpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_Cluster_Endpoint as Endpoint;
    impl crate::value::ToValue for Endpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.address {
                properties.insert(
                    "Address".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.port {
                properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-cluster-loggingproperties.html>
    pub struct LoggingProperties_ {
        pub bucket_name: Option<crate::value::ExpString>,
        pub log_destination_type: Option<crate::value::ExpString>,
        pub log_exports: Option<Vec<crate::value::ExpString>>,
        pub s3_key_prefix: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_Cluster_LoggingProperties {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::Cluster.LoggingProperties"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_Cluster_LoggingProperties as LoggingProperties;
    impl crate::value::ToValue for LoggingProperties_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bucket_name {
                properties.insert(
                    "BucketName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_destination_type {
                properties.insert(
                    "LogDestinationType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.log_exports {
                properties.insert(
                    "LogExports".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.s3_key_prefix {
                properties.insert(
                    "S3KeyPrefix".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod clusterparametergroup {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-clusterparametergroup-parameter.html>
    pub struct Parameter_ {
        pub parameter_name: crate::value::ExpString,
        pub parameter_value: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_ClusterParameterGroup_Parameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::ClusterParameterGroup.Parameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_ClusterParameterGroup_Parameter as Parameter;
    impl crate::value::ToValue for Parameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ParameterName".to_string(),
                crate::value::ToValue::to_value(&self.parameter_name),
            );
            properties.insert(
                "ParameterValue".to_string(),
                crate::value::ToValue::to_value(&self.parameter_value),
            );
            properties.into()
        }
    }
}
pub mod endpointaccess {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-networkinterface.html>
    pub struct NetworkInterface_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub network_interface_id: Option<crate::value::ExpString>,
        pub private_ip_address: Option<crate::value::ExpString>,
        pub subnet_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_EndpointAccess_NetworkInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::EndpointAccess.NetworkInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_EndpointAccess_NetworkInterface as NetworkInterface;
    impl crate::value::ToValue for NetworkInterface_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interface_id {
                properties.insert(
                    "NetworkInterfaceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_ip_address {
                properties.insert(
                    "PrivateIpAddress".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_id {
                properties.insert(
                    "SubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcendpoint.html>
    pub struct VpcEndpoint_ {
        pub network_interfaces: Option<Vec<NetworkInterface_>>,
        pub vpc_endpoint_id: Option<crate::value::ExpString>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_EndpointAccess_VpcEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::EndpointAccess.VpcEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_EndpointAccess_VpcEndpoint as VpcEndpoint;
    impl crate::value::ToValue for VpcEndpoint_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.network_interfaces {
                properties.insert(
                    "NetworkInterfaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_endpoint_id {
                properties.insert(
                    "VpcEndpointId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.vpc_id {
                properties.insert("VpcId".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-endpointaccess-vpcsecuritygroup.html>
    pub struct VpcSecurityGroup_ {
        pub status: Option<crate::value::ExpString>,
        pub vpc_security_group_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_EndpointAccess_VpcSecurityGroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::EndpointAccess.VpcSecurityGroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_EndpointAccess_VpcSecurityGroup as VpcSecurityGroup;
    impl crate::value::ToValue for VpcSecurityGroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.vpc_security_group_id {
                properties.insert(
                    "VpcSecurityGroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod scheduledaction {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-pauseclustermessage.html>
    pub struct PauseClusterMessage_ {
        pub cluster_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_ScheduledAction_PauseClusterMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::ScheduledAction.PauseClusterMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_ScheduledAction_PauseClusterMessage as PauseClusterMessage;
    impl crate::value::ToValue for PauseClusterMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.cluster_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resizeclustermessage.html>
    pub struct ResizeClusterMessage_ {
        pub classic: Option<crate::value::ExpBool>,
        pub cluster_identifier: crate::value::ExpString,
        pub cluster_type: Option<crate::value::ExpString>,
        pub node_type: Option<crate::value::ExpString>,
        pub number_of_nodes: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_ScheduledAction_ResizeClusterMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::ScheduledAction.ResizeClusterMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_ScheduledAction_ResizeClusterMessage as ResizeClusterMessage;
    impl crate::value::ToValue for ResizeClusterMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.classic {
                properties.insert(
                    "Classic".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.cluster_identifier),
            );
            if let Some(ref value) = self.cluster_type {
                properties.insert(
                    "ClusterType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.node_type {
                properties.insert(
                    "NodeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.number_of_nodes {
                properties.insert(
                    "NumberOfNodes".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-resumeclustermessage.html>
    pub struct ResumeClusterMessage_ {
        pub cluster_identifier: crate::value::ExpString,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_ScheduledAction_ResumeClusterMessage {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::ScheduledAction.ResumeClusterMessage"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_ScheduledAction_ResumeClusterMessage as ResumeClusterMessage;
    impl crate::value::ToValue for ResumeClusterMessage_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            properties.insert(
                "ClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(&self.cluster_identifier),
            );
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshift-scheduledaction-scheduledactiontype.html>
    pub struct ScheduledActionType_ {
        pub pause_cluster: Option<Box<PauseClusterMessage_>>,
        pub resize_cluster: Option<Box<ResizeClusterMessage_>>,
        pub resume_cluster: Option<Box<ResumeClusterMessage_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshift_ScheduledAction_ScheduledActionType {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::Redshift::ScheduledAction.ScheduledActionType"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshift_ScheduledAction_ScheduledActionType as ScheduledActionType;
    impl crate::value::ToValue for ScheduledActionType_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.pause_cluster {
                properties.insert(
                    "PauseCluster".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resize_cluster {
                properties.insert(
                    "ResizeCluster".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.resume_cluster {
                properties.insert(
                    "ResumeCluster".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-cluster.html>
pub struct Cluster_ {
    pub allow_version_upgrade: Option<crate::value::ExpBool>,
    pub aqua_configuration_status: Option<crate::value::ExpString>,
    pub automated_snapshot_retention_period: Option<i32>,
    pub availability_zone: Option<crate::value::ExpString>,
    pub availability_zone_relocation: Option<crate::value::ExpBool>,
    pub availability_zone_relocation_status: Option<crate::value::ExpString>,
    pub classic: Option<crate::value::ExpBool>,
    pub cluster_identifier: Option<crate::value::ExpString>,
    pub cluster_parameter_group_name: Option<crate::value::ExpString>,
    pub cluster_security_groups: Option<Vec<crate::value::ExpString>>,
    pub cluster_subnet_group_name: Option<crate::value::ExpString>,
    pub cluster_type: crate::value::ExpString,
    pub cluster_version: Option<crate::value::ExpString>,
    pub db_name: crate::value::ExpString,
    pub defer_maintenance: Option<crate::value::ExpBool>,
    pub defer_maintenance_duration: Option<i32>,
    pub defer_maintenance_end_time: Option<crate::value::ExpString>,
    pub defer_maintenance_start_time: Option<crate::value::ExpString>,
    pub destination_region: Option<crate::value::ExpString>,
    pub elastic_ip: Option<crate::value::ExpString>,
    pub encrypted: Option<crate::value::ExpBool>,
    pub endpoint: Option<super::redshift::cluster::Endpoint_>,
    pub enhanced_vpc_routing: Option<crate::value::ExpBool>,
    pub hsm_client_certificate_identifier: Option<crate::value::ExpString>,
    pub hsm_configuration_identifier: Option<crate::value::ExpString>,
    pub iam_roles: Option<Vec<crate::value::ExpString>>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub logging_properties: Option<super::redshift::cluster::LoggingProperties_>,
    pub maintenance_track_name: Option<crate::value::ExpString>,
    pub manage_master_password: Option<crate::value::ExpBool>,
    pub manual_snapshot_retention_period: Option<i32>,
    pub master_password_secret_kms_key_id: Option<crate::value::ExpString>,
    pub master_user_password: Option<crate::value::ExpString>,
    pub master_username: crate::value::ExpString,
    pub multi_az: Option<crate::value::ExpBool>,
    pub namespace_resource_policy: Option<serde_json::Value>,
    pub node_type: crate::value::ExpString,
    pub number_of_nodes: Option<i32>,
    pub owner_account: Option<crate::value::ExpString>,
    pub port: Option<i32>,
    pub preferred_maintenance_window: Option<crate::value::ExpString>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub resource_action: Option<crate::value::ExpString>,
    pub revision_target: Option<crate::value::ExpString>,
    pub rotate_encryption_key: Option<crate::value::ExpBool>,
    pub snapshot_cluster_identifier: Option<crate::value::ExpString>,
    pub snapshot_copy_grant_name: Option<crate::value::ExpString>,
    pub snapshot_copy_manual: Option<crate::value::ExpBool>,
    pub snapshot_copy_retention_period: Option<i32>,
    pub snapshot_identifier: Option<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub vpc_security_group_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_Cluster {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::Cluster"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_Cluster as Cluster;
impl crate::template::ToResource for Cluster_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Cluster"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.allow_version_upgrade {
            properties.insert(
                "AllowVersionUpgrade".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.aqua_configuration_status {
            properties.insert(
                "AquaConfigurationStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.automated_snapshot_retention_period {
            properties.insert(
                "AutomatedSnapshotRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone {
            properties.insert(
                "AvailabilityZone".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_relocation {
            properties.insert(
                "AvailabilityZoneRelocation".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.availability_zone_relocation_status {
            properties.insert(
                "AvailabilityZoneRelocationStatus".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.classic {
            properties.insert(
                "Classic".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_identifier {
            properties.insert(
                "ClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_parameter_group_name {
            properties.insert(
                "ClusterParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_security_groups {
            properties.insert(
                "ClusterSecurityGroups".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.cluster_subnet_group_name {
            properties.insert(
                "ClusterSubnetGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ClusterType".to_string(),
            crate::value::ToValue::to_value(&self.cluster_type),
        );
        if let Some(ref value) = self.cluster_version {
            properties.insert(
                "ClusterVersion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "DBName".to_string(),
            crate::value::ToValue::to_value(&self.db_name),
        );
        if let Some(ref value) = self.defer_maintenance {
            properties.insert(
                "DeferMaintenance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.defer_maintenance_duration {
            properties.insert(
                "DeferMaintenanceDuration".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.defer_maintenance_end_time {
            properties.insert(
                "DeferMaintenanceEndTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.defer_maintenance_start_time {
            properties.insert(
                "DeferMaintenanceStartTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.destination_region {
            properties.insert(
                "DestinationRegion".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.elastic_ip {
            properties.insert(
                "ElasticIp".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.encrypted {
            properties.insert(
                "Encrypted".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.endpoint {
            properties.insert(
                "Endpoint".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enhanced_vpc_routing {
            properties.insert(
                "EnhancedVpcRouting".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hsm_client_certificate_identifier {
            properties.insert(
                "HsmClientCertificateIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.hsm_configuration_identifier {
            properties.insert(
                "HsmConfigurationIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_roles {
            properties.insert(
                "IamRoles".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.logging_properties {
            properties.insert(
                "LoggingProperties".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.maintenance_track_name {
            properties.insert(
                "MaintenanceTrackName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manage_master_password {
            properties.insert(
                "ManageMasterPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manual_snapshot_retention_period {
            properties.insert(
                "ManualSnapshotRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.master_password_secret_kms_key_id {
            properties.insert(
                "MasterPasswordSecretKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
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
        if let Some(ref value) = self.multi_az {
            properties.insert(
                "MultiAZ".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.namespace_resource_policy {
            properties.insert(
                "NamespaceResourcePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NodeType".to_string(),
            crate::value::ToValue::to_value(&self.node_type),
        );
        if let Some(ref value) = self.number_of_nodes {
            properties.insert(
                "NumberOfNodes".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.owner_account {
            properties.insert(
                "OwnerAccount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
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
        if let Some(ref value) = self.resource_action {
            properties.insert(
                "ResourceAction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.revision_target {
            properties.insert(
                "RevisionTarget".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.rotate_encryption_key {
            properties.insert(
                "RotateEncryptionKey".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_cluster_identifier {
            properties.insert(
                "SnapshotClusterIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_copy_grant_name {
            properties.insert(
                "SnapshotCopyGrantName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_copy_manual {
            properties.insert(
                "SnapshotCopyManual".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_copy_retention_period {
            properties.insert(
                "SnapshotCopyRetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_identifier {
            properties.insert(
                "SnapshotIdentifier".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_security_group_ids {
            properties.insert(
                "VpcSecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clusterparametergroup.html>
pub struct ClusterParameterGroup_ {
    pub description: crate::value::ExpString,
    pub parameter_group_family: crate::value::ExpString,
    pub parameter_group_name: Option<crate::value::ExpString>,
    pub parameters: Option<Vec<super::redshift::clusterparametergroup::Parameter_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_ClusterParameterGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::ClusterParameterGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_ClusterParameterGroup as ClusterParameterGroup;
impl crate::template::ToResource for ClusterParameterGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ClusterParameterGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "ParameterGroupFamily".to_string(),
            crate::value::ToValue::to_value(&self.parameter_group_family),
        );
        if let Some(ref value) = self.parameter_group_name {
            properties.insert(
                "ParameterGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.parameters {
            properties.insert(
                "Parameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroup.html>
pub struct ClusterSecurityGroup_ {
    pub description: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_ClusterSecurityGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::ClusterSecurityGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_ClusterSecurityGroup as ClusterSecurityGroup;
impl crate::template::ToResource for ClusterSecurityGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ClusterSecurityGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersecuritygroupingress.html>
pub struct ClusterSecurityGroupIngress_ {
    pub cidrip: Option<crate::value::ExpString>,
    pub cluster_security_group_name: crate::value::ExpString,
    pub ec2_security_group_name: Option<crate::value::ExpString>,
    pub ec2_security_group_owner_id: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_ClusterSecurityGroupIngress {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::ClusterSecurityGroupIngress"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_ClusterSecurityGroupIngress as ClusterSecurityGroupIngress;
impl crate::template::ToResource for ClusterSecurityGroupIngress_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName(
                "ClusterSecurityGroupIngress",
            ),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.cidrip {
            properties.insert("CIDRIP".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "ClusterSecurityGroupName".to_string(),
            crate::value::ToValue::to_value(&self.cluster_security_group_name),
        );
        if let Some(ref value) = self.ec2_security_group_name {
            properties.insert(
                "EC2SecurityGroupName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.ec2_security_group_owner_id {
            properties.insert(
                "EC2SecurityGroupOwnerId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-clustersubnetgroup.html>
pub struct ClusterSubnetGroup_ {
    pub description: crate::value::ExpString,
    pub subnet_ids: Vec<crate::value::ExpString>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_ClusterSubnetGroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::ClusterSubnetGroup"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_ClusterSubnetGroup as ClusterSubnetGroup;
impl crate::template::ToResource for ClusterSubnetGroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ClusterSubnetGroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Description".to_string(),
            crate::value::ToValue::to_value(&self.description),
        );
        properties.insert(
            "SubnetIds".to_string(),
            crate::value::ToValue::to_value(&self.subnet_ids),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointaccess.html>
pub struct EndpointAccess_ {
    pub cluster_identifier: crate::value::ExpString,
    pub endpoint_name: crate::value::ExpString,
    pub resource_owner: Option<crate::value::ExpString>,
    pub subnet_group_name: crate::value::ExpString,
    pub vpc_security_group_ids: Vec<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_EndpointAccess {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::EndpointAccess"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_EndpointAccess as EndpointAccess;
impl crate::template::ToResource for EndpointAccess_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EndpointAccess"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "ClusterIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.cluster_identifier),
        );
        properties.insert(
            "EndpointName".to_string(),
            crate::value::ToValue::to_value(&self.endpoint_name),
        );
        if let Some(ref value) = self.resource_owner {
            properties.insert(
                "ResourceOwner".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubnetGroupName".to_string(),
            crate::value::ToValue::to_value(&self.subnet_group_name),
        );
        properties.insert(
            "VpcSecurityGroupIds".to_string(),
            crate::value::ToValue::to_value(&self.vpc_security_group_ids),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-endpointauthorization.html>
pub struct EndpointAuthorization_ {
    pub account: crate::value::ExpString,
    pub cluster_identifier: crate::value::ExpString,
    pub force: Option<crate::value::ExpBool>,
    pub vpc_ids: Option<Vec<crate::value::ExpString>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_EndpointAuthorization {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::EndpointAuthorization"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_EndpointAuthorization as EndpointAuthorization;
impl crate::template::ToResource for EndpointAuthorization_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EndpointAuthorization"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Account".to_string(),
            crate::value::ToValue::to_value(&self.account),
        );
        properties.insert(
            "ClusterIdentifier".to_string(),
            crate::value::ToValue::to_value(&self.cluster_identifier),
        );
        if let Some(ref value) = self.force {
            properties.insert("Force".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.vpc_ids {
            properties.insert("VpcIds".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-eventsubscription.html>
pub struct EventSubscription_ {
    pub enabled: Option<crate::value::ExpBool>,
    pub event_categories: Option<Vec<crate::value::ExpString>>,
    pub severity: Option<crate::value::ExpString>,
    pub sns_topic_arn: Option<crate::value::ExpString>,
    pub source_ids: Option<Vec<crate::value::ExpString>>,
    pub source_type: Option<crate::value::ExpString>,
    pub subscription_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_EventSubscription {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::EventSubscription"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_EventSubscription as EventSubscription;
impl crate::template::ToResource for EventSubscription_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("EventSubscription"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enabled {
            properties.insert(
                "Enabled".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.event_categories {
            properties.insert(
                "EventCategories".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.severity {
            properties.insert(
                "Severity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.sns_topic_arn {
            properties.insert(
                "SnsTopicArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_ids {
            properties.insert(
                "SourceIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.source_type {
            properties.insert(
                "SourceType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SubscriptionName".to_string(),
            crate::value::ToValue::to_value(&self.subscription_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-integration.html>
pub struct Integration_ {
    pub additional_encryption_context:
        Option<std::collections::BTreeMap<String, crate::value::ExpString>>,
    pub integration_name: Option<crate::value::ExpString>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub source_arn: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
    pub target_arn: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_Integration {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::Integration"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_Integration as Integration;
impl crate::template::ToResource for Integration_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Integration"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.additional_encryption_context {
            properties.insert(
                "AdditionalEncryptionContext".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.integration_name {
            properties.insert(
                "IntegrationName".to_string(),
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
            "SourceArn".to_string(),
            crate::value::ToValue::to_value(&self.source_arn),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties.insert(
            "TargetArn".to_string(),
            crate::value::ToValue::to_value(&self.target_arn),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshift-scheduledaction.html>
pub struct ScheduledAction_ {
    pub enable: Option<crate::value::ExpBool>,
    pub end_time: Option<crate::value::ExpString>,
    pub iam_role: Option<crate::value::ExpString>,
    pub schedule: Option<crate::value::ExpString>,
    pub scheduled_action_description: Option<crate::value::ExpString>,
    pub scheduled_action_name: crate::value::ExpString,
    pub start_time: Option<crate::value::ExpString>,
    pub target_action: Option<super::redshift::scheduledaction::ScheduledActionType_>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshift_ScheduledAction {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::Redshift::ScheduledAction"
        $($field $value)*)
    };
}
pub use crate::__aws_redshift_ScheduledAction as ScheduledAction;
impl crate::template::ToResource for ScheduledAction_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("Redshift"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("ScheduledAction"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.enable {
            properties.insert("Enable".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.end_time {
            properties.insert(
                "EndTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iam_role {
            properties.insert(
                "IamRole".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.schedule {
            properties.insert(
                "Schedule".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.scheduled_action_description {
            properties.insert(
                "ScheduledActionDescription".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "ScheduledActionName".to_string(),
            crate::value::ToValue::to_value(&self.scheduled_action_name),
        );
        if let Some(ref value) = self.start_time {
            properties.insert(
                "StartTime".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.target_action {
            properties.insert(
                "TargetAction".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
