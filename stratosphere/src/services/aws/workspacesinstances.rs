pub mod volume {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-volume-tagspecification.html>
    pub struct TagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_Volume_TagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::Volume.TagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_Volume_TagSpecification as TagSpecification;
    impl crate::value::ToValue for TagSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_type {
                properties.insert(
                    "ResourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
pub mod workspaceinstance {
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-blockdevicemapping.html>
    pub struct BlockDeviceMapping_ {
        pub device_name: Option<crate::value::ExpString>,
        pub ebs: Option<Box<EbsBlockDevice_>>,
        pub no_device: Option<crate::value::ExpString>,
        pub virtual_name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_BlockDeviceMapping {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.BlockDeviceMapping"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_BlockDeviceMapping as BlockDeviceMapping;
    impl crate::value::ToValue for BlockDeviceMapping_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.device_name {
                properties.insert(
                    "DeviceName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs {
                properties.insert("Ebs".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.no_device {
                properties.insert(
                    "NoDevice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.virtual_name {
                properties.insert(
                    "VirtualName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-capacityreservationspecification.html>
    pub struct CapacityReservationSpecification_ {
        pub capacity_reservation_preference: Option<crate::value::ExpString>,
        pub capacity_reservation_target: Option<Box<CapacityReservationTarget_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_CapacityReservationSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.CapacityReservationSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_CapacityReservationSpecification as CapacityReservationSpecification;
    impl crate::value::ToValue for CapacityReservationSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_reservation_preference {
                properties.insert(
                    "CapacityReservationPreference".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_target {
                properties.insert(
                    "CapacityReservationTarget".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-capacityreservationtarget.html>
    pub struct CapacityReservationTarget_ {
        pub capacity_reservation_id: Option<crate::value::ExpString>,
        pub capacity_reservation_resource_group_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_CapacityReservationTarget {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.CapacityReservationTarget"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_CapacityReservationTarget as CapacityReservationTarget;
    impl crate::value::ToValue for CapacityReservationTarget_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.capacity_reservation_id {
                properties.insert(
                    "CapacityReservationId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_resource_group_arn {
                properties.insert(
                    "CapacityReservationResourceGroupArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-cpuoptionsrequest.html>
    pub struct CpuOptionsRequest_ {
        pub core_count: Option<i32>,
        pub threads_per_core: Option<i32>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_CpuOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.CpuOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_CpuOptionsRequest as CpuOptionsRequest;
    impl crate::value::ToValue for CpuOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.core_count {
                properties.insert(
                    "CoreCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.threads_per_core {
                properties.insert(
                    "ThreadsPerCore".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-creditspecificationrequest.html>
    pub struct CreditSpecificationRequest_ {
        pub cpu_credits: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_CreditSpecificationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.CreditSpecificationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_CreditSpecificationRequest as CreditSpecificationRequest;
    impl crate::value::ToValue for CreditSpecificationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.cpu_credits {
                properties.insert(
                    "CpuCredits".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-ec2managedinstance.html>
    pub struct EC2ManagedInstance_ {
        pub instance_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_EC2ManagedInstance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.EC2ManagedInstance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_EC2ManagedInstance as EC2ManagedInstance;
    impl crate::value::ToValue for EC2ManagedInstance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_id {
                properties.insert(
                    "InstanceId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-ebsblockdevice.html>
    pub struct EbsBlockDevice_ {
        pub encrypted: Option<crate::value::ExpBool>,
        pub iops: Option<i32>,
        pub kms_key_id: Option<crate::value::ExpString>,
        pub throughput: Option<i32>,
        pub volume_size: Option<i32>,
        pub volume_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_EbsBlockDevice {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.EbsBlockDevice"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_EbsBlockDevice as EbsBlockDevice;
    impl crate::value::ToValue for EbsBlockDevice_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.encrypted {
                properties.insert(
                    "Encrypted".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iops {
                properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.kms_key_id {
                properties.insert(
                    "KmsKeyId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.throughput {
                properties.insert(
                    "Throughput".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_size {
                properties.insert(
                    "VolumeSize".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.volume_type {
                properties.insert(
                    "VolumeType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-enclaveoptionsrequest.html>
    pub struct EnclaveOptionsRequest_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_EnclaveOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.EnclaveOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_EnclaveOptionsRequest as EnclaveOptionsRequest;
    impl crate::value::ToValue for EnclaveOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-hibernationoptionsrequest.html>
    pub struct HibernationOptionsRequest_ {
        pub configured: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_HibernationOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.HibernationOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_HibernationOptionsRequest as HibernationOptionsRequest;
    impl crate::value::ToValue for HibernationOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.configured {
                properties.insert(
                    "Configured".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-iaminstanceprofilespecification.html>
    pub struct IamInstanceProfileSpecification_ {
        pub arn: Option<crate::value::ExpString>,
        pub name: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_IamInstanceProfileSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.IamInstanceProfileSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_IamInstanceProfileSpecification as IamInstanceProfileSpecification;
    impl crate::value::ToValue for IamInstanceProfileSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.arn {
                properties.insert("Arn".to_string(), crate::value::ToValue::to_value(value));
            }
            if let Some(ref value) = self.name {
                properties.insert("Name".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-instancemaintenanceoptionsrequest.html>
    pub struct InstanceMaintenanceOptionsRequest_ {
        pub auto_recovery: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_InstanceMaintenanceOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.InstanceMaintenanceOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_InstanceMaintenanceOptionsRequest as InstanceMaintenanceOptionsRequest;
    impl crate::value::ToValue for InstanceMaintenanceOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.auto_recovery {
                properties.insert(
                    "AutoRecovery".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-instancemarketoptionsrequest.html>
    pub struct InstanceMarketOptionsRequest_ {
        pub market_type: Option<crate::value::ExpString>,
        pub spot_options: Option<Box<SpotMarketOptions_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_InstanceMarketOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.InstanceMarketOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_InstanceMarketOptionsRequest as InstanceMarketOptionsRequest;
    impl crate::value::ToValue for InstanceMarketOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.market_type {
                properties.insert(
                    "MarketType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_options {
                properties.insert(
                    "SpotOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-instancemetadataoptionsrequest.html>
    pub struct InstanceMetadataOptionsRequest_ {
        pub http_endpoint: Option<crate::value::ExpString>,
        pub http_protocol_ipv6: Option<crate::value::ExpString>,
        pub http_put_response_hop_limit: Option<i32>,
        pub http_tokens: Option<crate::value::ExpString>,
        pub instance_metadata_tags: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_InstanceMetadataOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.InstanceMetadataOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_InstanceMetadataOptionsRequest as InstanceMetadataOptionsRequest;
    impl crate::value::ToValue for InstanceMetadataOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.http_endpoint {
                properties.insert(
                    "HttpEndpoint".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_protocol_ipv6 {
                properties.insert(
                    "HttpProtocolIpv6".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_put_response_hop_limit {
                properties.insert(
                    "HttpPutResponseHopLimit".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.http_tokens {
                properties.insert(
                    "HttpTokens".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.instance_metadata_tags {
                properties.insert(
                    "InstanceMetadataTags".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-instancenetworkinterfacespecification.html>
    pub struct InstanceNetworkInterfaceSpecification_ {
        pub description: Option<crate::value::ExpString>,
        pub device_index: Option<i32>,
        pub groups: Option<Vec<crate::value::ExpString>>,
        pub subnet_id: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_InstanceNetworkInterfaceSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.InstanceNetworkInterfaceSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_InstanceNetworkInterfaceSpecification as InstanceNetworkInterfaceSpecification;
    impl crate::value::ToValue for InstanceNetworkInterfaceSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.description {
                properties.insert(
                    "Description".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.device_index {
                properties.insert(
                    "DeviceIndex".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.groups {
                properties.insert("Groups".to_string(), crate::value::ToValue::to_value(value));
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
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-instancenetworkperformanceoptionsrequest.html>
    pub struct InstanceNetworkPerformanceOptionsRequest_ {
        pub bandwidth_weighting: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_InstanceNetworkPerformanceOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.InstanceNetworkPerformanceOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_InstanceNetworkPerformanceOptionsRequest as InstanceNetworkPerformanceOptionsRequest;
    impl crate::value::ToValue for InstanceNetworkPerformanceOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.bandwidth_weighting {
                properties.insert(
                    "BandwidthWeighting".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-licenseconfigurationrequest.html>
    pub struct LicenseConfigurationRequest_ {
        pub license_configuration_arn: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_LicenseConfigurationRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.LicenseConfigurationRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_LicenseConfigurationRequest as LicenseConfigurationRequest;
    impl crate::value::ToValue for LicenseConfigurationRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.license_configuration_arn {
                properties.insert(
                    "LicenseConfigurationArn".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-managedinstance.html>
    pub struct ManagedInstance_ {
        pub block_device_mappings: Option<Vec<BlockDeviceMapping_>>,
        pub capacity_reservation_specification: Option<Box<CapacityReservationSpecification_>>,
        pub cpu_options: Option<Box<CpuOptionsRequest_>>,
        pub credit_specification: Option<Box<CreditSpecificationRequest_>>,
        pub disable_api_stop: Option<crate::value::ExpBool>,
        pub ebs_optimized: Option<crate::value::ExpBool>,
        pub enable_primary_ipv6: Option<crate::value::ExpBool>,
        pub enclave_options: Option<Box<EnclaveOptionsRequest_>>,
        pub hibernation_options: Option<Box<HibernationOptionsRequest_>>,
        pub iam_instance_profile: Option<Box<IamInstanceProfileSpecification_>>,
        pub image_id: crate::value::ExpString,
        pub instance_market_options: Option<Box<InstanceMarketOptionsRequest_>>,
        pub instance_type: crate::value::ExpString,
        pub ipv6_address_count: Option<i32>,
        pub key_name: Option<crate::value::ExpString>,
        pub license_specifications: Option<Vec<LicenseConfigurationRequest_>>,
        pub maintenance_options: Option<Box<InstanceMaintenanceOptionsRequest_>>,
        pub metadata_options: Option<Box<InstanceMetadataOptionsRequest_>>,
        pub monitoring: Option<Box<RunInstancesMonitoringEnabled_>>,
        pub network_interfaces: Option<Vec<InstanceNetworkInterfaceSpecification_>>,
        pub network_performance_options: Option<Box<InstanceNetworkPerformanceOptionsRequest_>>,
        pub placement: Option<Box<Placement_>>,
        pub private_dns_name_options: Option<Box<PrivateDnsNameOptionsRequest_>>,
        pub subnet_id: Option<crate::value::ExpString>,
        pub tag_specifications: Option<Vec<TagSpecification_>>,
        pub user_data: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_ManagedInstance {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.ManagedInstance"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_ManagedInstance as ManagedInstance;
    impl crate::value::ToValue for ManagedInstance_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.block_device_mappings {
                properties.insert(
                    "BlockDeviceMappings".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.capacity_reservation_specification {
                properties.insert(
                    "CapacityReservationSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.cpu_options {
                properties.insert(
                    "CpuOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.credit_specification {
                properties.insert(
                    "CreditSpecification".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.disable_api_stop {
                properties.insert(
                    "DisableApiStop".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.ebs_optimized {
                properties.insert(
                    "EbsOptimized".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_primary_ipv6 {
                properties.insert(
                    "EnablePrimaryIpv6".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enclave_options {
                properties.insert(
                    "EnclaveOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hibernation_options {
                properties.insert(
                    "HibernationOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.iam_instance_profile {
                properties.insert(
                    "IamInstanceProfile".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "ImageId".to_string(),
                crate::value::ToValue::to_value(&self.image_id),
            );
            if let Some(ref value) = self.instance_market_options {
                properties.insert(
                    "InstanceMarketOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.insert(
                "InstanceType".to_string(),
                crate::value::ToValue::to_value(&self.instance_type),
            );
            if let Some(ref value) = self.ipv6_address_count {
                properties.insert(
                    "Ipv6AddressCount".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.key_name {
                properties.insert(
                    "KeyName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.license_specifications {
                properties.insert(
                    "LicenseSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.maintenance_options {
                properties.insert(
                    "MaintenanceOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.metadata_options {
                properties.insert(
                    "MetadataOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.monitoring {
                properties.insert(
                    "Monitoring".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_interfaces {
                properties.insert(
                    "NetworkInterfaces".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.network_performance_options {
                properties.insert(
                    "NetworkPerformanceOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.placement {
                properties.insert(
                    "Placement".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.private_dns_name_options {
                properties.insert(
                    "PrivateDnsNameOptions".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.subnet_id {
                properties.insert(
                    "SubnetId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tag_specifications {
                properties.insert(
                    "TagSpecifications".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.user_data {
                properties.insert(
                    "UserData".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-placement.html>
    pub struct Placement_ {
        pub availability_zone: Option<crate::value::ExpString>,
        pub group_id: Option<crate::value::ExpString>,
        pub group_name: Option<crate::value::ExpString>,
        pub partition_number: Option<i32>,
        pub tenancy: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_Placement {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.Placement"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_Placement as Placement;
    impl crate::value::ToValue for Placement_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.availability_zone {
                properties.insert(
                    "AvailabilityZone".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_id {
                properties.insert(
                    "GroupId".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.group_name {
                properties.insert(
                    "GroupName".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.partition_number {
                properties.insert(
                    "PartitionNumber".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tenancy {
                properties.insert(
                    "Tenancy".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-privatednsnameoptionsrequest.html>
    pub struct PrivateDnsNameOptionsRequest_ {
        pub enable_resource_name_dns_aaaa_record: Option<crate::value::ExpBool>,
        pub enable_resource_name_dns_a_record: Option<crate::value::ExpBool>,
        pub hostname_type: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_PrivateDnsNameOptionsRequest {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.PrivateDnsNameOptionsRequest"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_PrivateDnsNameOptionsRequest as PrivateDnsNameOptionsRequest;
    impl crate::value::ToValue for PrivateDnsNameOptionsRequest_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enable_resource_name_dns_aaaa_record {
                properties.insert(
                    "EnableResourceNameDnsAAAARecord".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.enable_resource_name_dns_a_record {
                properties.insert(
                    "EnableResourceNameDnsARecord".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.hostname_type {
                properties.insert(
                    "HostnameType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-runinstancesmonitoringenabled.html>
    pub struct RunInstancesMonitoringEnabled_ {
        pub enabled: Option<crate::value::ExpBool>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_RunInstancesMonitoringEnabled {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.RunInstancesMonitoringEnabled"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_RunInstancesMonitoringEnabled as RunInstancesMonitoringEnabled;
    impl crate::value::ToValue for RunInstancesMonitoringEnabled_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.enabled {
                properties.insert(
                    "Enabled".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-spotmarketoptions.html>
    pub struct SpotMarketOptions_ {
        pub instance_interruption_behavior: Option<crate::value::ExpString>,
        pub max_price: Option<crate::value::ExpString>,
        pub spot_instance_type: Option<crate::value::ExpString>,
        pub valid_until_utc: Option<crate::value::ExpString>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_SpotMarketOptions {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.SpotMarketOptions"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_SpotMarketOptions as SpotMarketOptions;
    impl crate::value::ToValue for SpotMarketOptions_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.instance_interruption_behavior {
                properties.insert(
                    "InstanceInterruptionBehavior".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.max_price {
                properties.insert(
                    "MaxPrice".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.spot_instance_type {
                properties.insert(
                    "SpotInstanceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.valid_until_utc {
                properties.insert(
                    "ValidUntilUtc".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            properties.into()
        }
    }
    ///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-properties-workspacesinstances-workspaceinstance-tagspecification.html>
    pub struct TagSpecification_ {
        pub resource_type: Option<crate::value::ExpString>,
        pub tags: Option<Vec<crate::Tag_>>,
    }
    #[doc(hidden)]
    #[macro_export]
    macro_rules! __aws_workspacesinstances_WorkspaceInstance_TagSpecification {
        ($($field:ident : $value:expr),* $(,)?) => {
            stratosphere::generator::construct_property_type!("AWS::WorkspacesInstances::WorkspaceInstance.TagSpecification"
            $($field $value)*)
        };
    }
    pub use crate::__aws_workspacesinstances_WorkspaceInstance_TagSpecification as TagSpecification;
    impl crate::value::ToValue for TagSpecification_ {
        fn to_value(&self) -> serde_json::Value {
            let mut properties = serde_json::Map::new();
            if let Some(ref value) = self.resource_type {
                properties.insert(
                    "ResourceType".to_string(),
                    crate::value::ToValue::to_value(value),
                );
            }
            if let Some(ref value) = self.tags {
                properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
            }
            properties.into()
        }
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesinstances-volume.html>
pub struct Volume_ {
    pub availability_zone: crate::value::ExpString,
    pub encrypted: Option<crate::value::ExpBool>,
    pub iops: Option<i32>,
    pub kms_key_id: Option<crate::value::ExpString>,
    pub size_in_gb: Option<i32>,
    pub snapshot_id: Option<crate::value::ExpString>,
    pub tag_specifications: Option<Vec<super::workspacesinstances::volume::TagSpecification_>>,
    pub throughput: Option<i32>,
    pub volume_type: Option<crate::value::ExpString>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesinstances_Volume {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkspacesInstances::Volume"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesinstances_Volume as Volume;
impl crate::template::ToResource for Volume_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkspacesInstances"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("Volume"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "AvailabilityZone".to_string(),
            crate::value::ToValue::to_value(&self.availability_zone),
        );
        if let Some(ref value) = self.encrypted {
            properties.insert(
                "Encrypted".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.iops {
            properties.insert("Iops".to_string(), crate::value::ToValue::to_value(value));
        }
        if let Some(ref value) = self.kms_key_id {
            properties.insert(
                "KmsKeyId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.size_in_gb {
            properties.insert(
                "SizeInGB".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.snapshot_id {
            properties.insert(
                "SnapshotId".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tag_specifications {
            properties.insert(
                "TagSpecifications".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.throughput {
            properties.insert(
                "Throughput".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.volume_type {
            properties.insert(
                "VolumeType".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesinstances-volumeassociation.html>
pub struct VolumeAssociation_ {
    pub device: crate::value::ExpString,
    pub disassociate_mode: Option<crate::value::ExpString>,
    pub volume_id: crate::value::ExpString,
    pub workspace_instance_id: crate::value::ExpString,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesinstances_VolumeAssociation {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkspacesInstances::VolumeAssociation"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesinstances_VolumeAssociation as VolumeAssociation;
impl crate::template::ToResource for VolumeAssociation_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkspacesInstances"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("VolumeAssociation"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        properties.insert(
            "Device".to_string(),
            crate::value::ToValue::to_value(&self.device),
        );
        if let Some(ref value) = self.disassociate_mode {
            properties.insert(
                "DisassociateMode".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        properties.insert(
            "VolumeId".to_string(),
            crate::value::ToValue::to_value(&self.volume_id),
        );
        properties.insert(
            "WorkspaceInstanceId".to_string(),
            crate::value::ToValue::to_value(&self.workspace_instance_id),
        );
        properties
    }
}
///<http://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/aws-resource-workspacesinstances-workspaceinstance.html>
pub struct WorkspaceInstance_ {
    pub managed_instance: Option<super::workspacesinstances::workspaceinstance::ManagedInstance_>,
    pub tags: Option<Vec<crate::Tag_>>,
}
#[doc(hidden)]
#[macro_export]
macro_rules! __aws_workspacesinstances_WorkspaceInstance {
    ($($field:ident : $value:expr),* $(,)?) => {
        stratosphere::generator::construct_resource_type!("AWS::WorkspacesInstances::WorkspaceInstance"
        $($field $value)*)
    };
}
pub use crate::__aws_workspacesinstances_WorkspaceInstance as WorkspaceInstance;
impl crate::template::ToResource for WorkspaceInstance_ {
    const RESOURCE_TYPE_NAME: crate::resource_specification::ResourceTypeName<'static> =
        crate::resource_specification::ResourceTypeName {
            service: crate::resource_specification::ServiceIdentifier {
                service_name: crate::resource_specification::ServiceName("WorkspacesInstances"),
                vendor_name: crate::resource_specification::VendorName("AWS"),
            },
            resource_name: crate::resource_specification::ResourceName("WorkspaceInstance"),
        };
    fn to_resource_properties(&self) -> crate::template::ResourceProperties {
        let mut properties = crate::template::ResourceProperties::new();
        if let Some(ref value) = self.managed_instance {
            properties.insert(
                "ManagedInstance".to_string(),
                crate::value::ToValue::to_value(value),
            );
        }
        if let Some(ref value) = self.tags {
            properties.insert("Tags".to_string(), crate::value::ToValue::to_value(value));
        }
        properties
    }
}
