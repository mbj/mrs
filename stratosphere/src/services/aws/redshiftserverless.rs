pub mod namespace {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-namespace.html>
    pub struct Namespace_ {
        pub admin_password_secret_arn: Option<crate::value::ExpString>,
        pub admin_password_secret_kms_key_id: Option<crate::value::ExpString>,
        pub admin_username: Option<crate::value::ExpString>,
        pub creation_date: Option<crate::value::ExpString>,
        pub db_name: Option<crate::value::ExpString>,
        pub default_iam_role_arn: Option<crate::value::ExpString>,
        pub iam_roles: Option<Vec<crate::value::ExpString>>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub log_exports: Option<Vec<crate::value::ExpString>>,
        pub namespace_arn: Option<crate::value::ExpString>,
        pub namespace_id: Option<crate::value::ExpString>,
        pub namespace_name: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Namespace_Namespace {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Namespace.Namespace"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Namespace_Namespace as Namespace;
    impl crate::value::ToValue for Namespace_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.admin_password_secret_arn {
                properties.insert(
                    "AdminPasswordSecretArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.admin_password_secret_kms_key_id {
                properties.insert(
                    "AdminPasswordSecretKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.admin_username {
                properties.insert(
                    "AdminUsername".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.creation_date {
                properties.insert(
                    "CreationDate".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.db_name {
                properties.insert("DbName".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.default_iam_role_arn {
                properties.insert(
                    "DefaultIamRoleArn".to_string(),
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
            if let Some(ref value) = self.log_exports {
                properties.insert(
                    "LogExports".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace_arn {
                properties.insert(
                    "NamespaceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace_id {
                properties.insert(
                    "NamespaceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace_name {
                properties.insert(
                    "NamespaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-namespace-snapshotcopyconfiguration.html>
    pub struct SnapshotCopyConfiguration_ {
        pub destination_kms_key_id: Option<crate::value::ExpString>,
        pub destination_region: crate::value::ExpString,
        pub snapshot_retention_period: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Namespace_SnapshotCopyConfiguration {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Namespace.SnapshotCopyConfiguration"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Namespace_SnapshotCopyConfiguration as SnapshotCopyConfiguration;
    impl crate::value::ToValue for SnapshotCopyConfiguration_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.destination_kms_key_id {
                properties.insert(
                    "DestinationKmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "DestinationRegion".to_string(),
                crate::value::ToValue::to_value(&self.destination_region),
            );
            if let Some(ref value) = self.snapshot_retention_period {
                properties.insert(
                    "SnapshotRetentionPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
pub mod snapshot {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-snapshot-snapshot.html>
    pub struct Snapshot_ {
        pub admin_username: Option<crate::value::ExpString>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub namespace_arn: Option<crate::value::ExpString>,
        pub namespace_name: Option<crate::value::ExpString>,
        pub owner_account: Option<crate::value::ExpString>,
        pub retention_period: Option<i32>,
        pub snapshot_arn: Option<crate::value::ExpString>,
        pub snapshot_create_time: Option<crate::value::ExpString>,
        pub snapshot_name: Option<crate::value::ExpString>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Snapshot_Snapshot {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Snapshot.Snapshot"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Snapshot_Snapshot as Snapshot;
    impl crate::value::ToValue for Snapshot_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.admin_username {
                properties.insert(
                    "AdminUsername".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace_arn {
                properties.insert(
                    "NamespaceArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace_name {
                properties.insert(
                    "NamespaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.owner_account {
                properties.insert(
                    "OwnerAccount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.retention_period {
                properties.insert(
                    "RetentionPeriod".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshot_arn {
                properties.insert(
                    "SnapshotArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshot_create_time {
                properties.insert(
                    "SnapshotCreateTime".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.snapshot_name {
                properties.insert(
                    "SnapshotName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod workgroup {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-configparameter.html>
    pub struct ConfigParameter_ {
        pub parameter_key: Option<crate::value::ExpString>,
        pub parameter_value: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Workgroup_ConfigParameter {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Workgroup.ConfigParameter"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Workgroup_ConfigParameter as ConfigParameter;
    impl crate::value::ToValue for ConfigParameter_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.parameter_key {
                properties.insert(
                    "ParameterKey".to_string(),
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-endpoint.html>
    pub struct Endpoint_ {
        pub address: Option<crate::value::ExpString>,
        pub port: Option<i32>,
        pub vpc_endpoints: Option<Vec<VpcEndpoint_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Workgroup_Endpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Workgroup.Endpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Workgroup_Endpoint as Endpoint;
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
            if let Some(ref value) = self.vpc_endpoints {
                properties.insert(
                    "VpcEndpoints".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-networkinterface.html>
    pub struct NetworkInterface_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub network_interface_id: Option<crate::value::ExpString>,
        pub private_ip_address: Option<crate::value::ExpString>,
        pub subnet_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Workgroup_NetworkInterface {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Workgroup.NetworkInterface"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Workgroup_NetworkInterface as NetworkInterface;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-performancetarget.html>
    pub struct PerformanceTarget_ {
        pub level: Option<i32>,
        pub status: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Workgroup_PerformanceTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Workgroup.PerformanceTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Workgroup_PerformanceTarget as PerformanceTarget;
    impl crate::value::ToValue for PerformanceTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.level {
                properties.insert("Level".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-vpcendpoint.html>
    pub struct VpcEndpoint_ {
        pub network_interfaces: Option<Vec<NetworkInterface_>>,
        pub vpc_endpoint_id: Option<crate::value::ExpString>,
        pub vpc_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Workgroup_VpcEndpoint {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Workgroup.VpcEndpoint"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Workgroup_VpcEndpoint as VpcEndpoint;
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-redshiftserverless-workgroup-workgroup.html>
    pub struct Workgroup_ {
        pub base_capacity: Option<i32>,
        pub config_parameters: Option<Vec<ConfigParameter_>>,
        pub creation_date: Option<crate::value::ExpString>,
        pub endpoint: Option<Box<Endpoint_>>,
        pub enhanced_vpc_routing: Option<crate::value::ExpBool>,
        pub max_capacity: Option<i32>,
        pub namespace_name: Option<crate::value::ExpString>,
        pub price_performance_target: Option<Box<PerformanceTarget_>>,
        pub publicly_accessible: Option<crate::value::ExpBool>,
        pub security_group_ids: Option<Vec<crate::value::ExpString>>,
        pub status: Option<crate::value::ExpString>,
        pub subnet_ids: Option<Vec<crate::value::ExpString>>,
        pub track_name: Option<crate::value::ExpString>,
        pub workgroup_arn: Option<crate::value::ExpString>,
        pub workgroup_id: Option<crate::value::ExpString>,
        pub workgroup_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_redshiftserverless_Workgroup_Workgroup {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::RedshiftServerless::Workgroup.Workgroup"
            $($field $value)*)
        };
    }
    pub use crate::__aws_redshiftserverless_Workgroup_Workgroup as Workgroup;
    impl crate::value::ToValue for Workgroup_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.base_capacity {
                properties.insert(
                    "BaseCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.config_parameters {
                properties.insert(
                    "ConfigParameters".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.creation_date {
                properties.insert(
                    "CreationDate".to_string(),
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
            if let Some(ref value) = self.max_capacity {
                properties.insert(
                    "MaxCapacity".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.namespace_name {
                properties.insert(
                    "NamespaceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.price_performance_target {
                properties.insert(
                    "PricePerformanceTarget".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.publicly_accessible {
                properties.insert(
                    "PubliclyAccessible".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.security_group_ids {
                properties.insert(
                    "SecurityGroupIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.status {
                properties.insert("Status".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.subnet_ids {
                properties.insert(
                    "SubnetIds".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.track_name {
                properties.insert(
                    "TrackName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workgroup_arn {
                properties.insert(
                    "WorkgroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workgroup_id {
                properties.insert(
                    "WorkgroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.workgroup_name {
                properties.insert(
                    "WorkgroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-namespace.html>
pub struct Namespace_ {
    pub admin_password_secret_kms_key_id: Option<crate::value::ExpString>,
    pub admin_user_password: Option<crate::value::ExpString>,
    pub admin_username: Option<crate::value::ExpString>,
    pub db_name: Option<crate::value::ExpString>,
    pub default_iam_role_arn: Option<crate::value::ExpString>,
    pub final_snapshot_name: Option<crate::value::ExpString>,
    pub final_snapshot_retention_period: Option<i32>,
    pub iam_roles: Option<Vec<crate::value::ExpString>>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub log_exports: Option<Vec<crate::value::ExpString>>,
    pub manage_admin_password: Option<crate::value::ExpBool>,
    pub namespace_name: crate::value::ExpString,
    pub namespace_resource_policy: Option<serde_json::Value>,
    pub redshift_idc_application_arn: Option<crate::value::ExpString>,
    pub snapshot_copy_configurations:
        Option<Vec<super::redshiftserverless::namespace::SnapshotCopyConfiguration_>>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshiftserverless_Namespace {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RedshiftServerless::Namespace"
        $($field $value)*)
    };
}
pub use crate::__aws_redshiftserverless_Namespace as Namespace;
impl crate::template::ToResource for Namespace_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RedshiftServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Namespace"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.admin_password_secret_kms_key_id {
            properties.insert(
                "AdminPasswordSecretKmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.admin_user_password {
            properties.insert(
                "AdminUserPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.admin_username {
            properties.insert(
                "AdminUsername".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.db_name {
            properties.insert("DbName".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.default_iam_role_arn {
            properties.insert(
                "DefaultIamRoleArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.final_snapshot_name {
            properties.insert(
                "FinalSnapshotName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.final_snapshot_retention_period {
            properties.insert(
                "FinalSnapshotRetentionPeriod".to_string(),
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
        if let Some(ref value) = self.log_exports {
            properties.insert(
                "LogExports".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.manage_admin_password {
            properties.insert(
                "ManageAdminPassword".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "NamespaceName".to_string(),
            crate::value::ToValue::to_value(&self.namespace_name),
        );
        if let Some(ref value) = self.namespace_resource_policy {
            properties.insert(
                "NamespaceResourcePolicy".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.redshift_idc_application_arn {
            properties.insert(
                "RedshiftIdcApplicationArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_copy_configurations {
            properties.insert(
                "SnapshotCopyConfigurations".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-snapshot.html>
pub struct Snapshot_ {
    pub namespace_name: Option<crate::value::ExpString>,
    pub retention_period: Option<i32>,
    pub snapshot_name: crate::value::ExpString,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshiftserverless_Snapshot {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RedshiftServerless::Snapshot"
        $($field $value)*)
    };
}
pub use crate::__aws_redshiftserverless_Snapshot as Snapshot;
impl crate::template::ToResource for Snapshot_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RedshiftServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Snapshot"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.namespace_name {
            properties.insert(
                "NamespaceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.retention_period {
            properties.insert(
                "RetentionPeriod".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "SnapshotName".to_string(),
            crate::value::ToValue::to_value(&self.snapshot_name),
        );
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-redshiftserverless-workgroup.html>
pub struct Workgroup_ {
    pub base_capacity: Option<i32>,
    pub config_parameters: Option<Vec<super::redshiftserverless::workgroup::ConfigParameter_>>,
    pub enhanced_vpc_routing: Option<crate::value::ExpBool>,
    pub max_capacity: Option<i32>,
    pub namespace_name: Option<crate::value::ExpString>,
    pub port: Option<i32>,
    pub price_performance_target: Option<super::redshiftserverless::workgroup::PerformanceTarget_>,
    pub publicly_accessible: Option<crate::value::ExpBool>,
    pub recovery_point_id: Option<crate::value::ExpString>,
    pub security_group_ids: Option<Vec<crate::value::ExpString>>,
    pub snapshot_arn: Option<crate::value::ExpString>,
    pub snapshot_name: Option<crate::value::ExpString>,
    pub snapshot_owner_account: Option<crate::value::ExpString>,
    pub subnet_ids: Option<Vec<crate::value::ExpString>>,
    pub tags: Option<Vec<crate::Tag_>>,
    pub track_name: Option<crate::value::ExpString>,
    pub workgroup: Option<super::redshiftserverless::workgroup::Workgroup_>,
    pub workgroup_name: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_redshiftserverless_Workgroup {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::RedshiftServerless::Workgroup"
        $($field $value)*)
    };
}
pub use crate::__aws_redshiftserverless_Workgroup as Workgroup;
impl crate::template::ToResource for Workgroup_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("RedshiftServerless"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Workgroup"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.base_capacity {
            properties.insert(
                "BaseCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.config_parameters {
            properties.insert(
                "ConfigParameters".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.enhanced_vpc_routing {
            properties.insert(
                "EnhancedVpcRouting".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.max_capacity {
            properties.insert(
                "MaxCapacity".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.namespace_name {
            properties.insert(
                "NamespaceName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.port {
            properties.insert("Port".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.price_performance_target {
            properties.insert(
                "PricePerformanceTarget".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.publicly_accessible {
            properties.insert(
                "PubliclyAccessible".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.recovery_point_id {
            properties.insert(
                "RecoveryPointId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.security_group_ids {
            properties.insert(
                "SecurityGroupIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_arn {
            properties.insert(
                "SnapshotArn".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_name {
            properties.insert(
                "SnapshotName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_owner_account {
            properties.insert(
                "SnapshotOwnerAccount".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.subnet_ids {
            properties.insert(
                "SubnetIds".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.track_name {
            properties.insert(
                "TrackName".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.workgroup {
            properties.insert(
                "Workgroup".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "WorkgroupName".to_string(),
            crate::value::ToValue::to_value(&self.workgroup_name),
        );
        properties
    }
}
